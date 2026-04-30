use oxc_ast::ast::{CallExpression, Expression, IdentifierReference};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_call_expression;
use oxc_span::GetSpan;
use rustc_hash::FxHashSet;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::utils::script_info::is_rune_name;
use crate::{AnalysisData, BindingSemantics};

pub(super) fn validate(
    data: &AnalysisData<'_>,
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
    data: &'a AnalysisData<'a>,
}

impl StoreValidator<'_> {
    fn span(&self, oxc_span: oxc_span::Span) -> Span {
        Span {
            start: oxc_span.start + self.offset,
            end: oxc_span.end + self.offset,
        }
    }

    fn check_scoped_subscription(&mut self, ident: &IdentifierReference<'_>) {
        let name = ident.name.as_str();
        if !name.starts_with('$') || name.len() <= 1 || name.starts_with("$$") {
            return;
        }
        let base = &name[1..];

        if is_rune_name(name) {
            return;
        }

        let root = self.data.scoping.root_scope_id();

        if self.data.scoping.find_binding(root, base).is_some() {
            return;
        }

        if self.data.scoping.find_binding_in_any_scope(base).is_some() {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::StoreInvalidScopedSubscription,
                self.span(ident.span()),
            ));
        }
    }
}

pub(super) fn validate_module(
    data: &AnalysisData<'_>,
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

pub(super) fn validate_standalone_module(
    data: &AnalysisData<'_>,
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = StandaloneModuleStoreValidator {
        diags,
        offset,
        data,
        reported_bindings: FxHashSet::default(),
    };
    v.visit_program(program);
}

struct ModuleStoreValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    offset: u32,
    data: &'a AnalysisData<'a>,
}

struct StandaloneModuleStoreValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    offset: u32,
    data: &'a AnalysisData<'a>,
    reported_bindings: FxHashSet<oxc_syntax::symbol::SymbolId>,
}

impl ModuleStoreValidator<'_> {
    fn span(&self, oxc_span: oxc_span::Span) -> Span {
        Span {
            start: oxc_span.start + self.offset,
            end: oxc_span.end + self.offset,
        }
    }
}

impl StandaloneModuleStoreValidator<'_> {
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
        let base = &name[1..];
        let root = self.data.scoping.root_scope_id();
        if self
            .data
            .scoping
            .find_binding(root, base)
            .is_some_and(|sym| {
                matches!(self.data.binding_semantics(sym), BindingSemantics::Store(_),)
            })
        {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::StoreInvalidSubscription,
                self.span(ident.span()),
            ));
        }
    }
}

impl<'ast> Visit<'ast> for StandaloneModuleStoreValidator<'_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'ast>) {
        let name = ident.name.as_str();
        if !name.starts_with('$') || name.len() <= 1 || name.starts_with("$$") {
            return;
        }
        if is_rune_name(name) {
            return;
        }

        let root = self.data.scoping.root_scope_id();
        let Some(sym_id) = self.data.scoping.find_binding(root, &name[1..]) else {
            return;
        };

        if !self.reported_bindings.insert(sym_id) {
            return;
        }

        self.diags.push(Diagnostic::error(
            DiagnosticKind::StoreInvalidSubscriptionModule,
            self.span(ident.span()),
        ));
    }
}

impl<'ast> Visit<'ast> for StoreValidator<'_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'ast>) {
        self.check_scoped_subscription(ident);
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'ast>) {
        if let Expression::Identifier(callee) = &call.callee {
            let name = callee.name.as_str();
            if is_rune_name(name) && name.starts_with('$') && name.len() > 1 {
                let base = &name[1..];
                let root = self.data.scoping.root_scope_id();
                if self
                    .data
                    .scoping
                    .find_binding(root, base)
                    .is_some_and(|sym_id| !is_rune_or_prop_origin(self.data, sym_id))
                {
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

fn is_rune_or_prop_origin(data: &AnalysisData, sym_id: oxc_syntax::symbol::SymbolId) -> bool {
    matches!(
        data.reactivity.binding_semantics(sym_id),
        BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Prop(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_),
    )
}
