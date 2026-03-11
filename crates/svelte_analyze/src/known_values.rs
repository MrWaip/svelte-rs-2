use svelte_ast::Component;
use svelte_js::DeclarationKind;

use crate::data::AnalysisData;

/// Evaluate const declarations with literal initializers and store their values.
pub fn collect_known_values(component: &Component, data: &mut AnalysisData) {
    let script = match &data.script {
        Some(s) => s,
        None => return,
    };

    for decl in &script.declarations {
        if decl.kind != DeclarationKind::Const {
            continue;
        }
        let Some(init_span) = decl.init_span else {
            continue;
        };
        // Skip runes
        let root = data.scoping.root_scope_id();
        if let Some(sym_id) = data.scoping.find_binding(root, &decl.name) {
            if data.scoping.is_rune(sym_id) {
                continue;
            }
        }

        let src = component.source_text(init_span).trim();
        if let Some(val) = try_eval_literal(src) {
            data.known_values.insert(decl.name.clone(), val);
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
