use oxc_ast::ast::{ClassType, Expression, MethodDefinitionKind};
use oxc_ast::{AstKind, AstType};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::js_walker::{JsFlow, JsNodeMask, JsVisitor};
use crate::types::data::AnalysisData;

use super::class_state_fields::check_class;
use super::stores::{check_legacy_rune_invalid_usage, check_store_rune_conflict};
use super::typescript::namespace_has_non_type_node;

pub(super) fn new_instance_validator<'a>(
    data: &'a AnalysisData<'a>,
    runes: bool,
    diags: &'a mut Vec<Diagnostic>,
) -> SyntaxValidator<'a> {
    SyntaxValidator {
        diags,
        data,
        in_constructor: false,
        constructor_stack: Vec::new(),
        function_depth: 0,
        base_function_depth: 1,
        check_class_state: runes,
        check_legacy_placement: !runes,
        check_stores: true,
    }
}

pub(super) fn new_module_validator<'a>(
    data: &'a AnalysisData<'a>,
    check_class_state: bool,
    diags: &'a mut Vec<Diagnostic>,
) -> SyntaxValidator<'a> {
    SyntaxValidator {
        diags,
        data,
        in_constructor: false,
        constructor_stack: Vec::new(),
        function_depth: 0,
        base_function_depth: 0,
        check_class_state,
        check_legacy_placement: false,
        check_stores: false,
    }
}

pub(super) struct SyntaxValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    data: &'a AnalysisData<'a>,
    in_constructor: bool,
    constructor_stack: Vec<bool>,
    function_depth: u32,
    base_function_depth: u32,
    check_class_state: bool,
    check_legacy_placement: bool,
    check_stores: bool,
}

impl SyntaxValidator<'_> {
    fn push_ts(&mut self, feature: &str, span: oxc_span::Span) {
        self.diags.push(Diagnostic::error(
            DiagnosticKind::TypescriptInvalidFeature {
                feature: feature.to_string(),
            },
            Span::new(span.start, span.end),
        ));
    }
}

const SYNTAX_LEAVE_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::MethodDefinition,
    AstType::FormalParameter,
    AstType::Decorator,
    AstType::AccessorProperty,
    AstType::TSEnumDeclaration,
    AstType::TSModuleDeclaration,
]);

const SYNTAX_INTERESTS: JsNodeMask = JsNodeMask::new(&[
    AstType::Function,
    AstType::ArrowFunctionExpression,
    AstType::Class,
    AstType::CallExpression,
    AstType::VariableDeclarator,
    AstType::NewExpression,
    AstType::LabeledStatement,
    AstType::MethodDefinition,
    AstType::FormalParameter,
    AstType::Decorator,
    AstType::AccessorProperty,
    AstType::TSEnumDeclaration,
    AstType::TSModuleDeclaration,
]);

impl<'a> JsVisitor<'a> for SyntaxValidator<'_> {
    fn enter_interests(&self) -> JsNodeMask {
        SYNTAX_INTERESTS
    }

    fn leave_interests(&self) -> JsNodeMask {
        SYNTAX_LEAVE_INTERESTS
    }

    fn enter_js_node(&mut self, kind: AstKind<'a>) -> JsFlow {
        match kind {
            AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => {
                self.function_depth += 1;
            }
            AstKind::Class(class) => {
                if self.check_class_state {
                    check_class(class, &self.data.reactivity, self.diags);
                }
                if class.r#type == ClassType::ClassDeclaration && self.function_depth > 0 {
                    self.diags.push(Diagnostic::warning(
                        DiagnosticKind::PerfAvoidNestedClass,
                        Span::new(class.span.start, class.span.end),
                    ));
                }
            }
            AstKind::CallExpression(call) => {
                if self.check_stores {
                    check_store_rune_conflict(call, self.data, self.diags);
                }
            }
            AstKind::VariableDeclarator(declarator) => {
                if self.check_stores {
                    check_legacy_rune_invalid_usage(declarator, self.data, self.diags);
                }
            }
            AstKind::NewExpression(expr) => {
                if self.base_function_depth + self.function_depth > 0
                    && matches!(
                        expr.callee.get_inner_expression(),
                        Expression::ClassExpression(_)
                    )
                {
                    self.diags.push(Diagnostic::warning(
                        DiagnosticKind::PerfAvoidInlineClass,
                        Span::new(expr.span.start, expr.span.end),
                    ));
                }
            }
            AstKind::LabeledStatement(stmt) => {
                if self.check_legacy_placement && self.function_depth > 0 && stmt.label.name == "$"
                {
                    self.diags.push(Diagnostic::warning(
                        DiagnosticKind::ReactiveDeclarationInvalidPlacement,
                        Span::new(stmt.span.start, stmt.span.end),
                    ));
                }
            }
            AstKind::MethodDefinition(it) => {
                self.constructor_stack.push(self.in_constructor);
                if it.kind == MethodDefinitionKind::Constructor {
                    self.in_constructor = true;
                }
            }
            AstKind::FormalParameter(it) => {
                if self.in_constructor && (it.accessibility.is_some() || it.readonly) {
                    self.push_ts("accessibility modifiers on constructor parameters", it.span);
                }
                return JsFlow::SkipSubtree;
            }
            AstKind::Decorator(it) => {
                self.push_ts(
                    "decorators (related TSC proposal is not stage 4 yet)",
                    it.span,
                );
                return JsFlow::SkipSubtree;
            }
            AstKind::AccessorProperty(it) => {
                self.push_ts(
                    "accessor fields (related TSC proposal is not stage 4 yet)",
                    it.span,
                );
                return JsFlow::SkipSubtree;
            }
            AstKind::TSEnumDeclaration(it) => {
                self.push_ts("enums", it.span);
                return JsFlow::SkipSubtree;
            }
            AstKind::TSModuleDeclaration(it) => {
                if namespace_has_non_type_node(it) {
                    self.push_ts("namespaces with non-type nodes", it.span);
                }
                return JsFlow::SkipSubtree;
            }
            _ => {}
        }
        JsFlow::Continue
    }

    fn leave_js_node(&mut self, kind: AstKind<'a>) {
        match kind {
            AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => {
                self.function_depth -= 1;
            }
            AstKind::MethodDefinition(_) => {
                if let Some(prev) = self.constructor_stack.pop() {
                    self.in_constructor = prev;
                }
            }
            _ => {}
        }
    }
}
