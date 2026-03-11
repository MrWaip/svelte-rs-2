use svelte_ast::{Attribute, Component, Fragment, Node};

use crate::data::AnalysisData;

/// Detect which rune symbols are mutated: assigned in script (already tracked by OXC semantic),
/// template expression assignments, or bound via bind directives.
/// After detection, caches the rune name sets on AnalysisData.
pub fn detect_mutations(component: &Component, data: &mut AnalysisData) {
    // Script mutations are already tracked by OXC's SemanticBuilder
    // (symbol_is_mutated() returns true for symbols with write references).

    let rune_names = data.scoping.rune_names();
    if rune_names.is_empty() {
        data.cache_rune_sets();
        return;
    }

    // Template expression assignments (e.g. `{title = 30}`)
    // TODO: walk_template_mutations is not scope-aware — it matches rune names by string,
    // so an each-block variable shadowing a rune would incorrectly mark the rune as mutated.
    walk_template_mutations(&component.fragment, &rune_names, data);

    // Bind directives imply mutation
    // TODO: walk_binds always resolves in root scope — bind: on each-block variables won't
    // be detected (currently fine since bind: targets are always root-scope runes).
    walk_binds(&component.fragment, component, data);

    // Cache the computed rune name sets
    data.cache_rune_sets();
}

fn walk_template_mutations(
    fragment: &Fragment,
    rune_names: &std::collections::HashSet<String>,
    data: &mut AnalysisData,
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
                                let root = data.scoping.root_scope_id();
                                if let Some(sym_id) = data.scoping.find_binding(root, &r.name) {
                                    data.scoping.mark_symbol_mutated(sym_id);
                                }
                            }
                        }
                    }
                }
            }
            Node::Element(el) => {
                walk_template_mutations(&el.fragment, rune_names, data);
            }
            Node::IfBlock(b) => {
                walk_template_mutations(&b.consequent, rune_names, data);
                if let Some(alt) = &b.alternate {
                    walk_template_mutations(alt, rune_names, data);
                }
            }
            Node::EachBlock(b) => {
                walk_template_mutations(&b.body, rune_names, data);
                if let Some(fb) = &b.fallback {
                    walk_template_mutations(fb, rune_names, data);
                }
            }
            _ => {}
        }
    }
}

fn walk_binds(fragment: &Fragment, component: &Component, data: &mut AnalysisData) {
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
                        let root = data.scoping.root_scope_id();
                        if let Some(sym_id) = data.scoping.find_binding(root, &name) {
                            if data.scoping.is_rune(sym_id) {
                                data.scoping.mark_bind_mutated(sym_id);
                                data.scoping.mark_symbol_mutated(sym_id);
                            }
                        }
                    }
                }
                walk_binds(&el.fragment, component, data);
            }
            Node::IfBlock(b) => {
                walk_binds(&b.consequent, component, data);
                if let Some(alt) = &b.alternate {
                    walk_binds(alt, component, data);
                }
            }
            Node::EachBlock(b) => {
                walk_binds(&b.body, component, data);
                if let Some(fb) = &b.fallback {
                    walk_binds(fb, component, data);
                }
            }
            _ => {}
        }
    }
}
