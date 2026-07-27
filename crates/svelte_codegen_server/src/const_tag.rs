use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use std::mem;
use svelte_analyze::{BlockSemantics, FragmentDeclarationAsyncKind};
use svelte_ast::{FragmentId, NodeId};
use svelte_ast_builder::Arg;
use svelte_component_semantics::{SymbolId, walk_bindings};

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

pub(crate) struct DeclarationGroupRun<'a> {
    name: String,
    thunks: Vec<Expression<'a>>,
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn const_tag(&mut self, id: NodeId) -> Result<()> {
        let sem = match self.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(sem) => sem.clone(),
            _ => return Err(CodegenError::Unsupported(id, "const tag")),
        };
        if !matches!(sem.async_kind, FragmentDeclarationAsyncKind::Sync) {
            return Err(CodegenError::Unsupported(id, "async const tag"));
        }
        let stmt = self
            .js_arena
            .take_stmt(sem.decl_node_id)
            .ok_or(CodegenError::MissingExpression(id))?;
        self.push_stmt(stmt);
        Ok(())
    }

    pub(crate) fn prepare_declaration_groups(&mut self) {
        if !self.experimental_async {
            return;
        }
        let order: Vec<FragmentId> = self.analysis.fragment_declaration_group_order().to_vec();
        for fragment in order {
            let members: Vec<NodeId> = self.analysis.fragment_declaration_group(fragment).to_vec();
            if members.is_empty() {
                continue;
            }
            let promises_name = self.gen_ident("promises");
            let mut slot = 0u32;
            for id in members {
                if self.declaration_has_blockers(id) {
                    slot += 1;
                }
                self.declaration_blocker_slots
                    .insert(id, (promises_name.clone(), slot));
                slot += 1;
            }
            self.declaration_group_idents
                .insert(fragment, promises_name);
        }
    }

    fn declaration_has_blockers(&self, id: NodeId) -> bool {
        match self.fragment_declaration_async_kind(id) {
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

    pub(crate) fn build_fragment_declaration_group(
        &mut self,
        fragment_id: FragmentId,
        ids: &[NodeId],
    ) -> Result<Option<DeclarationGroupRun<'a>>> {
        let Some(promises_name) = self.declaration_group_idents.get(&fragment_id).cloned() else {
            return Err(CodegenError::Unsupported(
                ids[0],
                "fragment declaration group",
            ));
        };
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let async_kind = self.fragment_declaration_async_kind(id);
            let (symbols, assignments) = match self.component.store.get(id) {
                svelte_ast::Node::ConstTag(_) => {
                    let BlockSemantics::ConstTag(sem) = self.analysis.block_semantics(id) else {
                        return Err(CodegenError::Unsupported(id, "const tag"));
                    };
                    let decl_node_id = sem.decl_node_id;
                    self.take_const_parts(id, decl_node_id)?
                }
                svelte_ast::Node::DeclarationTag(_) => {
                    let decl_id = self.declaration_tag_statement_id(id)?;
                    self.take_declaration_parts(id, decl_id)?
                }
                _ => return Err(CodegenError::Unsupported(id, "fragment declaration")),
            };

            let is_awaited = match &async_kind {
                FragmentDeclarationAsyncKind::Awaited {
                    blockers,
                    declaration_blockers,
                } => {
                    self.push_blocker_thunk(blockers, declaration_blockers, &mut thunks);
                    true
                }
                FragmentDeclarationAsyncKind::Deferred {
                    blockers,
                    declaration_blockers,
                } => {
                    self.push_blocker_thunk(blockers, declaration_blockers, &mut thunks);
                    false
                }
                FragmentDeclarationAsyncKind::Sync => false,
            };

            thunks.push(self.build_declaration_thunk(assignments, is_awaited));
            let thunk_idx = (thunks.len() - 1) as u32;
            for symbol in symbols {
                self.const_tag_blockers
                    .insert(symbol, (promises_name.clone(), thunk_idx));
            }
        }

        for name in mem::take(&mut self.pending_group_declarations) {
            self.push_stmt(self.b.let_stmt(&name));
        }
        if thunks.is_empty() {
            return Ok(None);
        }
        Ok(Some(DeclarationGroupRun {
            name: promises_name,
            thunks,
        }))
    }

    pub(crate) fn push_declaration_group(&mut self, group: Option<DeclarationGroupRun<'a>>) {
        let Some(group) = group else {
            return;
        };
        let run = self.b.call_expr(
            "$$renderer.run",
            [Arg::Expr(self.b.array_expr(group.thunks))],
        );
        self.hoist_stmt(self.b.var_stmt(&group.name, run));
    }

    fn take_const_parts(
        &mut self,
        id: NodeId,
        decl_id: OxcNodeId,
    ) -> Result<(Vec<SymbolId>, Vec<Expression<'a>>)> {
        let stmt = self
            .js_arena
            .take_stmt(decl_id)
            .ok_or(CodegenError::MissingExpression(id))?;
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return Err(CodegenError::Unsupported(id, "const tag declaration"));
        };

        let mut symbols: Vec<SymbolId> = Vec::new();
        let mut assignments: Vec<Expression<'a>> = Vec::new();
        let mut declared: Vec<String> = Vec::new();
        for declarator in decl.declarations.drain(..) {
            walk_bindings(&declarator.id, |v| {
                if symbols.contains(&v.symbol) {
                    return;
                }
                symbols.push(v.symbol);
                declared.push(self.analysis.scoping.symbol_name(v.symbol).to_string());
            });

            let Some(init) = declarator.init else {
                continue;
            };
            let target = self
                .b
                .binding_pattern_to_assignment_target(declarator.id)
                .ok_or(CodegenError::Unsupported(id, "const tag pattern"))?;
            assignments.push(self.b.assign_expr_raw(target, init));
        }

        self.pending_group_declarations.extend(declared);
        Ok((symbols, assignments))
    }

    pub(crate) fn push_blocker_thunk(
        &self,
        blockers: &[u32],
        declaration_blockers: &[NodeId],
        thunks: &mut Vec<Expression<'a>>,
    ) {
        let mut members: Vec<Expression<'a>> = Vec::new();
        for &idx in blockers {
            members.push(self.blocker_member(idx));
        }
        for id in declaration_blockers {
            let Some((name, idx)) = self.declaration_blocker_slots.get(id) else {
                continue;
            };
            members.push(
                self.b
                    .computed_member_expr(self.b.rid_expr(name), self.b.num_expr(*idx as f64)),
            );
        }
        if members.is_empty() {
            return;
        }
        let expr = if members.len() == 1 {
            members.remove(0)
        } else {
            self.b
                .call_expr("Promise.all", [Arg::Expr(self.b.array_expr(members))])
        };
        thunks.push(self.b.thunk(expr));
    }
}
