use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{BlockSemantics, FragmentDeclarationAsyncKind};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;
use svelte_component_semantics::{SymbolId, walk_bindings};

use crate::error::{CodegenError, Result};
use crate::model::ServerCodegen;

impl<'a> ServerCodegen<'a> {
    pub(crate) fn declaration_tag(&mut self, id: NodeId) -> Result<()> {
        let svelte_ast::Node::DeclarationTag(tag) = self.component.store.get(id) else {
            return Err(CodegenError::Unsupported(id, "declaration tag"));
        };
        let stmt = self
            .js_arena
            .take_stmt(tag.declaration.id())
            .ok_or(CodegenError::MissingExpression(id))?;
        self.push_stmt(stmt);
        Ok(())
    }

    fn declaration_tag_async_kind(&self, id: NodeId) -> FragmentDeclarationAsyncKind {
        match self.analysis.block_semantics(id) {
            BlockSemantics::DeclarationTag(sem) => sem.async_kind.clone(),
            _ => FragmentDeclarationAsyncKind::Sync,
        }
    }

    pub(crate) fn emit_declaration_tags(&mut self, ids: &[NodeId]) -> Result<()> {
        let mut grouped: Vec<NodeId> = Vec::new();
        for &id in ids {
            if !self.experimental_async {
                self.declaration_tag(id)?;
                continue;
            }
            match self.declaration_tag_async_kind(id) {
                FragmentDeclarationAsyncKind::Sync => self.declaration_tag(id)?,
                FragmentDeclarationAsyncKind::Awaited { .. }
                | FragmentDeclarationAsyncKind::Deferred { .. } => grouped.push(id),
            }
        }
        if !grouped.is_empty() {
            self.emit_declaration_tags_async(&grouped)?;
        }
        Ok(())
    }

    fn emit_declaration_tags_async(&mut self, ids: &[NodeId]) -> Result<()> {
        let promises_name = self.gen_ident("promises");
        let mut thunks: Vec<Expression<'a>> = Vec::new();

        for &id in ids {
            let async_kind = self.declaration_tag_async_kind(id);

            let decl_id = {
                let svelte_ast::Node::DeclarationTag(tag) = self.component.store.get(id) else {
                    return Err(CodegenError::Unsupported(id, "declaration tag"));
                };
                tag.declaration.id()
            };
            let (symbols, assignments) = self.take_declaration_parts(id, decl_id)?;

            let is_awaited = match &async_kind {
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

    fn take_declaration_parts(
        &mut self,
        id: NodeId,
        decl_id: OxcNodeId,
    ) -> Result<(Vec<SymbolId>, Vec<Expression<'a>>)> {
        let stmt = self
            .js_arena
            .take_stmt(decl_id)
            .ok_or(CodegenError::MissingExpression(id))?;
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return Err(CodegenError::Unsupported(id, "declaration tag declaration"));
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
                .ok_or(CodegenError::Unsupported(id, "declaration tag pattern"))?;
            assignments.push(self.b.assign_expr_raw(target, init));
        }

        for name in &declared {
            self.push_stmt(self.b.let_stmt(name));
        }
        Ok((symbols, assignments))
    }

    pub(crate) fn build_declaration_thunk(
        &self,
        mut assignments: Vec<Expression<'a>>,
        is_awaited: bool,
    ) -> Expression<'a> {
        if assignments.len() == 1 {
            let single = assignments.remove(0);
            if is_awaited {
                return self.b.async_arrow_expr_body(single);
            }
            return self.b.thunk(single);
        }
        let stmts: Vec<Statement<'a>> = assignments
            .into_iter()
            .map(|expr| self.b.expr_stmt(expr))
            .collect();
        if is_awaited {
            return self.b.async_thunk_block(stmts);
        }
        self.b.thunk_block(stmts)
    }
}
