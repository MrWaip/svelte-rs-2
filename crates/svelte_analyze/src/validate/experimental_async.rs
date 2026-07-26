use oxc_ast::{AstKind, AstType};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::js_walker::{JsFlow, JsNodeMask, JsVisitor};
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::types::data::AnalysisData;

pub(super) fn new_instance_validator<'a>(
    data: &'a AnalysisData<'_>,
    diags: &'a mut Vec<Diagnostic>,
) -> Option<ExperimentalAsyncValidator<'a>> {
    new_validator(data, diags, true)
}

pub(super) fn new_module_validator<'a>(
    data: &'a AnalysisData<'_>,
    diags: &'a mut Vec<Diagnostic>,
) -> Option<ExperimentalAsyncValidator<'a>> {
    new_validator(data, diags, false)
}

fn new_validator<'a>(
    data: &'a AnalysisData<'_>,
    diags: &'a mut Vec<Diagnostic>,
    check_top_level: bool,
) -> Option<ExperimentalAsyncValidator<'a>> {
    if data.script.experimental_async {
        return None;
    }
    Some(ExperimentalAsyncValidator {
        reactivity: &data.reactivity,
        diags,
        function_depth: 0,
        expression_active: false,
        expression_active_stack: Vec::new(),
        check_top_level,
    })
}

pub(super) struct ExperimentalAsyncValidator<'a> {
    reactivity: &'a ReactivitySemantics,
    diags: &'a mut Vec<Diagnostic>,
    function_depth: u32,
    expression_active: bool,
    expression_active_stack: Vec<bool>,
    check_top_level: bool,
}

impl ExperimentalAsyncValidator<'_> {
    fn enter_isolated(&mut self, active: bool) {
        self.expression_active_stack.push(self.expression_active);
        self.expression_active = active;
        self.function_depth += 1;
    }

    fn leave_isolated(&mut self) {
        self.function_depth -= 1;
        if let Some(prev) = self.expression_active_stack.pop() {
            self.expression_active = prev;
        }
    }

    fn is_derived_call(&self, kind: AstKind<'_>) -> bool {
        matches!(kind, AstKind::CallExpression(call)
            if self
                .reactivity
                .declarator_semantics(call.node_id())
                .is_rune_derived())
    }
}

const EXPERIMENTAL_ASYNC_LEAVE_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::CallExpression,
]);

const EXPERIMENTAL_ASYNC_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::CallExpression,
    AstType::AwaitExpression,
]);

impl<'a> JsVisitor<'a> for ExperimentalAsyncValidator<'_> {
    fn enter_interests(&self) -> JsNodeMask {
        EXPERIMENTAL_ASYNC_INTERESTS
    }

    fn leave_interests(&self) -> JsNodeMask {
        EXPERIMENTAL_ASYNC_LEAVE_INTERESTS
    }

    fn enter_js_node(&mut self, kind: AstKind<'a>) -> JsFlow {
        match kind {
            AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => {
                self.enter_isolated(false)
            }
            AstKind::CallExpression(_) if self.is_derived_call(kind) => self.enter_isolated(true),
            AstKind::AwaitExpression(expr)
                if (self.check_top_level && self.function_depth == 0) || self.expression_active =>
            {
                self.diags.push(Diagnostic::error(
                    DiagnosticKind::ExperimentalAsync,
                    Span::new(expr.span.start, expr.span.end),
                ));
            }
            _ => {}
        }
        JsFlow::Continue
    }

    fn leave_js_node(&mut self, kind: AstKind<'a>) {
        match kind {
            AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => self.leave_isolated(),
            AstKind::CallExpression(_) if self.is_derived_call(kind) => self.leave_isolated(),
            _ => {}
        }
    }
}
