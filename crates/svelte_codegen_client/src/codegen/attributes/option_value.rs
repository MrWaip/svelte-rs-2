use std::iter;

use oxc_ast::ast::{BinaryOperator, Expression};
use svelte_ast::NodeId;
use svelte_ast_builder::AssignLeft;

use super::super::Codegen;
use super::super::data_structures::EmitState;
use super::super::expr::evaluation_is_defined;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_option_expr_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        attr_id: NodeId,
        val_expr: Expression<'a>,
    ) {
        let needs_coalesce = !self
            .ctx
            .expression_data(attr_id)
            .is_some_and(|d| evaluation_is_defined(&d.evaluation));
        self.emit_option_special_value(state, el_name, val_expr, needs_coalesce);
    }

    pub(super) fn emit_option_concat_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
    ) {
        self.emit_option_special_value(state, el_name, val_expr, false);
    }

    fn emit_option_special_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
        needs_coalesce: bool,
    ) {
        let mut prefix = String::with_capacity(el_name.len() + 6);
        prefix.push_str(el_name);
        prefix.push_str("_value");
        let cache_name = self.ctx.state.gen_ident(&prefix);

        let init = self.ctx.b.object_expr(iter::empty());
        state
            .pending_pre_update
            .push(self.ctx.b.var_stmt(&cache_name, init));

        let val_expr2 = self.ctx.b.clone_expr(&val_expr);

        let cache_assign = self
            .ctx
            .b
            .assign_expr(AssignLeft::Ident(cache_name.clone()), val_expr);

        let test = self.ctx.b.ast.expression_binary(
            oxc_span::SPAN,
            self.ctx.b.rid_expr(&cache_name),
            BinaryOperator::StrictInequality,
            cache_assign,
        );

        let dunder_value_assign = self.ctx.b.assign_expr(
            AssignLeft::StaticMember(
                self.ctx
                    .b
                    .static_member(self.ctx.b.rid_expr(el_name), "__value"),
            ),
            val_expr2,
        );

        let value_rhs = if needs_coalesce {
            self.ctx
                .b
                .logical_coalesce(dunder_value_assign, self.ctx.b.str_expr(""))
        } else {
            dunder_value_assign
        };

        let value_assign = self.ctx.b.assign_stmt(
            AssignLeft::StaticMember(
                self.ctx
                    .b
                    .static_member(self.ctx.b.rid_expr(el_name), "value"),
            ),
            value_rhs,
        );

        let if_body = self.ctx.b.block_stmt(vec![value_assign]);
        let if_stmt = self.ctx.b.if_stmt(test, if_body, None);

        state.update.push(if_stmt);
    }
}
