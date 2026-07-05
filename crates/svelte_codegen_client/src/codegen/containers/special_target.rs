use std::{iter, mem};
use svelte_emit_builders::runes::rune_get;
use svelte_emit_builders::store::build_store_base_read;

use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{
    AttributeSemantics, DocumentBindKind, DocumentBindSemantics, EventEmit, HandlerEmit,
    HtmlBindKind, WindowBindKind, WindowBindSemantics, is_passive_event, strip_capture_event,
};
use svelte_ast::{Attribute, BindDirective, Node, NodeId};
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::data_structures::EmitState;
use super::super::data_structures::FragmentCtx;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_special_target(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
        _existing_var: Option<&str>,
    ) -> Result<String> {
        let node = self.ctx.query.component.store.get(el_id);
        match node {
            Node::SvelteHead(_) => self.emit_svelte_head(state, ctx, el_id),
            Node::SvelteWindow(w) => {
                self.emit_special_target_with_owner(state, el_id, "$.window", &w.attributes.clone())
            }
            Node::SvelteDocument(d) => self.emit_special_target_with_owner(
                state,
                el_id,
                "$.document",
                &d.attributes.clone(),
            ),
            Node::SvelteBody(b) => self.emit_special_target_with_owner(
                state,
                el_id,
                "$.document.body",
                &b.attributes.clone(),
            ),
            _ => CodegenError::unexpected_node(el_id, "special target"),
        }
    }

    fn emit_special_target_with_owner(
        &mut self,
        state: &mut EmitState<'a>,
        el_id: NodeId,
        owner_var: &str,
        attributes: &[Attribute],
    ) -> Result<String> {
        if attributes.is_empty() {
            return Ok(String::new());
        }
        for attr in attributes {
            let attr_id = attr.id();
            match attr {
                Attribute::ExpressionAttribute(ea) => {
                    if let Some(raw_event_name) = ea.event_name.as_deref() {
                        self.emit_special_target_event_attr(
                            state,
                            attr_id,
                            ea.expression.id(),
                            owner_var,
                            raw_event_name,
                            ea.expression.span.start,
                        )?;
                    }
                }
                Attribute::ConcatenationAttribute(ca) => {
                    if let Some(expr) = svelte_analyze::concat_single_dynamic_expr(ca)
                        && let Some(raw_event_name) = ca.name.strip_prefix("on")
                    {
                        self.emit_special_target_event_attr(
                            state,
                            attr_id,
                            expr.id(),
                            owner_var,
                            raw_event_name,
                            expr.span.start,
                        )?;
                    }
                }
                Attribute::OnDirectiveLegacy(od) => {
                    let after_len_before = state.after_update.len();
                    self.emit_on_directive_legacy(state, el_id, owner_var, od)?;
                    let drained: Vec<_> = state.after_update.drain(after_len_before..).collect();
                    for stmt in drained {
                        state.init.push(stmt);
                    }
                }
                Attribute::UseDirective(ud) => {
                    self.emit_use_directive(state, el_id, owner_var, ud)?;
                }
                Attribute::AttachTag(a) => {
                    self.emit_attach_tag(state, el_id, owner_var, a)?;
                }
                Attribute::BindDirective(bind) => {
                    self.emit_special_target_bind(state, owner_var, bind)?;
                }
                _ => {}
            }
        }
        let drained = mem::take(&mut state.pending_element_init);
        state.init.extend(drained);
        Ok(String::new())
    }

    fn emit_special_target_bind(
        &mut self,
        state: &mut EmitState<'a>,
        owner_var: &str,
        bind: &BindDirective,
    ) -> Result<()> {
        enum HostProp {
            Window(WindowBindKind),
            Document(DocumentBindKind),
        }
        let (host_prop, bind_kind) = match self.ctx.query.analysis.attributes.get(bind.id) {
            AttributeSemantics::WindowBind(WindowBindSemantics { property, kind, .. }) => {
                (HostProp::Window(*property), kind.clone())
            }
            AttributeSemantics::DocumentBind(DocumentBindSemantics { property, kind, .. }) => {
                (HostProp::Document(*property), kind.clone())
            }
            _ => return Ok(()),
        };

        let var_name = if bind.shorthand {
            bind.name.clone()
        } else {
            self.ctx
                .query
                .component
                .source_text(bind.expression.span)
                .to_string()
        };

        let bind_expr = self.ctx.state.parsed.take_expr(bind.expression.id());
        if self.ctx.state.dev
            && self.ctx.query.runes()
            && !self.ctx.binding_property_non_reactive_ignored(bind.id)
            && let Some(member) = &bind_expr
            && let Some(stmt) = self.build_validate_binding_from_member(bind, member)
        {
            state.init.push(stmt);
        }

        let stmt = match host_prop {
            HostProp::Window(WindowBindKind::ScrollX) => {
                self.build_window_scroll_stmt("x", &var_name, &bind_kind)
            }
            HostProp::Window(WindowBindKind::ScrollY) => {
                self.build_window_scroll_stmt("y", &var_name, &bind_kind)
            }
            HostProp::Window(
                size_kind @ (WindowBindKind::InnerWidth
                | WindowBindKind::InnerHeight
                | WindowBindKind::OuterWidth
                | WindowBindKind::OuterHeight),
            ) => {
                let setter = self.build_binding_setter_silent(&var_name, &bind_kind);
                self.ctx.b.call_stmt(
                    "$.bind_window_size",
                    [Arg::StrRef(size_kind.name()), Arg::Expr(setter)],
                )
            }
            HostProp::Window(WindowBindKind::Online) => {
                let setter = self.build_binding_setter_silent(&var_name, &bind_kind);
                self.ctx.b.call_stmt("$.bind_online", [Arg::Expr(setter)])
            }
            HostProp::Window(WindowBindKind::DevicePixelRatio) => {
                let setter = self.build_binding_setter_silent(&var_name, &bind_kind);
                self.bind_property_stmt("devicePixelRatio", "resize", owner_var, setter)
            }
            HostProp::Document(DocumentBindKind::ActiveElement) => {
                let setter = self.build_binding_setter_silent(&var_name, &bind_kind);
                self.ctx
                    .b
                    .call_stmt("$.bind_active_element", [Arg::Expr(setter)])
            }
            HostProp::Document(DocumentBindKind::FullscreenElement) => {
                let setter = self.build_binding_setter_silent(&var_name, &bind_kind);
                self.bind_property_stmt("fullscreenElement", "fullscreenchange", owner_var, setter)
            }
            HostProp::Document(DocumentBindKind::PointerLockElement) => {
                let setter = self.build_binding_setter_silent(&var_name, &bind_kind);
                self.bind_property_stmt(
                    "pointerLockElement",
                    "pointerlockchange",
                    owner_var,
                    setter,
                )
            }
            HostProp::Document(DocumentBindKind::VisibilityState) => {
                let setter = self.build_binding_setter_silent(&var_name, &bind_kind);
                self.bind_property_stmt("visibilityState", "visibilitychange", owner_var, setter)
            }
        };

        state.after_update.push(stmt);
        Ok(())
    }

    fn build_window_scroll_stmt(
        &self,
        axis: &'static str,
        var: &str,
        kind: &HtmlBindKind,
    ) -> Statement<'a> {
        let getter = self.build_binding_getter(var, kind);
        let mut args: Vec<Arg<'a, '_>> = vec![Arg::StrRef(axis), Arg::Expr(getter)];
        if !matches!(kind, HtmlBindKind::BindableProp) || self.ctx.state.dev {
            let setter = self.build_binding_setter_silent(var, kind);
            args.push(Arg::Expr(setter));
        }
        self.ctx.b.call_stmt("$.bind_window_scroll", args)
    }

    fn build_binding_getter(&self, var: &str, kind: &HtmlBindKind) -> Expression<'a> {
        if let HtmlBindKind::BindableProp | HtmlBindKind::StoreSubscribed { .. } = kind {
            if self.ctx.state.dev {
                let call = self
                    .ctx
                    .b
                    .call_expr_callee(self.ctx.b.rid_expr(var), iter::empty::<Arg<'_, '_>>());
                return self.ctx.b.named_function_expr(
                    "get",
                    self.ctx.b.no_params(),
                    vec![self.ctx.b.return_stmt(call)],
                    false,
                );
            }
            return self.ctx.b.rid_expr(var);
        }
        let body = match kind {
            HtmlBindKind::Rune | HtmlBindKind::LegacyState => rune_get(&self.ctx.b, var),
            _ => self.ctx.b.rid_expr(var),
        };
        if self.ctx.state.dev {
            self.ctx.b.named_function_expr(
                "get",
                self.ctx.b.no_params(),
                vec![self.ctx.b.return_stmt(body)],
                false,
            )
        } else {
            self.ctx
                .b
                .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(body)])
        }
    }

    fn build_binding_setter_silent(&self, var: &str, kind: &HtmlBindKind) -> Expression<'a> {
        if let HtmlBindKind::BindableProp = kind {
            if self.ctx.state.dev {
                let call = self
                    .ctx
                    .b
                    .call_expr_callee(self.ctx.b.rid_expr(var), [Arg::Ident("$$value")]);
                return self.ctx.b.named_function_expr(
                    "set",
                    self.ctx.b.params(["$$value"]),
                    vec![self.ctx.b.expr_stmt(call)],
                    false,
                );
            }
            return self.ctx.b.rid_expr(var);
        }
        let body = match kind {
            HtmlBindKind::Rune => self.ctx.b.call_expr(
                "$.set",
                [Arg::Ident(var), Arg::Ident("$$value"), Arg::Bool(true)],
            ),
            HtmlBindKind::LegacyState => self
                .ctx
                .b
                .call_expr("$.set", [Arg::Ident(var), Arg::Ident("$$value")]),
            HtmlBindKind::StoreSubscribed { base_symbol } => {
                let base =
                    build_store_base_read(&self.ctx.b, self.ctx.query.analysis, *base_symbol);
                self.ctx
                    .b
                    .call_expr("$.store_set", [Arg::Expr(base), Arg::Ident("$$value")])
            }
            _ => self.ctx.b.assign_expr(
                AssignLeft::Ident(var.to_string()),
                self.ctx.b.rid_expr("$$value"),
            ),
        };
        let body_stmt = self.ctx.b.expr_stmt(body);
        if self.ctx.state.dev {
            self.ctx.b.named_function_expr(
                "set",
                self.ctx.b.params(["$$value"]),
                vec![body_stmt],
                false,
            )
        } else {
            self.ctx
                .b
                .arrow_expr(self.ctx.b.params(["$$value"]), [body_stmt])
        }
    }

    fn bind_property_stmt(
        &self,
        property_name: &'static str,
        event_name: &'static str,
        target: &str,
        setter: Expression<'a>,
    ) -> Statement<'a> {
        self.ctx.b.call_stmt(
            "$.bind_property",
            [
                Arg::StrRef(property_name),
                Arg::StrRef(event_name),
                Arg::Ident(target),
                Arg::Expr(setter),
            ],
        )
    }

    fn emit_special_target_event_attr(
        &mut self,
        state: &mut EmitState<'a>,
        attr_id: NodeId,
        expr_id: OxcNodeId,
        owner_var: &str,
        raw_event_name: &str,
        expr_offset: u32,
    ) -> Result<()> {
        let (event_name, capture) = if let Some(base) = strip_capture_event(raw_event_name) {
            (base.to_string(), true)
        } else {
            (raw_event_name.to_string(), false)
        };

        let handler_emit = match self.ctx.query.analysis.attributes.get(attr_id) {
            AttributeSemantics::Event(ev) => match &ev.emit {
                EventEmit::HtmlDelegated { handler }
                | EventEmit::HtmlDirect { handler, .. }
                | EventEmit::Component { handler } => *handler,
                EventEmit::HtmlBubble => HandlerEmit::Direct,
            },
            _ => HandlerEmit::Direct,
        };
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return CodegenError::missing_expression(attr_id);
        };
        let handler =
            self.build_event_handler_s5(attr_id, expr, handler_emit, &mut state.init, expr_offset);
        let handler = self.dev_event_handler(attr_id, handler, &event_name)?;

        let passive = is_passive_event(&event_name);
        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::StrRef(&event_name),
            Arg::Ident(owner_var),
            Arg::Expr(handler),
        ];
        if capture || passive {
            args.push(if capture {
                Arg::Bool(true)
            } else {
                Arg::Expr(self.ctx.b.void_zero_expr())
            });
        }
        if passive {
            args.push(Arg::Bool(true));
        }
        state.init.push(self.ctx.b.call_stmt("$.event", args));
        Ok(())
    }

    fn emit_svelte_head(
        &mut self,
        state: &mut EmitState<'a>,
        ctx: &FragmentCtx<'a>,
        el_id: NodeId,
    ) -> Result<String> {
        let head_fragment = match self.ctx.query.component.store.get(el_id) {
            Node::SvelteHead(head) => head.fragment,
            _ => return CodegenError::unexpected_node(el_id, "SvelteHead"),
        };
        let inner_ctx = ctx.child_of_svelte_head(self.ctx, head_fragment);
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, head_fragment)?;
        let body = self.pack_callback_body(inner_state, "$$anchor")?;

        let hash_str = hash(self.ctx.state.filename);
        let body_fn = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.params(["$$anchor"]), body);

        state.init.push(
            self.ctx
                .b
                .call_stmt("$.head", [Arg::Str(hash_str), Arg::Expr(body_fn)]),
        );
        Ok(String::new())
    }
}

fn hash(s: &str) -> String {
    let mut h: u32 = 5381;
    for &b in s.as_bytes().iter().rev() {
        if b == b'\r' {
            continue;
        }
        h = (h.wrapping_shl(5).wrapping_sub(h)) ^ (b as u32);
    }
    to_base36(h)
}

fn to_base36(mut n: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut result = Vec::new();
    while n > 0 {
        result.push(CHARS[(n % 36) as usize]);
        n /= 36;
    }
    result.reverse();
    match String::from_utf8(result) {
        Ok(s) => s,
        Err(_) => "0".to_string(),
    }
}
