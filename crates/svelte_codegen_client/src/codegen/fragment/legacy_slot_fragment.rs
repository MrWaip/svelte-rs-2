use compact_str::CompactString;
use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::Volatility;
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
        let append_inside = matches!(node, svelte_ast::Node::ComponentNode(_));

        let slot_fragment_id = match node {
            Node::SvelteFragmentLegacy(el) => Some(el.fragment),
            _ => None,
        };
        let inner_ctx = parent_ctx.child_of_named_slot(
            self.ctx,
            slot_fragment_id,
            FragmentAnchor::callback_param("$$anchor", append_inside),
        );
        let mut inner_state = EmitState::new();

        match node {
            Node::Element(_) => {
                let let_stmts = self.emit_let_directive_legacy_stmts(slot_el_id)?;
                self.emit_single_slot_element(&mut inner_state, &inner_ctx, slot_el_id, let_stmts)?;
            }
            Node::SvelteFragmentLegacy(el) => {
                let let_stmts = self.emit_let_directive_legacy_stmts(slot_el_id)?;
                inner_state.init.extend(let_stmts);
                self.emit_fragment(&mut inner_state, &inner_ctx, el.fragment)?;
            }
            Node::SlotElementLegacy(_) => {
                self.emit_legacy_slot_like(&mut inner_state, &inner_ctx, slot_el_id, None)?;
            }
            Node::SvelteElement(_) => {
                let let_stmts = self.emit_let_directive_legacy_stmts(slot_el_id)?;
                inner_state.init.extend(let_stmts);
                self.emit_element(&mut inner_state, &inner_ctx, slot_el_id, None)?;
            }
            n if n.as_component_like().is_some() => {
                let let_stmts = self.emit_let_directive_legacy_stmts(slot_el_id)?;
                inner_state.init.extend(let_stmts);
                match (
                    n,
                    self.ctx.expression_data(slot_el_id).map(|d| d.volatility),
                ) {
                    (Node::ComponentNode(_), Some(Volatility::Static) | None) => {
                        if self.ctx.has_component_css_props(slot_el_id) {
                            self.emit_component_with_css_wrapper(
                                &mut inner_state,
                                &inner_ctx,
                                slot_el_id,
                            )?;
                        } else {
                            let _ = self.ctx.state.gen_ident("fragment");
                            self.emit_element(&mut inner_state, &inner_ctx, slot_el_id, None)?;
                        }
                    }
                    _ => {
                        self.emit_element(&mut inner_state, &inner_ctx, slot_el_id, None)?;
                    }
                }
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
            Node::Element(_) => return false,
            Node::SvelteElement(_) => return false,
            Node::SvelteFragmentLegacy(el) => el.fragment,
            Node::SlotElementLegacy(_) => return false,
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
        let init_len_before = state.init.len();
        state.legacy_slot_const_tag_start = None;
        state.legacy_slot_const_tag_end = None;
        state.legacy_slot_record_const_tag_end = true;
        let el_name = self.emit_element(state, ctx, slot_el_id, None)?;
        state.legacy_slot_record_const_tag_end = false;
        let pre_const = state
            .legacy_slot_const_tag_start
            .take()
            .unwrap_or(init_len_before);
        let post_const = state.legacy_slot_const_tag_end.take().unwrap_or(pre_const);
        if pre_const > init_len_before && post_const > pre_const {
            let attr_len = pre_const - init_len_before;
            state.init[init_len_before..post_const].rotate_left(attr_len);
        }
        let after_const_tags = init_len_before + (post_const.saturating_sub(pre_const));
        state.root_var = Some(CompactString::from(el_name.as_str()));
        let slot_fragment = match self.ctx.query.component.store.get(slot_el_id) {
            Node::Element(el) => el.fragment,
            Node::SlotElementLegacy(el) => el.fragment,
            Node::SvelteFragmentLegacy(el) => el.fragment,
            _ => return Ok(()),
        };
        self.finalize_slot_root_template(state, ctx, after_const_tags, slot_el_id, slot_fragment)?;
        for (i, stmt) in let_stmts.into_iter().enumerate() {
            state.init.insert(init_len_before + i, stmt);
        }
        Ok(())
    }
}
