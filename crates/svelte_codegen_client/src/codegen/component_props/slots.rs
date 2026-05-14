use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, FragmentEmitKind, Result};
use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{FragmentId, Node, NodeId};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    fn unwrap_default_svelte_fragment_legacy(
        &self,
        fragment: FragmentId,
    ) -> Option<(NodeId, FragmentId)> {
        let store = &self.ctx.query.component.store;
        let source = self.ctx.query.component.source.as_str();
        let mut wrapper: Option<(NodeId, FragmentId)> = None;
        for &child_id in store.fragment_nodes(fragment) {
            match store.get(child_id) {
                Node::Text(t)
                    if t.raw_value(source)
                        .chars()
                        .all(|c| c.is_ascii_whitespace()) =>
                {
                    continue;
                }
                Node::SvelteFragmentLegacy(el) => {
                    if wrapper.is_some() {
                        return None;
                    }
                    wrapper = Some((child_id, el.fragment));
                }
                _ => return None,
            }
        }
        wrapper
    }

    pub(in super::super) fn build_component_default_children(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        fragment: svelte_ast::FragmentId,
    ) -> Result<Option<Expression<'a>>> {
        let wrapped = self.unwrap_default_svelte_fragment_legacy(fragment);
        let effective_fragment = wrapped.map(|(_, inner)| inner).unwrap_or(fragment);
        if wrapped.is_some() {
            let _ = self.ctx.state.gen_ident("root");
        }
        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            effective_fragment,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        inner_state.skip_snippets = true;
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

    pub(in super::super) fn build_component_default_children_with_let(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        fragment: svelte_ast::FragmentId,
    ) -> Result<Option<Expression<'a>>> {
        let wrapped = self.unwrap_default_svelte_fragment_legacy(fragment);
        let (let_owner, effective_fragment) = match wrapped {
            Some((wrapper_id, inner)) => (wrapper_id, inner),
            None => (el_id, fragment),
        };
        if wrapped.is_some() {
            let _ = self.ctx.state.gen_ident("root");
        }
        let let_stmts = self.emit_let_directive_legacy_stmts(let_owner);

        let inner_ctx = parent_ctx.child_of_block(
            self.ctx,
            effective_fragment,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
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
