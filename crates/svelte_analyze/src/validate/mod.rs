mod non_reactive_update;
mod runes;
mod stores;

use oxc_ast::ast::{
    ArrowFunctionExpression, BindingPattern, Declaration, Function, ImportDeclarationSpecifier,
    ModuleExportName, NewExpression, Program, Statement,
};
use oxc_ast_visit::walk::{
    walk_arrow_function_expression, walk_declaration, walk_function, walk_new_expression,
    walk_program,
};
use oxc_ast_visit::Visit;
use oxc_semantic::ScopeFlags;
use svelte_ast::Component;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::types::script::RuneKind;
use crate::{types::data::ParserResult, AnalysisData};

pub fn validate(
    component: &Component,
    data: &AnalysisData,
    parsed: &ParserResult,
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
        stores::validate_module(data, module_program, offset, diags);
        validate_perf_class_warnings(module_program, offset, 0, diags);
    }
    non_reactive_update::validate(component, data, parsed, runes, diags);
    validate_snippet_exports(component, parsed, diags);
    validate_svelte_options_warnings(component, data, runes, diags);
    validate_custom_element_props(data, diags);
    validate_script_context(component, runes, diags);
}

/// Warn when `<script context="module">` is used in runes mode — the modern
/// equivalent is `<script module>`.
fn validate_script_context(component: &Component, runes: bool, diags: &mut Vec<Diagnostic>) {
    if !runes {
        return;
    }
    if let Some(script) = &component.module_script {
        if script.context_deprecated {
            diags.push(Diagnostic::warning(
                DiagnosticKind::ScriptContextDeprecated,
                script.span,
            ));
        }
    }
}

pub fn validate_program(
    data: &AnalysisData,
    program: &Program<'_>,
    offset: u32,
    runes: bool,
    diags: &mut Vec<Diagnostic>,
) {
    runes::validate(data, program, offset, runes, diags);
    stores::validate(data, program, offset, diags);
    validate_perf_class_warnings(program, offset, 1, diags);
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
        if let Declaration::ClassDeclaration(class) = decl {
            if self.function_depth > self.base_function_depth {
                self.diags.push(Diagnostic::warning(
                    DiagnosticKind::PerfAvoidNestedClass,
                    self.span(class.span),
                ));
            }
        }

        walk_declaration(self, decl);
    }

    fn visit_new_expression(&mut self, expr: &NewExpression<'a>) {
        if self.function_depth > 0 && matches!(expr.callee, oxc_ast::ast::Expression::ClassExpression(_))
        {
            self.diags.push(Diagnostic::warning(
                DiagnosticKind::PerfAvoidInlineClass,
                self.span(expr.span),
            ));
        }

        walk_new_expression(self, expr);
    }
}

fn validate_module_program(parsed: &ParserResult, diags: &mut Vec<Diagnostic>) {
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

/// Error when `<script module>` exports a name that is a template snippet.
///
/// Snippets are template-level constructs; exporting them from a module block is invalid
/// because they are not accessible as module-scope bindings.
fn validate_snippet_exports(
    component: &Component,
    parsed: &ParserResult,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(module_program) = &parsed.module_program else {
        return;
    };

    // Collect all snippet names defined in the component template.
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
        // Only `export { ... }` (re-exports with `source` are skipped).
        if export.declaration.is_some() || export.source.is_some() {
            continue;
        }
        for specifier in &export.specifiers {
            let oxc_ast::ast::ModuleExportName::IdentifierReference(ident) = &specifier.local
            else {
                continue;
            };
            let name = ident.name.as_str();
            // Only fire if the name is NOT declared in module scope (matches reference compiler).
            // If bound in module scope, it's a valid export of a module-local binding.
            if snippet_names.iter().any(|&s| s == name) && !is_module_bound(module_program, name) {
                let span = Span::new(specifier.span.start, specifier.span.end);
                diags.push(Diagnostic::error(
                    DiagnosticKind::SnippetInvalidExport,
                    span,
                ));
            }
        }
    }
}

/// Returns `true` if `name` is declared at the top level of the module program.
///
/// Mirrors `analysis.module.scope.get(name)` from the reference compiler: covers variable
/// declarations, function/class declarations, and imports (including re-exported declarations).
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

/// Warn when `$props()` uses identifier pattern or rest element in a custom element
/// without explicit `customElement.props` config.
fn validate_custom_element_props(data: &AnalysisData, diags: &mut Vec<Diagnostic>) {
    if !data.custom_element {
        return;
    }

    // Explicit `customElement.props` config suppresses the warning.
    if data.ce_config.as_ref().is_some_and(|c| !c.props.is_empty()) {
        return;
    }

    let Some(props) = &data.props else {
        return;
    };

    let should_warn = props.is_identifier_pattern || props.props.iter().any(|p| p.is_rest);
    if !should_warn {
        return;
    }

    // Use the $props() declaration span from script info.
    let span = data
        .script
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
            "customElement" if !data.custom_element => Some(DiagnosticKind::OptionsMissingCustomElement),
            _ => None,
        };

        if let Some(kind) = kind {
            let span = component.store.get(attr.id()).span();
            diags.push(Diagnostic::warning(kind, span));
        }
    }
}
