use crate::data::{AnalysisData, PropAnalysis, PropsAnalysis};

pub fn analyze_props(data: &mut AnalysisData) {
    let decl = match data.script.as_ref().and_then(|s| s.props_declaration.as_ref()) {
        Some(d) => d,
        None => return,
    };

    let root = data.scoping.root_scope_id();

    let props = decl
        .props
        .iter()
        .map(|p| {
            // In runes mode, a prop needs $.prop() source when it has a default,
            // is reassigned, is mutated, or is bindable.
            let sym_id = data.scoping.find_binding(root, p.local_name.as_str());
            let is_mutated = sym_id.is_some_and(|id| data.scoping.is_mutated(id));
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

    // Populate SymbolId-keyed classifications in scoping.
    for p in &props {
        if p.is_rest { continue; }
        if let Some(sym_id) = data.scoping.find_binding(root, &p.local_name) {
            if p.is_prop_source {
                data.scoping.mark_prop_source(sym_id);
            } else {
                data.scoping.mark_prop_non_source(sym_id, p.prop_name.clone());
            }
        }
    }

    data.props = Some(PropsAnalysis { props, has_bindable });
}
