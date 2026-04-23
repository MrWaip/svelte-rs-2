use svelte_ast::{AnimateDirective, NodeId};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_animate_directive(
        &mut self,
        state: &mut EmitState<'a>,
        _owner_id: NodeId,
        owner_var: &str,
        ad: &AnimateDirective,
    ) -> Result<()> {
        let attr_id = ad.id;
        let blockers = self.attr_blockers(attr_id);

        let Some(name_expr) = self.take_expr_at_offset(ad.name.start) else {
            return CodegenError::missing_expression(attr_id);
        };
        let name_thunk = self.ctx.b.thunk(name_expr);

        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(owner_var), Arg::Expr(name_thunk)];

        if ad.expression_span.is_some() {
            let expr = self.take_attr_expr(attr_id)?;
            args.push(Arg::Expr(self.ctx.b.thunk(expr)));
        } else {
            args.push(Arg::Expr(self.ctx.b.null_expr()));
        }

        let stmt = self.ctx.b.call_stmt("$.animation", args);
        let stmt = self.wrap_run_after_blockers(stmt, &blockers);
        state.after_update.push(stmt);

        Ok(())
    }
}
