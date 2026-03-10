use svelte_ast::Component;
use svelte_js::DeclarationKind;

use crate::data::AnalysisData;

/// Evaluate const declarations with literal initializers and store their values.
pub fn collect_known_values(component: &Component, data: &mut AnalysisData) {
    for sym in &data.symbols {
        if sym.kind != DeclarationKind::Const {
            continue;
        }
        let Some(init_span) = sym.init_span else {
            continue;
        };
        if data.symbol_by_name.get(&sym.name).and_then(|id| data.runes.get(id)).is_some() {
            continue;
        }

        let src = component.source_text(init_span).trim();
        if let Some(val) = try_eval_literal(src) {
            data.known_values.insert(sym.name.clone(), val);
        }
    }
}

fn try_eval_literal(src: &str) -> Option<String> {
    // String literal: "..." or '...' or `...` (simple, no escapes)
    if (src.starts_with('"') && src.ends_with('"'))
        || (src.starts_with('\'') && src.ends_with('\''))
        || (src.starts_with('`') && src.ends_with('`'))
    {
        return Some(src[1..src.len() - 1].to_string());
    }

    // Boolean
    if src == "true" || src == "false" {
        return Some(src.to_string());
    }

    // Number
    if src.parse::<f64>().is_ok() {
        return Some(src.to_string());
    }

    None
}
