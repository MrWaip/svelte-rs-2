use svelte_ast::{NodeId, UseDirective};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_use_directive(
        &mut self,
        state: &mut EmitState<'a>,
        _owner_id: NodeId,
        owner_var: &str,
        ud: &UseDirective,
    ) -> Result<()> {
        let attr_id = ud.id;
        let blockers = self.attr_blockers(attr_id);

        let Some(name_expr) = self.take_expr_by_ref(&ud.name_ref) else {
            return CodegenError::missing_expression(attr_id);
        };

        let params = if ud.expression.is_some() {
            self.ctx.b.params(["$$node", "$$action_arg"])
        } else {
            self.ctx.b.params(["$$node"])
        };

        let mut call_args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$node")];
        if ud.expression.is_some() {
            call_args.push(Arg::Ident("$$action_arg"));
        }
        let action_call = self.ctx.b.maybe_call_expr(name_expr, call_args);
        let handler = self
            .ctx
            .b
            .arrow_expr(params, [self.ctx.b.expr_stmt(action_call)]);

        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(owner_var), Arg::Expr(handler)];
        if let Some(expr_ref) = ud.expression.as_ref() {
            let expr = self.take_attr_expr(attr_id, expr_ref)?;
            args.push(Arg::Expr(self.ctx.b.thunk(expr)));
        }

        let stmt = self.ctx.b.call_stmt("$.action", args);
        let stmt = self.wrap_run_after_blockers(stmt, &blockers);
        state.init.push(stmt);

        Ok(())
    }
}
