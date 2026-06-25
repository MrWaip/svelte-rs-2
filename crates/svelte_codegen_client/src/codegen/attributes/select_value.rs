use oxc_ast::ast::{BinaryOperator, Expression, Statement};
use svelte_analyze::Volatility;
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::data_structures::{EmitState, MemoValueRef};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_select_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        attr_id: NodeId,
        val_expr: Expression<'a>,
        coalesce: bool,
    ) -> Result<()> {
        match self.ctx.expression_data(attr_id).map(|d| d.volatility) {
            Some(Volatility::Heavy | Volatility::Asynchronous) => {
                let Some(data) = self.ctx.expression_data(attr_id).cloned() else {
                    return CodegenError::missing_expression_deps(attr_id);
                };
                let placeholder = match state
                    .shared_memo
                    .add_memoized_expr(self.ctx, &data, val_expr)
                {
                    Some(MemoValueRef::Sync(i)) => state.shared_memo.sync_param_expr(self.ctx, i),
                    Some(MemoValueRef::Async(i)) => state.shared_memo.async_param_expr(self.ctx, i),
                    None => return CodegenError::missing_expression_deps(attr_id),
                };
                self.emit_select_value_core(state, el_name, placeholder, coalesce, true);
            }
            Some(Volatility::Reactive) => {
                self.emit_select_value_core(state, el_name, val_expr, coalesce, true);
            }
            Some(Volatility::Static) | None => {
                self.emit_select_value_core(state, el_name, val_expr, coalesce, false);
            }
        }
        Ok(())
    }

    pub(in super::super) fn emit_select_value_core(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
        coalesce: bool,
        volatile: bool,
    ) {
        if volatile {
            self.push_select_value_guarded(state, el_name, val_expr, coalesce);
        } else {
            let val_for_select_option = self.ctx.b.clone_expr(&val_expr);
            let sequence_stmt = self.build_select_value_sequence_stmt(
                el_name,
                val_expr,
                val_for_select_option,
                coalesce,
            );
            state.pending_element_init.push(sequence_stmt);
        }

        state
            .pending_element_init
            .push(self.ctx.b.call_stmt("$.init_select", [Arg::Ident(el_name)]));
    }

    fn push_select_value_guarded(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
        coalesce: bool,
    ) {
        let val_for_select_option = self.ctx.b.clone_expr(&val_expr);
        let val_for_assign = self.ctx.b.clone_expr(&val_expr);
        let sequence_stmt = self.build_select_value_sequence_stmt(
            el_name,
            val_for_assign,
            val_for_select_option,
            coalesce,
        );

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
            .assign_expr(AssignLeft::Ident(cache_name.clone()), val_expr);
        let test = self.ctx.b.ast.expression_binary(
            oxc_span::SPAN,
            self.ctx.b.rid_expr(&cache_name),
            BinaryOperator::StrictInequality,
            cache_assign,
        );
        let if_body = self.ctx.b.block_stmt(vec![sequence_stmt]);
        let if_stmt = self.ctx.b.if_stmt(test, if_body, None);
        state.pending_element_update.push(if_stmt);
    }

    fn build_select_value_sequence_stmt(
        &self,
        el_name: &str,
        val_for_assign: Expression<'a>,
        val_for_select_option: Expression<'a>,
        coalesce: bool,
    ) -> Statement<'a> {
        let b = &self.ctx.b;
        let dunder = b.assign_expr(
            AssignLeft::StaticMember(b.static_member(b.rid_expr(el_name), "__value")),
            val_for_assign,
        );
        let value_rhs = if coalesce {
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

    pub(in super::super) fn emit_input_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
        coalesce: bool,
    ) {
        let b = &self.ctx.b;
        let dunder = b.assign_expr(
            AssignLeft::StaticMember(b.static_member(b.rid_expr(el_name), "__value")),
            val_expr,
        );
        let value_rhs = if coalesce {
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
