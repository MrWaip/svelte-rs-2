use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use std::collections::HashMap;
use svelte_analyze::{AnalysisData, ExpressionBlocker, IdentGen, JsAst};
use svelte_ast::{Component, ExprRef, NodeId};
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::SymbolId;
use svelte_span::LineIndex;

use crate::error::{CodegenError, Result};
use crate::renderer::TemplateItem;

pub(crate) struct ServerCodegen<'a> {
    pub b: Builder<'a>,
    pub component: &'a Component,
    pub analysis: &'a AnalysisData<'a>,
    pub js_arena: &'a mut JsAst<'a>,
    pub ident_gen: &'a mut IdentGen,
    pub line_index: &'a LineIndex,
    pub dev: bool,
    pub hmr: bool,
    pub experimental_async: bool,
    pub filename: &'a str,
    pub items: Vec<TemplateItem<'a>>,
    pub hoisted: Vec<Statement<'a>>,
    pub render_hoists: Vec<Statement<'a>>,
    pub each_index_names: HashMap<NodeId, String>,
    pub each_array_names: HashMap<NodeId, String>,
    pub bind_pair_names: HashMap<NodeId, (String, String)>,
    pub svelte_element_tag_refs: HashMap<NodeId, String>,
    pub save_block_awaits: bool,
    pub promise_hoists: Option<Vec<Expression<'a>>>,
    pub emitted_blockers: Option<Vec<(u32, u32)>>,
    pub pending_group_declarations: Vec<String>,
    pub suppress_async_hoist: bool,
    pub const_tag_blockers: HashMap<SymbolId, (String, u32)>,
    pub declaration_blocker_slots: HashMap<NodeId, (String, u32)>,
    pub declaration_group_idents: HashMap<svelte_ast::FragmentId, String>,
    pub injected_css_text: Option<&'a str>,
    pub emitted_snippet_names: Vec<&'a str>,
}

