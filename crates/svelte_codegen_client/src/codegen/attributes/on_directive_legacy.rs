use oxc_ast::ast::Expression;
use svelte_ast::{NodeId, OnDirectiveLegacy};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_on_directive_legacy(
        &mut self,
        state: &mut EmitState<'a>,
        _owner_id: NodeId,
        owner_var: &str,
        od: &OnDirectiveLegacy,
    ) -> Result<()> {
        let attr_id = od.id;
        let expr_offset = od.expression.as_ref().map(|r| r.span.start);
        let has_call = self
            .ctx
            .attr_expression(attr_id)
            .is_some_and(|e| e.has_call());

        let handler: Expression<'a> =
            if let (Some(offset), Some(expr_ref)) = (expr_offset, od.expression.as_ref()) {
                let expr = self.take_attr_expr(attr_id, expr_ref)?;
                self.build_event_handler_s5(attr_id, expr, has_call, &mut state.init, offset)
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
                self.ctx.b.function_expr(
                    self.ctx.b.params(["$$arg"]),
                    vec![self.ctx.b.expr_stmt(call)],
                )
            };

        let handler = if let Some(offset) = expr_offset {
            self.dev_event_handler(attr_id, handler, &od.name, offset)?
        } else {
            handler
        };

        let mods = od.parsed_modifiers();
        let mut wrapped = handler;
        for wrapper in mods.handler_wrappers() {
            let fn_name = format!("$.{wrapper}");
            wrapped = self.ctx.b.call_expr(&fn_name, [Arg::Expr(wrapped)]);
        }

        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::StrRef(&od.name),
            Arg::Ident(owner_var),
            Arg::Expr(wrapped),
        ];
        if mods.capture || mods.passive.is_some() {
            args.push(if mods.capture {
                Arg::Bool(true)
            } else {
                Arg::Expr(self.ctx.b.void_zero_expr())
            });
        }
        if let Some(p) = mods.passive {
            args.push(Arg::Bool(p));
        }

        state
            .after_update
            .push(self.ctx.b.call_stmt("$.event", args));
        Ok(())
    }
}
