use oxc_ast::ast::{Expression, Statement};

use svelte_analyze::ExprSite;
use svelte_ast::NodeId;

use svelte_ast_builder::Arg;
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

    pub(crate) fn has_await(&self) -> bool {
        self.has_await
    }

    pub(crate) fn blockers(&self) -> &[u32] {
        &self.blockers
    }

    /// Returns true if this plan's blockers are a superset of `other`.
    pub(crate) fn blockers_contain_all(&self, other: &[u32]) -> bool {
        other.iter().all(|b| self.blockers.contains(b))
    }

    pub(crate) fn async_thunk<'a>(
        &self,
        ctx: &mut Ctx<'a>,
        expr: Expression<'a>,
    ) -> Option<Expression<'a>> {
        self.has_await.then(|| ctx.b.async_thunk(expr))
    }

    pub(crate) fn wrap_async_block<'a>(
        &self,
        ctx: &mut Ctx<'a>,
        anchor: Expression<'a>,
        node_param: &str,
        condition_param: &str,
        async_thunk: Option<Expression<'a>>,
        inner_stmts: Vec<Statement<'a>>,
    ) -> Statement<'a> {
        let blockers = if self.blockers.is_empty() {
            ctx.b.empty_array_expr()
        } else {
            ctx.b.promises_array(self.blockers())
        };
        let async_values = if self.has_await {
            Arg::Expr(
                ctx.b
                    .array_expr([async_thunk.expect("async thunk missing for await plan")]),
            )
        } else {
            Arg::Expr(ctx.b.void_zero_expr())
        };
        let callback_params = if self.has_await {
            ctx.b.params([node_param, condition_param])
        } else {
            ctx.b.params([node_param])
        };
        let callback = ctx.b.arrow_block_expr(callback_params, inner_stmts);
        ctx.b.call_stmt(
            "$.async",
            [
                Arg::Expr(anchor),
                Arg::Expr(blockers),
                async_values,
                Arg::Expr(callback),
            ],
        )
    }
}
