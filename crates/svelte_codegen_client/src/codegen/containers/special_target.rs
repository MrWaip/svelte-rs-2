use oxc_ast::ast::Expression;
use svelte_analyze::{BindPropertyKind, DocumentBindKind, ReferenceSemantics, WindowBindKind};
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
        Ok(String::new())
    }

    fn emit_special_target_bind(
        &mut self,
        state: &mut EmitState<'a>,
        owner_var: &str,
        bind: &BindDirective,
    ) -> Result<()> {
        let Some(bind_semantics) = self.ctx.bind_target_semantics(bind.id) else {
            return Ok(());
        };

        let bind_attr = svelte_ast::Attribute::BindDirective(bind.clone());
        let is_rune = matches!(
            self.ctx.directive_root_reference_semantics(&bind_attr),
            ReferenceSemantics::SignalWrite { .. }
                | ReferenceSemantics::SignalUpdate { .. }
                | ReferenceSemantics::SignalRead { .. }
        );

        let var_name = if bind.shorthand {
            bind.name.clone()
        } else {
            self.ctx
                .query
                .component
                .source_text(bind.expression.span)
                .to_string()
        };

        let _ = self.ctx.state.parsed.take_expr(bind.expression.id());

        let stmt = match bind_semantics.property() {
            BindPropertyKind::Window(WindowBindKind::ScrollX) => {
                let getter = self.build_binding_getter(&var_name, is_rune);
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.ctx.b.call_stmt(
                    "$.bind_window_scroll",
                    [Arg::StrRef("x"), Arg::Expr(getter), Arg::Expr(setter)],
                )
            }
            BindPropertyKind::Window(WindowBindKind::ScrollY) => {
                let getter = self.build_binding_getter(&var_name, is_rune);
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.ctx.b.call_stmt(
                    "$.bind_window_scroll",
                    [Arg::StrRef("y"), Arg::Expr(getter), Arg::Expr(setter)],
                )
            }
            BindPropertyKind::Window(
                kind @ (WindowBindKind::InnerWidth
                | WindowBindKind::InnerHeight
                | WindowBindKind::OuterWidth
                | WindowBindKind::OuterHeight),
            ) => {
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.ctx.b.call_stmt(
                    "$.bind_window_size",
                    [Arg::StrRef(kind.name()), Arg::Expr(setter)],
                )
            }
            BindPropertyKind::Window(WindowBindKind::Online) => {
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.ctx.b.call_stmt("$.bind_online", [Arg::Expr(setter)])
            }
            BindPropertyKind::Window(WindowBindKind::DevicePixelRatio) => {
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.bind_property_stmt("devicePixelRatio", "resize", owner_var, setter)
            }
            BindPropertyKind::Document(DocumentBindKind::ActiveElement) => {
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.ctx
                    .b
                    .call_stmt("$.bind_active_element", [Arg::Expr(setter)])
            }
            BindPropertyKind::Document(DocumentBindKind::FullscreenElement) => {
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.bind_property_stmt("fullscreenElement", "fullscreenchange", owner_var, setter)
            }
            BindPropertyKind::Document(DocumentBindKind::PointerLockElement) => {
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.bind_property_stmt(
                    "pointerLockElement",
                    "pointerlockchange",
                    owner_var,
                    setter,
                )
            }
            BindPropertyKind::Document(DocumentBindKind::VisibilityState) => {
                let setter = self.build_binding_setter_silent(&var_name, is_rune);
                self.bind_property_stmt("visibilityState", "visibilitychange", owner_var, setter)
            }
            _ => return Ok(()),
        };

        state.init.push(stmt);
        Ok(())
    }

    fn build_binding_getter(&self, var: &str, is_rune: bool) -> Expression<'a> {
        let body = if is_rune {
            self.ctx.b.call_expr("$.get", [Arg::Ident(var)])
        } else {
            self.ctx.b.rid_expr(var)
        };
        self.ctx
            .b
            .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(body)])
    }

    fn build_binding_setter_silent(&self, var: &str, is_rune: bool) -> Expression<'a> {
        let body = if is_rune {
            self.ctx.b.call_expr(
                "$.set",
                [Arg::Ident(var), Arg::Ident("$$value"), Arg::Bool(true)],
            )
        } else {
            self.ctx.b.assign_expr(
                AssignLeft::Ident(var.to_string()),
                self.ctx.b.rid_expr("$$value"),
            )
        };
        self.ctx
            .b
            .arrow_expr(self.ctx.b.params(["$$value"]), [self.ctx.b.expr_stmt(body)])
    }

    fn bind_property_stmt(
        &self,
        property_name: &'static str,
        event_name: &'static str,
        target: &str,
        setter: Expression<'a>,
    ) -> oxc_ast::ast::Statement<'a> {
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
        expr_id: oxc_syntax::node::NodeId,
        owner_var: &str,
        raw_event_name: &str,
        expr_offset: u32,
    ) -> Result<()> {
        let (event_name, capture) =
            if let Some(base) = svelte_analyze::strip_capture_event(raw_event_name) {
                (base.to_string(), true)
            } else {
                (raw_event_name.to_string(), false)
            };

        let has_call = self
            .ctx
            .attr_expression(attr_id)
            .is_some_and(|info| info.has_call());
        let Some(expr) = self.ctx.state.parsed.take_expr(expr_id) else {
            return crate::codegen::CodegenError::missing_expression(attr_id);
        };
        let expr = self.maybe_wrap_legacy_slots_read(expr);
        let handler =
            self.build_event_handler_s5(attr_id, expr, has_call, &mut state.init, expr_offset);
        let handler = self.dev_event_handler(attr_id, handler, &event_name, expr_offset)?;

        let passive = svelte_analyze::is_passive_event(&event_name);
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
