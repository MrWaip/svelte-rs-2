use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::ExprSite;
use svelte_ast::NodeId;

use super::super::async_plan::emit_async_call_stmt;
use crate::context::Ctx;

pub(crate) struct AsyncEmissionPlan {
    has_await: bool,
    blockers: Vec<u32>,
}

impl AsyncEmissionPlan {
    pub(crate) fn for_node(ctx: &Ctx<'_>, id: NodeId) -> Self {
        let Some(deps) = ctx.expr_deps(ExprSite::Node(id)) else {
            return Self {
                has_await: false,
                blockers: Vec::new(),
            };
        };
        Self {
            has_await: deps.has_await(),
            blockers: deps.blockers.to_vec(),
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
