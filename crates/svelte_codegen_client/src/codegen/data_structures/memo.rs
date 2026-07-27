use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, IdentifierReference};
use oxc_ast_visit::VisitMut;
use svelte_analyze::{ExpressionData, Suspension, Volatility};
use svelte_ast::NodeId;

use crate::context::Ctx;

#[derive(Clone, Copy)]
pub(crate) enum MemoValueRef {
    Sync(usize),
    Async(usize),
}

#[derive(Default)]
pub(crate) struct TemplateMemoState<'a> {
    pub(crate) sync_values: Vec<Expression<'a>>,
    pub(crate) async_values: Vec<(Expression<'a>, Suspension)>,
    pub(crate) blockers: Vec<u32>,
    pub(crate) blocker_members: Vec<u32>,
    pub(crate) extra_blockers: Vec<(String, usize)>,
    late_slots: Vec<Option<MemoValueRef>>,
}

pub(crate) struct DeferredMemoValue<'a> {
    pub(crate) node: NodeId,
    pub(crate) late_id: Option<usize>,
    pub(crate) data: ExpressionData,
    pub(crate) expr: Option<Expression<'a>>,
}

impl<'a> TemplateMemoState<'a> {
    pub(crate) fn push_blocker_slot(&mut self, slot: svelte_analyze::BlockerSlot) {
        if self.blocker_members.contains(&slot.member) {
            return;
        }
        self.blocker_members.push(slot.member);
        self.blockers.push(slot.entry);
    }

    pub(crate) fn reserve_late(&mut self) -> usize {
        self.late_slots.push(None);
        self.late_slots.len() - 1
    }

