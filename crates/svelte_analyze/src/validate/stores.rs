//! Store subscription validation — scoped subscription, rune conflict.

use oxc_ast::ast::{CallExpression, Expression, IdentifierReference};
use oxc_ast_visit::walk::walk_call_expression;
use oxc_ast_visit::Visit;
use oxc_span::GetSpan;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::utils::script_info::is_rune_name;
use crate::AnalysisData;

pub(super) fn validate(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = StoreValidator {
        diags,
        offset,
        data,
    };
    v.visit_program(program);
}

struct StoreValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    offset: u32,
    data: &'a AnalysisData,
}

impl StoreValidator<'_> {
    fn span(&self, oxc_span: oxc_span::Span) -> Span {
        Span {
            start: oxc_span.start + self.offset,
            end: oxc_span.end + self.offset,
        }
    }

    /// Check if `$X` where `X` is declared in a nested (non-root) scope.
    fn check_scoped_subscription(&mut self, ident: &IdentifierReference<'_>) {
        let name = ident.name.as_str();
        if !name.starts_with('$') || name.len() <= 1 || name.starts_with("$$") {
            return;
        }
        let base = &name[1..];

        // Skip rune names — those are handled by rune validation
        if is_rune_name(name) {
            return;
        }

        let root = self.data.scoping.root_scope_id();

        // If already resolved in root scope, it's a valid store subscription
        if self.data.scoping.find_binding(root, base).is_some() {
            return;
        }

        // Check if base name exists in any non-root scope
        if self.data.scoping.find_binding_in_any_scope(base).is_some() {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::StoreInvalidScopedSubscription,
                self.span(ident.span()),
            ));
        }
    }
}

pub(super) fn validate_module(
    data: &AnalysisData,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = ModuleStoreValidator {
        diags,
        offset,
        data,
    };
    v.visit_program(program);
}

struct ModuleStoreValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    offset: u32,
    data: &'a AnalysisData,
}

impl ModuleStoreValidator<'_> {
    fn span(&self, oxc_span: oxc_span::Span) -> Span {
        Span {
            start: oxc_span.start + self.offset,
            end: oxc_span.end + self.offset,
        }
    }
}

impl<'ast> Visit<'ast> for ModuleStoreValidator<'_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'ast>) {
        let name = ident.name.as_str();
        if !name.starts_with('$') || name.len() <= 1 || name.starts_with("$$") {
            return;
        }
        if is_rune_name(name) {
            return;
        }
        if self.data.scoping.store_base_name(name).is_some() {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::StoreInvalidSubscription,
                self.span(ident.span()),
            ));
        }
    }
}

impl<'ast> Visit<'ast> for StoreValidator<'_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'ast>) {
        self.check_scoped_subscription(ident);
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'ast>) {
        // store_rune_conflict: $X(...) where is_rune_name("$X") and X is a local binding
        if let Expression::Identifier(callee) = &call.callee {
            let name = callee.name.as_str();
            if is_rune_name(name) && name.starts_with('$') && name.len() > 1 {
                let base = &name[1..];
                let root = self.data.scoping.root_scope_id();
                if self.data.scoping.find_binding(root, base).is_some() {
                    self.diags.push(Diagnostic::warning(
                        DiagnosticKind::StoreRuneConflict {
                            name: base.to_string(),
                        },
                        self.span(callee.span()),
                    ));
                }
            }
        }

        walk_call_expression(self, call);
    }
}
