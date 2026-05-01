use oxc_ast::ast::{ArrowFunctionExpression, AwaitExpression, CallExpression, Function, Program};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_call_expression, walk_function, walk_program,
};
use oxc_semantic::ScopeFlags;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::types::data::AnalysisData;
use crate::types::script::RuneKind;
use crate::utils::script_info::detect_rune_from_call;

pub(super) fn validate_instance_program(
    data: &AnalysisData<'_>,
    program: &Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    if data.script.experimental_async {
        return;
    }
    let mut visitor = ExperimentalAsyncValidator {
        diags,
        offset,
        function_depth: 0,
        expression_active: false,
    };
    visitor.visit_program(program);
}

struct ExperimentalAsyncValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    offset: u32,
    function_depth: u32,
    expression_active: bool,
}

impl<'a> Visit<'a> for ExperimentalAsyncValidator<'_> {
    fn visit_program(&mut self, program: &Program<'a>) {
        walk_program(self, program);
    }

    fn visit_function(&mut self, function: &Function<'a>, flags: ScopeFlags) {
        let prev_expression_active = self.expression_active;
        self.expression_active = false;
        self.function_depth += 1;
        walk_function(self, function, flags);
        self.function_depth -= 1;
        self.expression_active = prev_expression_active;
    }

    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression<'a>) {
        let prev_expression_active = self.expression_active;
        self.expression_active = false;
        self.function_depth += 1;
        walk_arrow_function_expression(self, expr);
        self.function_depth -= 1;
        self.expression_active = prev_expression_active;
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if matches!(
            detect_rune_from_call(call),
            Some(RuneKind::Derived | RuneKind::DerivedBy)
        ) {
            let prev_expression_active = self.expression_active;
            self.expression_active = true;
            self.function_depth += 1;
            for arg in &call.arguments {
                if let Some(expr) = arg.as_expression() {
                    self.visit_expression(expr);
                }
            }
            self.function_depth -= 1;
            self.expression_active = prev_expression_active;
            return;
        }
        walk_call_expression(self, call);
    }

    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        if self.function_depth == 0 || self.expression_active {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::ExperimentalAsync,
                Span::new(expr.span.start + self.offset, expr.span.end + self.offset),
            ));
        }
        oxc_ast_visit::walk::walk_await_expression(self, expr);
    }
}
