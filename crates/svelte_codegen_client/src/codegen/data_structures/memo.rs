use oxc_ast::ast::Expression;
use svelte_analyze::{ExprSite, ExpressionInfo};
use svelte_ast::NodeId;

use crate::context::Ctx;

pub(crate) enum MemoValueRef {
    Sync(usize),
    Async(usize),
}

#[derive(Default)]
pub(crate) struct TemplateMemoState<'a> {
    pub(crate) sync_values: Vec<Expression<'a>>,
    pub(crate) async_values: Vec<Expression<'a>>,
    pub(crate) blockers: Vec<u32>,
    pub(crate) extra_blockers: Vec<Expression<'a>>,
}

impl<'a> TemplateMemoState<'a> {
    pub(crate) fn push_script_blocker(&mut self, idx: u32) {
        if !self.blockers.contains(&idx) {
            self.blockers.push(idx);
        }
    }

    pub(crate) fn push_expr_info(&mut self, ctx: &Ctx<'a>, info: &ExpressionInfo) {
        for sym in info.ref_symbols() {
            if let Some(idx) = ctx.symbol_blocker(*sym) {
                self.push_script_blocker(idx);
            }
            if let Some(expr) = ctx.const_tag_symbol_blocker_expr(*sym) {
                self.extra_blockers.push(expr);
            }
        }
    }

    pub(crate) fn push_node_deps(&mut self, ctx: &mut Ctx<'a>, id: NodeId) {
        let blockers = ctx
            .expr_deps(ExprSite::Node(id))
            .map(|deps| deps.blockers)
            .unwrap_or_default();
        for idx in blockers {
            self.push_script_blocker(idx);
        }
        self.extra_blockers.extend(ctx.const_tag_blocker_exprs(id));
    }

    pub(crate) fn add_memoized_expr(
        &mut self,
        ctx: &Ctx<'a>,
        info: &ExpressionInfo,
        expr: Expression<'a>,
    ) -> Option<MemoValueRef> {
        self.push_expr_info(ctx, info);

        if !info.needs_memoized_value() {
            return None;
        }

        if info.has_await() {
            let index = self.async_values.len();
            self.async_values.push(expr);
            Some(MemoValueRef::Async(index))
        } else {
            let index = self.sync_values.len();
            self.sync_values.push(expr);
            Some(MemoValueRef::Sync(index))
        }
    }

    pub(crate) fn async_values_push(&mut self, expr: Expression<'a>) -> usize {
        let index = self.async_values.len();
        self.async_values.push(expr);
        index
    }

    pub(crate) fn sync_values_push(&mut self, expr: Expression<'a>) -> usize {
        let index = self.sync_values.len();
        self.sync_values.push(expr);
        index
    }

    pub(crate) fn has_deps(&self) -> bool {
        !self.sync_values.is_empty()
            || !self.async_values.is_empty()
            || !self.blockers.is_empty()
            || !self.extra_blockers.is_empty()
    }

    pub(crate) fn has_sync_values(&self) -> bool {
        !self.sync_values.is_empty()
    }

    pub(crate) fn has_async_values(&self) -> bool {
        !self.async_values.is_empty()
    }

    pub(crate) fn has_blockers(&self) -> bool {
        !self.blockers.is_empty() || !self.extra_blockers.is_empty()
    }

    pub(crate) fn param_names(&self) -> Vec<String> {
        let total = self.sync_values.len() + self.async_values.len();
        (0..total).map(|i| format!("${i}")).collect()
    }

    pub(crate) fn sync_param_expr(&self, ctx: &Ctx<'a>, index: usize) -> Expression<'a> {
        ctx.b.rid_expr(&format!("${index}"))
    }

    pub(crate) fn async_param_expr(&self, ctx: &Ctx<'a>, index: usize) -> Expression<'a> {
        ctx.b
            .rid_expr(&format!("${}", self.sync_values.len() + index))
    }

    pub(crate) fn sync_values_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        if self.sync_values.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(
                self.sync_values
                    .drain(..)
                    .map(|expr| ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(expr)])),
            )
        }
    }

    pub(crate) fn async_values_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        if self.async_values.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(
                self.async_values
                    .drain(..)
                    .map(|expr| super::super::effect::async_value_thunk(ctx, expr)),
            )
        }
    }

    pub(crate) fn blockers_expr(&mut self, ctx: &Ctx<'a>) -> Expression<'a> {
        let mut all_blockers: Vec<Expression<'a>> = self
            .blockers
            .iter()
            .map(|&idx| {
                ctx.b
                    .computed_member_expr(ctx.b.rid_expr("$$promises"), ctx.b.num_expr(idx as f64))
            })
            .collect();
        all_blockers.append(&mut self.extra_blockers);
        if all_blockers.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(all_blockers)
        }
    }
}

pub(crate) enum MemoAttrUpdate {
    Call {
        setter_fn: &'static str,
        attr_name: Option<String>,
    },
    Assignment {
        property: String,
    },
}

pub(crate) struct MemoAttr<'a> {
    pub attr_id: NodeId,
    pub el_name: String,
    pub update: MemoAttrUpdate,
    pub expr: Expression<'a>,
    pub is_node_site: bool,
}