    pub(crate) fn late_param_expr(&self, ctx: &Ctx<'a>, late_id: usize) -> Expression<'a> {
        ctx.b.rid_expr(&late_placeholder(late_id))
    }

    pub(crate) fn resolve_late(&mut self, late_id: usize, slot: Option<MemoValueRef>) {
        let Some(entry) = self.late_slots.get_mut(late_id) else {
            return;
        };
        *entry = slot;
    }

    pub(crate) fn push_const_tag_blocker(&mut self, slot: (String, usize)) {
        push_blocker_slot(&mut self.extra_blockers, slot);
    }

    pub(crate) fn push_expression_data(&mut self, ctx: &Ctx<'a>, data: &ExpressionData) {
        for &sym in data.blocker_references.iter() {
            if let Some(slot) = ctx.symbol_blocker(sym) {
                self.push_blocker_slot(slot);
            }
            if let Some(slot) = ctx.const_tag_symbol_blocker_slot(sym) {
                self.push_const_tag_blocker(slot);
            }
        }
    }

    pub(crate) fn push_node_deps(&mut self, ctx: &mut Ctx<'a>, id: NodeId) {
        let references = ctx
            .expression_data(id)
            .map(|d| d.blocker_references.clone())
            .unwrap_or_default();
        for sym in references {
            if let Some(slot) = ctx.symbol_blocker(sym) {
                self.push_blocker_slot(slot);
            }
        }
        for slot in ctx.const_tag_blocker_slots(id) {
            self.push_const_tag_blocker(slot);
        }
    }

    pub(crate) fn add_memoized_expr(
        &mut self,
        ctx: &Ctx<'a>,
        data: &ExpressionData,
        expr: Expression<'a>,
    ) -> Option<MemoValueRef> {
        self.push_expression_data(ctx, data);
        match data.volatility {
            Volatility::Asynchronous => {
                let index = self.async_values.len();
                self.async_values.push((expr, data.suspension));
                Some(MemoValueRef::Async(index))
            }
            Volatility::Heavy => {
                let index = self.sync_values.len();
                self.sync_values.push(expr);
                Some(MemoValueRef::Sync(index))
            }
            Volatility::Reactive | Volatility::Static => None,
        }
    }

    pub(crate) fn async_values_push(
        &mut self,
        expr: Expression<'a>,
        suspension: Suspension,
    ) -> usize {
        let index = self.async_values.len();
        self.async_values.push((expr, suspension));
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

    pub(crate) fn blocker_count(&self) -> usize {
        self.blockers.len() + self.extra_blockers.len()
    }

    pub(crate) fn param_names(&self) -> Vec<String> {
        let total = self.sync_values.len() + self.async_values.len();
        (0..total).map(|i| format!("${i}")).collect()
    }

    pub(crate) fn sync_param_expr(&self, ctx: &Ctx<'a>, index: usize) -> Expression<'a> {
        ctx.b.rid_expr(&sync_placeholder(index))
    }

    pub(crate) fn async_param_expr(&self, ctx: &Ctx<'a>, index: usize) -> Expression<'a> {
        ctx.b.rid_expr(&async_placeholder(index))
    }

    pub(crate) fn resolve_param_names(&self, ctx: &Ctx<'a>, expr: &mut Expression<'a>) {
        if self.sync_values.is_empty() && self.async_values.is_empty() {
            return;
        }
        let mut renamer = ParamRenamer {
            allocator: ctx.b.ast.allocator,
            sync_count: self.sync_values.len(),
            late_slots: &self.late_slots,
        };
        renamer.visit_expression(expr);
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
            ctx.b
                .array_expr(self.async_values.drain(..).map(|(expr, suspension)| {
                    super::super::effect::suspending_thunk(ctx, expr, suspension)
                }))
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
        for slot in self.extra_blockers.drain(..) {
            all_blockers.push(ctx.blocker_slot_expr(&slot));
        }
        if all_blockers.is_empty() {
            ctx.b.void_zero_expr()
        } else {
            ctx.b.array_expr(all_blockers)
        }
    }
}

fn sync_placeholder(index: usize) -> String {
    format!("#sync{index}")
}

fn async_placeholder(index: usize) -> String {
    format!("#async{index}")
}

fn late_placeholder(late_id: usize) -> String {
    format!("#late{late_id}")
}

struct ParamRenamer<'a, 'm> {
    allocator: &'a Allocator,
    sync_count: usize,
    late_slots: &'m [Option<MemoValueRef>],
}

impl ParamRenamer<'_, '_> {
    fn slot_name(&self, slot: MemoValueRef) -> String {
        match slot {
            MemoValueRef::Sync(index) => format!("${index}"),
            MemoValueRef::Async(index) => format!("${}", self.sync_count + index),
        }
    }

    fn resolved(&self, name: &str) -> Option<String> {
        if let Some(rest) = name.strip_prefix("#late")
            && let Ok(late_id) = rest.parse::<usize>()
        {
            let slot = self.late_slots.get(late_id).copied().flatten()?;
            return Some(self.slot_name(slot));
        }
        if let Some(rest) = name.strip_prefix("#sync")
            && let Ok(index) = rest.parse::<usize>()
        {
            return Some(format!("${index}"));
        }
        if let Some(rest) = name.strip_prefix("#async")
            && let Ok(index) = rest.parse::<usize>()
        {
            return Some(format!("${}", self.sync_count + index));
        }
        None
    }
}

impl<'a> VisitMut<'a> for ParamRenamer<'a, '_> {
    fn visit_identifier_reference(&mut self, it: &mut IdentifierReference<'a>) {
        let Some(resolved) = self.resolved(it.name.as_str()) else {
            return;
        };
        let name: &'a str = self.allocator.alloc_str(&resolved);
        it.name = name.into();
    }
}

pub(crate) fn push_blocker_slot(slots: &mut Vec<(String, usize)>, slot: (String, usize)) {
    if slots.contains(&slot) {
        return;
    }
    slots.push(slot);
}

pub(crate) fn extend_blocker_slots(slots: &mut Vec<(String, usize)>, more: Vec<(String, usize)>) {
    for slot in more {
        push_blocker_slot(slots, slot);
    }
}
