use std::iter;

use oxc_ast::ast::{BinaryOperator, Expression, Statement};
use svelte_ast::NodeId;
use svelte_ast_builder::AssignLeft;

use super::super::Codegen;
use super::super::Result;
use super::super::data_structures::EmitState;

pub(in super::super) enum OptionValueForm {
    Reflected { coalesce: bool },
    Hidden,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_option_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
        form: OptionValueForm,
        volatile: bool,
    ) {
        if !volatile {
            let value_assign = self.build_option_value_assign(el_name, val_expr, form);
            state.pending_element_init.push(value_assign);
            return;
        }

        let mut prefix = String::with_capacity(el_name.len() + 6);
        prefix.push_str(el_name);
        prefix.push_str("_value");
        let cache_name = self.ctx.state.gen_ident(&prefix);

        let init = self.ctx.b.object_expr(iter::empty());
        state
            .pending_pre_update
            .push(self.ctx.b.var_stmt(&cache_name, init));

        let val_for_assign = self.ctx.b.clone_expr(&val_expr);

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

        let value_assign = self.build_option_value_assign(el_name, val_for_assign, form);
        let if_body = self.ctx.b.block_stmt(vec![value_assign]);
        let if_stmt = self.ctx.b.if_stmt(test, if_body, None);

        state.pending_element_update.push(if_stmt);
    }

    pub(in super::super) fn emit_option_synthetic_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        expr_id: NodeId,
    ) -> Result<()> {
        let data = self.ctx.expression_data(expr_id).cloned();
        let volatile = data
            .as_ref()
            .map(|d| d.volatility.is_volatile())
            .unwrap_or(false);
        let expr = self.clone_node_expr(expr_id)?;
        let Some(data) = data else {
            self.emit_option_value(state, el_name, expr, OptionValueForm::Hidden, volatile);
            return Ok(());
        };
        let value = self.defer_memo_value(state, expr_id, &data, expr);
        self.emit_option_value(state, el_name, value, OptionValueForm::Hidden, volatile);
        Ok(())
    }

    pub(in super::super) fn emit_special_value_static(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
        coalesce: bool,
    ) {
        let value_assign = self.build_option_value_assign(
            el_name,
            val_expr,
            OptionValueForm::Reflected { coalesce },
        );
        state.pending_element_init.push(value_assign);
    }

    fn build_option_value_assign(
        &self,
        el_name: &str,
        val_expr: Expression<'a>,
        form: OptionValueForm,
    ) -> Statement<'a> {
        let dunder_member = self
            .ctx
            .b
            .static_member(self.ctx.b.rid_expr(el_name), "__value");

        let coalesce = match form {
            OptionValueForm::Hidden => {
                return self
                    .ctx
                    .b
                    .assign_stmt(AssignLeft::StaticMember(dunder_member), val_expr);
            }
            OptionValueForm::Reflected { coalesce } => coalesce,
        };

        let dunder_value_assign = self
            .ctx
            .b
            .assign_expr(AssignLeft::StaticMember(dunder_member), val_expr);

        let value_rhs = if coalesce {
            self.ctx
                .b
                .logical_coalesce(dunder_value_assign, self.ctx.b.str_expr(""))
        } else {
            dunder_value_assign
        };

        self.ctx.b.assign_stmt(
            AssignLeft::StaticMember(
                self.ctx
                    .b
                    .static_member(self.ctx.b.rid_expr(el_name), "value"),
            ),
            value_rhs,
        )
    }
}
