mod legacy;
mod non_reactive_update;
mod runes;
mod stores;

use oxc_ast::ast::{
    ArrowFunctionExpression, BindingPattern, Declaration, Function, ImportDeclarationSpecifier,
    ModuleExportName, NewExpression, Program, Statement,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_declaration, walk_function, walk_new_expression,
    walk_program,
};
use oxc_semantic::ScopeFlags;
use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::types::script::RuneKind;
use crate::{AnalysisData, types::data::JsAst};

pub fn validate(
    component: &Component,
    data: &AnalysisData,
    parsed: &JsAst,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    if let Some(program) = &parsed.program {
        let offset = parsed.script_content_span.map_or(0, |s| s.start);
        validate_program(data, program, offset, runes, diags);
    }

    validate_module_program(parsed, diags);
    if let Some(module_program) = &parsed.module_program {
        let offset = parsed.module_script_content_span.map_or(0, |s| s.start);
        runes::validate_module_props_runes(data, module_program, offset, runes, diags);
        stores::validate_module(data, module_program, offset, diags);
        validate_perf_class_warnings(module_program, offset, 0, diags);
    }
    non_reactive_update::validate(component, data, parsed, runes, diags);
    validate_snippet_exports(component, parsed, diags);
    validate_svelte_options_warnings(component, data, runes, diags);
    validate_custom_element_props(data, diags);
    validate_script_context(component, runes, diags);
}

fn validate_script_context(component: &Component, runes: bool, diags: &mut Vec<Diagnostic>) {
    if !runes {
        return;
    }
    if let Some(script) = &component.module_script
        && script.context_deprecated
    {
        diags.push(Diagnostic::warning(
            DiagnosticKind::ScriptContextDeprecated,
            script.span,
        ));
    }
}

pub fn validate_program(
    data: &AnalysisData,
    program: &Program<'_>,
    offset: u32,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    legacy::validate_legacy_diagnostics(data, program, offset, runes, diags);
    runes::validate(data, program, offset, runes, diags);
    stores::validate(data, program, offset, diags);
    validate_perf_class_warnings(program, offset, 1, diags);
}

pub(crate) fn span_already_taken(diags: &[Diagnostic], span: Span) -> bool {
    diags.iter().any(|d| d.span == span)
}

pub fn validate_standalone_module(
    data: &AnalysisData,
    program: &Program<'_>,
    offset: u32,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    runes::validate(data, program, offset, runes, diags);
    stores::validate_standalone_module(data, program, offset, diags);
    validate_perf_class_warnings(program, offset, 0, diags);
}

fn validate_perf_class_warnings(
    program: &Program<'_>,
    offset: u32,
    base_function_depth: u32,
    diags: &mut Vec<Diagnostic>,
) {
    let mut visitor = PerfClassWarningValidator {
        diags,
        offset,
        base_function_depth,
        function_depth: base_function_depth,
    };
    visitor.visit_program(program);
}

struct PerfClassWarningValidator<'a> {
    diags: &'a mut Vec<Diagnostic>,
    offset: u32,
    base_function_depth: u32,
    function_depth: u32,
}

impl PerfClassWarningValidator<'_> {
    fn span(&self, span: oxc_span::Span) -> Span {
        Span::new(span.start + self.offset, span.end + self.offset)
    }
}

impl<'a> Visit<'a> for PerfClassWarningValidator<'_> {
    fn visit_program(&mut self, program: &Program<'a>) {
        walk_program(self, program);
    }

    fn visit_function(&mut self, function: &Function<'a>, flags: ScopeFlags) {
        self.function_depth += 1;
        walk_function(self, function, flags);
        self.function_depth -= 1;
    }

    fn visit_arrow_function_expression(&mut self, expr: &ArrowFunctionExpression<'a>) {
        self.function_depth += 1;
        walk_arrow_function_expression(self, expr);
        self.function_depth -= 1;
    }

    fn visit_declaration(&mut self, decl: &Declaration<'a>) {
        if let Declaration::ClassDeclaration(class) = decl
            && self.function_depth > self.base_function_depth
        {
            self.diags.push(Diagnostic::warning(
                DiagnosticKind::PerfAvoidNestedClass,
                self.span(class.span),
            ));
        }

        walk_declaration(self, decl);
    }

    fn visit_new_expression(&mut self, expr: &NewExpression<'a>) {
        if self.function_depth > 0
            && matches!(expr.callee, oxc_ast::ast::Expression::ClassExpression(_))
        {
            self.diags.push(Diagnostic::warning(
                DiagnosticKind::PerfAvoidInlineClass,
                self.span(expr.span),
            ));
        }

        walk_new_expression(self, expr);
    }
}

fn validate_module_program(parsed: &JsAst, diags: &mut Vec<Diagnostic>) {
    let Some(module_program) = &parsed.module_program else {
        return;
    };

    for stmt in &module_program.body {
        match stmt {
            Statement::ExportDefaultDeclaration(export) => {
                diags.push(Diagnostic::error(
                    DiagnosticKind::ModuleIllegalDefaultExport,
                    Span::new(export.span.start, export.span.end),
                ));
            }
            Statement::ExportNamedDeclaration(export)
                if export.specifiers.iter().any(export_specifier_is_default) =>
            {
                diags.push(Diagnostic::error(
                    DiagnosticKind::ModuleIllegalDefaultExport,
                    Span::new(export.span.start, export.span.end),
                ));
            }
            _ => {}
        }
    }
}

