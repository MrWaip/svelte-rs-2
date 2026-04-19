//! `{@const}` codegen — fragment-level pre-pass before DOM init.
//!
//! Consumer form: Plan → builders → emit. One `block_semantics(id)`
//! query per tag resolves every declaration-shape decision; emit-time
//! AST clone from `sem.stmt_handle` is the only remaining AST read.

// File was originally prefixed with `#![allow(deprecated)]` while the
// migration was in flight. After this slice's consumer migration landed
// the const-tag path reads exclusively through block_semantics — no
// deprecated accessors remain in this module.
use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_analyze::{
    BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics, FragmentKey, StmtHandle,
};
use svelte_ast::NodeId;

use crate::context::Ctx;
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

/// Emit ConstTag declarations for one fragment.
///
/// Returns `Some(var promises = $.run([...]))` when the fragment enters
/// async mode — any tag in this fragment carries a `ConstTagAsyncKind::Async`
/// under `experimental_async`. The caller splices the returned statement
/// after the emitted declarations.
pub(crate) fn gen_const_tags<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    stmts: &mut Vec<Statement<'a>>,
) -> Option<Statement<'a>> {
    let ids = ctx.const_tags_for_fragment(&key).cloned()?;
    let plan = ConstTagsPlan::build(ctx, &ids);
    if plan.items.is_empty() {
        return None;
    }
    match plan.mode {
        ConstTagsMode::Sync => {
            emit_sync(ctx, &plan, stmts);
            None
        }
        ConstTagsMode::Async => emit_async(ctx, &plan, stmts),
    }
}

// -- Plan ---------------------------------------------------------------

struct ConstTagsPlan {
    items: Vec<ConstTagItem>,
    mode: ConstTagsMode,
}

enum ConstTagsMode {
    Sync,
    Async,
}

struct ConstTagItem {
    id: NodeId,
    bindings: Vec<SymbolId>,
    is_destructured: bool,
    stmt_handle: StmtHandle,
    async_kind: ConstTagAsyncKind,
}

impl ConstTagsPlan {
    fn build(ctx: &Ctx<'_>, ids: &[NodeId]) -> Self {
        let mut items: Vec<ConstTagItem> = Vec::with_capacity(ids.len());
        for &id in ids {
            // Silence legacy AnalysisData accessors while the migration
            // is in flight — swallow tags without a BlockSemantics entry.
            let sem: ConstTagBlockSemantics = match ctx.query.analysis.block_semantics(id) {
                BlockSemantics::ConstTag(s) => s.clone(),
                _ => continue,
            };
            items.push(ConstTagItem {
                id,
                bindings: sem.bindings.to_vec(),
                is_destructured: sem.is_destructured,
                stmt_handle: sem.stmt_handle,
                async_kind: sem.async_kind.clone(),
            });
        }
        let mode = if ctx.state.experimental_async
            && items
                .iter()
                .any(|it| matches!(it.async_kind, ConstTagAsyncKind::Async { .. }))
        {
            ConstTagsMode::Async
        } else {
            ConstTagsMode::Sync
        };
        Self { items, mode }
    }
}

// -- Sync emission ------------------------------------------------------

fn emit_sync<'a>(ctx: &mut Ctx<'a>, plan: &ConstTagsPlan, stmts: &mut Vec<Statement<'a>>) {
    for item in &plan.items {
        let init_expr = take_init(ctx, item.stmt_handle, item.id);
        if item.is_destructured {
            build_sync_destructured(ctx, item, init_expr, stmts);
        } else {
            build_sync_simple(ctx, item, init_expr, stmts);
        }
    }
}

fn build_sync_simple<'a>(
    ctx: &mut Ctx<'a>,
    item: &ConstTagItem,
    init_expr: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let name = binding_name(ctx, item.bindings[0]);
    let thunk = ctx.b.thunk(init_expr);
    let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);

    let final_expr = if ctx.state.dev {
        let name_str = ctx.b.alloc_str(&name);
        ctx.b
            .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef(name_str)])
    } else {
        derived
    };

    stmts.push(ctx.b.const_stmt(&name, final_expr));

    if ctx.state.dev {
        let name_str = ctx.b.alloc_str(&name);
        stmts.push(ctx.b.call_stmt("$.get", [Arg::Ident(name_str)]));
    }
}

