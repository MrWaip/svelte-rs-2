use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{BlockSemantics, ConstTagBlockSemantics, FragmentDeclarationAsyncKind};
use svelte_ast::{FragmentId, NodeId};
use svelte_ast_builder::{Arg, AssignLeft};
use svelte_component_semantics::{SymbolId, walk_bindings};

use crate::codegen::binding_pattern::{BindingPatternOutput, BindingPatternSource};
use crate::context::Ctx;

use super::super::data_structures::{EmitState, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_hoisted_const_tag(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        id: NodeId,
    ) -> Result<()> {
        let sem = match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(s) => s.clone(),
            _ => return CodegenError::unexpected_block_semantics(id, "ConstTag expected"),
        };

        self.emit_const_tag_sync(state, id, sem)
    }

    fn emit_const_tag_sync(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
        sem: ConstTagBlockSemantics,
    ) -> Result<()> {
        let out =
            self.emit_binding_pattern(sem.decl_node_id, BindingPatternSource::ConstTag { id })?;
        let BindingPatternOutput::ConstTagDerived(d) = out else {
            return CodegenError::unexpected_child("const tag derived", "statements");
        };
        state.init.push(self.ctx.b.const_stmt(d.target, d.derived));
        if self.ctx.state.dev {
            state
                .init
                .push(self.ctx.b.call_stmt("$.get", [Arg::Ident(d.target)]));
        }
        Ok(())
    }

    pub(in crate::codegen) fn emit_fragment_declaration_group(
        &mut self,
        state: &mut EmitState<'a>,
        fragment_id: FragmentId,
        ids: &[NodeId],
    ) -> Result<()> {
        let Some(promises_name) = self.ctx.declaration_group_idents.get(&fragment_id).cloned()
        else {
            return CodegenError::unexpected_child("fragment declaration group", "statements");
        };
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let async_kind = self.fragment_declaration_async_kind(id);
            match &async_kind {
                FragmentDeclarationAsyncKind::Awaited {
                    blockers,
                    declaration_blockers,
                }
                | FragmentDeclarationAsyncKind::Deferred {
                    blockers,
                    declaration_blockers,
                } => build_blocker_thunks(self.ctx, blockers, declaration_blockers, &mut thunks),
                FragmentDeclarationAsyncKind::Sync => {}
            }
            let is_awaited = matches!(async_kind, FragmentDeclarationAsyncKind::Awaited { .. });

            let (symbols, assignments) = match self.ctx.query.component.store.get(id) {
                svelte_ast::Node::ConstTag(_) => self.const_tag_group_parts(state, id)?,
                svelte_ast::Node::DeclarationTag(_) => {
                    self.declaration_tag_group_parts(state, id)?
                }
                _ => return CodegenError::unexpected_child("fragment declaration", "statements"),
            };

            thunks.push(self.build_declaration_thunk(assignments, is_awaited));
            let thunk_idx = thunks.len() - 1;
            for sym_id in symbols {
                self.ctx
                    .const_tag_blockers
                    .insert(sym_id, (promises_name.clone(), thunk_idx));
            }
        }

        if !thunks.is_empty() {
            let thunks_array = self.ctx.b.array_expr(thunks);
            let run_call = self.ctx.b.call_expr("$.run", [Arg::Expr(thunks_array)]);
            state
                .init
                .push(self.ctx.b.var_stmt(&promises_name, run_call));
        }
        Ok(())
    }

    pub(in crate::codegen) fn fragment_declaration_async_kind(
        &self,
        id: NodeId,
    ) -> FragmentDeclarationAsyncKind {
        match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(sem) => sem.async_kind.clone(),
            BlockSemantics::DeclarationTag(sem) => sem.async_kind.clone(),
            _ => FragmentDeclarationAsyncKind::Sync,
        }
    }

    fn const_tag_group_parts(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
    ) -> Result<(Vec<SymbolId>, Vec<Expression<'a>>)> {
        let sem: ConstTagBlockSemantics = match self.ctx.query.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(s) => s.clone(),
            _ => return CodegenError::unexpected_block_semantics(id, "ConstTag expected"),
        };
        let out =
            self.emit_binding_pattern(sem.decl_node_id, BindingPatternSource::ConstTag { id })?;
        let BindingPatternOutput::ConstTagDerived(d) = out else {
            return CodegenError::unexpected_child("const tag derived", "statements");
        };
        state.init.push(self.ctx.b.let_stmt(d.target));
        let assignment = self
            .ctx
            .b
            .assign_expr(AssignLeft::Ident(d.target.to_string()), d.derived);
        Ok((d.symbols.to_vec(), vec![assignment]))
    }
}

