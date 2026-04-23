use svelte_ast::{NodeId, TransitionDirection, TransitionDirective};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_transition_directive(
        &mut self,
        state: &mut EmitState<'a>,
        _owner_id: NodeId,
        owner_var: &str,
        td: &TransitionDirective,
    ) -> Result<()> {
        let attr_id = td.id;
        let blockers = self.attr_blockers(attr_id);

        let mut flags: u32 = if self
            .ctx
            .query
            .view
            .event_modifiers(attr_id)
            .contains(svelte_analyze::EventModifier::GLOBAL)
        {
            4
        } else {
            0
        };
        match td.direction {
            TransitionDirection::Both => flags |= 1 | 2,
            TransitionDirection::In => flags |= 1,
            TransitionDirection::Out => flags |= 2,
        }

        let Some(name_expr) = self.take_expr_at_offset(td.name.start) else {
            return CodegenError::missing_expression(attr_id);
        };
        let name_thunk = self.ctx.b.thunk(name_expr);

        let mut args: Vec<Arg<'a, '_>> = vec![
            Arg::Num(flags as f64),
            Arg::Ident(owner_var),
            Arg::Expr(name_thunk),
        ];

        if td.expression_span.is_some() {
            let expr = self.take_attr_expr(attr_id)?;
            args.push(Arg::Expr(self.ctx.b.thunk(expr)));
        }

        let stmt = self.ctx.b.call_stmt("$.transition", args);
        let stmt = self.wrap_run_after_blockers(stmt, &blockers);
        state.after_update.push(stmt);

        Ok(())
    }
}