fn build_sync_destructured<'a>(
    ctx: &mut Ctx<'a>,
    item: &ConstTagItem,
    init_expr: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let tmp_name = tmp_name(ctx, item.id);
    let names = binding_names(ctx, &item.bindings);
    let tmp_ref: &str = ctx.b.alloc_str(&tmp_name);

    let destruct_stmt = ctx.b.const_object_destruct_stmt(&names, init_expr);
    let props: Vec<ObjProp<'a>> = names
        .iter()
        .map(|n| ObjProp::Shorthand(ctx.b.alloc_str(n)))
        .collect();
    let ret = ctx.b.return_stmt(ctx.b.object_expr(props));
    let thunk = ctx
        .b
        .arrow_block_expr(ctx.b.no_params(), [destruct_stmt, ret]);
    let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);

    let final_expr = if ctx.state.dev {
        ctx.b
            .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef("[@const]")])
    } else {
        derived
    };

    stmts.push(ctx.b.const_stmt(tmp_ref, final_expr));
}

// -- Async emission -----------------------------------------------------

fn emit_async<'a>(
    ctx: &mut Ctx<'a>,
    plan: &ConstTagsPlan,
    stmts: &mut Vec<Statement<'a>>,
) -> Option<Statement<'a>> {
    let promises_name = ctx.gen_ident("promises");
    let mut thunks: Vec<Expression<'a>> = Vec::new();

    for item in &plan.items {
        let (has_await, blockers_slice) = match &item.async_kind {
            ConstTagAsyncKind::Async {
                has_await,
                blockers,
            } => (*has_await, blockers.as_slice()),
            // Sync-items inside an async fragment still go through the
            // `$.run([...])` pack — mirror the legacy behaviour where
            // async mode pulls every tag into the same pipeline.
            ConstTagAsyncKind::Sync => (false, &[][..]),
        };

        let init_expr = take_init(ctx, item.stmt_handle, item.id);

        if item.is_destructured {
            build_async_destructured(
                ctx,
                item,
                init_expr,
                has_await,
                blockers_slice,
                &promises_name,
                &mut thunks,
                stmts,
            );
        } else {
            build_async_simple(
                ctx,
                item,
                init_expr,
                has_await,
                blockers_slice,
                &promises_name,
                &mut thunks,
                stmts,
            );
        }
    }

    if thunks.is_empty() {
        return None;
    }
    let thunks_array = ctx.b.array_expr(thunks);
    let run_call = ctx.b.call_expr("$.run", [Arg::Expr(thunks_array)]);
    Some(ctx.b.var_stmt(&promises_name, run_call))
}

#[allow(clippy::too_many_arguments)]
fn build_async_simple<'a>(
    ctx: &mut Ctx<'a>,
    item: &ConstTagItem,
    init_expr: Expression<'a>,
    has_await: bool,
    blockers: &[u32],
    promises_name: &str,
    thunks: &mut Vec<Expression<'a>>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let name = binding_name(ctx, item.bindings[0]);
    stmts.push(ctx.b.let_stmt(&name));
    build_blocker_thunks(ctx, blockers, thunks);

    let derived = create_derived(ctx, init_expr, has_await);
    let final_derived = if ctx.state.dev {
        let name_str = ctx.b.alloc_str(&name);
        ctx.b
            .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef(name_str)])
    } else {
        derived
    };

    let lhs = AssignLeft::Ident(name.clone());
    let assignment = ctx.b.assign_expr(lhs, final_derived);
    let body = if ctx.state.dev {
        let name_str = ctx.b.alloc_str(&name);
        let get_call = ctx.b.call_stmt("$.get", [Arg::Ident(name_str)]);
        let assign_stmt = ctx.b.expr_stmt(assignment);
        let body_stmts = vec![assign_stmt, get_call];
        if has_await {
            ctx.b.async_thunk_block(body_stmts)
        } else {
            ctx.b.thunk_block(body_stmts)
        }
    } else if has_await {
        ctx.b.async_thunk(assignment)
    } else {
        ctx.b.thunk(assignment)
    };
    thunks.push(body);

    let thunk_idx = thunks.len() - 1;
    if let Some(&sym_id) = item.bindings.first() {
        ctx.const_tag_blockers
            .insert(sym_id, (promises_name.to_string(), thunk_idx));
    }
}