pub fn prepare_declaration_groups(ctx: &mut Ctx<'_>) {
    if !ctx.state.experimental_async {
        return;
    }
    let order: Vec<FragmentId> = ctx
        .query
        .analysis
        .fragment_declaration_group_order()
        .to_vec();
    for fragment in order {
        let members: Vec<NodeId> = ctx
            .query
            .analysis
            .fragment_declaration_group(fragment)
            .to_vec();
        if members.is_empty() {
            continue;
        }
        let promises_name = ctx.gen_ident("promises");
        let mut slot = 0usize;
        for id in members {
            if declaration_has_blockers(ctx, id) {
                slot += 1;
            }
            ctx.declaration_blocker_slots
                .insert(id, (promises_name.clone(), slot));
            for symbol in declaration_member_symbols(ctx, id) {
                ctx.const_tag_blockers
                    .insert(symbol, (promises_name.clone(), slot));
            }
            slot += 1;
        }
        ctx.declaration_group_idents.insert(fragment, promises_name);
    }
}

fn declaration_member_symbols(ctx: &Ctx<'_>, id: NodeId) -> Vec<SymbolId> {
    let decl_id = match ctx.query.analysis.block_semantics(id) {
        BlockSemantics::ConstTag(sem) => sem.decl_node_id,
        BlockSemantics::DeclarationTag(sem) => sem.decl_node_id,
        _ => return Vec::new(),
    };
    let Some(Statement::VariableDeclaration(decl)) = ctx.state.parsed.stmt(decl_id) else {
        return Vec::new();
    };
    let mut symbols: Vec<SymbolId> = Vec::new();
    for declarator in &decl.declarations {
        walk_bindings(&declarator.id, |v| {
            if symbols.contains(&v.symbol) {
                return;
            }
            symbols.push(v.symbol);
        });
    }
    symbols
}

fn declaration_has_blockers(ctx: &Ctx<'_>, id: NodeId) -> bool {
    let async_kind = match ctx.query.analysis.block_semantics(id) {
        BlockSemantics::ConstTag(sem) => &sem.async_kind,
        BlockSemantics::DeclarationTag(sem) => &sem.async_kind,
        _ => return false,
    };
    match async_kind {
        FragmentDeclarationAsyncKind::Sync => false,
        FragmentDeclarationAsyncKind::Awaited {
            blockers,
            declaration_blockers,
        }
        | FragmentDeclarationAsyncKind::Deferred {
            blockers,
            declaration_blockers,
        } => !blockers.is_empty() || !declaration_blockers.is_empty(),
    }
}

pub(super) fn build_blocker_thunks<'a>(
    ctx: &mut Ctx<'a>,
    blockers: &[u32],
    declaration_blockers: &[NodeId],
    thunks: &mut Vec<Expression<'a>>,
) {
    let mut members: Vec<Expression<'a>> = Vec::new();
    for &idx in blockers {
        members.push(
            ctx.b
                .computed_member_expr(ctx.b.rid_expr("$$promises"), ctx.b.num_expr(idx as f64)),
        );
    }
    for id in declaration_blockers {
        let Some(slot) = ctx.declaration_blocker_slots.get(id).cloned() else {
            continue;
        };
        members.push(ctx.blocker_slot_expr(&slot));
    }
    if members.is_empty() {
        return;
    }
    if members.len() == 1 {
        let member = members.remove(0);
        let promise_access = ctx.b.static_member_expr(member, "promise");
        thunks.push(ctx.b.thunk(promise_access));
        return;
    }
    let arr = ctx.b.array_expr(members);
    let wait_call = ctx.b.call_expr("$.wait", [Arg::Expr(arr)]);
    thunks.push(ctx.b.thunk(wait_call));
}
