use svelte_analyze::{normalize_regular_attribute_name, EventHandlerMode, ExprSite};
use svelte_ast::{ExpressionAttribute, NodeId};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_attr_expression(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        attr: &ExpressionAttribute,
    ) -> Result<()> {
        if attr.name == "class" {
            return Ok(());
        }

        if let Some(raw_event_name) = attr.event_name.as_deref() {
            return self.emit_event_attribute(state, owner_var, attr, raw_event_name);
        }

        if attr.name == "autofocus" {
            let val = self.take_attr_expr(attr.id)?;
            state.init.push(
                self.ctx
                    .b
                    .call_stmt("$.autofocus", [Arg::Ident(owner_var), Arg::Expr(val)]),
            );
            return Ok(());
        }

        if attr.name == "value" && owner_tag == "input" && self.ctx.has_bind_group(owner_id) {
            let val = self.take_attr_expr(attr.id)?;
            self.emit_bind_group_value(state, owner_var, val);
            return Ok(());
        }

        let attr_id = attr.id;
        let is_dyn = self.ctx.is_dynamic_attr(attr_id);
        let needs_memo = is_dyn
            && self
                .ctx
                .expr_deps(ExprSite::Attr(attr_id))
                .map(|deps| deps.needs_memo)
                .unwrap_or(false);

        let expr = self.take_attr_expr(attr_id)?;
        let html_attr_namespace = self.is_html_attr_namespace(owner_id);
        let attr_name = normalize_regular_attribute_name(&attr.name, html_attr_namespace);
        let attr_update = self.regular_attr_update(owner_id, owner_tag, &attr_name);

        if needs_memo {
            self.memoize_regular_attr_update(
                &mut state.memo_attrs,
                attr_id,
                owner_var,
                attr_update,
                expr,
            );
        } else {
            let target = if is_dyn {
                &mut state.update
            } else {
                &mut state.init
            };
            self.push_regular_attr_update(target, owner_var, attr_update, expr);
        }

        Ok(())
    }

    fn emit_event_attribute(
        &mut self,
        state: &mut EmitState<'a>,
        owner_var: &str,
        attr: &ExpressionAttribute,
        raw_event_name: &str,
    ) -> Result<()> {
        let attr_id = attr.id;
        let Some(mode) = self.ctx.event_handler_mode(attr_id) else {
            return Ok(());
        };

        let event_name = svelte_analyze::strip_capture_event(raw_event_name)
            .unwrap_or(raw_event_name)
            .to_string();

        let has_call = self
            .ctx
            .attr_expression(attr_id)
            .is_some_and(|info| info.has_call());
        let expr_offset = attr.expression_span.start;
        let expr = self.take_attr_expr(attr_id)?;

        let handler =
            self.build_event_handler_s5(attr_id, expr, has_call, &mut state.init, expr_offset);
        let handler = self.dev_event_handler(attr_id, handler, &event_name, expr_offset)?;

        match mode {
            EventHandlerMode::Delegated { passive } => {
                let mut args: Vec<Arg<'a, '_>> = vec![
                    Arg::StrRef(&event_name),
                    Arg::Ident(owner_var),
                    Arg::Expr(handler),
                ];
                if passive {
                    args.push(Arg::Expr(self.ctx.b.void_zero_expr()));
                    args.push(Arg::Bool(true));
                }
                state
                    .after_update
                    .push(self.ctx.b.call_stmt("$.delegated", args));
                self.ctx.add_delegated_event(event_name);
            }
            EventHandlerMode::Direct { capture, passive } => {
                let mut args: Vec<Arg<'a, '_>> = vec![
                    Arg::Str(event_name),
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
                state
                    .after_update
                    .push(self.ctx.b.call_stmt("$.event", args));
            }
        }
        Ok(())
    }
}
