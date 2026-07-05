use oxc_ast::ast::{CallExpression, Expression, IdentifierReference, Program, VariableDeclarator};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{walk_call_expression, walk_variable_declarator};
use oxc_span::{GetSpan, Span as OxcSpan};
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::symbol::SymbolId;
use rustc_hash::FxHashSet;
use svelte_component_semantics::SymbolOwner;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::reactivity_semantics::detect_rune_from_call;
use crate::{AnalysisData, BindingSemantics};

pub(super) fn validate_global_references(
    data: &AnalysisData<'_>,
    legacy_explicit: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if legacy_explicit {
        return;
    }

    let mut global_store_names: FxHashSet<&str> = FxHashSet::default();
    for (store_symbol, store) in data.reactivity.iter_store_bindings() {
        if is_undeclared_global_store(data, store.base_symbol) {
            global_store_names.insert(data.scoping.symbol_name(store_symbol));
        }
    }
    if global_store_names.is_empty() {
        return;
    }

    for (name, refs) in data.scoping.root_unresolved_references() {
        let name = name.as_str();
        if !global_store_names.contains(name) {
            continue;
        }
        if !name.as_bytes().get(1).is_some_and(u8::is_ascii_lowercase) {
            continue;
        }
        push_global_reference_invalid(data, name, refs, diags);
    }
}

fn is_undeclared_global_store(data: &AnalysisData<'_>, base_symbol: SymbolId) -> bool {
    if data.scoping.symbol_owner(base_symbol) != SymbolOwner::Synthetic {
        return false;
    }
    data.reactivity
        .binding_semantics(base_symbol)
        .is_non_reactive()
}

fn push_global_reference_invalid(
    data: &AnalysisData<'_>,
    name: &str,
    refs: &[ReferenceId],
    diags: &mut Vec<Diagnostic>,
) {
    let Some(span) = earliest_reference_span(data, refs) else {
        return;
    };
    diags.push(Diagnostic::error(
        DiagnosticKind::GlobalReferenceInvalid {
            name: name.to_string(),
        },
        Span {
            start: span.start,
            end: span.end,
        },
    ));
}

fn earliest_reference_span(data: &AnalysisData<'_>, refs: &[ReferenceId]) -> Option<OxcSpan> {
    refs.iter()
        .filter_map(|&ref_id| {
            let node_id = data.scoping.get_reference(ref_id).node_id();
            data.scoping.js_kind(node_id).map(|kind| kind.span())
        })
        .min_by_key(|span| span.start)
}

pub(super) fn validate(
    data: &AnalysisData<'_>,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    for &ref_id in data.scoping.scoped_store_subscription_refs() {
        let Some(span) = earliest_reference_span(data, &[ref_id]) else {
            continue;
        };
        diags.push(Diagnostic::error(
            DiagnosticKind::StoreInvalidScopedSubscription,
            Span {
                start: span.start,
                end: span.end,
            },
        ));
    }

    let mut v = StoreValidator { diags, data };
    v.visit_program(program);
}

struct StoreValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    data: &'a AnalysisData<'a>,
}

impl StoreValidator<'_> {
    fn span(&self, oxc_span: OxcSpan) -> Span {
        Span {
            start: oxc_span.start,
            end: oxc_span.end,
        }
    }
}

pub(super) fn validate_module(
    data: &AnalysisData<'_>,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = ModuleStoreValidator { diags, data };
    v.visit_program(program);
}

pub(super) fn validate_standalone_module(
    data: &AnalysisData<'_>,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = StandaloneModuleStoreValidator {
        diags,
        data,
        reported_bindings: FxHashSet::default(),
    };
    v.visit_program(program);
}

struct ModuleStoreValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    data: &'a AnalysisData<'a>,
}

struct StandaloneModuleStoreValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    data: &'a AnalysisData<'a>,
    reported_bindings: FxHashSet<SymbolId>,
}

impl ModuleStoreValidator<'_> {
    fn span(&self, oxc_span: OxcSpan) -> Span {
        Span {
            start: oxc_span.start,
            end: oxc_span.end,
        }
    }
}

impl StandaloneModuleStoreValidator<'_> {
    fn span(&self, oxc_span: OxcSpan) -> Span {
        Span {
            start: oxc_span.start,
            end: oxc_span.end,
        }
    }
}

impl<'ast> Visit<'ast> for ModuleStoreValidator<'_> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'ast>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        if self
            .data
            .reactivity
            .reference_semantics(ref_id)
            .is_store_subscription()
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

        if svelte_ast::is_rune_name(name) {
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

impl StoreValidator<'_> {
    fn check_store_rune_conflict(&mut self, call: &CallExpression<'_>) {
        let Expression::Identifier(callee) = call.callee.get_inner_expression() else {
            return;
        };
        if !self.data.reactivity.uses_runes() {
            return;
        }
        let name = callee.name.as_str();
        if !svelte_ast::is_rune_name(name) {
            return;
        }
        let root = self.data.scoping.root_scope_id();
        let base = &name[1..];
        let Some(base_sym) = self.data.scoping.find_binding(root, base) else {
            return;
        };
        if declared_as_rune_or_prop(self.data, base_sym) {
            return;
        }
        self.diags.push(Diagnostic::warning(
            DiagnosticKind::StoreRuneConflict {
                name: base.to_string(),
            },
            self.span(callee.span()),
        ));
    }

    fn check_legacy_rune_invalid_usage(&mut self, declarator: &VariableDeclarator<'_>) {
        if self.data.reactivity.uses_runes() {
            return;
        }
        let Some(Expression::CallExpression(call)) = declarator
            .init
            .as_ref()
            .map(|init| init.get_inner_expression())
        else {
            return;
        };
        let Some(rune) = detect_rune_from_call(call) else {
            return;
        };
        if !rune.is_bare_value_rune() {
            return;
        }
        let Expression::Identifier(callee) = &call.callee else {
            return;
        };
        let is_store = callee.reference_id.get().is_some_and(|ref_id| {
            self.data
                .reactivity
                .reference_semantics(ref_id)
                .is_store_subscription()
        });
        if is_store {
            return;
        }
        self.diags.push(Diagnostic::error(
            DiagnosticKind::RuneInvalidUsage {
                rune: callee.name.to_string(),
            },
            self.span(call.span),
        ));
    }
}

impl<'ast> Visit<'ast> for StoreValidator<'_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'ast>) {
        self.check_store_rune_conflict(call);
        walk_call_expression(self, call);
    }

    fn visit_variable_declarator(&mut self, declarator: &VariableDeclarator<'ast>) {
        self.check_legacy_rune_invalid_usage(declarator);
        walk_variable_declarator(self, declarator);
    }
}

fn declared_as_rune_or_prop(data: &AnalysisData, sym_id: SymbolId) -> bool {
    match data.reactivity.binding_semantics(sym_id) {
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_) => true,
        BindingSemantics::Store(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => false,
    }
}
