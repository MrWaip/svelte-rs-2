use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_ast::NodeId;
use svelte_component_semantics::walk_bindings;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_hoisted_declaration_tag(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
    ) -> Result<()> {
        let svelte_ast::Node::DeclarationTag(tag) = self.ctx.query.component.store.get(id) else {
            return CodegenError::unexpected_child("declaration tag", "statements");
        };
        let Some(stmt) = self.ctx.state.parsed.take_stmt(tag.declaration.id()) else {
            return CodegenError::unexpected_child("declaration tag statement", "statements");
        };
        state.init.push(stmt);
        Ok(())
    }

    pub(in crate::codegen) fn declaration_tag_group_parts(
        &mut self,
        state: &mut EmitState<'a>,
        id: NodeId,
    ) -> Result<(Vec<SymbolId>, Vec<Expression<'a>>)> {
        let svelte_ast::Node::DeclarationTag(tag) = self.ctx.query.component.store.get(id) else {
            return CodegenError::unexpected_child("declaration tag", "statements");
        };
        let decl_stmt_id = tag.declaration.id();
        let Some(stmt) = self.ctx.state.parsed.take_stmt(decl_stmt_id) else {
            return CodegenError::unexpected_child("declaration tag statement", "statements");
        };
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return CodegenError::unexpected_child("declaration tag statement", "statements");
        };

        let mut symbols: Vec<SymbolId> = Vec::new();
        let mut assignments: Vec<Expression<'a>> = Vec::new();
        for declarator in decl.declarations.drain(..) {
            walk_bindings(&declarator.id, |v| {
                let name = self.ctx.query.view.symbol_name(v.symbol).to_string();
                if symbols.contains(&v.symbol) {
                    return;
                }
                symbols.push(v.symbol);
                state
                    .init
                    .push(self.ctx.b.let_stmt(self.ctx.b.alloc_str(&name)));
            });

            let Some(init) = declarator.init else {
                continue;
            };
            let Some(target) = self
                .ctx
                .b
                .binding_pattern_to_assignment_target(declarator.id)
            else {
                return CodegenError::unexpected_child(
                    "declaration tag pattern",
                    "assignment target",
                );
            };
            assignments.push(self.ctx.b.assign_expr_raw(target, init));
        }
        Ok((symbols, assignments))
    }

    pub(in crate::codegen) fn build_declaration_thunk(
        &mut self,
        mut assignments: Vec<Expression<'a>>,
        is_awaited: bool,
    ) -> Expression<'a> {
        if assignments.len() == 1 {
            let single = assignments.remove(0);
            if is_awaited {
                return self.ctx.b.async_arrow_expr_body(single);
            }
            return self.ctx.b.thunk(single);
        }
        let stmts: Vec<Statement<'a>> = assignments
            .into_iter()
            .map(|expr| self.ctx.b.expr_stmt(expr))
            .collect();
        if is_awaited {
            return self.ctx.b.async_thunk_block(stmts);
        }
        self.ctx.b.thunk_block(stmts)
    }
}
