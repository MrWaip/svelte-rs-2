use oxc_ast::ast::{BinaryOperator, Expression, Statement};
use svelte_analyze::Volatility;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::Codegen;
use super::super::data_structures::EmitState;
use super::super::expr::evaluation_is_defined;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_select_expr_value(
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
        self.emit_select_special_value(state, el_name, val_expr, attr_id, needs_coalesce);
    }

    pub(super) fn emit_select_concat_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        attr_id: NodeId,
        val_expr: Expression<'a>,
    ) {
        self.emit_select_special_value(state, el_name, val_expr, attr_id, false);
    }

    pub(super) fn emit_input_bind_checked_value(
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
        self.emit_input_special_value(state, el_name, val_expr, needs_coalesce);
    }

    pub(super) fn emit_input_special_concat_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
    ) {
        self.emit_input_special_value(state, el_name, val_expr, false);
    }

    fn emit_select_special_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
        attr_id: NodeId,
        needs_coalesce: bool,
    ) {
        let val_for_select_option = self.ctx.b.clone_expr(&val_expr);
        let val_for_assign = self.ctx.b.clone_expr(&val_expr);
        let val_for_cache = match self.ctx.expression_data(attr_id).map(|d| d.volatility) {
            Some(Volatility::Reactive | Volatility::Heavy | Volatility::Asynchronous) => {
                Some(val_expr)
            }
            Some(Volatility::Static) | None => None,
        };

        let sequence_stmt = self.build_select_value_sequence_stmt(
            el_name,
            val_for_assign,
            val_for_select_option,
            needs_coalesce,
        );

        if let Some(val_for_cache) = val_for_cache {
            let mut prefix = String::with_capacity(el_name.len() + 6);
            prefix.push_str(el_name);
            prefix.push_str("_value");
            let cache_name = self.ctx.state.gen_ident(&prefix);

            state
                .pending_element_init
                .push(self.ctx.b.var_uninit_stmt(&cache_name));

            let cache_assign = self
                .ctx
                .b
                .assign_expr(AssignLeft::Ident(cache_name.clone()), val_for_cache);
            let test = self.ctx.b.ast.expression_binary(
                oxc_span::SPAN,
                self.ctx.b.rid_expr(&cache_name),
                BinaryOperator::StrictInequality,
                cache_assign,
            );
            let if_body = self.ctx.b.block_stmt(vec![sequence_stmt]);
            let if_stmt = self.ctx.b.if_stmt(test, if_body, None);
            state.update.push(if_stmt);
        } else {
            state.pending_element_init.push(sequence_stmt);
        }

        state
            .pending_element_init
            .push(self.ctx.b.call_stmt("$.init_select", [Arg::Ident(el_name)]));
    }

    fn build_select_value_sequence_stmt(
        &self,
        el_name: &str,
        val_for_assign: Expression<'a>,
        val_for_select_option: Expression<'a>,
        needs_coalesce: bool,
    ) -> Statement<'a> {
        let b = &self.ctx.b;
        let dunder = b.assign_expr(
            AssignLeft::StaticMember(b.static_member(b.rid_expr(el_name), "__value")),
            val_for_assign,
        );
        let value_rhs = if needs_coalesce {
            b.logical_coalesce(dunder, b.str_expr(""))
        } else {
            dunder
        };
        let value_assign = b.assign_expr(
            AssignLeft::StaticMember(b.static_member(b.rid_expr(el_name), "value")),
            value_rhs,
        );
        let select_option_call = b.call_expr(
            "$.select_option",
            [Arg::Ident(el_name), Arg::Expr(val_for_select_option)],
        );
        let sequence = b.seq_expr([value_assign, select_option_call]);
        b.expr_stmt(sequence)
    }

    fn emit_input_special_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
        needs_coalesce: bool,
    ) {
        let b = &self.ctx.b;
        let dunder = b.assign_expr(
            AssignLeft::StaticMember(b.static_member(b.rid_expr(el_name), "__value")),
            val_expr,
        );
        let value_rhs = if needs_coalesce {
            b.logical_coalesce(dunder, b.str_expr(""))
        } else {
            dunder
        };
        let value_assign = b.assign_stmt(
            AssignLeft::StaticMember(b.static_member(b.rid_expr(el_name), "value")),
            value_rhs,
        );
        state.update.push(value_assign);
    }
}
