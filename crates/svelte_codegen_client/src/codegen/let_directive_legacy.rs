use oxc_ast::ast::{BindingPattern, Statement};
use svelte_ast::{Attribute, NodeId};

use crate::codegen::binding_pattern::{BindingPatternOutput, BindingPatternSource};

use super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_let_directive_legacy_stmts(
        &mut self,
        owner_id: NodeId,
    ) -> Result<Vec<Statement<'a>>> {
        let let_dirs: Vec<svelte_ast::LetDirectiveLegacy> = self
            .ctx
            .query
            .component
            .store
            .get(owner_id)
            .attributes()
            .iter()
            .filter_map(|a| match a {
                Attribute::LetDirectiveLegacy(d) => Some(d.clone()),
                _ => None,
            })
            .collect();
        let mut out: Vec<Statement<'a>> = Vec::new();
        for dir in &let_dirs {
            let stmts = self.emit_let_directive_legacy(dir)?;
            out.extend(stmts);
        }
        Ok(out)
    }

    fn emit_let_directive_legacy(
        &mut self,
        dir: &svelte_ast::LetDirectiveLegacy,
    ) -> Result<Vec<Statement<'a>>> {
        let Some(binding_ref) = dir.binding.as_ref() else {
            return Ok(Vec::new());
        };
        let stmt_id = binding_ref.id();
        let stmt = match self.ctx.state.parsed.take_stmt(stmt_id) {
            Some(s) => s,
            None => return Ok(Vec::new()),
        };
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return Ok(Vec::new());
        };
        let decl_oxc_node_id = decl.node_id();
        if decl.declarations.is_empty() {
            return Ok(Vec::new());
        }
        let declarator = decl.declarations.remove(0);
        let slot_prop_name: &'a str = self.ctx.b.alloc_str(&dir.name);
        let pattern_ref: &'a BindingPattern<'a> = self.ctx.b.ast.allocator.alloc(declarator.id);
        let out = self.emit_binding_pattern(
            decl_oxc_node_id,
            BindingPatternSource::LetCarrier {
                slot_prop_name,
                pattern: pattern_ref,
            },
        )?;
        let BindingPatternOutput::Statements(stmts) = out else {
            return CodegenError::unexpected_child(
                "let carrier statements",
                "other binding output",
            );
        };
        Ok(stmts)
    }
}
