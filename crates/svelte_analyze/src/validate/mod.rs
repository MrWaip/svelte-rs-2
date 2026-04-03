mod runes;
mod stores;

use oxc_ast::ast::Program;
use svelte_diagnostics::{Diagnostic, DiagnosticKind};

use crate::types::script::RuneKind;
use crate::{types::data::ParserResult, AnalysisData};

pub fn validate(data: &AnalysisData, parsed: &ParserResult, diags: &mut Vec<Diagnostic>) {
    let Some(program) = &parsed.program else {
        return;
    };
    let offset = parsed.script_content_span.map_or(0, |s| s.start);

    validate_program(data, program, offset, diags);
    validate_custom_element_props(data, diags);
}

pub fn validate_program(
    data: &AnalysisData,
    program: &Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    runes::validate(data, program, offset, diags);
    stores::validate(data, program, offset, diags);
}

/// Warn when `$props()` uses identifier pattern or rest element in a custom element
/// without explicit `customElement.props` config.
fn validate_custom_element_props(data: &AnalysisData, diags: &mut Vec<Diagnostic>) {
    if !data.custom_element {
        return;
    }

    // Explicit `customElement.props` config suppresses the warning.
    if data
        .ce_config
        .as_ref()
        .is_some_and(|c| !c.props.is_empty())
    {
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
        .unwrap_or_default();

    diags.push(Diagnostic::warning(
        DiagnosticKind::CustomElementPropsIdentifier,
        span,
    ));
}
