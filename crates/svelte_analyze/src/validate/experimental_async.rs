use oxc_ast::{AstKind, AstType};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::js_walker::{JsFlow, JsNodeMask, JsVisitor};
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::types::data::AnalysisData;

pub(super) fn new_instance_validator<'a>(
    data: &'a AnalysisData<'_>,
    runes: bool,
    diags: &'a mut Vec<Diagnostic>,
) -> Option<SuspendingAwaitValidator<'a>> {
    new_validator(data, runes, diags, true)
}

pub(super) fn new_module_validator<'a>(
    data: &'a AnalysisData<'_>,
    diags: &'a mut Vec<Diagnostic>,
) -> Option<SuspendingAwaitValidator<'a>> {
    new_validator(data, true, diags, false)
}

fn new_validator<'a>(
    data: &'a AnalysisData<'_>,
    runes: bool,
    diags: &'a mut Vec<Diagnostic>,
    check_top_level: bool,
) -> Option<SuspendingAwaitValidator<'a>> {
    let legacy_mode = if !data.script.experimental_async {
        false
    } else if !runes {
        true
    } else {
        return None;
    };
    Some(SuspendingAwaitValidator {
        reactivity: &data.reactivity,
        diags,
        legacy_mode,
        function_depth: 0,
        expression_active: false,
        expression_active_stack: Vec::new(),
        check_top_level,
    })
}

pub(super) struct SuspendingAwaitValidator<'a> {
    reactivity: &'a ReactivitySemantics,
    diags: &'a mut Vec<Diagnostic>,
    legacy_mode: bool,
    function_depth: u32,
    expression_active: bool,
    expression_active_stack: Vec<bool>,
    check_top_level: bool,
}

impl SuspendingAwaitValidator<'_> {
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

const SUSPENDING_AWAIT_LEAVE_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::CallExpression,
]);

const SUSPENDING_AWAIT_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::CallExpression,
    AstType::AwaitExpression,
]);

impl<'a> JsVisitor<'a> for SuspendingAwaitValidator<'_> {
    fn enter_interests(&self) -> JsNodeMask {
        SUSPENDING_AWAIT_INTERESTS
    }

    fn leave_interests(&self) -> JsNodeMask {
        SUSPENDING_AWAIT_LEAVE_INTERESTS
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
                let kind = if self.legacy_mode {
                    DiagnosticKind::LegacyAwaitInvalid
                } else {
                    DiagnosticKind::ExperimentalAsync
                };
                self.diags.push(Diagnostic::error(
                    kind,
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
