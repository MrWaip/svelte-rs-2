use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::ObjProp;

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

        let cn = self.ctx.query.component_node(el_id);
        let event_expr_offsets: Vec<(NodeId, u32)> = cn
            .attributes
            .iter()
            .filter_map(|attr| {
                let Attribute::OnDirectiveLegacy(dir) = attr else {
                    return None;
                };
                dir.expression_span.map(|span| (dir.id, span.start))
            })
            .collect();

        let mut event_groups: Vec<(String, Vec<Expression<'a>>)> = Vec::new();
        for ev in events {
            if !ev.has_expression {
                continue;
            }
            let Some(expr_offset) = event_expr_offsets
                .iter()
                .find_map(|(id, offset)| (*id == ev.attr_id).then_some(*offset))
            else {
                return CodegenError::missing_expression(ev.attr_id);
            };
            let has_call = self
                .ctx
                .attr_expression(ev.attr_id)
                .is_some_and(|info| info.has_call());
            let handler_expr = self.take_attr_expr(ev.attr_id)?;
            let handler =
                self.build_event_handler_s5(ev.attr_id, handler_expr, has_call, init, expr_offset);
            let handler = self.dev_event_handler(ev.attr_id, handler, &ev.name, expr_offset)?;
            let handler = if ev.has_once_modifier {
                self.ctx
                    .b
                    .call_expr("$.once", [svelte_ast_builder::Arg::Expr(handler)])
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
                    if matches!(&handler, Expression::FunctionExpression(_)) {
                        return ObjProp::Method(key, handler);
                    }
                    if let Expression::Identifier(id) = &handler {
                        if id.name.as_str() == name {
                            return ObjProp::Shorthand(key);
                        }
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
}
