use std::mem;

use oxc_ast::ast::Statement;
use svelte_ast::{Attribute, Namespace, Node, NodeId};
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::namespace::from_namespace;
use super::super::{Codegen, CodegenError, Result};

fn is_load_error_element(name: &str) -> bool {
    matches!(
        name,
        "body" | "embed" | "iframe" | "img" | "link" | "object" | "script" | "style" | "track"
    )
}

fn needs_textarea_content_reset(attributes: &[Attribute], has_spread: bool) -> bool {
    if has_spread {
        return true;
    }
    for attr in attributes {
        match attr {
            Attribute::BindDirective(b) if b.name == "value" => return true,
            Attribute::ExpressionAttribute(a) if a.name == "value" => return true,
            Attribute::ConcatenationAttribute(a) if a.name == "value" => return true,
            _ => {}
        }
    }
    false
}

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
            Node::ComponentNode(_) | Node::SvelteComponentLegacy(_) | Node::SvelteSelf(_) => {
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

        let is_noscript = is_html && el_name_hint == "noscript";

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
        let has_spread = self.ctx.has_spread(el_id);

        let prev_pending_element_init = mem::take(&mut state.pending_element_init);
        let prev_pending_element_update = mem::take(&mut state.pending_element_update);
        let prev_pending_pre_update = mem::take(&mut state.pending_pre_update);
        let element_after_update_len_before = state.element_after_update.len();

        if !is_ghost && !is_noscript && !has_spread {
            self.emit_element_directives(state, el_id, &el_name_hint, &el_name, &attributes)?;
        }

        if !is_ghost && self.ctx.needs_input_defaults(el_id) && !has_spread {
            state.init.push(
                self.ctx
                    .b
                    .call_stmt("$.remove_input_defaults", [Arg::Ident(&el_name)]),
            );
        }

        if !is_ghost
            && el_name_hint == "textarea"
            && !self.ctx.needs_textarea_value_lowering(el_id)
            && needs_textarea_content_reset(&attributes, has_spread)
        {
            state.init.push(
                self.ctx
                    .b
                    .call_stmt("$.remove_textarea_child", [Arg::Ident(&el_name)]),
            );
        }

        let (attach_attrs, non_attach_attrs): (Vec<_>, Vec<_>) = attributes
            .iter()
            .cloned()
            .partition(|a| matches!(a, Attribute::AttachTag(_)));

        if !is_noscript {
            self.emit_dom_attributes(
                state,
                el_id,
                &el_name_hint,
                &el_name,
                &non_attach_attrs,
                is_html,
            )?;
        }
        if !is_ghost
            && is_load_error_element(&el_name_hint)
            && (self.ctx.has_spread(el_id)
                || self.ctx.has_use_directive(el_id)
                || self.ctx.has_attribute(el_id, "onload")
                || self.ctx.has_attribute(el_id, "onerror"))
        {
            state.element_after_update.push(
                self.ctx
                    .b
                    .call_stmt("$.replay_events", [Arg::Ident(&el_name)]),
            );
        }
        if let Some(expr_id) = self.ctx.query.view.option_synthetic_value_expr(el_id) {
            self.emit_option_synthetic_value(state, &el_name, expr_id)?;
        }
        let my_element_init = mem::take(&mut state.pending_element_init);
        state.pending_element_init = prev_pending_element_init;
        let my_element_update = mem::take(&mut state.pending_element_update);
        state.pending_element_update = prev_pending_element_update;
        let my_pre_update = mem::take(&mut state.pending_pre_update);
        state.pending_pre_update = prev_pending_pre_update;

        if !is_noscript && !self.ctx.query.view.is_void(el_id) {
            if self.ctx.is_customizable_select(el_id) {
                self.emit_customizable_select(state, ctx, el_id, &el_name, el_ns)?;
                state.last_fragment_needs_reset = true;
            } else if self.ctx.needs_textarea_value_lowering(el_id) {
                self.emit_textarea_value_lowering(state, el_id, &el_name)?;
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

        state.init.extend(my_element_init);
        state.init.extend(my_pre_update);
        state.update.extend(my_element_update);
        let scoped: Vec<Statement<'a>> = state
            .element_after_update
            .split_off(element_after_update_len_before);
        state.after_update.extend(scoped);

        for attr in &attach_attrs {
            if let Attribute::AttachTag(a) = attr {
                self.emit_attach_tag(state, el_id, &el_name, a)?;
            }
        }

        state.template.pop_element();

        Ok(el_name)
    }

    fn emit_element_directives(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attributes: &[Attribute],
    ) -> Result<()> {
        let saved_after_update = mem::take(&mut state.after_update);
        for attr in attributes {
            match attr {
                Attribute::OnDirectiveLegacy(d) => {
                    self.emit_on_directive_legacy(state, owner_id, owner_var, d)?;
                }
                Attribute::BindDirective(d) => {
                    self.emit_bind_directive(state, owner_id, owner_tag, owner_var, d)?;
                }
                Attribute::TransitionDirective(d) => {
                    self.emit_transition_directive(state, owner_id, owner_var, d)?;
                }
                Attribute::UseDirective(d) => {
                    self.emit_use_directive(state, owner_id, owner_var, d)?;
                }
                Attribute::AnimateDirective(d) => {
                    self.emit_animate_directive(state, owner_id, owner_var, d)?;
                }
                _ => {}
            }
        }
        let scoped = mem::replace(&mut state.after_update, saved_after_update);
        state.element_after_update.extend(scoped);
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

        let (el_fragment, el_tag) = match self.ctx.query.component.store.get(el_id) {
            Node::Element(el) => (el.fragment, el.name.clone()),
            _ => return CodegenError::unexpected_node(el_id, "Element"),
        };

        let tpl_name = self.ctx.state.gen_ident(&format!("{el_tag}_content"));

        let fragment_name = self.ctx.state.gen_ident("fragment");
        let anchor_name = self.ctx.state.gen_ident("anchor");

        let child_ctx = ctx.child_of_element(
            self.ctx,
            &el_tag,
            el_fragment,
            el_ns,
            FragmentAnchor::CallbackParam {
                name: anchor_name.clone(),
                append_inside: false,
            },
        );
        let mut inner_state = EmitState::new();
        inner_state.suppress_root_finalize = true;
        inner_state.pending_anchor_idents = Some((fragment_name.clone(), String::new()));

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
        let from_fn = from_namespace(el_ns);
        let tpl_expr = self.ctx.b.template_str_expr(&html_str);
        let flags = if needs_import { 3.0 } else { 1.0 };
        let from_call = self
            .ctx
            .b
            .call_expr(from_fn, [Arg::Expr(tpl_expr), Arg::Num(flags)]);
        self.hoist(self.ctx.b.var_stmt(&tpl_name, from_call));

        let EmitState {
            init: inner_init,
            update: inner_update,
            after_update: inner_after,
            root_var: inner_root_var,
            shared_memo: inner_shared_memo,
            script_blockers: inner_script_blockers,
            extra_blockers: inner_extra_blockers,
            ..
        } = inner_state;

        let content_elements: Vec<NodeId> = self
            .ctx
            .query
            .component
            .store
            .fragment(el_fragment)
            .nodes
            .iter()
            .copied()
            .filter(|nid| matches!(self.ctx.query.component.store.get(*nid), Node::Element(_)))
            .collect();
        let single_content_needs_var = content_elements.len() == 1
            && selectedcontent_child.is_none()
            && self.ctx.needs_var(content_elements[0]);

        let single_first_child = inner_root_var
            .filter(|name| *name != fragment_name && single_content_needs_var)
            .map(|name| {
                self.ctx.b.var_stmt(
                    &name,
                    self.ctx
                        .b
                        .call_expr("$.first_child", [Arg::Ident(&fragment_name)]),
                )
            });

        let mut body: Vec<Statement<'a>> = Vec::new();
        body.push(self.ctx.b.var_stmt(
            &anchor_name,
            self.ctx.b.call_expr("$.child", [Arg::Ident(el_name)]),
        ));
        body.push(
            self.ctx
                .b
                .var_stmt(&fragment_name, self.ctx.b.call_expr(&tpl_name, [])),
        );
        if let Some(stmt) = single_first_child {
            body.push(stmt);
        }
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
            inner_shared_memo,
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
