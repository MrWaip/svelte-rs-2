use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{BlockSemantics, FragmentDeclarationAsyncKind};
use svelte_ast::NodeId;
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

    pub(crate) fn fragment_declaration_async_kind(
        &self,
        id: NodeId,
    ) -> FragmentDeclarationAsyncKind {
        match self.analysis.block_semantics(id) {
            BlockSemantics::ConstTag(sem) => sem.async_kind.clone(),
            BlockSemantics::DeclarationTag(sem) => sem.async_kind.clone(),
            _ => FragmentDeclarationAsyncKind::Sync,
        }
    }

    pub(crate) fn declaration_tag_statement_id(&self, id: NodeId) -> Result<OxcNodeId> {
        let svelte_ast::Node::DeclarationTag(tag) = self.component.store.get(id) else {
            return Err(CodegenError::Unsupported(id, "declaration tag"));
        };
        Ok(tag.declaration.id())
    }

    pub(crate) fn take_declaration_parts(
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
