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

    // Template expression assignments (e.g. `{title = 30}`)
    walk_template_mutations(&component.fragment, &rune_names, data, &mut mutated);

    // Bind directives imply mutation
    walk_binds(&component.fragment, component, &rune_names, &mut data.bind_mutated_runes);

    // Merge bind mutations into the full set
    mutated.extend(data.bind_mutated_runes.iter().cloned());

    data.mutated_runes = mutated;
}

fn walk_template_mutations(
    fragment: &Fragment,
    rune_names: &HashSet<String>,
    data: &AnalysisData,
    out: &mut HashSet<String>,
) {
    for node in &fragment.nodes {
        match node {
            Node::ExpressionTag(tag) => {
                if let Some(info) = data.expressions.get(&tag.id) {
                    for r in &info.references {
                        if r.flags == svelte_js::ReferenceFlags::Write
                            || r.flags == svelte_js::ReferenceFlags::ReadWrite
                        {
                            if rune_names.contains(&r.name) {
                                out.insert(r.name.clone());
                            }
                        }
                    }
                }
            }
            Node::Element(el) => {
                walk_template_mutations(&el.fragment, rune_names, data, out);
            }
            Node::IfBlock(b) => {
                walk_template_mutations(&b.consequent, rune_names, data, out);
                if let Some(alt) = &b.alternate {
                    walk_template_mutations(alt, rune_names, data, out);
                }
            }
            Node::EachBlock(b) => {
                walk_template_mutations(&b.body, rune_names, data, out);
                if let Some(fb) = &b.fallback {
                    walk_template_mutations(fb, rune_names, data, out);
                }
            }
            _ => {}
        }
    }
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
