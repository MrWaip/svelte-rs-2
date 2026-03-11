use oxc_semantic::ScopeId;
use svelte_ast::{Attribute, Component, Fragment, Node};

use crate::data::AnalysisData;

/// Detect which rune symbols are mutated: assigned in script (already tracked by OXC semantic),
/// template expression assignments, or bound via bind directives.
/// After detection, caches the rune name sets on AnalysisData.
pub fn detect_mutations(component: &Component, data: &mut AnalysisData) {
    // Script mutations are already tracked by OXC's SemanticBuilder
    // (symbol_is_mutated() returns true for symbols with write references).

    let root_scope = data.scoping.root_scope_id();

    // Template expression assignments (e.g. `{title = 30}`)
    walk_template_mutations(&component.fragment, data, root_scope);

    // Bind directives imply mutation
    walk_binds(&component.fragment, component, data, root_scope);

    // Cache the computed rune name sets
    data.cache_rune_sets();
}

fn walk_template_mutations(
    fragment: &Fragment,
    data: &mut AnalysisData,
    current_scope: ScopeId,
) {
    for node in &fragment.nodes {
        match node {
            Node::ExpressionTag(tag) => {
                if let Some(info) = data.expressions.get(&tag.id) {
                    for r in &info.references {
                        if r.flags == svelte_js::ReferenceFlags::Write
                            || r.flags == svelte_js::ReferenceFlags::ReadWrite
                        {
                            if let Some(sym_id) = data.scoping.find_binding(current_scope, &r.name)
                            {
                                if data.scoping.is_rune(sym_id) {
                                    data.scoping.mark_template_mutated(sym_id);
                                }
                            }
                        }
                    }
                }
            }
            Node::Element(el) => {
                walk_template_mutations(&el.fragment, data, current_scope);
            }
            Node::IfBlock(b) => {
                walk_template_mutations(&b.consequent, data, current_scope);
                if let Some(alt) = &b.alternate {
                    walk_template_mutations(alt, data, current_scope);
                }
            }
            Node::EachBlock(b) => {
                let body_scope = data
                    .scoping
                    .node_scope(b.id)
                    .unwrap_or(current_scope);
                walk_template_mutations(&b.body, data, body_scope);
                if let Some(fb) = &b.fallback {
                    walk_template_mutations(fb, data, current_scope);
                }
            }
            Node::SnippetBlock(b) => {
                walk_template_mutations(&b.body, data, current_scope);
            }
            _ => {}
        }
    }
}

fn walk_binds(
    fragment: &Fragment,
    component: &Component,
    data: &mut AnalysisData,
    current_scope: ScopeId,
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
                        if let Some(sym_id) = data.scoping.find_binding(current_scope, &name) {
                            if data.scoping.is_rune(sym_id) {
                                data.scoping.mark_bind_mutated(sym_id);
                                data.scoping.mark_template_mutated(sym_id);
                            }
                        }
                    }
                }
                walk_binds(&el.fragment, component, data, current_scope);
            }
            Node::IfBlock(b) => {
                walk_binds(&b.consequent, component, data, current_scope);
                if let Some(alt) = &b.alternate {
                    walk_binds(alt, component, data, current_scope);
                }
            }
            Node::EachBlock(b) => {
                let body_scope = data
                    .scoping
                    .node_scope(b.id)
                    .unwrap_or(current_scope);
                walk_binds(&b.body, component, data, body_scope);
                if let Some(fb) = &b.fallback {
                    walk_binds(fb, component, data, current_scope);
                }
            }
            Node::SnippetBlock(b) => {
                walk_binds(&b.body, component, data, current_scope);
            }
            _ => {}
        }
    }
}
