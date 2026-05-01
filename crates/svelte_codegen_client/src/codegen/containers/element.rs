use oxc_allocator::CloneIn;
use svelte_ast::{Namespace, Node, NodeId};
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn element_ident_prefix(&self, name: &str) -> String {
        let mut out = String::with_capacity(name.len());
        for ch in name.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                out.push(ch);
            } else {
                out.push('_');
            }
        }
        if out.is_empty() {
            "node".to_string()
        } else {
            out
        }
    }

    pub(in crate::codegen) fn element_namespace(
        &self,
        el_id: NodeId,
        fallback: Namespace,
    ) -> Namespace {
        self.ctx
            .query
            .view
            .creation_namespace(el_id)
            .unwrap_or(fallback)
    }

    pub(in crate::codegen) fn emit_element(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        existing_var: Option<&str>,
    ) -> Result<String> {
        let node = self.ctx.query.component.store.get(el_id);
        match node {
            Node::Element(_) => self.emit_element_html(state, ctx, el_id, existing_var),
            Node::ComponentNode(_) | Node::SvelteComponentLegacy(_) => {
                self.emit_component(state, ctx, el_id, existing_var)
            }
            Node::SvelteElement(_) => self.emit_svelte_element(state, ctx, el_id, existing_var),
            Node::SvelteBoundary(_) => self.emit_svelte_boundary(state, ctx, el_id, existing_var),
            Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::SvelteHead(_) => self.emit_special_target(state, ctx, el_id, existing_var),
            Node::SlotElementLegacy(_) | Node::SvelteFragmentLegacy(_) => {
                self.emit_legacy_slot_like(state, ctx, el_id, existing_var)
            }
            _ => CodegenError::unexpected_node(el_id, "element-like"),
        }
    }

    pub(in crate::codegen) fn emit_element_ghost(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
    ) -> Result<()> {
        self.emit_element_html(state, ctx, el_id, Some(""))?;
        Ok(())
    }

    fn emit_element_html(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        existing_var: Option<&str>,
    ) -> Result<String> {
        let node = self.ctx.query.component.store.get(el_id);
        let el = match node {
            Node::Element(el) => el,
            _ => return CodegenError::unexpected_node(el_id, "Element"),
        };

        let el_name_hint = el.name.clone();
        let attributes = el.attributes.clone();
        let el_ns = self.element_namespace(el_id, ctx.namespace);

        let is_html = matches!(el_ns, Namespace::Html) && el_name_hint != "svg";
        state.template.push_element(&el_name_hint, is_html);

        if is_html && el_name_hint == "noscript" {
            state.template.pop_element();
            return Ok(String::new());
        }

        let has_is_attr = self.ctx.has_attribute(el_id, "is");
        state.template.needs_import_node |=
            el_name_hint == "video" || self.ctx.query.view.is_custom_element(el_id) || has_is_attr;
        state.template.contains_script_tag |= el_name_hint == "script";

        let el_name = match existing_var {
            Some("") => String::new(),
            Some(name) => name.to_string(),
            None => {
                let prefix = self.element_ident_prefix(&el_name_hint);
                self.ctx.state.gen_ident(&prefix)
            }
        };
        let is_ghost = el_name.is_empty();

        if !is_ghost && self.ctx.needs_input_defaults(el_id) {
            state.init.push(
                self.ctx
                    .b
                    .call_stmt("$.remove_input_defaults", [Arg::Ident(&el_name)]),
            );
        }

        let (attach_attrs, non_attach_attrs): (Vec<_>, Vec<_>) = attributes
            .iter()
            .cloned()
            .partition(|a| matches!(a, svelte_ast::Attribute::AttachTag(_)));

        let prev_pending_bind_this = std::mem::take(&mut state.pending_bind_this);
        self.emit_dom_attributes(state, el_id, &el_name_hint, &el_name, &non_attach_attrs)?;
        let my_bind_this = std::mem::take(&mut state.pending_bind_this);
        state.pending_bind_this = prev_pending_bind_this;

        if !self.ctx.query.view.is_void(el_id) {
            if self.ctx.is_customizable_select(el_id) {
                self.emit_customizable_select(state, ctx, el_id, &el_name, el_ns)?;
                state.last_fragment_needs_reset = true;
            } else if self.ctx.needs_textarea_value_lowering(el_id) {
                self.emit_textarea_value_lowering(state, el_id, &el_name)?;
            } else if let Some(expr_id) = self.ctx.query.view.option_synthetic_value_expr(el_id) {
                self.emit_option_synthetic_value(state, &el_name, expr_id)?;
                state.last_fragment_needs_reset = true;
            } else {
                let child_ctx = ctx.child_of_element(
                    self.ctx,
                    &el_name_hint,
                    el.fragment,
                    el_ns,
                    FragmentAnchor::Child {
                        parent_var: el_name.clone(),
                    },
                );
                let prev_bound_ce = state.bound_contenteditable;
                if self.ctx.is_bound_contenteditable(el_id) {
                    state.bound_contenteditable = true;
                }
                self.emit_fragment(state, &child_ctx, el.fragment)?;
                state.bound_contenteditable = prev_bound_ce;
                let has_var = !is_ghost && (existing_var.is_some() || self.ctx.needs_var(el_id));
                if state.last_fragment_needs_reset && has_var {
                    state
                        .init
                        .push(self.ctx.b.call_stmt("$.reset", [Arg::Ident(&el_name)]));
                    state.last_fragment_needs_reset = false;
                } else if !has_var {
                    state.last_fragment_needs_reset = false;
                }
            }
        }

        state.init.extend(my_bind_this);

        for attr in &attach_attrs {
            if let svelte_ast::Attribute::AttachTag(a) = attr {
                self.emit_attach_tag(state, el_id, &el_name, a)?;
            }
        }

        state.template.pop_element();

        Ok(el_name)
    }

    fn emit_option_synthetic_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        expr_id: NodeId,
    ) -> Result<()> {
        let expr = self.take_node_expr(expr_id)?;
        let text_expr = if let oxc_ast::ast::Expression::Identifier(ident) = &expr {
            self.ctx
                .query
                .view
                .known_value(ident.name.as_str())
                .map(|s| self.ctx.b.str_expr(s))
        } else {
            None
        };

        let text_expr = match text_expr {
            Some(e) => e,
            None => expr.clone_in(self.ctx.b.ast.allocator),
        };

        let b = &self.ctx.b;
        let text_member = b.static_member(b.rid_expr(el_name), "textContent");
        state
            .init
            .push(b.assign_stmt(AssignLeft::StaticMember(text_member), text_expr));
        let value_member = b.static_member(b.rid_expr(el_name), "__value");
        state
            .init
            .push(b.assign_stmt(AssignLeft::StaticMember(value_member), expr));
        Ok(())
    }

    fn emit_customizable_select(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        el_name: &str,
        el_ns: Namespace,
    ) -> Result<()> {
        state.template.push_comment(None);

        let tpl_name = self.ctx.state.gen_ident(&format!("{el_name}_content"));

        let el_fragment = match self.ctx.query.component.store.get(el_id) {
            Node::Element(el) => el.fragment,
            _ => return CodegenError::unexpected_node(el_id, "Element"),
        };
        let child_ctx = ctx.child_of_element(
            self.ctx,
            "",
            el_fragment,
            el_ns,
            FragmentAnchor::CallbackParam {
                name: "anchor".to_string(),
                append_inside: true,
            },
        );
        let mut inner_state = EmitState::new();
        inner_state.suppress_root_finalize = true;

        let selectedcontent_child: Option<NodeId> =
            if let Node::Element(parent_el) = self.ctx.query.component.store.get(el_id) {
                self.ctx
                    .query
                    .component
                    .store
                    .fragment(parent_el.fragment)
                    .nodes
                    .iter()
                    .copied()
                    .find(|nid| {
                        matches!(
                            self.ctx.query.component.store.get(*nid),
                            Node::Element(el) if el.name == "selectedcontent"
                        )
                    })
            } else {
                None
            };
        let selectedcontent_ident =
            selectedcontent_child.map(|_| self.ctx.state.gen_ident("selectedcontent"));

        self.emit_fragment(&mut inner_state, &child_ctx, el_fragment)?;

        let html_str = inner_state.template.as_html();
        let needs_import = inner_state.template.needs_import_node;
        let from_fn = super::super::namespace::from_namespace(el_ns);
        let tpl_expr = self.ctx.b.template_str_expr(&html_str);
        let flags = if needs_import { 3.0 } else { 1.0 };
        let from_call = self
            .ctx
            .b
            .call_expr(from_fn, [Arg::Expr(tpl_expr), Arg::Num(flags)]);
        self.hoist(self.ctx.b.var_stmt(&tpl_name, from_call));

        let super::super::data_structures::EmitState {
            init: inner_init,
            update: inner_update,
            after_update: inner_after,
            root_var: _,
            memo_attrs: inner_memo,
            script_blockers: inner_script_blockers,
            extra_blockers: inner_extra_blockers,
            ..
        } = inner_state;

        let anchor_name = self.ctx.state.gen_ident("anchor");
        let fragment_name = self.ctx.state.gen_ident("fragment");

        let mut body: Vec<oxc_ast::ast::Statement<'a>> = Vec::new();
        body.push(self.ctx.b.var_stmt(
            &anchor_name,
            self.ctx.b.call_expr("$.child", [Arg::Ident(el_name)]),
        ));
        body.push(
            self.ctx
                .b
                .var_stmt(&fragment_name, self.ctx.b.call_expr(&tpl_name, [])),
        );
        if let Some(sc_ident) = selectedcontent_ident.clone() {
            body.push(
                self.ctx.b.var_stmt(
                    &sc_ident,
                    self.ctx
                        .b
                        .call_expr("$.first_child", [Arg::Ident(&fragment_name)]),
                ),
            );
            let assign = self.ctx.b.assign_expr(
                AssignLeft::Ident(sc_ident.clone()),
                self.ctx.b.rid_expr("$$element"),
            );
            let setter = self.ctx.b.arrow_expr(
                self.ctx.b.params(["$$element"]),
                [self.ctx.b.expr_stmt(assign)],
            );
            body.push(self.ctx.b.call_stmt(
                "$.selectedcontent",
                [Arg::Ident(&sc_ident), Arg::Expr(setter)],
            ));
        }
        body.extend(inner_init);
        super::super::effect::emit_template_effect_with_memo(
            self.ctx,
            &mut body,
            inner_update,
            inner_memo,
            inner_script_blockers,
            inner_extra_blockers,
        )?;
        body.extend(inner_after);
        body.push(self.ctx.b.call_stmt(
            "$.append",
            [Arg::Ident(&anchor_name), Arg::Ident(&fragment_name)],
        ));

        let callback = self.ctx.b.arrow_block_expr(self.ctx.b.no_params(), body);
        state.init.push(self.ctx.b.call_stmt(
            "$.customizable_select",
            [Arg::Ident(el_name), Arg::Expr(callback)],
        ));

        Ok(())
    }

    fn emit_textarea_value_lowering(
        &mut self,
        state: &mut EmitState<'a>,
        el_id: NodeId,
        el_name: &str,
    ) -> Result<()> {
        let el = self.ctx.element(el_id);
        let child_ids: Vec<NodeId> = self
            .ctx
            .query
            .component
            .store
            .fragment(el.fragment)
            .nodes
            .clone();

        let mut expr_id: Option<NodeId> = None;
        for child_id in &child_ids {
            match self.ctx.query.component.store.get(*child_id) {
                svelte_ast::Node::ExpressionTag(ex) => {
                    expr_id = Some(ex.id);
                    break;
                }
                _ => continue,
            }
        }

        let Some(id) = expr_id else {
            return Ok(());
        };
        let expr = self.take_node_expr(id)?;
        state.init.push(
            self.ctx
                .b
                .call_stmt("$.remove_textarea_child", [Arg::Ident(el_name)]),
        );
        state.init.push(
            self.ctx
                .b
                .call_stmt("$.set_value", [Arg::Ident(el_name), Arg::Expr(expr)]),
        );
        Ok(())
    }
}
