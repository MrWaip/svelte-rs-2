use oxc_ast::ast::{
    ArrowFunctionExpression, BindingPattern, CatchParameter, Class, ClassType, Function,
    FunctionType, IdentifierReference, ImportDeclaration, ImportDeclarationSpecifier, Program,
    VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_catch_parameter, walk_class, walk_function,
    walk_import_declaration, walk_variable_declarator,
};
use oxc_span::Span as OxcSpan;
use svelte_ast::STORE_SUBSCRIPTION_SIGIL;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

pub(super) const MODULE_FUNCTION_DEPTH: u32 = 0;
pub(super) const INSTANCE_FUNCTION_DEPTH: u32 = 1;

const REPORTED_FUNCTION_DEPTH: u32 = 1;

#[derive(Clone, Copy)]
enum DeclarationReach {
    UpToReportedDepth,
    AnyDepthInRunes,
}

pub(super) fn validate(
    program: &Program<'_>,
    runes: bool,
    base_function_depth: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut v = DollarNameValidator {
        diags,
        runes,
        base_function_depth,
        function_depth: base_function_depth,
    };
    v.visit_program(program);
}

struct DollarNameValidator<'b> {
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

impl<'a> Visit<'a> for DollarNameValidator<'_> {
    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        self.check_pattern(&it.id, DeclarationReach::AnyDepthInRunes);
        walk_variable_declarator(self, it);
    }

    fn visit_catch_parameter(&mut self, it: &CatchParameter<'a>) {
        self.check_pattern(&it.pattern, DeclarationReach::UpToReportedDepth);
        walk_catch_parameter(self, it);
    }

    fn visit_function(&mut self, it: &Function<'a>, flags: oxc_semantic::ScopeFlags) {
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
        walk_function(self, it, flags);
        self.function_depth -= 1;
    }

    fn visit_arrow_function_expression(&mut self, it: &ArrowFunctionExpression<'a>) {
        if it.expression {
            walk_arrow_function_expression(self, it);
            return;
        }
        self.function_depth += 1;
        walk_arrow_function_expression(self, it);
        self.function_depth -= 1;
    }

    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {
        if self.function_depth == self.base_function_depth && it.name == "arguments" {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::InvalidArgumentsUsage,
                Span::new(it.span.start, it.span.end),
            ));
        }
    }

    fn visit_class(&mut self, it: &Class<'a>) {
        if let Some(id) = &it.id {
            let reach = match it.r#type {
                ClassType::ClassDeclaration => DeclarationReach::AnyDepthInRunes,
                ClassType::ClassExpression => DeclarationReach::UpToReportedDepth,
            };
            self.check(id.name.as_str(), id.span, reach);
        }
        walk_class(self, it);
    }

    fn visit_import_declaration(&mut self, it: &ImportDeclaration<'a>) {
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
        walk_import_declaration(self, it);
    }
}
