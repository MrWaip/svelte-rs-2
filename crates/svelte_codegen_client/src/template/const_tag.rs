use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::FragmentKey;

use crate::builder::{Arg, AssignLeft, ObjProp};
use crate::context::Ctx;

/// Emit `const name = $.derived(() => init_expr)` for each ConstTag in a fragment.
///
/// Called before DOM init code in every fragment codegen path.
/// Returns an optional `var promises = $.run([...])` statement when async mode
/// is triggered (any const tag has upstream blockers or contains `await`).
pub(crate) fn gen_const_tags<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    stmts: &mut Vec<Statement<'a>>,
) -> Option<Statement<'a>> {
    let Some(ids) = ctx.const_tags_for_fragment(&key).cloned() else {
        return None;
    };

    // Check if any const tag triggers async mode
    let needs_async = ctx.experimental_async && ids.iter().any(|&id| {
        ctx.expr_has_await(id) || !ctx.analysis().expression_blockers(id).is_empty()
    });

    if needs_async {
        gen_const_tags_async(ctx, key, &ids, stmts)
    } else {
        emit_const_tags_sync(ctx, &ids, stmts);
        None
    }
}

/// Sync path: `const name = $.derived(() => init)` for each const tag.
fn emit_const_tags_sync<'a>(
    ctx: &mut Ctx<'a>,
    ids: &[svelte_ast::NodeId],
    stmts: &mut Vec<Statement<'a>>,
) {
    for &id in ids {
        let names = ctx.const_tag_names(id).cloned().unwrap_or_default();
        let init_expr = extract_const_init(ctx, id);

        if names.len() == 1 {
            let thunk = ctx.b.thunk(init_expr);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);

            let final_expr = if ctx.dev {
                let name_str = ctx.b.alloc_str(&names[0]);
                ctx.b.call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef(name_str)])
            } else {
                derived
            };

            stmts.push(ctx.b.const_stmt(&names[0], final_expr));

            if ctx.dev {
                let name_str = ctx.b.alloc_str(&names[0]);
                stmts.push(ctx.b.call_stmt("$.get", [Arg::Ident(name_str)]));
            }
        } else if names.len() > 1 {
            emit_destructured_const_sync(ctx, id, &names, init_expr, stmts);
        }
    }
}

/// Async path: `let name;` + `var promises = $.run([blocker_thunks..., main_thunks...])`.
/// Once any const tag in a fragment triggers async, ALL const tags in that fragment
/// go through the async path (matching reference compiler behavior).
fn gen_const_tags_async<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    ids: &[svelte_ast::NodeId],
    stmts: &mut Vec<Statement<'a>>,
) -> Option<Statement<'a>> {
    let promises_name = ctx.gen_ident("promises");
    let mut thunks: Vec<Expression<'a>> = Vec::new();

    let scope = ctx.analysis().scoping.fragment_scope(&key);

    for &id in ids {
        let names = ctx.const_tag_names(id).cloned().unwrap_or_default();
        let has_await = ctx.expr_has_await(id);
        let blockers = ctx.analysis().expression_blockers(id);
        let init_expr = extract_const_init(ctx, id);

        if names.len() == 1 {
            stmts.push(ctx.b.let_stmt(&names[0]));
            emit_blocker_thunks(ctx, &blockers, &mut thunks);

            let derived = create_derived(ctx, init_expr, has_await);
            let final_derived = if ctx.dev {
                let name_str = ctx.b.alloc_str(&names[0]);
                ctx.b.call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef(name_str)])
            } else {
                derived
            };

            let lhs = AssignLeft::Ident(names[0].clone());
            let assignment = ctx.b.assign_expr(lhs, final_derived);

            let body = if ctx.dev {
                let name_str = ctx.b.alloc_str(&names[0]);
                let get_call = ctx.b.call_stmt("$.get", [Arg::Ident(name_str)]);
                let assign_stmt = ctx.b.expr_stmt(assignment);
                let stmts = vec![assign_stmt, get_call];
                if has_await {
                    ctx.b.async_thunk_block(stmts)
                } else {
                    ctx.b.thunk_block(stmts)
                }
            } else if has_await {
                ctx.b.async_thunk(assignment)
            } else {
                ctx.b.thunk(assignment)
            };
            thunks.push(body);

            let thunk_idx = thunks.len() - 1;
            if let Some(scope_id) = scope {
                if let Some(sym_id) = ctx.analysis().scoping.find_binding(scope_id, &names[0]) {
                    ctx.const_tag_blockers.insert(sym_id, (promises_name.clone(), thunk_idx));
                }
            }
        } else if names.len() > 1 {
            let tmp_name = ctx.transform_data.const_tag_tmp_names.get(&id)
                .expect("destructured const tag must have tmp_name from transform")
                .clone();

            stmts.push(ctx.b.let_stmt(&tmp_name));
            emit_blocker_thunks(ctx, &blockers, &mut thunks);
            let destruct_stmt = ctx.b.const_object_destruct_stmt(&names, init_expr);
            let props: Vec<ObjProp<'a>> = names.iter()
                .map(|n| ObjProp::Shorthand(ctx.b.alloc_str(n)))
                .collect();
            let ret = ctx.b.return_stmt(ctx.b.object_expr(props));
            let block_arrow = ctx.b.arrow_block_expr(ctx.b.no_params(), [destruct_stmt, ret]);
            let derived = create_derived(ctx, block_arrow, has_await);

            let final_derived = if ctx.dev {
                ctx.b.call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef("[@const]")])
            } else {
                derived
            };

            let lhs = AssignLeft::Ident(tmp_name.clone());
            let assignment = ctx.b.assign_expr(lhs, final_derived);

            let body = if has_await {
                ctx.b.async_thunk(assignment)
            } else {
                ctx.b.thunk(assignment)
            };
            thunks.push(body);

            let thunk_idx = thunks.len() - 1;
            if let Some(scope_id) = scope {
                for name in &names {
                    if let Some(sym_id) = ctx.analysis().scoping.find_binding(scope_id, name) {
                        ctx.const_tag_blockers.insert(sym_id, (promises_name.clone(), thunk_idx));
                    }
                }
            }
        }
    }

    if thunks.is_empty() {
        return None;
    }

    // var promises = $.run([...thunks])
    let thunks_array = ctx.b.array_expr(thunks);
    let run_call = ctx.b.call_expr("$.run", [Arg::Expr(thunks_array)]);
    Some(ctx.b.var_stmt(&promises_name, run_call))
}