fn export_specifier_is_default(specifier: &oxc_ast::ast::ExportSpecifier<'_>) -> bool {
    match &specifier.exported {
        ModuleExportName::IdentifierName(name) => name.name == "default",
        ModuleExportName::IdentifierReference(name) => name.name == "default",
        ModuleExportName::StringLiteral(name) => name.value == "default",
    }
}

fn validate_snippet_exports(component: &Component, parsed: &JsAst, diags: &mut Vec<Diagnostic>) {
    let Some(module_program) = &parsed.module_program else {
        return;
    };

    let snippet_names: Vec<&str> = (0..component.store.len())
        .filter_map(|i| {
            component
                .store
                .get(svelte_ast::NodeId(i))
                .as_snippet_block()
                .map(|s| s.name(&component.source))
        })
        .collect();

    if snippet_names.is_empty() {
        return;
    }

    for stmt in &module_program.body {
        let Statement::ExportNamedDeclaration(export) = stmt else {
            continue;
        };

        if export.declaration.is_some() || export.source.is_some() {
            continue;
        }
        for specifier in &export.specifiers {
            let oxc_ast::ast::ModuleExportName::IdentifierReference(ident) = &specifier.local
            else {
                continue;
            };
            let name = ident.name.as_str();

            if snippet_names.contains(&name) && !is_module_bound(module_program, name) {
                let span = Span::new(specifier.span.start, specifier.span.end);
                diags.push(Diagnostic::error(
                    DiagnosticKind::SnippetInvalidExport,
                    span,
                ));
            }
        }
    }
}

fn is_module_bound<'a>(program: &Program<'a>, name: &str) -> bool {
    for stmt in &program.body {
        match stmt {
            Statement::VariableDeclaration(decl) => {
                if decl
                    .declarations
                    .iter()
                    .any(|d| binding_contains(&d.id, name))
                {
                    return true;
                }
            }
            Statement::FunctionDeclaration(func) => {
                if func.id.as_ref().is_some_and(|id| id.name == name) {
                    return true;
                }
            }
            Statement::ClassDeclaration(cls) => {
                if cls.id.as_ref().is_some_and(|id| id.name == name) {
                    return true;
                }
            }
            Statement::ImportDeclaration(import) => {
                if let Some(specifiers) = &import.specifiers {
                    for spec in specifiers {
                        let local = match spec {
                            ImportDeclarationSpecifier::ImportSpecifier(s) => s.local.name.as_str(),
                            ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                                s.local.name.as_str()
                            }
                            ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                                s.local.name.as_str()
                            }
                        };
                        if local == name {
                            return true;
                        }
                    }
                }
            }
            Statement::ExportNamedDeclaration(export) => {
                if let Some(decl) = &export.declaration {
                    match decl {
                        Declaration::VariableDeclaration(d) => {
                            if d.declarations.iter().any(|v| binding_contains(&v.id, name)) {
                                return true;
                            }
                        }
                        Declaration::FunctionDeclaration(f) => {
                            if f.id.as_ref().is_some_and(|id| id.name == name) {
                                return true;
                            }
                        }
                        Declaration::ClassDeclaration(c) => {
                            if c.id.as_ref().is_some_and(|id| id.name == name) {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn binding_contains(pattern: &BindingPattern<'_>, name: &str) -> bool {
    match pattern {
        BindingPattern::BindingIdentifier(id) => id.name == name,
        BindingPattern::ObjectPattern(obj) => {
            obj.properties
                .iter()
                .any(|p| binding_contains(&p.value, name))
                || obj
                    .rest
                    .as_ref()
                    .is_some_and(|r| binding_contains(&r.argument, name))
        }
        BindingPattern::ArrayPattern(arr) => {
            arr.elements
                .iter()
                .flatten()
                .any(|e| binding_contains(e, name))
                || arr
                    .rest
                    .as_ref()
                    .is_some_and(|r| binding_contains(&r.argument, name))
        }
        BindingPattern::AssignmentPattern(assign) => binding_contains(&assign.left, name),
    }
}

fn validate_custom_element_props(data: &AnalysisData, diags: &mut Vec<Diagnostic>) {
    if !data.output.custom_element {
        return;
    }

    if data
        .script
        .ce_config
        .as_ref()
        .is_some_and(|c| !c.props.is_empty())
    {
        return;
    }

    let Some(props) = data.script.props_declaration() else {
        return;
    };

    let should_warn = props.is_identifier_pattern || props.props.iter().any(|p| p.is_rest);
    if !should_warn {
        return;
    }

    let span = data
        .script
        .info
        .as_ref()
        .and_then(|s| {
            s.declarations
                .iter()
                .find(|d| d.is_rune == Some(RuneKind::Props))
                .map(|d| d.span)
        })
        .unwrap_or_else(|| panic!("data.props exists but no $props() declaration in script info"));

    diags.push(Diagnostic::warning(
        DiagnosticKind::CustomElementPropsIdentifier,
        span,
    ));
}

fn validate_svelte_options_warnings(
    component: &Component,
    data: &AnalysisData,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(options) = &component.options else {
        return;
    };

    for attr in &options.attributes {
        let kind = match attr.html_name() {
            "accessors" if runes => Some(DiagnosticKind::OptionsDeprecatedAccessors),
            "immutable" if runes => Some(DiagnosticKind::OptionsDeprecatedImmutable),
            "customElement" if !data.output.custom_element => {
                Some(DiagnosticKind::OptionsMissingCustomElement)
            }
            _ => None,
        };

        if let Some(kind) = kind {
            diags.push(Diagnostic::warning(kind, attr.span()));
        }
    }
}
