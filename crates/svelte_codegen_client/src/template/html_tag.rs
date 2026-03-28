//! HtmlTag codegen — `{@html expr}`

use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{Namespace, NodeId};

use crate::builder::Arg;
use crate::context::Ctx;

/// Generate `$.html(anchor, () => expr, is_controlled?, is_svg?, is_mathml?)`.
pub(crate) fn gen_html_tag<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor_expr: Expression<'a>,
    is_controlled: bool,
    stmts: &mut Vec<Statement<'a>>,
) {
    let has_await = ctx.expr_has_await(id);
    let needs_async = has_await || ctx.expr_has_blockers(id);

    // Namespace flags only matter for non-controlled path (controlled inherits parent namespace)
    let is_svg = !is_controlled
        && ctx.component.options.as_ref().and_then(|o| o.namespace.as_ref())
            == Some(&Namespace::Svg);
    let is_mathml = !is_controlled
        && ctx.component.options.as_ref().and_then(|o| o.namespace.as_ref())
            == Some(&Namespace::Mathml);

    if needs_async {
        let expression = super::expression::get_node_expr(ctx, id);

        // Inside $.async callback: $.html(node, () => $.get($$html), ...)
        let html_value = ctx.b.thunk(ctx.b.call_expr("$.get", [Arg::Ident("$$html")]));
        let mut html_args: Vec<Arg<'a, '_>> = vec![Arg::Ident("node"), Arg::Expr(html_value)];

        if is_controlled || is_svg || is_mathml {
            html_args.push(if is_controlled { Arg::Bool(true) } else { Arg::Expr(ctx.b.void_zero_expr()) });
        }
        if is_svg || is_mathml {
            html_args.push(if is_svg { Arg::Bool(true) } else { Arg::Expr(ctx.b.void_zero_expr()) });
        }
        if is_mathml {
            html_args.push(Arg::Bool(true));
        }

        let html_stmt = ctx.b.call_stmt("$.html", html_args);

        let async_thunk = if has_await { Some(ctx.b.async_thunk(expression)) } else { None };
        stmts.push(ctx.gen_async_block(id, anchor_expr, has_await, async_thunk, "$$html", vec![html_stmt]));
    } else {
        let thunk = super::expression::build_node_thunk(ctx, id);

        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor_expr), Arg::Expr(thunk)];

        if is_controlled || is_svg || is_mathml {
            args.push(if is_controlled { Arg::Bool(true) } else { Arg::Expr(ctx.b.void_zero_expr()) });
        }
        if is_svg || is_mathml {
            args.push(if is_svg { Arg::Bool(true) } else { Arg::Expr(ctx.b.void_zero_expr()) });
        }
        if is_mathml {
            args.push(Arg::Bool(true));
        }

        stmts.push(ctx.b.call_stmt("$.html", args));
    }
}
