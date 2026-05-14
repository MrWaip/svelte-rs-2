use oxc_ast::ast::{Expression, Statement};

use svelte_ast::NodeId;

use super::super::async_emit::emit_async_call_stmt;
use crate::context::Ctx;

pub(crate) struct AsyncEmission {
    has_await: bool,
    blockers: Vec<u32>,
}

impl AsyncEmission {
    pub(crate) fn for_node(ctx: &Ctx<'_>, id: NodeId) -> Self {
        let Some(data) = ctx.expression_data(id) else {
            return Self {
                has_await: false,
                blockers: Vec::new(),
            };
        };
        Self {
            has_await: matches!(
                data.kind,
                svelte_analyze::ExprKind::Async { has_await: true }
            ),
            blockers: data.blockers.to_vec(),
        }
    }

    pub(crate) fn needs_async(&self) -> bool {
        self.has_await || !self.blockers.is_empty()
    }

    pub(crate) fn async_thunk<'a>(
        &self,
        ctx: &mut Ctx<'a>,
        expr: Expression<'a>,
    ) -> Option<Expression<'a>> {
        self.has_await.then(|| ctx.b.async_thunk(expr))
    }

    pub(crate) fn emit_async_call_stmt<'a>(
        &self,
        ctx: &mut Ctx<'a>,
        anchor: Expression<'a>,
        node_param: &str,
        condition_param: &str,
        thunk: Option<Expression<'a>>,
        inner_stmts: Vec<Statement<'a>>,
    ) -> Statement<'a> {
        emit_async_call_stmt(
            ctx,
            self.has_await,
            &self.blockers,
            anchor,
            node_param,
            condition_param,
            thunk,
            inner_stmts,
        )
    }
}
