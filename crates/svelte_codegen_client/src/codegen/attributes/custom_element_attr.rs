use oxc_ast::ast::Expression;
use svelte_ast_builder::Arg;

use super::super::Codegen;
use super::super::data_structures::EmitState;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_custom_element_data(
        &self,
        state: &mut EmitState<'a>,
        owner_var: &str,
        attr_name: &str,
        value: Expression<'a>,
        reactive: bool,
    ) {
        let b = &self.ctx.b;
        let call = b.call_expr(
            "$.set_custom_element_data",
            [
                Arg::Ident(owner_var),
                Arg::Str(attr_name.to_string()),
                Arg::Expr(value),
            ],
        );

        if !reactive {
            state.init.push(b.expr_stmt(call));
            return;
        }

        let thunk = b.thunk(call);
        state
            .init
            .push(b.call_stmt("$.template_effect", [Arg::Expr(thunk)]));
    }
}
