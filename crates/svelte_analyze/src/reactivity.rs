use oxc_semantic::ScopeId;
use svelte_ast::{Attribute, Component, Fragment, Node, NodeId};

use crate::data::AnalysisData;

pub fn mark_reactivity(component: &Component, data: &mut AnalysisData) {
    let root = data.scoping.root_scope_id();
    walk_fragment(&component.fragment, component, data, root);
}

fn walk_fragment(
    fragment: &Fragment,
    component: &Component,
    data: &mut AnalysisData,
    current_scope: ScopeId,
) {
    for node in &fragment.nodes {
        walk_node(node, component, data, current_scope);
    }
}

fn walk_node(
    node: &Node,
    component: &Component,
    data: &mut AnalysisData,
    current_scope: ScopeId,
) {
    match node {
        Node::ExpressionTag(tag) => {
            if expr_is_dynamic(&tag.id, data, current_scope) {
                data.dynamic_nodes.insert(tag.id);
            }
        }
        Node::Element(el) => {
            let mut needs_ref = false;
            for (attr_idx, attr) in el.attributes.iter().enumerate() {
                if attr_is_dynamic(attr, attr_idx, el.id, data, current_scope) {
                    data.dynamic_attrs.insert((el.id, attr_idx));
                    needs_ref = true;
                }
                // Bind directives always need a DOM ref
                if matches!(attr, Attribute::BindDirective(_)) {
                    needs_ref = true;
                }
            }
            if needs_ref {
                data.node_needs_ref.insert(el.id);
            }
            walk_fragment(&el.fragment, component, data, current_scope);
        }
        Node::IfBlock(block) => {
            if expr_is_dynamic(&block.id, data, current_scope) {
                data.dynamic_nodes.insert(block.id);
            }
            walk_fragment(&block.consequent, component, data, current_scope);
            if let Some(alt) = &block.alternate {
                walk_fragment(alt, component, data, current_scope);
            }
        }
        Node::EachBlock(block) => {
            if expr_is_dynamic(&block.id, data, current_scope) {
                data.dynamic_nodes.insert(block.id);
            }
            // Each block body uses the child scope (with context/index bindings)
            let body_scope = data
                .scoping
                .node_scope(block.id)
                .unwrap_or(current_scope);
            walk_fragment(&block.body, component, data, body_scope);
            // Fallback uses parent scope
            if let Some(fb) = &block.fallback {
                walk_fragment(fb, component, data, current_scope);
            }
        }
        Node::Text(_) | Node::Comment(_) => {}
    }
}

fn expr_is_dynamic(node_id: &NodeId, data: &AnalysisData, current_scope: ScopeId) -> bool {
    if let Some(info) = data.expressions.get(node_id) {
        return info
            .references
            .iter()
            .any(|r| data.scoping.is_dynamic_ref(current_scope, &r.name));
    }
    false
}

// Attributes are dynamic only for *mutated* runes — unlike expressions where
// any rune makes the expression dynamic. This is because attributes generate
// `$.attr_effect()` which requires a signal that can change.
fn attr_is_dynamic(
    attr: &Attribute,
    attr_idx: usize,
    el_id: NodeId,
    data: &AnalysisData,
    current_scope: ScopeId,
) -> bool {
    if matches!(
        attr,
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
    ) {
        return false;
    }
    if let Some(info) = data.attr_expressions.get(&(el_id, attr_idx)) {
        return info.references.iter().any(|r| {
            // For attributes, we check if the reference is a mutated rune
            // (same logic as before, but scope-aware)
            let root = data.scoping.root_scope_id();
            if let Some(sym_id) = data.scoping.find_binding(current_scope, &r.name) {
                // Non-root scope binding (each-block var) is always dynamic
                if data.scoping.symbol_scope_id(sym_id) != root {
                    return true;
                }
                // Root-scope rune that is mutated
                if data.scoping.is_rune(sym_id) && data.scoping.is_mutated(sym_id) {
                    return true;
                }
            }
            false
        });
    }
    false
}
