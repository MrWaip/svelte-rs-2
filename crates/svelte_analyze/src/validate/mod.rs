mod runes;

use svelte_diagnostics::Diagnostic;

use crate::types::data::ParserResult;

pub fn validate(parsed: &ParserResult, diags: &mut Vec<Diagnostic>) {
    let Some(program) = &parsed.program else { return };
    let offset = parsed.script_content_span.map_or(0, |s| s.start);

    runes::validate(program, offset, diags);
}
