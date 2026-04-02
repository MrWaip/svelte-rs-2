//! SvelteElement code generation — `<svelte:element this={tag}>`.

use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::async_plan::AsyncEmissionPlan;
use super::attributes::{process_attrs_spread, process_svelte_element_class_directives, process_svelte_element_style_directives};
use super::expression::get_node_expr;
use super::gen_fragment;

/// Generate `$.element(anchor, () => tag, is_svg, ($$element, $$anchor) => { ... })`.
pub(crate) fn gen_svelte_element<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let async_plan = AsyncEmissionPlan::for_node(ctx, id);
    let has_await = async_plan.has_await();
    let needs_async = async_plan.needs_async();
    let el = ctx.svelte_element(id);
    let static_tag = el.static_tag;
    let tag_value = if static_tag {
        Some(ctx.query.component.source_text(el.tag_span).to_string())
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
                && ctx.query.component.source_text(sa.value_span) == "http://www.w3.org/2000/svg"
        } else {
            false
        }
    });

    // Generate $$element ident for the inner callback
    let el_name = ctx.gen_ident("$$element");

    let mut inner_init: Vec<Statement<'a>> = Vec::new();
    let mut inner_after_update: Vec<Statement<'a>> = Vec::new();

    // Optimization: when the only attribute is a static class, use $.set_class
    // instead of $.attribute_effect (matches Svelte reference SvelteElement.js).
    let sole_static_class = if el_clone.attributes.len() == 1 {
        if let svelte_ast::Attribute::StringAttribute(sa) = &el_clone.attributes[0] {
            if sa.name.eq_ignore_ascii_case("class") {
                Some(ctx.query.component.source_text(sa.value_span).to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(class_value) = sole_static_class {
        inner_init.push(ctx.b.call_stmt(
            "$.set_class",
            [Arg::Ident(&el_name), Arg::Num(0.0), Arg::Str(class_value)],
        ));
    } else if has_attrs {
        // Generic spread-like handling for svelte:element
        // because the element tag is unknown at compile time.
        process_attrs_spread(
            ctx,
            id,
            "",
            &el_clone.attributes,
            &el_name,
            false,
            true,
            &mut inner_init,
            &mut inner_after_update,
        );
    }

    // Class directives on svelte:element use $.set_class with flag 0
    process_svelte_element_class_directives(ctx, &el_clone, &el_name, &mut inner_init);

    // Style directives on svelte:element are handled via process_attrs_spread computed [$.STYLE] property
    process_svelte_element_style_directives(ctx, &el_clone, &el_name, &mut inner_init);

    // Generate children
    let child_body = gen_fragment(ctx, FragmentKey::SvelteElementBody(id));

    let is_svg = ctx.b.bool_expr(is_svg_ns);

    // Assemble inner body: init + update + after_update + children
    let mut inner = inner_init;
    inner.extend(inner_after_update);
    inner.extend(child_body);

    let callback = (!inner.is_empty()).then(|| {
        ctx.b.arrow_block_expr(
            ctx.b.params([el_name.as_str(), "$$anchor"]),
            inner,
        )
    });

    if needs_async {
        let async_tag_thunk = has_await.then(|| {
            let tag_expr = if let Some(ref value) = tag_value {
                ctx.b.str_expr(value)
            } else {
                get_node_expr(ctx, id)
            };
            async_plan.async_thunk(ctx, tag_expr).expect("async tag thunk missing for await plan")
        });

        let get_tag = ctx.b.thunk(ctx.b.call_expr("$.get", [Arg::Ident("$$tag")]));
        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::Ident("node"),
            Arg::Expr(get_tag),
            Arg::Expr(is_svg),
        ];
        if let Some(cb) = callback {
            args.push(Arg::Expr(cb));
        }

        let element_stmt = ctx.b.call_stmt("$.element", args);
        stmts.push(async_plan.wrap_async_block(ctx, anchor, "$$tag", async_tag_thunk, vec![element_stmt]));
    } else {
        // Build tag thunk: () => tag_expression (or () => "literal" for static tags)
        let tag_expr = if let Some(ref value) = tag_value {
            ctx.b.str_expr(value)
        } else {
            get_node_expr(ctx, id)
        };
        let get_tag = ctx.b.thunk(tag_expr);

        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::Expr(anchor),
            Arg::Expr(get_tag),
            Arg::Expr(is_svg),
        ];
        if let Some(cb) = callback {
            args.push(Arg::Expr(cb));
        }

        stmts.push(ctx.b.call_stmt("$.element", args));
    }
}
