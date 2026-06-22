use oxc_ast::ast::{Expression, Statement};
use svelte_ast_builder::Arg;

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

pub(in crate::codegen) fn async_value_thunk<'a>(
    ctx: &Ctx<'a>,
    expr: Expression<'a>,
) -> Expression<'a> {
    let is_await = matches!(expr.get_inner_expression(), Expression::AwaitExpression(_));
    if is_await {
        let Expression::AwaitExpression(await_expr) = expr.into_inner_expression() else {
            unreachable!()
        };
        let inner = await_expr.unbox().argument;
        ctx.b
            .arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(inner)])
    } else {
        ctx.b.async_arrow_expr_body(expr)
    }
}

fn emit_template_effect_with_blockers<'a>(
    ctx: &mut Ctx<'a>,
    update: Vec<Statement<'a>>,
    script_blockers: Vec<u32>,
    extra_blockers: Vec<Expression<'a>>,
    body: &mut Vec<Statement<'a>>,
) {
    if update.is_empty() {
        return;
    }
    let eff = ctx.b.arrow_expr(ctx.b.no_params(), update);
    let mut deps = TemplateMemoState::default();
    for idx in script_blockers {
        deps.push_script_blocker(idx);
    }
    deps.extra_blockers.extend(extra_blockers);
    emit_effect_call(ctx, "$.template_effect", eff, &mut deps, body);
}

pub(in crate::codegen) fn emit_template_effect_with_memo<'a>(
    ctx: &mut Ctx<'a>,
    body: &mut Vec<Statement<'a>>,
    regular_updates: Vec<Statement<'a>>,
    mut shared_memo: TemplateMemoState<'a>,
    script_blockers: Vec<u32>,
    extra_blockers: Vec<Expression<'a>>,
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

    for idx in script_blockers {
        shared_memo.push_script_blocker(idx);
    }
    shared_memo.extra_blockers.extend(extra_blockers);

    let param_names = shared_memo.param_names();
    let params = ctx.b.params(param_names.iter().map(|s| s.as_str()));
    let callback = ctx.b.arrow_expr(params, regular_updates);
    emit_effect_call(ctx, "$.template_effect", callback, &mut shared_memo, body);
    Ok(())
}
