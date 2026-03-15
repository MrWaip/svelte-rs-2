use crate::data::AnalysisData;

/// Detect store subscriptions by scanning template and script references.
///
/// A store subscription exists when:
/// 1. An expression references `$X` (dollar-prefixed identifier)
/// 2. `X` is declared at root scope
/// 3. `X` is NOT a rune
///
/// Populates `data.store_subscriptions` with base names (e.g. "count" for "$count").
pub fn detect_store_subscriptions(data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();

    // Collect candidate names first to avoid borrow conflict
    let mut candidates: Vec<String> = Vec::new();

    // Template expression references
    for info in data.expressions.values() {
        for r in &info.references {
            collect_store_candidate(&r.name, &mut candidates);
        }
    }

    // Attribute expression references
    for info in data.attr_expressions.values() {
        for r in &info.references {
            collect_store_candidate(&r.name, &mut candidates);
        }
    }

    // Script body references (from svelte_js store_candidates)
    if let Some(script) = &data.script {
        for name in &script.store_candidates {
            candidates.push(name.to_string());
        }
    }

    // Now check and insert with mutable access to data
    for name in candidates {
        if data.store_subscriptions.contains(&name) {
            continue;
        }
        let Some(sym_id) = data.scoping.find_binding(root, &name) else {
            continue;
        };
        if data.scoping.is_rune(sym_id) {
            continue;
        }
        data.store_subscriptions.insert(name);
    }
}

fn collect_store_candidate(name: &str, candidates: &mut Vec<String>) {
    if name.starts_with('$') && name.len() > 1 {
        candidates.push(name[1..].to_string());
    }
}
