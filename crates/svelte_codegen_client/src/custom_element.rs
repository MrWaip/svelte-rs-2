use oxc_ast::ast::{Expression, ObjectPropertyKind, PropertyKey, Statement};
use svelte_ast::CustomElementConfig;
use svelte_parser::{CePropConfig, CeShadowMode, ParsedCeConfig};

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
        let name = e.alias.as_deref().unwrap_or(e.name.as_str());
        Arg::StrRef(name)
    }));

    let is_shadow_none = parsed.is_some_and(|o| o.shadow == CeShadowMode::None);
    let delegates_focus = parsed.is_some_and(|o| o.delegates_focus);

    let extend_arg = take_extend_expr(ctx, ce_config);
    let b = &ctx.b;

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Ident(ctx.state.name),
        Arg::Expr(props_obj),
        Arg::Expr(slots),
        Arg::Expr(accessors),
    ];

    if !is_shadow_none {
        let mut shadow_props: Vec<ObjProp<'_>> =
            vec![ObjProp::KeyValue("mode", b.str_expr("open"))];
        if delegates_focus {
            shadow_props.push(ObjProp::KeyValue("delegatesFocus", b.bool_expr(true)));
        }
        args.push(Arg::Expr(b.object_expr(shadow_props)));
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
        stmts.push(b.expr_stmt(define_call));
    } else {
        stmts.push(b.expr_stmt(create_ce));
    }

    stmts
}

fn take_extend_expr<'a>(
    ctx: &mut Ctx<'a>,
    ce_config: Option<&CustomElementConfig>,
) -> Option<Expression<'a>> {
    let Some(CustomElementConfig::Expression(span)) = ce_config else {
        return None;
    };
    let config = ctx.ce_config()?;
    config.extend_span?;

    let _ = config;
    let Some(Expression::ObjectExpression(object)) = ctx.state.parsed.take_pending_expr(span.start)
    else {
        return None;
    };
    let mut object = object;

    for prop_kind in object.properties.drain(..) {
        let ObjectPropertyKind::ObjectProperty(prop) = prop_kind else {
            continue;
        };
        let prop = prop.unbox();
        if let PropertyKey::StaticIdentifier(id) = &prop.key
            && id.name.as_str() == "extend"
        {
            return Some(prop.value);
        }
    }

    None
}

fn build_props_metadata<'a>(ctx: &Ctx<'a>, parsed_opts: Option<&ParsedCeConfig>) -> Expression<'a> {
    let b = &ctx.b;
    let mut obj_props: Vec<ObjProp<'a>> = Vec::new();

    let ce_prop_names: Vec<&str> = parsed_opts
        .map(|o| o.props.iter().map(|p| p.name.as_str()).collect())
        .unwrap_or_default();

    if let Some(opts) = parsed_opts {
        for prop in &opts.props {
            let prop_key = resolve_prop_key(ctx, &prop.name);
            let value = build_prop_def_expr(b, prop);
            obj_props.push(ObjProp::KeyValue(b.alloc_str(&prop_key), value));
        }
    }

    if let Some(props_decl) = ctx.query.props() {
        for prop in &props_decl.props {
            if prop.is_rest || prop.is_reserved() {
                continue;
            }
            let key: &str = prop.prop_name.as_str();

            if ce_prop_names.contains(&key) {
                continue;
            }
            obj_props.push(ObjProp::KeyValue(
                b.alloc_str(key),
                b.object_expr(std::iter::empty::<ObjProp<'_>>()),
            ));
        }
    }

    b.object_expr(obj_props)
}

fn resolve_prop_key(ctx: &Ctx<'_>, name: &str) -> String {
    if let Some(props) = ctx.query.props() {
        for prop in &props.props {
            if prop.local_name == name || prop.prop_name == name {
                return prop.prop_name.to_string();
            }
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
