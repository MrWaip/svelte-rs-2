mod runes;

use oxc_ast::ast::Program;
use svelte_diagnostics::Diagnostic;

use crate::{AnalysisData, types::data::ParserResult};

pub fn validate(data: &AnalysisData, parsed: &ParserResult, diags: &mut Vec<Diagnostic>) {
    let Some(program) = &parsed.program else { return };
    let offset = parsed.script_content_span.map_or(0, |s| s.start);

    validate_program(data, program, offset, diags);
}

pub fn validate_program(
    data: &AnalysisData,
    program: &Program<'_>,
    offset: u32,
    diags: &mut Vec<Diagnostic>,
) {
    runes::validate(data, program, offset, diags);
}
