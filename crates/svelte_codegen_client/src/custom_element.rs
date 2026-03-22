use oxc_ast::ast::{Expression, Statement};
use svelte_ast::CustomElementConfig;
use svelte_parser::{CePropConfig, CeShadowMode, ParsedCeConfig};

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

// ---------------------------------------------------------------------------
// Codegen
// ---------------------------------------------------------------------------

/// Generate the `customElements.define(tag, $.create_custom_element(...))` statement(s).
pub fn gen_custom_element<'a>(
    ctx: &mut Ctx<'a>,
    ce_config: &CustomElementConfig,
) -> Vec<Statement<'a>> {
    let b = &ctx.b;

    // Determine tag and parsed options based on config variant
    let (simple_tag, parsed) = match ce_config {
        CustomElementConfig::Tag(tag) => (Some(tag.as_str()), None),
        CustomElementConfig::Expression(_) => (None, ctx.analysis.ce_config.as_ref()),
    };

    // Resolve tag: simple form uses tag directly, object form uses parsed tag
    let resolved_tag: Option<&str> = match (&simple_tag, &parsed) {
        (Some(t), _) => Some(t),
        (None, Some(opts)) => opts.tag.as_deref(),
        (None, None) => None,
    };

    // -- Arg 2: Props metadata object --
    let props_obj = build_props_metadata(ctx, parsed);

    // -- Arg 3: Slots array (always empty in Svelte 5 runes mode) --
    let slots = b.array_from_args(std::iter::empty::<Arg<'_, '_>>());

    // -- Arg 4: Accessors array (from exports) --
    let accessors = b.array_from_args(
        ctx.analysis.exports.iter().map(|e| {
            let name = e.alias.as_deref().unwrap_or(e.name.as_str());
            Arg::StrRef(name)
        })
    );

    // -- Arg 5: Shadow root config --
    let is_shadow_none = parsed.is_some_and(|o| o.shadow == CeShadowMode::None);

    // -- Arg 6: Extend (pre-parsed in analyze) --
    let extend_arg: Option<Expression<'a>> = ctx.analysis.ce_config.as_ref()
        .and_then(|c| c.extend_span)
        .and_then(|span| ctx.parsed.exprs.remove(&span.start));

    // Build $.create_custom_element() call
    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Ident(ctx.name),
        Arg::Expr(props_obj),
        Arg::Expr(slots),
        Arg::Expr(accessors),
    ];

    if !is_shadow_none {
        args.push(Arg::Expr(
            b.object_expr([ObjProp::KeyValue("mode", b.str_expr("open"))]),
        ));
    }

    if let Some(extend_expr) = extend_arg {
        args.push(Arg::Expr(extend_expr));
    }

    let create_ce = b.call_expr("$.create_custom_element", args);

    // Wrap in customElements.define() if tag is present
    let mut stmts = Vec::new();
    if let Some(tag_str) = resolved_tag {
        let define_callee = b.static_member_expr(
            b.rid_expr("customElements"),
            "define",
        );
        let define_call = b.call_expr_callee(define_callee, [
            Arg::StrRef(tag_str),
            Arg::Expr(create_ce),
        ]);
        stmts.push(b.expr_stmt(define_call));
    } else {
        stmts.push(b.expr_stmt(create_ce));
    }

    stmts
}

/// Build the props metadata object for `$.create_custom_element()`.
///
/// For each prop from CE config: build `{ attribute?, reflect?, type? }`.
/// For remaining component props not in config: add `propName: {}`.
fn build_props_metadata<'a>(
    ctx: &Ctx<'a>,
    parsed_opts: Option<&ParsedCeConfig>,
) -> Expression<'a> {
    let b = &ctx.b;
    let mut obj_props: Vec<ObjProp<'a>> = Vec::new();

    // Collect CE config prop names for dedup check
    let ce_prop_names: Vec<&str> = parsed_opts
        .map(|o| o.props.iter().map(|p| p.name.as_str()).collect())
        .unwrap_or_default();

    // First: emit props from CE config (preserving config order)
    if let Some(opts) = parsed_opts {
        for prop in &opts.props {
            let prop_key = resolve_prop_key(ctx, &prop.name);
            let value = build_prop_def_expr(b, prop);
            obj_props.push(ObjProp::KeyValue(b.alloc_str(&prop_key), value));
        }
    }

    // Second: emit remaining component props not already in CE config
    if let Some(ref props_analysis) = ctx.analysis.props {
        for prop in &props_analysis.props {
            if prop.is_rest || prop.is_reserved {
                continue;
            }
            let key = &prop.prop_name;
            // Skip if already covered by CE config
            if ce_prop_names.iter().any(|&n| n == key) {
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

/// Resolve the prop key: use prop_alias if the binding has one, otherwise use the name.
fn resolve_prop_key(ctx: &Ctx<'_>, name: &str) -> String {
    if let Some(ref props) = ctx.analysis.props {
        for prop in &props.props {
            if prop.local_name == name || prop.prop_name == name {
                return prop.prop_name.clone();
            }
        }
    }
    name.to_string()
}

/// Build the value expression for a single prop definition: `{ attribute?, reflect?, type? }`.
fn build_prop_def_expr<'a>(
    b: &crate::builder::Builder<'a>,
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
