use oxc_ast::ast::Statement;
use svelte_ast_builder::Arg;

use super::super::super::Codegen;

pub(super) enum BindPlacement<'a> {
    AfterUpdate(Statement<'a>),
    Init(Statement<'a>),
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn wrap_use_and_blockers(
        &self,
        stmt: Statement<'a>,
        has_use_directive: bool,
        bind_blockers: &[u32],
    ) -> Statement<'a> {
        let mut stmt = stmt;
        if has_use_directive {
            let effect_body = self.ctx.b.arrow_expr(self.ctx.b.no_params(), [stmt]);
            stmt = self.ctx.b.call_stmt("$.effect", [Arg::Expr(effect_body)]);
        }
        self.wrap_run_after_blockers(stmt, bind_blockers)
    }
}
