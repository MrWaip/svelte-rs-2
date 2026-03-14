//! HtmlTag codegen — `{@html expr}`

use oxc_ast::ast::{Expression, Statement};
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

/// Generate `$.html(anchor, () => expr)`.
pub(crate) fn gen_html_tag<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor_expr: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let thunk = super::expression::build_node_thunk(ctx, id);

    stmts.push(ctx.b.call_stmt("$.html", [Arg::Expr(anchor_expr), Arg::Expr(thunk)]));
}
