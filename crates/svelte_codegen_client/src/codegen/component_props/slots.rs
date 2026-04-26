use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, FragmentEmitKind, Result};
use oxc_ast::ast::{Expression, Statement};
use svelte_ast::NodeId;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_component_default_children(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        fragment: svelte_ast::FragmentId,
    ) -> Result<Option<Expression<'a>>> {
        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            fragment,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        inner_state.skip_snippets = true;
        match self.emit_fragment(&mut inner_state, &inner_ctx, fragment)? {
            FragmentEmitKind::Empty => return Ok(None),
            FragmentEmitKind::Rendered => {}
        }
        let body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;

        let arrow = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor", "$$slotProps"]), body);
        Ok(Some(arrow))
    }

    pub(in super::super) fn build_component_default_children_with_let(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        fragment: svelte_ast::FragmentId,
    ) -> Result<Option<Expression<'a>>> {
        let let_stmts = self.emit_let_directive_legacy_stmts(el_id);

        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            fragment,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        for stmt in let_stmts {
            inner_state.init.push(stmt);
        }
        match self.emit_fragment(&mut inner_state, &inner_ctx, fragment)? {
            FragmentEmitKind::Empty => return Ok(None),
            FragmentEmitKind::Rendered => {}
        }
        let body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;

        let arrow = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor", "$$slotProps"]), body);
        Ok(Some(arrow))
    }
}
