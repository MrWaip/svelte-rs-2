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
    let thunk = super::expression::build_node_thunk(ctx, id);

    // Namespace flags only matter for non-controlled path (controlled inherits parent namespace)
    let is_svg = !is_controlled
        && ctx.component.options.as_ref().and_then(|o| o.namespace.as_ref())
            == Some(&Namespace::Svg);
    let is_mathml = !is_controlled
        && ctx.component.options.as_ref().and_then(|o| o.namespace.as_ref())
            == Some(&Namespace::Mathml);

    // Build args, trimming trailing false-y values
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
