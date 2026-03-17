//! SvelteElement code generation — `<svelte:element this={tag}>`.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::attributes::process_attrs_spread;
use super::expression::get_node_expr;
use super::gen_fragment;

/// Generate `$.element(anchor, () => tag, is_svg, ($$element, $$anchor) => { ... })`.
pub(crate) fn gen_svelte_element<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let el = ctx.svelte_element(id);
    let static_tag = el.static_tag;
    let tag_value = if static_tag {
        Some(ctx.component.source_text(el.tag_span).to_string())
    } else {
        None
    };
    let el_clone = svelte_ast::Element {
        id: el.id,
        span: el.span,
        name: String::new(),
        self_closing: true,
        attributes: el.attributes.clone(),
        fragment: svelte_ast::Fragment::empty(),
    };
    let has_attrs = !el_clone.attributes.is_empty();

    // Detect SVG namespace from static xmlns attribute
    let is_svg_ns = el.attributes.iter().any(|attr| {
        if let svelte_ast::Attribute::StringAttribute(sa) = attr {
            sa.name == "xmlns"
                && ctx.component.source_text(sa.value_span) == "http://www.w3.org/2000/svg"
        } else {
            false
        }
    });

    // Generate $$element ident for the inner callback
    let el_name = ctx.gen_ident("$$element");

    let mut inner_init: Vec<Statement<'a>> = Vec::new();
    let mut inner_after_update: Vec<Statement<'a>> = Vec::new();

    // Process attributes — always use spread-like handling for svelte:element
    // because the element tag is unknown at compile time.
    if has_attrs {
        process_attrs_spread(ctx, &el_clone, &el_name, &mut inner_init, &mut inner_after_update);
    }

    // Generate children
    let child_body = gen_fragment(ctx, FragmentKey::SvelteElementBody(id));

    // Build tag thunk: () => tag_expression (or () => "literal" for static tags)
    let tag_expr = if let Some(ref value) = tag_value {
        ctx.b.str_expr(value)
    } else {
        get_node_expr(ctx, id)
    };
    let get_tag = ctx.b.thunk(tag_expr);

    let is_svg = ctx.b.bool_expr(is_svg_ns);

    // Assemble inner body: init + update + after_update + children
    let mut inner = inner_init;
    inner.extend(inner_after_update);
    inner.extend(child_body);

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Expr(anchor),
        Arg::Expr(get_tag),
        Arg::Expr(is_svg),
    ];

    if !inner.is_empty() {
        let callback = ctx.b.arrow_block_expr(
            ctx.b.params(["$$element", "$$anchor"]),
            inner,
        );
        args.push(Arg::Expr(callback));
    }

    stmts.push(ctx.b.call_stmt("$.element", args));
}
