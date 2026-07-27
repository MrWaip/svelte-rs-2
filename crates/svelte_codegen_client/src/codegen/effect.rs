use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::Suspension;
use svelte_ast_builder::{Arg, OutermostAwait};

use crate::context::Ctx;

use super::Result;
use super::data_structures::TemplateMemoState;

pub(in crate::codegen) fn emit_effect_call_extern<'a>(
    ctx: &Ctx<'a>,
    effect_name: &str,
    callback: Expression<'a>,
    deps: &mut TemplateMemoState<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    emit_effect_call(ctx, effect_name, callback, deps, body);
}

fn emit_effect_call<'a>(
    ctx: &Ctx<'a>,
    effect_name: &str,
    callback: Expression<'a>,
    deps: &mut TemplateMemoState<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    let mut callback = callback;
    deps.resolve_param_names(ctx, &mut callback);
    if !deps.has_deps() {
        body.push(ctx.b.call_stmt(effect_name, [Arg::Expr(callback)]));
        return;
    }

    let has_sync_values = deps.has_sync_values();
    let has_async_values = deps.has_async_values();
    let has_blockers = deps.has_blockers();
    let mut args = vec![Arg::Expr(callback)];
    if has_sync_values || has_async_values || has_blockers {
        args.push(Arg::Expr(deps.sync_values_expr(ctx)));
    }
    if has_async_values || has_blockers {
        args.push(Arg::Expr(deps.async_values_expr(ctx)));
    }
    if has_blockers {
        args.push(Arg::Expr(deps.blockers_expr(ctx)));
    }
    body.push(ctx.b.call_stmt(effect_name, args));
}

pub(in crate::codegen) fn suspending_thunk<'a>(
    ctx: &Ctx<'a>,
    expr: Expression<'a>,
    suspension: Suspension,
) -> Expression<'a> {
    collapse_when_outermost(ctx, expr, suspension, |ctx, operand| {
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(operand)])
    })
}

pub(in crate::codegen) fn suspending_block_thunk<'a>(
    ctx: &Ctx<'a>,
    expr: Expression<'a>,
    suspension: Suspension,
) -> Expression<'a> {
    collapse_when_outermost(ctx, expr, suspension, |ctx, operand| ctx.b.thunk(operand))
}

fn collapse_when_outermost<'a>(
    ctx: &Ctx<'a>,
    expr: Expression<'a>,
    suspension: Suspension,
    collapse: impl FnOnce(&Ctx<'a>, Expression<'a>) -> Expression<'a>,
) -> Expression<'a> {
    if !suspension.is_outermost() {
        return ctx.b.async_arrow_expr_body(expr);
    }
    match ctx.b.outermost_await(expr) {
        OutermostAwait::Operand(operand) => collapse(ctx, operand),
        OutermostAwait::Absent(expr) => ctx.b.async_arrow_expr_body(expr),
    }
}

fn emit_template_effect_with_blockers<'a>(
    ctx: &mut Ctx<'a>,
    update: Vec<Statement<'a>>,
    script_blockers: Vec<svelte_analyze::BlockerSlot>,
    extra_blockers: Vec<(String, usize)>,
    body: &mut Vec<Statement<'a>>,
) {
    if update.is_empty() {
        return;
    }
    let eff = ctx.b.arrow_expr(ctx.b.no_params(), update);
    let mut deps = TemplateMemoState::default();
    for slot in script_blockers {
        deps.push_blocker_slot(slot);
    }
    super::data_structures::extend_blocker_slots(&mut deps.extra_blockers, extra_blockers);
    emit_effect_call(ctx, "$.template_effect", eff, &mut deps, body);
}

pub(in crate::codegen) fn emit_template_effect_with_memo<'a>(
    ctx: &mut Ctx<'a>,
    body: &mut Vec<Statement<'a>>,
    regular_updates: Vec<Statement<'a>>,
    mut shared_memo: TemplateMemoState<'a>,
    script_blockers: Vec<svelte_analyze::BlockerSlot>,
    extra_blockers: Vec<(String, usize)>,
) -> Result<()> {
    if !shared_memo.has_deps() {
        emit_template_effect_with_blockers(
            ctx,
            regular_updates,
            script_blockers,
            extra_blockers,
            body,
        );
        return Ok(());
    }

    for slot in script_blockers {
        shared_memo.push_blocker_slot(slot);
    }
    super::data_structures::extend_blocker_slots(&mut shared_memo.extra_blockers, extra_blockers);

    let param_names = shared_memo.param_names();
    let params = ctx.b.params(param_names.iter().map(|s| s.as_str()));
    let callback = ctx.b.arrow_expr(params, regular_updates);
    emit_effect_call(ctx, "$.template_effect", callback, &mut shared_memo, body);
    Ok(())
}
