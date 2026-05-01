use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{Node, NodeId};

use crate::codegen::data_structures::{EmitState, FragmentAnchor, FragmentCtx};
use crate::codegen::fragment::prepare::prepare;
use crate::codegen::fragment::types::{ContentStrategy, HoistedBucket};
use crate::codegen::{Codegen, Result};

pub(in crate::codegen) enum SlotFragmentOutcome<'a> {
    Empty,
    Arrow(Expression<'a>),
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_slot_fragment_legacy_component_only_dont_use(
        &mut self,
        parent_ctx: &FragmentCtx<'a>,
        component_id: NodeId,
        slot_el_id: NodeId,
    ) -> Result<SlotFragmentOutcome<'a>> {
        let _ = component_id;

        if self.is_slot_fragment_empty(slot_el_id, parent_ctx) {
            return Ok(SlotFragmentOutcome::Empty);
        }

        let node = self.ctx.query.component.store.get(slot_el_id);
        let append_inside = node
            .as_component_like()
            .is_some_and(|view| view.name != svelte_ast::SVELTE_SELF);

        let inner_ctx = parent_ctx.child_of_named_slot(FragmentAnchor::CallbackParam {
            name: "$$anchor".to_string(),
            append_inside,
        });
        let mut inner_state = EmitState::new();

        if self.ctx.is_svelte_fragment_slot(slot_el_id) {
            let _ = self.ctx.state.gen_ident("root");
        }

        let let_stmts = self.emit_let_directive_legacy_stmts(slot_el_id);

        match node {
            Node::Element(_) => {
                self.emit_single_slot_element(&mut inner_state, &inner_ctx, slot_el_id, let_stmts)?;
            }
            Node::SvelteFragmentLegacy(el) => {
                inner_state.init.extend(let_stmts);
                self.emit_fragment(&mut inner_state, &inner_ctx, el.fragment)?;
            }
            n if n.as_component_like().is_some() => {
                inner_state.init.extend(let_stmts);
                self.emit_element(&mut inner_state, &inner_ctx, slot_el_id, None)?;
            }
            _ => {}
        }

        let body: Vec<Statement<'a>> = self.pack_callback_body(inner_state, "$$anchor")?;
        let arrow = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor", "$$slotProps"]), body);
        Ok(SlotFragmentOutcome::Arrow(arrow))
    }

    fn is_slot_fragment_empty(&self, slot_el_id: NodeId, ctx: &FragmentCtx<'a>) -> bool {
        if self.has_let_directives(slot_el_id) {
            return false;
        }

        let node = self.ctx.query.component.store.get(slot_el_id);
        let fragment_id = match node {
            Node::Element(el) => el.fragment,
            Node::SvelteFragmentLegacy(el) => el.fragment,
            n if n.as_component_like().is_some() => return false,
            _ => return true,
        };
        let mut bucket = HoistedBucket::default();
        let (_children, strategy) = prepare(
            self.ctx.query.component.fragment_nodes(fragment_id),
            &self.ctx.query.component.store,
            ctx,
            &mut bucket,
        );
        matches!(strategy, ContentStrategy::Empty) && bucket.is_empty()
    }

    fn has_let_directives(&self, owner_id: NodeId) -> bool {
        let node = self.ctx.query.component.store.get(owner_id);
        let attrs = match node {
            Node::Element(el) => &el.attributes[..],
            Node::SvelteFragmentLegacy(el) => &el.attributes[..],
            _ => match node.as_component_like() {
                Some(view) => view.attributes,
                None => return false,
            },
        };
        attrs
            .iter()
            .any(|a| matches!(a, svelte_ast::Attribute::LetDirectiveLegacy(_)))
    }

    fn emit_single_slot_element(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        slot_el_id: NodeId,
        let_stmts: Vec<Statement<'a>>,
    ) -> Result<()> {
        let tpl_name = self.ctx.state.gen_ident("root");
        let init_len_before = state.init.len();
        let el_name = self.emit_element(state, ctx, slot_el_id, None)?;
        state.root_var = Some(el_name);
        let slot_fragment = match self.ctx.query.component.store.get(slot_el_id) {
            Node::Element(el) => el.fragment,
            Node::SlotElementLegacy(el) => el.fragment,
            Node::SvelteFragmentLegacy(el) => el.fragment,
            _ => return Ok(()),
        };
        self.finalize_slot_root_template(state, ctx, init_len_before, tpl_name, slot_fragment)?;
        let insert_pos = init_len_before + 1;
        for (i, stmt) in let_stmts.into_iter().enumerate() {
            state.init.insert(insert_pos + i, stmt);
        }
        Ok(())
    }
}
