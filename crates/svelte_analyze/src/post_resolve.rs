use svelte_ast::Component;
use crate::script_types::{DeclarationKind, RuneKind};

use crate::data::{AnalysisData, PropAnalysis, PropsAnalysis};
use crate::store_subscriptions;

/// Run all passes that depend on resolve_references but are independent of each other.
/// Combines store detection, known value collection, and props analysis.
pub fn run_post_resolve_passes(component: &Component, data: &mut AnalysisData) {
    store_subscriptions::detect_store_subscriptions(data);
    analyze_declarations(component, data);
}

/// Single pass over script declarations for both known-value collection and props-id detection.
/// Then processes props_declaration separately (only needed for props analysis).
fn analyze_declarations(component: &Component, data: &mut AnalysisData) {
    let script = match &data.script {
        Some(s) => s,
        None => return,
    };
    let root = data.scoping.root_scope_id();

    // Single pass: known_values + props_id detection
    for decl in &script.declarations {
        // --- props_id detection (from props.rs) ---
        if decl.is_rune == Some(RuneKind::PropsId) && data.props_id.is_none() {
            data.props_id = Some(decl.name.to_string());
        }

        // --- known_values logic ---
        let Some(init_span) = decl.init_span else {
            continue;
        };

        let sym_id = data.scoping.find_binding(root, &decl.name);
        let rune_kind = sym_id.and_then(|id| data.scoping.rune_kind(id));
        let is_mutated = sym_id.map_or(false, |id| data.scoping.is_mutated(id));

        let is_foldable_rune = rune_kind == Some(RuneKind::State) && !is_mutated;

        if rune_kind.is_some() && !is_foldable_rune {
            continue;
        }

        if rune_kind.is_none() && decl.kind != DeclarationKind::Const {
            continue;
        }

        let src = component.source_text(init_span).trim();

        let literal_src = if is_foldable_rune {
            extract_rune_arg(src)
        } else {
            Some(src)
        };

        if let Some(lit) = literal_src.and_then(try_eval_literal) {
            if let Some(sym_id) = data.scoping.find_binding(root, &decl.name) {
                data.scoping.set_known_value(sym_id, lit);
            }
        }
    }

    // --- props analysis (from props.rs) ---
    analyze_props_declaration(data);
}

fn analyze_props_declaration(data: &mut AnalysisData) {
    let decl = match data.script.as_ref().and_then(|s| s.props_declaration.as_ref()) {
        Some(d) => d,
        None => return,
    };

    let root = data.scoping.root_scope_id();

    let props = decl
        .props
        .iter()
        .map(|p| {
            let sym_id = data.scoping.find_binding(root, p.local_name.as_str());
            let is_mutated = data.custom_element
                || sym_id.is_some_and(|id| data.scoping.is_mutated(id));
            let is_prop_source =
                data.custom_element || p.default_span.is_some() || is_mutated;

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

fn extract_rune_arg(src: &str) -> Option<&str> {
    let open = src.find('(')?;
    let close = src.rfind(')')?;
    if close <= open + 1 {
        return None;
    }
    Some(src[open + 1..close].trim())
}

fn try_eval_literal(src: &str) -> Option<String> {
    if (src.starts_with('"') && src.ends_with('"'))
        || (src.starts_with('\'') && src.ends_with('\''))
        || (src.starts_with('`') && src.ends_with('`'))
    {
        return Some(src[1..src.len() - 1].to_string());
    }

    if src == "true" || src == "false" {
        return Some(src.to_string());
    }

    if src.parse::<f64>().is_ok() {
        return Some(src.to_string());
    }

    None
}