pub(crate) enum AsyncInterpolation<'a> {
    Awaited { blockers: Vec<Expression<'a>> },
    Deferred { blockers: Vec<Expression<'a>> },
}

impl<'a> ServerCodegen<'a> {
    pub(crate) fn new(
        ctx: svelte_types::CompileContext<'a, 'a>,
        options: &svelte_types::CodegenOptions,
        injected_css_text: Option<&str>,
    ) -> Self {
        let b = Builder::new(ctx.alloc);
        let filename: &'a str = b.alloc_str(&options.filename);
        let injected_css_text: Option<&'a str> = injected_css_text.map(|t| b.alloc_str(t) as &str);
        Self {
            b,
            component: ctx.component,
            analysis: ctx.analysis,
            js_arena: ctx.js_arena,
            ident_gen: ctx.ident_gen,
            line_index: ctx.line_index,
            dev: options.dev,
            hmr: options.hmr,
            experimental_async: options.experimental_async,
            filename,
            items: Vec::new(),
            hoisted: Vec::new(),
            render_hoists: Vec::new(),
            each_index_names: HashMap::new(),
            each_array_names: HashMap::new(),
            bind_pair_names: HashMap::new(),
            svelte_element_tag_refs: HashMap::new(),
            save_block_awaits: false,
            promise_hoists: None,
            emitted_blockers: None,
            pending_group_declarations: Vec::new(),
            suppress_async_hoist: false,
            const_tag_blockers: HashMap::new(),
            declaration_blocker_slots: HashMap::new(),
            declaration_group_idents: HashMap::new(),
            injected_css_text,
            emitted_snippet_names: Vec::new(),
        }
    }

    pub(crate) fn gen_ident(&mut self, prefix: &str) -> String {
        self.ident_gen.generate(prefix)
    }

    pub(crate) fn take_expression(
        &mut self,
        node_id: NodeId,
        expr_ref: &ExprRef,
    ) -> Result<Expression<'a>> {
        self.take_expr_by_oxc_id(node_id, expr_ref.id())
    }

    pub(crate) fn take_expr_by_oxc_id(
        &mut self,
        node_id: NodeId,
        oxc_id: OxcNodeId,
    ) -> Result<Expression<'a>> {
        self.record_emitted_blockers(node_id);
        self.js_arena
            .take_expr(oxc_id)
            .ok_or(CodegenError::MissingExpression(node_id))
    }

    pub(crate) fn expression_is_volatile(&self, node_id: NodeId) -> bool {
        self.analysis
            .expression_data(node_id)
            .is_some_and(|data| data.volatility.is_volatile())
    }

    pub(crate) fn maybe_hoist_async_expr(
        &mut self,
        node_id: NodeId,
        expr: Expression<'a>,
    ) -> Expression<'a> {
        if self.suppress_async_hoist || self.promise_hoists.is_none() {
            return expr;
        }
        let is_async = self
            .analysis
            .expression_data(node_id)
            .is_some_and(|data| data.volatility.is_asynchronous());
        if !is_async {
            return expr;
        }
        self.push_promise_hoist(expr)
    }

    fn record_emitted_blockers(&mut self, node_id: NodeId) {
        if self.emitted_blockers.is_none() {
            return;
        }
        let Some(data) = self.analysis.expression_data(node_id) else {
            return;
        };
        let blocker_data = self.analysis.blocker_data();
        let Some(mut recorded) = self.emitted_blockers.take() else {
            return;
        };
        for sym in &data.blocker_references {
            let Some(slot) = blocker_data.symbol_blocker(*sym) else {
                continue;
            };
            if recorded.iter().any(|(member, _)| *member == slot.member) {
                continue;
            }
            recorded.push((slot.member, slot.entry));
        }
        self.emitted_blockers = Some(recorded);
    }

    fn push_promise_hoist(&mut self, expr: Expression<'a>) -> Expression<'a> {
        let Some(hoists) = self.promise_hoists.as_mut() else {
            return expr;
        };
        let name = format!("$${}", hoists.len());
        hoists.push(expr);
        self.b.rid_expr(&name)
    }

    pub(crate) fn with_promise_hoisting<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> T,
    ) -> (T, Vec<Statement<'a>>) {
        let prev_hoists = self.promise_hoists.replace(Vec::new());
        let out = f(self);
        let hoisted = self.promise_hoists.take().unwrap_or_default();
        self.promise_hoists = prev_hoists;
        (out, self.apply_promise_hoists(hoisted))
    }

    pub(crate) fn without_emitted_blockers<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        let previous = self.emitted_blockers.take();
        let out = f(self);
        self.emitted_blockers = previous;
        out
    }

    pub(crate) fn with_emitted_blockers<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> T,
    ) -> (T, Vec<u32>) {
        let previous = self.emitted_blockers.replace(Vec::new());
        let out = f(self);
        let recorded = self.emitted_blockers.take().unwrap_or_default();
        self.emitted_blockers = previous;
        (out, recorded.into_iter().map(|(_, entry)| entry).collect())
    }

    fn apply_promise_hoists(&self, hoisted: Vec<Expression<'a>>) -> Vec<Statement<'a>> {
        if hoisted.is_empty() {
            return Vec::new();
        }
        if hoisted.len() == 1 {
            let mut hoisted = hoisted;
            return vec![self.b.const_stmt("$$0", hoisted.remove(0))];
        }
        let names: Vec<String> = (0..hoisted.len()).map(|i| format!("$${i}")).collect();
        let name_refs: Vec<&str> = names.iter().map(String::as_str).collect();
        let mut members: Vec<Expression<'a>> = Vec::with_capacity(hoisted.len());
        for expr in hoisted {
            let thunk = self.b.async_arrow_expr_body(expr);
            members.push(self.b.call_expr_callee(thunk, []));
        }
        let all = self
            .b
            .call_expr("Promise.all", [Arg::Expr(self.b.array_expr(members))]);
        let save = self.b.call_expr("$.save", [Arg::Expr(all)]);
        let settled = self.b.call_expr_callee(self.b.await_expr(save), []);
        vec![self.b.const_array_destruct_stmt(&name_refs, settled)]
    }

    pub(crate) fn blocker_member(&self, idx: u32) -> Expression<'a> {
        self.b
            .computed_member_expr(self.b.rid_expr("$$promises"), self.b.num_expr(idx as f64))
    }

    pub(crate) fn async_interpolation(&self, node_id: NodeId) -> Option<AsyncInterpolation<'a>> {
        let (awaited, instance) = self
            .analysis
            .expression_data(node_id)
            .map(|data| (data.volatility.is_asynchronous(), data.blockers.to_vec()))
            .unwrap_or((false, Vec::new()));
        let mut blockers: Vec<Expression<'a>> =
            instance.iter().map(|&i| self.blocker_member(i)).collect();
        blockers.extend(self.const_tag_blocker_exprs(node_id));
        if awaited {
            Some(AsyncInterpolation::Awaited { blockers })
        } else if !blockers.is_empty() {
            Some(AsyncInterpolation::Deferred { blockers })
        } else {
            None
        }
    }

    pub(crate) fn blocker_exprs(&self, blockers: &[ExpressionBlocker]) -> Vec<Expression<'a>> {
        let mut result = Vec::new();
        for blocker in blockers {
            match blocker {
                ExpressionBlocker::Script { entry } => result.push(self.blocker_member(*entry)),
                ExpressionBlocker::FragmentDeclaration { node } => {
                    let Some((name, idx)) = self.declaration_blocker_slots.get(node) else {
                        continue;
                    };
                    result.push(
                        self.b.computed_member_expr(
                            self.b.rid_expr(name),
                            self.b.num_expr(*idx as f64),
                        ),
                    );
                }
            }
        }
        result
    }

    pub(crate) fn const_tag_blocker_exprs(&self, node_id: NodeId) -> Vec<Expression<'a>> {
        if self.const_tag_blockers.is_empty() {
            return Vec::new();
        }
        let Some(data) = self.analysis.expression_data(node_id) else {
            return Vec::new();
        };
        let mut result = Vec::new();
        for sym in &data.references {
            if let Some((name, idx)) = self.const_tag_blockers.get(sym) {
                let member = self
                    .b
                    .computed_member_expr(self.b.rid_expr(name), self.b.num_expr(*idx as f64));
                result.push(member);
            }
        }
        result
    }

    pub(crate) fn hoist_awaited_arg(&mut self, expr: Expression<'a>) -> Expression<'a> {
        let is_awaited_call = matches!(
            &expr,
            Expression::CallExpression(call)
                if call.arguments.is_empty()
                    && matches!(call.callee.get_inner_expression(), Expression::AwaitExpression(_))
        );
        if !is_awaited_call {
            return expr;
        }
        self.push_promise_hoist(expr)
    }

    pub(crate) fn save_block_await(&self, expr: Expression<'a>) -> Expression<'a> {
        let Expression::AwaitExpression(await_expr) = expr else {
            return expr;
        };
        let inner = await_expr.unbox().argument;
        let save = self.b.call_expr("$.save", [Arg::Expr(inner)]);
        self.b.call_expr_callee(self.b.await_expr(save), [])
    }

    pub(crate) fn wrap_async_block_exprs(
        &self,
        statements: Vec<Statement<'a>>,
        blockers: Vec<Expression<'a>>,
        is_async: bool,
    ) -> Statement<'a> {
        let arrow =
            self.b
                .arrow_block_expr_async(self.b.params(["$$renderer"]), statements, is_async);
        if blockers.is_empty() {
            return self
                .b
                .call_stmt("$$renderer.child_block", [Arg::Expr(arrow)]);
        }
        let promises = self.b.array_expr(blockers);
        self.b.call_stmt(
            "$$renderer.async_block",
            [Arg::Expr(promises), Arg::Expr(arrow)],
        )
    }

    pub(crate) fn wrap_async_block_flagged(
        &self,
        statements: Vec<Statement<'a>>,
        blockers: &[u32],
        is_async: bool,
    ) -> Statement<'a> {
        let arrow =
            self.b
                .arrow_block_expr_async(self.b.params(["$$renderer"]), statements, is_async);
        self.wrap_arrow(
            arrow,
            blockers,
            "$$renderer.child_block",
            "$$renderer.async_block",
        )
    }

    pub(crate) fn wrap_arrow(
        &self,
        arrow: Expression<'a>,
        blockers: &[u32],
        child_call: &str,
        async_call: &str,
    ) -> Statement<'a> {
        if blockers.is_empty() {
            self.b.call_stmt(child_call, [Arg::Expr(arrow)])
        } else {
            let promises = self.b.promises_array(blockers);
            self.b
                .call_stmt(async_call, [Arg::Expr(promises), Arg::Expr(arrow)])
        }
    }
}
