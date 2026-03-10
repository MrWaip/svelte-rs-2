use std::collections::HashSet;

use svelte_ast::{Attribute, Component, Fragment, Node, ScriptLanguage};

use crate::data::AnalysisData;

/// Detect which rune symbols are mutated: assigned in script (via OXC semantic) or bound via bind directives.
pub fn detect_mutations(component: &Component, data: &mut AnalysisData) {
    let rune_names: HashSet<String> = data
        .symbol_by_name
        .iter()
        .filter_map(|(name, sid)| data.runes.contains_key(sid).then(|| name.clone()))
        .collect();

    if rune_names.is_empty() {
        return;
    }

    let mut mutated = HashSet::new();

    // Script assignments (via OXC semantic — handles nested functions, arrow functions, etc.)
    if let Some(script) = &component.script {
        let is_ts = script.language == ScriptLanguage::TypeScript;
        let script_text = component.source_text(script.content_span);
        let assigned = svelte_js::find_script_mutations(script_text, is_ts);
        for name in assigned {
            if rune_names.contains(&name) {
                mutated.insert(name);
            }
        }
    }

    // Bind directives imply mutation
    walk_binds(&component.fragment, component, &rune_names, &mut data.bind_mutated_runes);

    // Merge bind mutations into the full set
    mutated.extend(data.bind_mutated_runes.iter().cloned());

    data.mutated_runes = mutated;
}

fn walk_binds(
    fragment: &Fragment,
    component: &Component,
    rune_names: &HashSet<String>,
    out: &mut HashSet<String>,
) {
    for node in &fragment.nodes {
        match node {
            Node::Element(el) => {
                for attr in &el.attributes {
                    if let Attribute::BindDirective(bind) = attr {
                        let name = if bind.shorthand {
                            bind.name.clone()
                        } else if let Some(span) = bind.expression_span {
                            component.source_text(span).trim().to_string()
                        } else {
                            continue;
                        };
                        if rune_names.contains(&name) {
                            out.insert(name);
                        }
                    }
                }
                walk_binds(&el.fragment, component, rune_names, out);
            }
            Node::IfBlock(b) => {
                walk_binds(&b.consequent, component, rune_names, out);
                if let Some(alt) = &b.alternate {
                    walk_binds(alt, component, rune_names, out);
                }
            }
            Node::EachBlock(b) => {
                walk_binds(&b.body, component, rune_names, out);
                if let Some(fb) = &b.fallback {
                    walk_binds(fb, component, rune_names, out);
                }
            }
            _ => {}
        }
    }
}
