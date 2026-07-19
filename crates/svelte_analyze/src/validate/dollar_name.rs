use oxc_ast::ast::{
    ArrowFunctionExpression, BindingPattern, Class, Function, IdentifierReference,
    ImportDeclaration, ImportDeclarationSpecifier, Program, VariableDeclarator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_class, walk_function, walk_import_declaration,
    walk_variable_declarator,
};
use oxc_span::Span as OxcSpan;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

pub(super) fn validate(program: &Program<'_>, runes: bool, diags: &mut Vec<Diagnostic>) {
    let mut v = DollarNameValidator {
        diags,
        depth: 0,
        threshold: if runes { 1 } else { 0 },
    };
    v.visit_program(program);
}

struct DollarNameValidator<'b> {
    diags: &'b mut Vec<Diagnostic>,
    depth: u32,
    threshold: u32,
}

impl DollarNameValidator<'_> {
    fn check(&mut self, name: &str, span: OxcSpan) {
        if self.depth > self.threshold {
            return;
        }
        if name == "$" {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::DollarBindingInvalid,
                Span::new(span.start, span.end),
            ));
        } else if name.starts_with('$') {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::DollarPrefixInvalid,
                Span::new(span.start, span.end),
            ));
        }
    }
}

impl<'a> Visit<'a> for DollarNameValidator<'_> {
    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        if let BindingPattern::BindingIdentifier(id) = &it.id {
            self.check(id.name.as_str(), id.span);
        }
        walk_variable_declarator(self, it);
    }

    fn visit_function(&mut self, it: &Function<'a>, flags: oxc_semantic::ScopeFlags) {
        if let Some(id) = &it.id {
            self.check(id.name.as_str(), id.span);
        }
        self.depth += 1;
        walk_function(self, it, flags);
        self.depth -= 1;
    }

    fn visit_arrow_function_expression(&mut self, it: &ArrowFunctionExpression<'a>) {
        self.depth += 1;
        walk_arrow_function_expression(self, it);
        self.depth -= 1;
    }

    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {
        if self.depth == 0 && it.name == "arguments" {
            self.diags.push(Diagnostic::error(
                DiagnosticKind::InvalidArgumentsUsage,
                Span::new(it.span.start, it.span.end),
            ));
        }
    }

    fn visit_class(&mut self, it: &Class<'a>) {
        if let Some(id) = &it.id {
            self.check(id.name.as_str(), id.span);
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
                self.check(name, span);
            }
        }
        walk_import_declaration(self, it);
    }
}
