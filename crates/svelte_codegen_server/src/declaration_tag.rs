use oxc_ast::ast::{AssignmentOperator, AssignmentTarget, Expression, Statement};
use oxc_span::SPAN;
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
        let first_async = if self.experimental_async {
            ids.iter()
                .position(|&id| self.declaration_tag_async_kind(id).is_async())
        } else {
            None
        };
        match first_async {
            None => {
                for &id in ids {
                    self.declaration_tag(id)?;
                }
            }
            Some(idx) => {
                for &id in &ids[..idx] {
                    self.declaration_tag(id)?;
                }
                self.emit_declaration_tags_async(&ids[idx..])?;
            }
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
            let (symbol, target, value) = self.take_declaration_parts(id, decl_id)?;

            self.push_stmt(self.b.let_stmt(&target));

            let target_atom = self.b.ast.atom(&target);
            let assignment_target = AssignmentTarget::AssignmentTargetIdentifier(
                self.b
                    .alloc(self.b.ast.identifier_reference(SPAN, target_atom)),
            );
            let assignment = self.b.ast.expression_assignment(
                SPAN,
                AssignmentOperator::Assign,
                assignment_target,
                value,
            );

            let body = match &async_kind {
                FragmentDeclarationAsyncKind::Awaited { blockers } => {
                    self.push_blocker_thunk(blockers, &mut thunks);
                    self.b.async_arrow_expr_body(assignment)
                }
                FragmentDeclarationAsyncKind::Deferred { blockers } => {
                    self.push_blocker_thunk(blockers, &mut thunks);
                    self.b.thunk(assignment)
                }
                FragmentDeclarationAsyncKind::Sync => self.b.thunk(assignment),
            };
            thunks.push(body);
            let thunk_idx = (thunks.len() - 1) as u32;
            self.const_tag_blockers
                .insert(symbol, (promises_name.clone(), thunk_idx));
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
    ) -> Result<(SymbolId, String, Expression<'a>)> {
        let stmt = self
            .js_arena
            .take_stmt(decl_id)
            .ok_or(CodegenError::MissingExpression(id))?;
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return Err(CodegenError::Unsupported(id, "declaration tag declaration"));
        };
        if decl.declarations.is_empty() {
            return Err(CodegenError::Unsupported(id, "declaration tag declarator"));
        }
        let declarator = decl.declarations.remove(0);

        let mut targets: Vec<(SymbolId, bool)> = Vec::new();
        walk_bindings(&declarator.id, |v| {
            targets.push((v.symbol, v.path.is_empty() && !v.is_rest));
        });
        let [(symbol, true)] = targets.as_slice() else {
            return Err(CodegenError::Unsupported(
                id,
                "destructured async declaration",
            ));
        };
        let symbol = *symbol;
        let name = self.analysis.scoping.symbol_name(symbol).to_string();
        let value = declarator.init.ok_or(CodegenError::MissingExpression(id))?;
        Ok((symbol, name, value))
    }
}