/// Emit blocker thunks for upstream script-level dependencies.
/// For 1 blocker: `() => $$promises[N].promise`
/// For N blockers: `() => $.wait([$$promises[N], ...])`
fn emit_blocker_thunks<'a>(
    ctx: &mut Ctx<'a>,
    blockers: &[u32],
    thunks: &mut Vec<Expression<'a>>,
) {
    if blockers.is_empty() {
        return;
    }
    if blockers.len() == 1 {
        let member = ctx.b.computed_member_expr(
            ctx.b.rid_expr("$$promises"),
            ctx.b.num_expr(blockers[0] as f64),
        );
        let promise_access = ctx.b.static_member_expr(member, "promise");
        thunks.push(ctx.b.thunk(promise_access));
    } else {
        let arr_elements: Vec<Expression<'a>> = blockers.iter()
            .map(|&idx| ctx.b.computed_member_expr(
                ctx.b.rid_expr("$$promises"),
                ctx.b.num_expr(idx as f64),
            ))
            .collect();
        let arr = ctx.b.array_expr(arr_elements);
        let wait_call = ctx.b.call_expr("$.wait", [Arg::Expr(arr)]);
        thunks.push(ctx.b.thunk(wait_call));
    }
}

/// Destructured const tag sync path helper.
fn emit_destructured_const_sync<'a>(
    ctx: &mut Ctx<'a>,
    id: svelte_ast::NodeId,
    names: &[String],
    init_expr: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let tmp_name = ctx.transform_data.const_tag_tmp_names.get(&id)
        .expect("destructured const tag must have tmp_name from transform");
    let tmp_name: &str = ctx.b.alloc_str(tmp_name);

    let destruct_stmt = ctx.b.const_object_destruct_stmt(names, init_expr);
    let props: Vec<ObjProp<'a>> = names.iter()
        .map(|n| ObjProp::Shorthand(ctx.b.alloc_str(n)))
        .collect();
    let ret = ctx.b.return_stmt(ctx.b.object_expr(props));
    let thunk = ctx.b.arrow_block_expr(ctx.b.no_params(), [destruct_stmt, ret]);
    let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);

    let final_expr = if ctx.dev {
        ctx.b.call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef("[@const]")])
    } else {
        derived
    };

    stmts.push(ctx.b.const_stmt(tmp_name, final_expr));
}

/// `$.derived(() => init)` or `$.async_derived(async () => init)` depending on has_await.
fn create_derived<'a>(ctx: &mut Ctx<'a>, init: Expression<'a>, has_await: bool) -> Expression<'a> {
    if has_await {
        let thunk = ctx.b.async_thunk(init);
        ctx.b.call_expr("$.async_derived", [Arg::Expr(thunk)])
    } else {
        let thunk = ctx.b.thunk(init);
        ctx.b.call_expr("$.derived", [Arg::Expr(thunk)])
    }
}

/// Extract the init expression from a pre-parsed const tag Statement.
fn extract_const_init<'a>(ctx: &mut Ctx<'a>, id: svelte_ast::NodeId) -> Expression<'a> {
    let offset = ctx.node_expr_offset(id);
    let stmt = ctx.parsed.stmts.remove(&offset)
        .expect("const tag stmt missing from parsed.stmts");
    let Statement::VariableDeclaration(mut decl) = stmt else {
        unreachable!("const tag stmt must be VariableDeclaration")
    };
    decl.declarations.remove(0).init.take()
        .expect("const tag declarator must have init expression")
}
