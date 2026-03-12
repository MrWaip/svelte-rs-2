use crate::data::{AnalysisData, PropAnalysis, PropsAnalysis};

pub fn analyze_props(data: &mut AnalysisData) {
    let decl = match data.script.as_ref().and_then(|s| s.props_declaration.as_ref()) {
        Some(d) => d,
        None => return,
    };

    let props = decl
        .props
        .iter()
        .map(|p| {
            // In runes mode, a prop needs $.prop() source when it has a default,
            // is reassigned, is mutated, or is bindable.
            let is_mutated = data.mutated_runes.contains(&p.local_name);
            let is_prop_source =
                p.default_span.is_some() || is_mutated;

            let is_lazy_default = p.default_text.as_ref()
                .is_some_and(|text| !svelte_js::is_simple_expression(text));

            PropAnalysis {
                local_name: p.local_name.clone(),
                prop_name: p.prop_name.clone(),
                default_span: p.default_span,
                default_text: p.default_text.clone(),
                is_bindable: p.is_bindable,
                is_rest: p.is_rest,
                is_prop_source,
                is_lazy_default,
            }
        })
        .collect();

    let has_bindable = decl.props.iter().any(|p| p.is_bindable);
    data.props = Some(PropsAnalysis { props, has_bindable });
}
