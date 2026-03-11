use svelte_ast::Component;
use svelte_js::DeclarationKind;

use crate::data::AnalysisData;

/// Evaluate declarations with literal initializers and store their known values.
/// For unmutated runes like `let x = $state("world")`, extracts the rune argument.
pub fn collect_known_values(component: &Component, data: &mut AnalysisData) {
    let script = match &data.script {
        Some(s) => s,
        None => return,
    };

    for decl in &script.declarations {
        let Some(init_span) = decl.init_span else {
            continue;
        };

        let root = data.scoping.root_scope_id();
        let sym_id = data.scoping.find_binding(root, &decl.name);
        let rune_kind = sym_id.and_then(|id| data.scoping.rune_kind(id));
        let is_mutated = sym_id.map_or(false, |id| data.scoping.is_mutated(id));

        // Only unmutated $state can be folded; other runes ($props etc.) are always dynamic
        let is_foldable_rune = rune_kind == Some(svelte_js::RuneKind::State) && !is_mutated;

        if rune_kind.is_some() && !is_foldable_rune {
            continue;
        }

        // Non-rune: only const declarations can be known
        if rune_kind.is_none() && decl.kind != DeclarationKind::Const {
            continue;
        }

        let src = component.source_text(init_span).trim();

        let literal_src = if is_foldable_rune {
            // Extract the argument from $state("world") → "world"
            extract_rune_arg(src)
        } else {
            Some(src)
        };

        if let Some(lit) = literal_src.and_then(try_eval_literal) {
            data.known_values.insert(decl.name.clone(), lit);
        }
    }
}

/// Extract the first argument from a rune call like `$state("world")` → `"world"`.
fn extract_rune_arg(src: &str) -> Option<&str> {
    let open = src.find('(')?;
    let close = src.rfind(')')?;
    if close <= open + 1 {
        return None; // empty args like $state()
    }
    Some(src[open + 1..close].trim())
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
