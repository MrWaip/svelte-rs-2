use oxc_ast::ast::Statement;
use smallvec::SmallVec;
use svelte_ast::{FragmentId, NodeId};
use svelte_component_semantics::{OxcNodeId, SymbolId, walk_bindings};

use super::super::{ExpressionBlocker, FragmentDeclarationAsyncKind};
use super::walker::Ctx;
use crate::expression_semantics::ExpressionSemantics;

pub(super) struct DeclarationOwner {
    pub(super) fragment: FragmentId,
    pub(super) node: NodeId,
    pub(super) is_async: bool,
}

pub(super) fn resolve(
    ctx: &mut Ctx<'_, '_>,
    node_id: NodeId,
    decl_stmt_id: OxcNodeId,
    base: FragmentDeclarationAsyncKind,
) -> FragmentDeclarationAsyncKind {
    let declaration_blockers = outer_group_blockers(ctx, node_id);
    let async_kind = join_group(base, declaration_blockers, ctx.declaration_group_is_open());
    register_owners(ctx, node_id, decl_stmt_id, async_kind.is_async());
    if async_kind.is_async() {
        ctx.push_declaration_group_member(node_id);
    }
    async_kind
}

fn join_group(
    base: FragmentDeclarationAsyncKind,
    declaration_blockers: SmallVec<[NodeId; 2]>,
    group_open: bool,
) -> FragmentDeclarationAsyncKind {
    match base {
        FragmentDeclarationAsyncKind::Awaited { blockers, .. } => {
            FragmentDeclarationAsyncKind::Awaited {
                blockers,
                declaration_blockers,
            }
        }
        FragmentDeclarationAsyncKind::Deferred { blockers, .. } => {
            FragmentDeclarationAsyncKind::Deferred {
                blockers,
                declaration_blockers,
            }
        }
        FragmentDeclarationAsyncKind::Sync => {
            if !group_open && declaration_blockers.is_empty() {
                return FragmentDeclarationAsyncKind::Sync;
            }
            FragmentDeclarationAsyncKind::Deferred {
                blockers: SmallVec::new(),
                declaration_blockers,
            }
        }
    }
}

pub(super) fn expression_blockers(
    ctx: &Ctx<'_, '_>,
    node_id: NodeId,
) -> SmallVec<[ExpressionBlocker; 2]> {
    let ExpressionSemantics::Expression(data) = ctx.expressions.get(node_id) else {
        return SmallVec::new();
    };
    let mut out: SmallVec<[ExpressionBlocker; 2]> = SmallVec::new();
    let mut seen_script: SmallVec<[u32; 2]> = SmallVec::new();
    for sym in &data.blocker_references {
        if let Some(owner) = ctx.declaration_owners.get(sym)
            && owner.is_async
        {
            let blocker = ExpressionBlocker::FragmentDeclaration { node: owner.node };
            if !out.contains(&blocker) {
                out.push(blocker);
            }
            continue;
        }
        let Some(slot) = ctx.blocker_data.symbol_blocker(*sym) else {
            continue;
        };
        if seen_script.contains(&slot.member) {
            continue;
        }
        seen_script.push(slot.member);
        out.push(ExpressionBlocker::Script { entry: slot.entry });
    }
    out
}

fn outer_group_blockers(ctx: &Ctx<'_, '_>, node_id: NodeId) -> SmallVec<[NodeId; 2]> {
    let ExpressionSemantics::Expression(data) = ctx.expressions.get(node_id) else {
        return SmallVec::new();
    };
    let mut blockers: SmallVec<[NodeId; 2]> = SmallVec::new();
    for sym in &data.references {
        let Some(owner) = ctx.declaration_owners.get(sym) else {
            continue;
        };
        if !owner.is_async {
            continue;
        }
        if owner.fragment == ctx.current_fragment_id {
            continue;
        }
        if blockers.contains(&owner.node) {
            continue;
        }
        blockers.push(owner.node);
    }
    blockers
}

fn register_owners(
    ctx: &mut Ctx<'_, '_>,
    node_id: NodeId,
    decl_stmt_id: OxcNodeId,
    is_async: bool,
) {
    let parsed = ctx.parsed;
    let Some(Statement::VariableDeclaration(decl)) = parsed.stmt(decl_stmt_id) else {
        return;
    };
    let mut declared: SmallVec<[SymbolId; 2]> = SmallVec::new();
    for declarator in &decl.declarations {
        walk_bindings(&declarator.id, |v| declared.push(v.symbol));
    }
    let fragment = ctx.current_fragment_id;
    for sym in declared {
        ctx.declaration_owners.insert(
            sym,
            DeclarationOwner {
                fragment,
                node: node_id,
                is_async,
            },
        );
    }
}
