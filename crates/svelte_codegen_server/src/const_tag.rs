use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{BlockSemantics, ConstTagBlockSemantics, FragmentDeclarationAsyncKind};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;
use svelte_component_semantics::{SymbolId, walk_bindings};

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

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

    pub(crate) fn emit_const_tags_async(&mut self, ids: &[NodeId]) -> Result<()> {
        let promises_name = self.gen_ident("promises");
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let sem: ConstTagBlockSemantics = match self.analysis.block_semantics(id) {
                BlockSemantics::ConstTag(s) => s.clone(),
                _ => continue,
            };
            let (symbols, assignments) = self.take_const_parts(id, sem.decl_node_id)?;

            let is_awaited = match &sem.async_kind {
                FragmentDeclarationAsyncKind::Awaited { blockers } => {
                    self.push_blocker_thunk(blockers, &mut thunks);
                    true
                }
                FragmentDeclarationAsyncKind::Deferred { blockers } => {
                    self.push_blocker_thunk(blockers, &mut thunks);
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

        if !thunks.is_empty() {
            let run = self
                .b
                .call_expr("$$renderer.run", [Arg::Expr(self.b.array_expr(thunks))]);
            self.push_stmt(self.b.var_stmt(&promises_name, run));
        }
        Ok(())
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

        for name in &declared {
            self.push_stmt(self.b.let_stmt(name));
        }
        Ok((symbols, assignments))
    }

    pub(crate) fn push_blocker_thunk(&self, blockers: &[u32], thunks: &mut Vec<Expression<'a>>) {
        if blockers.is_empty() {
            return;
        }
        let expr = if blockers.len() == 1 {
            self.blocker_member(blockers[0])
        } else {
            let elements: Vec<Expression<'a>> = blockers
                .iter()
                .map(|&idx| self.blocker_member(idx))
                .collect();
            self.b
                .call_expr("Promise.all", [Arg::Expr(self.b.array_expr(elements))])
        };
        thunks.push(self.b.thunk(expr));
    }
}
