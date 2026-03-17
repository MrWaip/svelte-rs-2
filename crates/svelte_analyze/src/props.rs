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
            let is_mutated = data.is_mutable_rune(p.local_name.as_str());
            let is_prop_source =
                p.default_span.is_some() || is_mutated;

            let is_lazy_default = p.default_text.as_ref()
                .is_some_and(|text| !svelte_js::is_simple_expression(text));

            PropAnalysis {
                local_name: p.local_name.to_string(),
                prop_name: p.prop_name.to_string(),
                default_span: p.default_span,
                default_text: p.default_text.clone(),
                is_bindable: p.is_bindable,
                is_rest: p.is_rest,
                is_prop_source,
                is_lazy_default,
            }
        })
        .collect::<Vec<PropAnalysis>>();

    let has_bindable = decl.props.iter().any(|p| p.is_bindable);

    let mut prop_sources = rustc_hash::FxHashSet::default();
    let mut prop_non_sources = rustc_hash::FxHashMap::default();
    for p in &props {
        if p.is_rest { continue; }
        if p.is_prop_source {
            prop_sources.insert(p.local_name.clone());
        } else {
            prop_non_sources.insert(p.local_name.clone(), p.prop_name.clone());
        }
    }

    data.props = Some(PropsAnalysis { props, has_bindable, prop_sources, prop_non_sources });
}
