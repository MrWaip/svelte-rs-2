use oxc_ast::ast::Expression;
use svelte_analyze::{AttributeSemantics, EventHandler, EventModifier};
use svelte_ast::{Node, NodeId, OnDirectiveLegacy};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, Result};

fn handler_wrappers(mods: EventModifier) -> impl Iterator<Item = &'static str> {
    [
        (EventModifier::STOP_PROPAGATION, "stopPropagation"),
        (
            EventModifier::STOP_IMMEDIATE_PROPAGATION,
            "stopImmediatePropagation",
        ),
        (EventModifier::PREVENT_DEFAULT, "preventDefault"),
        (EventModifier::SELF, "self"),
        (EventModifier::TRUSTED, "trusted"),
        (EventModifier::ONCE, "once"),
    ]
    .into_iter()
    .filter_map(move |(flag, name)| mods.contains(flag).then_some(name))
}

fn passive_arg(mods: EventModifier) -> Option<bool> {
    if mods.contains(EventModifier::PASSIVE) {
        return Some(true);
    }
    if mods.contains(EventModifier::NONPASSIVE) {
        return Some(false);
    }
    None
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    fn on_directive_handler_emit(&self, attr_id: NodeId) -> EventHandler {
        match self.ctx.query.analysis.attributes.get(attr_id) {
            AttributeSemantics::Event(ev) => ev.handler,
            _ => EventHandler::FunctionValue,
        }
    }

    fn on_directive_modifiers(&self, attr_id: NodeId) -> EventModifier {
        match self.ctx.query.analysis.attributes.get(attr_id) {
            AttributeSemantics::Event(ev) => ev.modifiers,
            _ => EventModifier::empty(),
        }
    }

    fn on_directive_has_handler(&self, attr_id: NodeId) -> bool {
        match self.ctx.query.analysis.attributes.get(attr_id) {
            AttributeSemantics::Event(ev) => ev.handler.is_user(),
            _ => false,
        }
    }

    fn build_on_directive_handler(
        &mut self,
        state: &mut EmitState<'a>,
        od: &OnDirectiveLegacy,
    ) -> Result<Expression<'a>> {
        let attr_id = od.id;
        let expr_offset = od.expression.as_ref().map(|r| r.span.start);
        let handler_emit = self.on_directive_handler_emit(attr_id);

        if let (Some(offset), Some(expr_ref)) = (expr_offset, od.expression.as_ref()) {
            let expr = self.take_attr_expr(attr_id, expr_ref)?;
            Ok(self.build_event_handler_s5(attr_id, expr, handler_emit, &mut state.init, offset))
        } else {
            let bubble_call = self
                .ctx
                .b
                .static_member_expr(self.ctx.b.rid_expr("$.bubble_event"), "call");
            let call = self.ctx.b.call_expr_callee(
                bubble_call,
                [
                    Arg::Expr(self.ctx.b.this_expr()),
                    Arg::Ident("$$props"),
                    Arg::Ident("$$arg"),
                ],
            );
            Ok(self.ctx.b.function_expr(
                self.ctx.b.params(["$$arg"]),
                vec![self.ctx.b.expr_stmt(call)],
            ))
        }
    }

    pub(in super::super) fn emit_on_directive_legacy(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_var: &str,
        od: &OnDirectiveLegacy,
    ) -> Result<()> {
        let handler = self.build_on_directive_handler(state, od)?;

        let mods = self.on_directive_modifiers(od.id);
        let mut wrapped = handler;
        for wrapper in handler_wrappers(mods) {
            let fn_name = format!("$.{wrapper}");
            wrapped = self.ctx.b.call_expr(&fn_name, [Arg::Expr(wrapped)]);
        }
        let wrapped = if self.on_directive_has_handler(od.id) {
            self.dev_event_handler(od.id, wrapped, &od.name)?
        } else {
            wrapped
        };

        let capture = mods.contains(EventModifier::CAPTURE);
        let passive = passive_arg(mods);
        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::StrRef(&od.name),
            Arg::Ident(owner_var),
            Arg::Expr(wrapped),
        ];
        if capture || passive.is_some() {
            args.push(if capture {
                Arg::Bool(true)
            } else {
                Arg::Expr(self.ctx.b.void_zero_expr())
            });
        }
        if let Some(p) = passive {
            args.push(Arg::Bool(p));
        }

        let stmt = self.ctx.b.call_stmt("$.event", args);
        let is_svelte_element = matches!(
            self.ctx.query.component.store.get(owner_id),
            Node::SvelteElement(_)
        );
        if !is_svelte_element && self.ctx.has_use_directive(owner_id) {
            let effect_body = self.ctx.b.arrow_expr(self.ctx.b.no_params(), [stmt]);
            let wrapped = self.ctx.b.call_stmt("$.effect", [Arg::Expr(effect_body)]);
            state.pending_element_init.push(wrapped);
        } else {
            state.after_update.push(stmt);
        }
        Ok(())
    }
}
