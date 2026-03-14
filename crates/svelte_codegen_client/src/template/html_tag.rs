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
    let tag = ctx.html_tag(id);
    let source = ctx.component.source_text(tag.expression_span);

    let thunk = build_html_thunk(ctx, source);

    stmts.push(ctx.b.call_stmt("$.html", [Arg::Expr(anchor_expr), Arg::Expr(thunk)]));
}

/// Build `() => expr` with rune transforms applied.
fn build_html_thunk<'a>(ctx: &mut Ctx<'a>, source: &str) -> Expression<'a> {
    let alloc = ctx.b.ast.allocator;
    let arena_source: &'a str = alloc.alloc_str(source);

    let mutated = &ctx.analysis.mutated_runes;
    let rune_names = &ctx.analysis.rune_names;
    let prop_sources = ctx.prop_sources.clone();
    let prop_non_sources = ctx.prop_non_sources.clone();
    let each_vars = &ctx.each_vars;

    let expr = super::expression::parse_and_transform(
        alloc,
        arena_source,
        mutated,
        rune_names,
        &prop_sources,
        &prop_non_sources,
        &[],
        each_vars,
    );

    let params = ctx.b.no_params();
    ctx.b.arrow_expr(params, [ctx.b.expr_stmt(expr)])
}
