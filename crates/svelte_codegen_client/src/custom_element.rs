use std::iter;

use oxc_ast::ast::{BinaryOperator, Expression, ObjectPropertyKind, PropertyKey, Statement};
use svelte_ast::CustomElementConfig;
use svelte_parser::{CeDomMode, CePropConfig, ParsedCeConfig};

use crate::context::Ctx;
use svelte_ast_builder::{Arg, ObjProp};

pub fn gen_custom_element<'a>(
    ctx: &mut Ctx<'a>,
    ce_config: Option<&CustomElementConfig>,
) -> Vec<Statement<'a>> {
    let parsed_config = ctx.ce_config().cloned();

    let (simple_tag, parsed) = match ce_config {
        Some(CustomElementConfig::Tag(tag)) => (Some(tag.as_str()), None),
        Some(CustomElementConfig::Expression(_)) => (None, parsed_config.as_ref()),
        None => (None, None),
    };

    let resolved_tag: Option<&str> = match (&simple_tag, &parsed) {
        (Some(t), _) => Some(t),
        (None, Some(opts)) => opts.tag.as_deref(),
        (None, None) => None,
    };

    let props_obj = build_props_metadata(ctx, parsed);

    let slots = ctx.b.array_from_args(
        ctx.query
            .custom_element_slot_names()
            .iter()
            .map(|name| Arg::StrRef(name.as_str())),
    );

    let accessors = ctx.b.array_from_args(ctx.query.exports().iter().map(|e| {
        let name = e
            .alias
            .as_deref()
            .unwrap_or_else(|| ctx.query.symbol_name(e.local));
        Arg::StrRef(name)
    }));

    let shadow = parsed.map_or(CeDomMode::Open, |o| o.shadow);

    let (shadow_expr, extend_arg) =
        take_ce_source_exprs(ctx, ce_config, shadow == CeDomMode::Custom);
    let hmr = ctx.state.hmr;
    let b = &ctx.b;

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Ident(ctx.state.name),
        Arg::Expr(props_obj),
        Arg::Expr(slots),
        Arg::Expr(accessors),
    ];

    let shadow_arg = match shadow {
        CeDomMode::None => None,
        CeDomMode::Open => Some(b.object_expr(vec![ObjProp::KeyValue("mode", b.str_expr("open"))])),
        CeDomMode::Custom => shadow_expr,
    };

    match shadow_arg {
        Some(arg) => args.push(Arg::Expr(arg)),
        None if extend_arg.is_some() => args.push(Arg::Expr(b.void_zero_expr())),
        None => {}
    }

    if let Some(extend_expr) = extend_arg {
        args.push(Arg::Expr(extend_expr));
    }

    let create_ce = b.call_expr("$.create_custom_element", args);

    let mut stmts = Vec::new();
    if let Some(tag_str) = resolved_tag {
        let define_callee = b.static_member_expr(b.rid_expr("customElements"), "define");
        let define_call =
            b.call_expr_callee(define_callee, [Arg::StrRef(tag_str), Arg::Expr(create_ce)]);
        let define_stmt = b.expr_stmt(define_call);
        if hmr {
            let get_call = b.call_expr("customElements.get", [Arg::StrRef(tag_str)]);
            let test = b.binary_expr(BinaryOperator::Equality, get_call, b.null_expr());
            stmts.push(b.if_stmt(test, define_stmt, None));
        } else {
            stmts.push(define_stmt);
        }
    } else {
        stmts.push(b.expr_stmt(create_ce));
    }

    stmts
}

fn take_ce_source_exprs<'a>(
    ctx: &mut Ctx<'a>,
    ce_config: Option<&CustomElementConfig>,
    want_shadow: bool,
) -> (Option<Expression<'a>>, Option<Expression<'a>>) {
    let Some(CustomElementConfig::Expression(span)) = ce_config else {
        return (None, None);
    };
    let need_extend = ctx.ce_config().is_some_and(|c| c.extend_span.is_some());
    if !want_shadow && !need_extend {
        return (None, None);
    }

    let Some(Expression::ObjectExpression(mut object)) = ctx
        .state
        .parsed
        .take_pending_expr(span.start)
        .map(|e| e.into_inner_expression())
    else {
        return (None, None);
    };

    let mut shadow = None;
    let mut extend = None;
    for prop_kind in object.properties.drain(..) {
        let ObjectPropertyKind::ObjectProperty(prop) = prop_kind else {
            continue;
        };
        let prop = prop.unbox();
        let PropertyKey::StaticIdentifier(id) = &prop.key else {
            continue;
        };
        match id.name.as_str() {
            "shadow" if want_shadow => shadow = Some(prop.value),
            "extend" if need_extend => extend = Some(prop.value),
            _ => {}
        }
    }

    (shadow, extend)
}

fn build_props_metadata<'a>(ctx: &Ctx<'a>, parsed_opts: Option<&ParsedCeConfig>) -> Expression<'a> {
    let b = &ctx.b;
    let mut obj_props: Vec<ObjProp<'a>> = Vec::new();
    let mut emitted_keys: Vec<String> = Vec::new();

    if let Some(opts) = parsed_opts {
        for prop in &opts.props {
            let prop_key = resolve_prop_key(ctx, &prop.name);
            let value = build_prop_def_expr(b, prop);
            obj_props.push(ObjProp::KeyValue(b.alloc_str(&prop_key), value));
            emitted_keys.push(prop.name.clone());
            emitted_keys.push(prop_key);
        }
    }

    for accessor in ctx.query.component_prop_accessors() {
        if emitted_keys.iter().any(|key| key == accessor.key.as_ref()) {
            continue;
        }
        obj_props.push(ObjProp::KeyValue(
            b.alloc_str(&accessor.key),
            b.object_expr(iter::empty::<ObjProp<'_>>()),
        ));
    }

    b.object_expr(obj_props)
}

fn resolve_prop_key(ctx: &Ctx<'_>, name: &str) -> String {
    for accessor in ctx.query.component_prop_accessors() {
        if accessor.local == name || accessor.key.as_ref() == name {
            return accessor.key.into_owned();
        }
    }
    name.to_string()
}

fn build_prop_def_expr<'a>(
    b: &svelte_ast_builder::Builder<'a>,
    def: &CePropConfig,
) -> Expression<'a> {
    let mut props: Vec<ObjProp<'a>> = Vec::new();

    if let Some(ref attr) = def.attribute {
        props.push(ObjProp::KeyValue("attribute", b.str_expr(attr)));
    }
    if def.reflect {
        props.push(ObjProp::KeyValue("reflect", b.bool_expr(true)));
    }
    if let Some(ref typ) = def.prop_type {
        props.push(ObjProp::KeyValue("type", b.str_expr(typ)));
    }

    b.object_expr(props)
}
