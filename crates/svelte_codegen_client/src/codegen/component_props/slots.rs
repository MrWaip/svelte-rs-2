use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::FragmentKey;
use svelte_ast::{Attribute, Node, NodeId};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn slot_name_of(&self, slot_el_id: NodeId) -> Result<String> {
        let attrs: &[Attribute] = match self.ctx.query.component.store.get(slot_el_id) {
            Node::Element(el) => &el.attributes,
            Node::SvelteFragmentLegacy(el) => &el.attributes,
            Node::ComponentNode(cn) => &cn.attributes,
            _ => return CodegenError::unexpected_node(slot_el_id, "named slot child element"),
        };
        for attr in attrs {
            if let Attribute::StringAttribute(sa) = attr {
                if sa.name == "slot" {
                    return Ok(self
                        .ctx
                        .query
                        .component
                        .source_text(sa.value_span)
                        .to_string());
                }
            }
        }
        CodegenError::unexpected_node(slot_el_id, "named slot child with slot=\"...\" attribute")
    }

    pub(in super::super) fn build_named_slot_arrow(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        slot_el_id: NodeId,
        key: FragmentKey,
    ) -> Result<Expression<'a>> {
        let node = self.ctx.query.component.store.get(slot_el_id);
        let append_inside = match node {
            Node::ComponentNode(cn) => cn.name != svelte_ast::SVELTE_SELF,
            _ => false,
        };
        let inner_ctx = parent_ctx.child_of_block(
            key,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside,
            },
        );
        let mut inner_state = EmitState::new();

        let let_stmts = self.emit_let_directive_legacy_stmts(slot_el_id);

        let node = self.ctx.query.component.store.get(slot_el_id);
        match node {
            Node::Element(_) => {
                self.emit_single_slot_element(&mut inner_state, &inner_ctx, slot_el_id, let_stmts)?;
            }
            Node::ComponentNode(_) => {
                for stmt in let_stmts {
                    inner_state.init.push(stmt);
                }
                self.emit_element(&mut inner_state, &inner_ctx, slot_el_id, None)?;
            }
            _ => {
                for stmt in let_stmts {
                    inner_state.init.push(stmt);
                }
                self.emit_fragment(&mut inner_state, &inner_ctx, key)?;
            }
        }
        let body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;
        Ok(self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor", "$$slotProps"]), body))
    }

    fn emit_single_slot_element(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        slot_el_id: NodeId,
        let_stmts: Vec<oxc_ast::ast::Statement<'a>>,
    ) -> Result<()> {
        let tpl_name = self.ctx.state.gen_ident("root");
        let init_len_before = state.init.len();
        let el_name = self.emit_element(state, ctx, slot_el_id, None)?;
        state.root_var = Some(el_name);
        let key = FragmentKey::Element(slot_el_id);
        self.finalize_root_template(
            state,
            ctx,
            super::super::fragment::StrategyKind::SingleElement,
            init_len_before,
            tpl_name,
            key,
        )?;
        let insert_pos = init_len_before + 1;
        for (i, stmt) in let_stmts.into_iter().enumerate() {
            state.init.insert(insert_pos + i, stmt);
        }
        Ok(())
    }

    pub(in super::super) fn build_component_default_children(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        key: FragmentKey,
    ) -> Result<Option<Expression<'a>>> {
        if super::super::fragment::prepare::fragment_is_effectively_empty(
            &key,
            self.ctx.query.component,
            parent_ctx,
        ) {
            return Ok(None);
        }

        let inner_ctx = parent_ctx.child_of_block(
            key,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        inner_state.skip_snippets = true;
        self.emit_fragment(&mut inner_state, &inner_ctx, key)?;
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
        key: FragmentKey,
    ) -> Result<Option<Expression<'a>>> {
        if super::super::fragment::prepare::fragment_is_effectively_empty(
            &key,
            self.ctx.query.component,
            parent_ctx,
        ) {
            return Ok(None);
        }

        let let_stmts = self.emit_let_directive_legacy_stmts(el_id);

        let inner_ctx = parent_ctx.child_of_block(
            key,
            FragmentAnchor::CallbackParam {
                name: "$$anchor".to_string(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        for stmt in let_stmts {
            inner_state.init.push(stmt);
        }
        self.emit_fragment(&mut inner_state, &inner_ctx, key)?;
        let body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;

        let arrow = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor", "$$slotProps"]), body);
        Ok(Some(arrow))
    }
}
