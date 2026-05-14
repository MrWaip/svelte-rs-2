use svelte_ast::Component;

use crate::passes::js_analyze::needs_context_for_program;
use crate::types::data::{AnalysisData, JsAst};
use crate::utils::script_info;

pub(crate) fn run<'a>(component: &Component, parsed: &JsAst<'a>, data: &mut AnalysisData<'a>) {
    let uses_runes = data.reactivity.uses_runes();

    if let Some(script) = data.script.info.as_mut() {
        script_info::enrich_from_component_scoping(&data.scoping, script);
        if let Some(program) = parsed.program.as_ref() {
            data.output.needs_context = needs_context_for_program(
                program,
                &data.scoping,
                &data.reactivity,
                script,
                uses_runes,
            );
        }
    }

    if let Some(module_program) = parsed.module_program.as_ref()
        && parsed.module_script_content_span.is_some()
    {
        let mut module_info =
            script_info::extract_script_info(module_program, &component.source, true);
        script_info::enrich_from_component_scoping(&data.scoping, &mut module_info);
        data.output.needs_context |= needs_context_for_program(
            module_program,
            &data.scoping,
            &data.reactivity,
            &module_info,
            uses_runes,
        );
    }
}
