use std::marker::PhantomData;

use oxc_ast::ast::SimpleAssignmentTarget;
use oxc_ast::{AstKind, AstType};
use oxc_syntax::reference::ReferenceId;
use oxc_syntax::symbol::{SymbolFlags, SymbolId};
use svelte_component_semantics::walk_assignment_target_idents;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::AnalysisData;
use crate::js_walker::{JsFlow, JsNodeMask, JsVisitor, JsWalk};
use crate::reactivity_semantics::data::{BindingSemantics, ContextualBindingSemantics};
use crate::types::data::JsAst;

pub(super) fn new_validator<'a, 'b>(
    data: &'b AnalysisData<'a>,
    diags: &'b mut Vec<Diagnostic>,
) -> ConstAssignValidator<'a, 'b> {
    ConstAssignValidator {
        data,
        diags,
        _phantom: PhantomData,
    }
}

pub(super) fn validate_template(
    data: &AnalysisData<'_>,
    parsed: &JsAst<'_>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = new_validator(data, diags);
    let mut visitors: [&mut dyn JsVisitor; 1] = [&mut v];
    let mut walk = JsWalk::new(&mut visitors);
    for expr in parsed.iter_exprs() {
        walk.walk_expression(expr);
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

pub(super) struct ConstAssignValidator<'a, 'b> {
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

const CONST_ASSIGN_INTERESTS: JsNodeMask =
    JsNodeMask::new(&[AstType::AssignmentExpression, AstType::UpdateExpression]);

impl<'a> JsVisitor<'a> for ConstAssignValidator<'a, '_> {
    fn enter_interests(&self) -> JsNodeMask {
        CONST_ASSIGN_INTERESTS
    }

    fn enter_js_node(&mut self, kind: AstKind<'a>) -> JsFlow {
        match kind {
            AstKind::AssignmentExpression(it) => {
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
            }
            AstKind::UpdateExpression(it) => {
                if let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &it.argument
                    && !id.name.starts_with('$')
                {
                    let symbol = self.resolve(id.reference_id.get());
                    self.check(symbol, Span::new(it.span.start, it.span.end));
                }
            }
            _ => {}
        }
        JsFlow::Continue
    }
}
