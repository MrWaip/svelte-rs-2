use std::iter;

use oxc_ast::ast::{Expression, Statement};
use svelte_ast_builder::Arg;

use super::super::async_values::AsyncValues;
use super::super::data_structures::{EmitState, FragmentAnchor, FragmentCtx};
use super::super::{Codegen, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    #[allow(clippy::too_many_arguments)]
    pub(in super::super) fn emit_async_call_stmt(
        &mut self,
        blockers: &[u32],
        const_blockers: &[(String, usize)],
        anchor: Expression<'a>,
        node_param: &str,
        condition_param: &str,
        thunk: Option<Expression<'a>>,
        inner_stmts: Vec<Statement<'a>>,
    ) -> Result<Statement<'a>> {
        let blockers_expr = if blockers.is_empty() && const_blockers.is_empty() {
            self.ctx.b.empty_array_expr()
        } else {
            let mut members: Vec<Expression<'a>> = blockers
                .iter()
                .map(|&idx| {
                    self.ctx.b.computed_member_expr(
                        self.ctx.b.rid_expr("$$promises"),
                        self.ctx.b.num_expr(idx as f64),
                    )
                })
                .collect();
            for slot in const_blockers {
                members.push(self.ctx.blocker_slot_expr(slot));
            }
            self.ctx.b.array_expr(members)
        };
        let (async_values, callback_params) = match thunk {
            Some(thunk) => (
                Arg::Expr(self.ctx.b.array_expr([thunk])),
                self.ctx.b.params([node_param, condition_param]),
            ),
            None => (
                Arg::Expr(self.ctx.b.void_zero_expr()),
                self.ctx.b.params([node_param]),
            ),
        };
        let callback = self.ctx.b.arrow_block_expr(callback_params, inner_stmts);
        Ok(self.ctx.b.call_stmt(
            "$.async",
            [
                Arg::Expr(anchor),
                Arg::Expr(blockers_expr),
                async_values,
                Arg::Expr(callback),
            ],
        ))
    }

    pub(in crate::codegen) fn emit_async_wrapped(
        &mut self,
        state: &mut EmitState<'a>,
        blockers: &[u32],
        async_values: AsyncValues<'a>,
        anchor: Expression<'a>,
        anchor_param: &str,
        emit_next: bool,
        inner_stmts: Vec<Statement<'a>>,
    ) {
        let blockers_expr = if blockers.is_empty() {
            self.ctx.b.void_zero_expr()
        } else {
            self.ctx.b.promises_array(blockers)
        };

        let mut param_names = vec![anchor_param.to_string()];
        param_names.extend(async_values.ids());
        let values_expr = if async_values.is_empty() {
            self.ctx.b.void_zero_expr()
        } else {
            self.ctx.b.array_expr(async_values.into_thunks(self.ctx))
        };

        let callback = self.ctx.b.arrow_block_expr(
            self.ctx.b.params(param_names.iter().map(|s| s.as_str())),
            inner_stmts,
        );

        let async_stmt = self.ctx.b.call_stmt(
            "$.async",
            [
                Arg::Expr(anchor),
                Arg::Expr(blockers_expr),
                Arg::Expr(values_expr),
                Arg::Expr(callback),
            ],
        );

        if emit_next {
            let next_stmt = self.ctx.b.call_stmt("$.next", iter::empty::<Arg>());
            state
                .init
                .push(self.ctx.b.block_stmt(vec![async_stmt, next_stmt]));
        } else {
            state.init.push(async_stmt);
        }
    }
}

pub(in crate::codegen) fn owns_fragment_anchor(ctx: &FragmentCtx<'_>) -> bool {
    matches!(
        ctx.anchor,
        FragmentAnchor::Root
            | FragmentAnchor::CallbackParam {
                append_inside: true,
                ..
            }
    )
}
