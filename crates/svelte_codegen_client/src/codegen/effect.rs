use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::ExprSite;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft};

use crate::context::Ctx;

use super::data_structures::{MemoAttr, MemoAttrUpdate, TemplateMemoState};
use super::{CodegenError, Result};

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
    if let Expression::AwaitExpression(await_expr) = expr {
        let inner = await_expr.unbox().argument;
        if let Expression::CallExpression(call) = &inner
            && call.arguments.is_empty()
            && let Expression::Identifier(ident) = &call.callee
        {
            return ctx.b.rid_expr(ident.name.as_str());
        }
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
    memo_attrs: Vec<MemoAttr<'a>>,
    script_blockers: Vec<u32>,
    extra_blockers: Vec<Expression<'a>>,
) -> Result<()> {
    if memo_attrs.is_empty() {
        emit_template_effect_with_blockers(
            ctx,
            regular_updates,
            script_blockers,
            extra_blockers,
            body,
        );
        return Ok(());
    }

    let memo_count = memo_attrs.len();
    let mut param_names: Vec<String> = Vec::with_capacity(memo_count);
    let mut memo_data: Vec<(NodeId, String, MemoAttrUpdate, Expression<'a>, bool)> =
        Vec::with_capacity(memo_count);

    for (i, memo) in memo_attrs.into_iter().enumerate() {
        param_names.push(format!("${i}"));
        memo_data.push((
            memo.attr_id,
            memo.el_name,
            memo.update,
            memo.expr,
            memo.is_node_site,
        ));
    }

    let mut deps = TemplateMemoState::default();
    for idx in script_blockers {
        deps.push_script_blocker(idx);
    }
    deps.extra_blockers.extend(extra_blockers);
    let mut callback_body = regular_updates;

    for (i, (attr_id, el_name, update, expr, is_node_site)) in memo_data.into_iter().enumerate() {
        let memo_expr = ctx.b.rid_expr(&param_names[i]);
        match update {
            MemoAttrUpdate::Call {
                setter_fn,
                attr_name,
            } => {
                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(ctx.b.rid_expr(&el_name))];
                if let Some(name) = attr_name {
                    args.push(Arg::Str(name));
                }
                args.push(Arg::Expr(memo_expr));
                callback_body.push(ctx.b.call_stmt(setter_fn, args));
            }
            MemoAttrUpdate::Assignment { property } => {
                callback_body.push(ctx.b.assign_stmt(
                    AssignLeft::StaticMember(
                        ctx.b.static_member(ctx.b.rid_expr(&el_name), &property),
                    ),
                    memo_expr,
                ));
            }
        }

        let site = if is_node_site {
            ExprSite::Node(attr_id)
        } else {
            ExprSite::Attr(attr_id)
        };
        let Some(attr_deps) = ctx.expr_deps(site) else {
            return CodegenError::missing_expression_deps(attr_id);
        };
        deps.push_expr_info(ctx, attr_deps.info);
        if attr_deps.has_await() {
            deps.async_values.push(expr);
        } else {
            deps.sync_values.push(expr);
        }
    }

    let params = ctx.b.params(param_names.iter().map(|s| s.as_str()));
    let callback = ctx.b.arrow_expr(params, callback_body);
    emit_effect_call(ctx, "$.template_effect", callback, &mut deps, body);
    Ok(())
}
