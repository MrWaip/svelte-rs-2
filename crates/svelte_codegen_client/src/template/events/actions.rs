use oxc_ast::ast::Statement;

use svelte_analyze::ExprSite;
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::super::expression::get_attr_expr;

pub(crate) fn gen_use_directive_on<'a>(
    ctx: &mut Ctx<'a>,
    ud: &svelte_ast::UseDirective,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    gen_use_directive(ctx, ud, attr_id, el_name, init);
}

pub(crate) fn gen_use_directive<'a>(
    ctx: &mut Ctx<'a>,
    ud: &svelte_ast::UseDirective,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    let attr_blockers = ctx
        .expr_deps(ExprSite::Attr(attr_id))
        .map(|deps| deps.blockers.into_iter().collect::<Vec<_>>())
        .unwrap_or_default();
    let has_expr = ud.expression_span.is_some();
    let params = if has_expr {
        ctx.b.params(["$$node", "$$action_arg"])
    } else {
        ctx.b.params(["$$node"])
    };

    let name_expr = ctx
        .state
        .parsed
        .expr_handle(ud.name.start)
        .and_then(|handle| ctx.state.parsed.take_expr(handle))
        .unwrap();

    let mut call_args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$node")];
    if has_expr {
        call_args.push(Arg::Ident("$$action_arg"));
    }
    let action_call = ctx.b.maybe_call_expr(name_expr, call_args);
    let handler = ctx.b.arrow_expr(params, [ctx.b.expr_stmt(action_call)]);

    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(el_name), Arg::Expr(handler)];

    if has_expr {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    }

    let mut stmt = ctx.b.call_stmt("$.action", args);
    if !attr_blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &attr_blockers);
    }
    init.push(stmt);
}

pub(crate) fn gen_attach_tag<'a>(
    ctx: &mut Ctx<'a>,
    attr_id: NodeId,
    el_name: &str,
    init: &mut Vec<Statement<'a>>,
) {
    let attr_blockers = ctx
        .expr_deps(ExprSite::Attr(attr_id))
        .map(|deps| deps.blockers.into_iter().collect::<Vec<_>>())
        .unwrap_or_default();
    let expr = get_attr_expr(ctx, attr_id);
    let thunk = ctx.b.thunk(expr);
    let mut stmt = ctx
        .b
        .call_stmt("$.attach", [Arg::Ident(el_name), Arg::Expr(thunk)]);

    if !attr_blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &attr_blockers);
    }

    init.push(stmt);
}

pub(crate) fn gen_transition_directive<'a>(
    ctx: &mut Ctx<'a>,
    td: &svelte_ast::TransitionDirective,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    let attr_blockers = ctx
        .expr_deps(ExprSite::Attr(attr_id))
        .map(|deps| deps.blockers.into_iter().collect::<Vec<_>>())
        .unwrap_or_default();
    let mut flags: u32 = if td.modifiers.iter().any(|m| m == "global") {
        4
    } else {
        0
    };
    match td.direction {
        svelte_ast::TransitionDirection::Both => flags |= 1 | 2,
        svelte_ast::TransitionDirection::In => flags |= 1,
        svelte_ast::TransitionDirection::Out => flags |= 2,
    }

    let name_expr = ctx
        .state
        .parsed
        .expr_handle(td.name.start)
        .and_then(|handle| ctx.state.parsed.take_expr(handle))
        .unwrap();
    let name_thunk = ctx.b.thunk(name_expr);

    let mut args: Vec<Arg<'a, '_>> = vec![
        Arg::Num(flags as f64),
        Arg::Ident(el_name),
        Arg::Expr(name_thunk),
    ];

    if td.expression_span.is_some() {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    }

    let mut stmt = ctx.b.call_stmt("$.transition", args);

    if !attr_blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &attr_blockers);
    }

    after_update.push(stmt);
}

pub(crate) fn gen_animate_directive<'a>(
    ctx: &mut Ctx<'a>,
    ad: &svelte_ast::AnimateDirective,
    attr_id: NodeId,
    el_name: &str,
    after_update: &mut Vec<Statement<'a>>,
) {
    let attr_blockers = ctx
        .expr_deps(ExprSite::Attr(attr_id))
        .map(|deps| deps.blockers.into_iter().collect::<Vec<_>>())
        .unwrap_or_default();
    let name_expr = ctx
        .state
        .parsed
        .expr_handle(ad.name.start)
        .and_then(|handle| ctx.state.parsed.take_expr(handle))
        .unwrap();
    let name_thunk = ctx.b.thunk(name_expr);

    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(el_name), Arg::Expr(name_thunk)];

    if ad.expression_span.is_some() {
        let expr = get_attr_expr(ctx, attr_id);
        let thunk = ctx.b.thunk(expr);
        args.push(Arg::Expr(thunk));
    } else {
        args.push(Arg::Expr(ctx.b.null_expr()));
    }

    let mut stmt = ctx.b.call_stmt("$.animation", args);

    if !attr_blockers.is_empty() {
        stmt = gen_run_after_blockers(ctx, stmt, &attr_blockers);
    }

    after_update.push(stmt);
}

fn gen_run_after_blockers<'a>(
    ctx: &mut Ctx<'a>,
    stmt: Statement<'a>,
    blockers: &[u32],
) -> Statement<'a> {
    let blockers_arr = ctx.b.promises_array(blockers);
    let thunk = ctx.b.thunk_block(vec![stmt]);
    ctx.b.call_stmt(
        "$.run_after_blockers",
        [Arg::Expr(blockers_arr), Arg::Expr(thunk)],
    )
}
