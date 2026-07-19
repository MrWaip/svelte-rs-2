use std::marker::PhantomData;

use oxc_ast::ast::{AssignmentExpression, Program, SimpleAssignmentTarget, UpdateExpression};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{walk_assignment_expression, walk_update_expression};
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::symbol::{SymbolFlags, SymbolId};
use svelte_component_semantics::walk_assignment_target_idents;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::AnalysisData;
use crate::reactivity_semantics::data::{BindingSemantics, ContextualBindingSemantics};
use crate::types::data::JsAst;

pub(super) fn validate(
    data: &AnalysisData<'_>,
    program: &Program<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = ConstAssignValidator {
        data,
        diags,
        _phantom: PhantomData,
    };
    v.visit_program(program);
}

pub(super) fn validate_template(
    data: &AnalysisData<'_>,
    parsed: &JsAst<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = ConstAssignValidator {
        data,
        diags,
        _phantom: PhantomData,
    };
    for expr in parsed.iter_exprs() {
        v.visit_expression(expr);
    }
}

pub(crate) fn constant_kind(data: &AnalysisData<'_>, symbol_id: SymbolId) -> Option<&'static str> {
    match data.binding_semantics(symbol_id) {
        BindingSemantics::Contextual(ContextualBindingSemantics::EachItem(_)) => return None,
        BindingSemantics::Contextual(
            ContextualBindingSemantics::AwaitValue | ContextualBindingSemantics::AwaitError,
        ) => return Some("constant"),
        _ => {}
    }
    if data.scoping.is_import(symbol_id) {
        return Some("import");
    }
    if data
        .scoping
        .semantics()
        .symbol_flags(symbol_id)
        .contains(SymbolFlags::ConstVariable)
    {
        return Some("constant");
    }
    None
}

struct ConstAssignValidator<'a, 'b> {
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,
    _phantom: PhantomData<&'a ()>,
}

impl ConstAssignValidator<'_, '_> {
    fn resolve(&self, reference_id: Option<ReferenceId>) -> Option<SymbolId> {
        reference_id
            .and_then(|r| self.data.scoping.try_get_reference(r))
            .and_then(|reference| reference.symbol_id())
    }

    fn check(&mut self, symbol_id: Option<SymbolId>, span: Span) {
        let Some(symbol_id) = symbol_id else {
            return;
        };
        if let Some(thing) = constant_kind(self.data, symbol_id) {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::ConstantAssignment {
                    thing: thing.to_string(),
                },
                span,
            ));
        }
    }
}

impl<'a> Visit<'a> for ConstAssignValidator<'a, '_> {
    fn visit_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {
        let span = Span::new(it.span.start, it.span.end);
        let mut targets: Vec<Option<SymbolId>> = Vec::new();
        walk_assignment_target_idents(&it.left, |id| {
            if !id.name.starts_with('$') {
                targets.push(self.resolve(id.reference_id.get()));
            }
        });
        for symbol in targets {
            self.check(symbol, span);
        }
        walk_assignment_expression(self, it);
    }

    fn visit_update_expression(&mut self, it: &UpdateExpression<'a>) {
        if let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &it.argument
            && !id.name.starts_with('$')
        {
            let symbol = self.resolve(id.reference_id.get());
            self.check(symbol, Span::new(it.span.start, it.span.end));
        }
        walk_update_expression(self, it);
    }
}