#[allow(clippy::too_many_arguments)]
fn build_async_destructured<'a>(
    ctx: &mut Ctx<'a>,
    item: &ConstTagItem,
    init_expr: Expression<'a>,
    has_await: bool,
    blockers: &[u32],
    promises_name: &str,
    thunks: &mut Vec<Expression<'a>>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let tmp_name = tmp_name(ctx, item.id);
    let names = binding_names(ctx, &item.bindings);

    stmts.push(ctx.b.let_stmt(&tmp_name));
    build_blocker_thunks(ctx, blockers, thunks);

    let destruct_stmt = ctx.b.const_object_destruct_stmt(&names, init_expr);
    let props: Vec<ObjProp<'a>> = names
        .iter()
        .map(|n| ObjProp::Shorthand(ctx.b.alloc_str(n)))
        .collect();
    let ret = ctx.b.return_stmt(ctx.b.object_expr(props));
    let block_arrow = ctx
        .b
        .arrow_block_expr(ctx.b.no_params(), [destruct_stmt, ret]);
    let derived = create_derived(ctx, block_arrow, has_await);

    let final_derived = if ctx.state.dev {
        ctx.b
            .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef("[@const]")])
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
    for &sym_id in &item.bindings {
        ctx.const_tag_blockers
            .insert(sym_id, (promises_name.to_string(), thunk_idx));
    }
}

// -- Shared helpers -----------------------------------------------------

/// `() => $$promises[N].promise` / `() => $.wait([$$promises[i], ...])`.
fn build_blocker_thunks<'a>(ctx: &mut Ctx<'a>, blockers: &[u32], thunks: &mut Vec<Expression<'a>>) {
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
        let arr_elements: Vec<Expression<'a>> = blockers
            .iter()
            .map(|&idx| {
                ctx.b
                    .computed_member_expr(ctx.b.rid_expr("$$promises"), ctx.b.num_expr(idx as f64))
            })
            .collect();
        let arr = ctx.b.array_expr(arr_elements);
        let wait_call = ctx.b.call_expr("$.wait", [Arg::Expr(arr)]);
        thunks.push(ctx.b.thunk(wait_call));
    }
}

fn create_derived<'a>(ctx: &mut Ctx<'a>, init: Expression<'a>, has_await: bool) -> Expression<'a> {
    if has_await {
        let thunk = ctx.b.async_thunk(init);
        ctx.b.call_expr("$.async_derived", [Arg::Expr(thunk)])
    } else {
        let thunk = ctx.b.thunk(init);
        ctx.b.call_expr("$.derived", [Arg::Expr(thunk)])
    }
}

/// Take the init expression out of the pre-parsed statement. After this
/// the handle is consumed — only the emit path is allowed to call this.
fn take_init<'a>(ctx: &mut Ctx<'a>, handle: StmtHandle, id: NodeId) -> Expression<'a> {
    let stmt = ctx
        .state
        .parsed
        .take_stmt(handle)
        .unwrap_or_else(|| panic!("const tag stmt missing for {id:?}"));
    let Statement::VariableDeclaration(mut decl) = stmt else {
        unreachable!("const tag stmt must be VariableDeclaration")
    };
    decl.declarations
        .remove(0)
        .init
        .take()
        .expect("const tag declarator must have init")
}

/// Pull-or-allocate the tmp name for a destructured const tag. The name
/// is written by `svelte_transform` before codegen starts; reading it
/// here (rather than calling `gen_ident`) keeps counter ordering
/// deterministic with transform-time references.
fn tmp_name(ctx: &Ctx<'_>, id: NodeId) -> String {
    ctx.transform_data
        .const_tag_tmp_names
        .get(&id)
        .cloned()
        .unwrap_or_else(|| panic!("destructured const tag missing tmp_name for {id:?}"))
}

fn binding_name(ctx: &Ctx<'_>, sym: SymbolId) -> String {
    ctx.query.view.symbol_name(sym).to_string()
}

fn binding_names(ctx: &Ctx<'_>, syms: &[SymbolId]) -> Vec<String> {
    syms.iter().map(|&s| binding_name(ctx, s)).collect()
}
