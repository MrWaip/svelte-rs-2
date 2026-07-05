use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, ObjProp};

use super::super::{Codegen, CodegenError, Result};
use super::dispatch::{EventRaw, PropOrSpread};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn build_component_events(
        &mut self,
        el_id: NodeId,
        events: Vec<EventRaw>,
        items: &mut Vec<PropOrSpread<'a>>,
        init: &mut Vec<Statement<'a>>,
    ) -> Result<()> {
        if events.is_empty() {
            return Ok(());
        }

        let Some(view) = self
            .ctx
            .query
            .component
            .store
            .get(el_id)
            .as_component_like()
        else {
            return CodegenError::unexpected_node(el_id, "component-like");
        };
        let event_expr_offsets: Vec<(NodeId, u32)> = view
            .attributes
            .iter()
            .filter_map(|attr| {
                let Attribute::OnDirectiveLegacy(dir) = attr else {
                    return None;
                };
                dir.expression.as_ref().map(|r| (dir.id, r.span.start))
            })
            .collect();

        let mut event_groups: Vec<(String, Vec<Expression<'a>>)> = Vec::new();
        for ev in events {
            let handler = if let Some(expr_id) = ev.expr_id {
                let expr_offset = event_expr_offsets
                    .iter()
                    .find_map(|(id, offset)| (*id == ev.attr_id).then_some(*offset))
                    .ok_or(CodegenError::MissingExpression(ev.attr_id))?;
                let handler_emit = match self.ctx.query.analysis.attributes.get(ev.attr_id) {
                    svelte_analyze::AttributeSemantics::Event(esem) => match &esem.emit {
                        svelte_analyze::EventEmit::HtmlDelegated { handler }
                        | svelte_analyze::EventEmit::HtmlDirect { handler, .. }
                        | svelte_analyze::EventEmit::Component { handler } => *handler,
                        svelte_analyze::EventEmit::HtmlBubble => {
                            svelte_analyze::HandlerEmit::Direct
                        }
                    },
                    _ => svelte_analyze::HandlerEmit::Direct,
                };
                let Some(handler_expr) = self.ctx.state.parsed.take_expr(expr_id) else {
                    return CodegenError::missing_expression(ev.attr_id);
                };
                let handler = self.build_event_handler_s5(
                    ev.attr_id,
                    handler_expr,
                    handler_emit,
                    init,
                    expr_offset,
                );
                self.dev_component_event_handler(ev.attr_id, handler)?
            } else {
                self.build_bubble_event_method_legacy()
            };
            let handler = if ev.has_once_modifier {
                self.ctx.b.call_expr("$.once", [Arg::Expr(handler)])
            } else {
                handler
            };

            if let Some((_, handlers)) = event_groups
                .iter_mut()
                .find(|(existing_name, _)| existing_name == &ev.name)
            {
                handlers.push(handler);
            } else {
                event_groups.push((ev.name, vec![handler]));
            }
        }

        if event_groups.is_empty() {
            return Ok(());
        }

        let event_props: Vec<ObjProp<'a>> = event_groups
            .into_iter()
            .map(|(name, handlers)| {
                let key = self.ctx.b.alloc_str(&name);
                if handlers.len() == 1 {
                    let Some(handler) = handlers.into_iter().next() else {
                        return ObjProp::Shorthand(key);
                    };
                    if matches!(
                        handler.get_inner_expression(),
                        Expression::FunctionExpression(_)
                    ) {
                        return ObjProp::Method(key, handler);
                    }
                    if let Expression::Identifier(id) = handler.get_inner_expression()
                        && id.name.as_str() == name
                    {
                        return ObjProp::Shorthand(key);
                    }
                    ObjProp::KeyValue(key, handler)
                } else {
                    ObjProp::KeyValue(key, self.ctx.b.array_expr(handlers))
                }
            })
            .collect();

        items.push(PropOrSpread::Prop(ObjProp::KeyValue(
            "$$events",
            self.ctx.b.object_expr(event_props),
        )));

        Ok(())
    }

    fn build_bubble_event_method_legacy(&self) -> Expression<'a> {
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
        self.ctx.b.function_expr(
            self.ctx.b.params(["$$arg"]),
            vec![self.ctx.b.expr_stmt(call)],
        )
    }
}
