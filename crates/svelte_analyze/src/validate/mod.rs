mod runes;
mod stores;

use oxc_ast::ast::{Program, Statement};
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

    validate_snippet_exports(component, parsed, diags);
    validate_custom_element_props(data, diags);
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
            if snippet_names.iter().any(|&s| s == name) {
                let span = Span::new(specifier.span.start, specifier.span.end);
                diags.push(Diagnostic::error(DiagnosticKind::SnippetInvalidExport, span));
            }
        }
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
