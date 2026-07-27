use oxc_ast::ast::{BindingPattern, ClassType, FunctionType, ImportDeclarationSpecifier};
use oxc_ast::{AstKind, AstType};
use oxc_span::Span as OxcSpan;
use svelte_ast::STORE_SUBSCRIPTION_SIGIL;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::js_walker::{JsFlow, JsNodeMask, JsVisitor};

pub(super) const MODULE_FUNCTION_DEPTH: u32 = 0;
pub(super) const INSTANCE_FUNCTION_DEPTH: u32 = 1;

const REPORTED_FUNCTION_DEPTH: u32 = 1;

#[derive(Clone, Copy)]
enum DeclarationReach {
    UpToReportedDepth,
    AnyDepthInRunes,
}

pub(super) fn new_validator(
    runes: bool,
    base_function_depth: u32,
    diags: &mut Vec<Diagnostic>,
) -> DollarNameValidator<'_> {
    DollarNameValidator {
        diags,
        runes,
        base_function_depth,
        function_depth: base_function_depth,
    }
}

pub(super) struct DollarNameValidator<'b> {
    diags: &'b mut Vec<Diagnostic>,
    runes: bool,
    base_function_depth: u32,
    function_depth: u32,
}

impl DollarNameValidator<'_> {
    fn reports(&self, reach: DeclarationReach) -> bool {
        if self.function_depth <= REPORTED_FUNCTION_DEPTH {
            return true;
        }
        match reach {
            DeclarationReach::UpToReportedDepth => false,
            DeclarationReach::AnyDepthInRunes => self.runes,
        }
    }

    fn check(&mut self, name: &str, span: OxcSpan, reach: DeclarationReach) {
        if !self.reports(reach) {
            return;
        }
        if name == STORE_SUBSCRIPTION_SIGIL {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::DollarBindingInvalid,
                Span::new(span.start, span.end),
            ));
            return;
        }
        if name.starts_with(STORE_SUBSCRIPTION_SIGIL) {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::DollarPrefixInvalid,
                Span::new(span.start, span.end),
            ));
        }
    }

    fn check_pattern(&mut self, pattern: &BindingPattern<'_>, reach: DeclarationReach) {
        match pattern {
            BindingPattern::BindingIdentifier(id) => {
                self.check(id.name.as_str(), id.span, reach);
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.check_pattern(&prop.value, reach);
                }
                if let Some(rest) = &obj.rest {
                    self.check_pattern(&rest.argument, reach);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for element in arr.elements.iter().flatten() {
                    self.check_pattern(element, reach);
                }
                if let Some(rest) = &arr.rest {
                    self.check_pattern(&rest.argument, reach);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                self.check_pattern(&assign.left, reach);
            }
        }
    }
}

const DOLLAR_NAME_LEAVE_INTERESTS: JsNodeMask =
    JsNodeMask::new(&[AstType::Function, AstType::ArrowFunctionExpression]);

const DOLLAR_NAME_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::VariableDeclarator,
    AstType::CatchParameter,
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::IdentifierReference,
    AstType::Class,
    AstType::ImportDeclaration,
]);

impl<'a> JsVisitor<'a> for DollarNameValidator<'_> {
    fn enter_interests(&self) -> JsNodeMask {
        DOLLAR_NAME_INTERESTS
    }

    fn leave_interests(&self) -> JsNodeMask {
        DOLLAR_NAME_LEAVE_INTERESTS
    }

    fn enter_js_node(&mut self, kind: AstKind<'a>) -> JsFlow {
        match kind {
            AstKind::VariableDeclarator(it) => {
                self.check_pattern(&it.id, DeclarationReach::AnyDepthInRunes);
            }
            AstKind::CatchParameter(it) => {
                self.check_pattern(&it.pattern, DeclarationReach::UpToReportedDepth);
            }
            AstKind::Function(it) => {
                if let Some(id) = &it.id {
                    let reach = match it.r#type {
                        FunctionType::FunctionDeclaration => DeclarationReach::AnyDepthInRunes,
                        FunctionType::FunctionExpression
                        | FunctionType::TSDeclareFunction
                        | FunctionType::TSEmptyBodyFunctionExpression => {
                            DeclarationReach::UpToReportedDepth
                        }
                    };
                    self.check(id.name.as_str(), id.span, reach);
                }
                self.function_depth += 1;
            }
            AstKind::ArrowFunctionExpression(it) => {
                if !it.expression {
                    self.function_depth += 1;
                }
            }
            AstKind::IdentifierReference(it) => {
                if self.function_depth == self.base_function_depth && it.name == "arguments" {
                    self.diags.push(Diagnostic::error(
                        DiagnosticKind::InvalidArgumentsUsage,
                        Span::new(it.span.start, it.span.end),
                    ));
                }
            }
            AstKind::Class(it) => {
                if let Some(id) = &it.id {
                    let reach = match it.r#type {
                        ClassType::ClassDeclaration => DeclarationReach::AnyDepthInRunes,
                        ClassType::ClassExpression => DeclarationReach::UpToReportedDepth,
                    };
                    self.check(id.name.as_str(), id.span, reach);
                }
            }
            AstKind::ImportDeclaration(it) => {
                if let Some(specifiers) = &it.specifiers {
                    for spec in specifiers {
                        let (name, span) = match spec {
                            ImportDeclarationSpecifier::ImportSpecifier(s) => {
                                (s.local.name.as_str(), s.local.span)
                            }
                            ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                                (s.local.name.as_str(), s.local.span)
                            }
                            ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                                (s.local.name.as_str(), s.local.span)
                            }
                        };
                        self.check(name, span, DeclarationReach::UpToReportedDepth);
                    }
                }
            }
            _ => {}
        }
        JsFlow::Continue
    }

    fn leave_js_node(&mut self, kind: AstKind<'a>) {
        match kind {
            AstKind::Function(_) => self.function_depth -= 1,
            AstKind::ArrowFunctionExpression(it) if !it.expression => self.function_depth -= 1,
            _ => {}
        }
    }
}
