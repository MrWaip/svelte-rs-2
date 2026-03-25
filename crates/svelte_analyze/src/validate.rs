use svelte_ast::Component;
use svelte_diagnostics::Diagnostic;

use crate::types::data::AnalysisData;

pub fn validate(_component: &Component, _data: &AnalysisData, _diags: &mut Vec<Diagnostic>) {
    // Expression parse errors are already collected in parse_js pass.
    // Additional semantic checks can be added here in future stages.
}
