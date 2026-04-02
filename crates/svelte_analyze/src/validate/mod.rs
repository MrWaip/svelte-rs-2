mod runes;

use svelte_diagnostics::Diagnostic;

use crate::{AnalysisData, types::data::ParserResult};

pub fn validate(data: &AnalysisData, parsed: &ParserResult, diags: &mut Vec<Diagnostic>) {
    let Some(program) = &parsed.program else { return };
    let offset = parsed.script_content_span.map_or(0, |s| s.start);

    runes::validate(data, program, offset, diags);
}
