use svelte_parser::RuneKind;

use crate::data::{AnalysisData, PropAnalysis, PropsAnalysis};

pub fn analyze_props(data: &mut AnalysisData) {
    // Detect $props.id() declaration
    if let Some(script) = data.script.as_ref() {
        for d in &script.declarations {
            if d.is_rune == Some(RuneKind::PropsId) {
                data.props_id = Some(d.name.to_string());
                break;
            }
        }
    }

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
            // In custom element mode, ALL props are prop sources (need getter/setter exports).
            let sym_id = data.scoping.find_binding(root, p.local_name.as_str());
            let is_mutated = data.custom_element
                || sym_id.is_some_and(|id| data.scoping.is_mutated(id));
            let is_prop_source =
                data.custom_element || p.default_span.is_some() || is_mutated;

            // Populate SymbolId-keyed classifications in scoping.
            if !p.is_rest {
                if let Some(sym_id) = sym_id {
                    if is_prop_source {
                        data.scoping.mark_prop_source(sym_id);
                    } else {
                        data.scoping.mark_prop_non_source(sym_id, p.prop_name.to_string());
                    }
                }
            }

            let is_lazy_default = p.default_span.is_some() && !p.is_simple_default;

            PropAnalysis {
                local_name: p.local_name.to_string(),
                prop_name: p.prop_name.to_string(),
                default_span: p.default_span,
                default_text: p.default_text.clone(),
                is_bindable: p.is_bindable,
                is_rest: p.is_rest,
                is_lazy_default,
                is_prop_source,
                is_mutated,
                is_reserved: p.prop_name.starts_with("$$"),
            }
        })
        .collect::<Vec<PropAnalysis>>();

    let has_bindable = decl.props.iter().any(|p| p.is_bindable);

    data.props = Some(PropsAnalysis { props, has_bindable });
}
