use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, FragmentEmitKind, Result};
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::ElementSemantics;
use svelte_ast::{Node, NodeId};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_component_default_children_with_let(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        fragment: svelte_ast::FragmentId,
    ) -> Result<Option<Expression<'a>>> {
        let (default_wrapper, default_let_scope_owners) =
            match self.ctx.query.analysis.element_semantics.query(el_id) {
                ElementSemantics::LegacyComponentSlots(sem) => {
                    (sem.default_wrapper, sem.default_let_scope_owners.clone())
                }
                _ => (None, Default::default()),
            };
        let (let_owner, effective_fragment) = match default_wrapper {
            Some(wrapper_id) => match self.ctx.query.component.store.get(wrapper_id) {
                Node::SvelteFragmentLegacy(el) => (wrapper_id, el.fragment),
                _ => (el_id, fragment),
            },
            None => (el_id, fragment),
        };
        let mut let_stmts = self.emit_let_directive_legacy_stmts(let_owner)?;

        for &child_id in &default_let_scope_owners {
            if child_id == let_owner {
                continue;
            }
            let stmts = self.emit_let_directive_legacy_stmts(child_id)?;
            let_stmts.extend(stmts);
        }

        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            effective_fragment,
            FragmentAnchor::callback_param("$$anchor", false),
        );
        let mut inner_state = EmitState::new();
        inner_state.skip_snippets = true;
        for stmt in let_stmts {
            inner_state.init.push(stmt);
        }
        match self.emit_fragment(&mut inner_state, &inner_ctx, effective_fragment)? {
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
