use svelte_ast::{Attribute, Component, Fragment, Node, NodeId};

use crate::data::AnalysisData;

pub fn mark_reactivity(component: &Component, data: &mut AnalysisData) {
    walk_fragment(&component.fragment, component, data);
}

fn walk_fragment(fragment: &Fragment, component: &Component, data: &mut AnalysisData) {
    for node in &fragment.nodes {
        walk_node(node, component, data);
    }
}

fn walk_node(node: &Node, component: &Component, data: &mut AnalysisData) {
    match node {
        Node::ExpressionTag(tag) => {
            if expr_references_rune(&tag.id, data) {
                data.dynamic_nodes.insert(tag.id);
            }
        }
        Node::Element(el) => {
            let mut needs_ref = false;
            for (attr_idx, attr) in el.attributes.iter().enumerate() {
                if attr_references_rune(attr, attr_idx, el.id, data) {
                    data.dynamic_attrs.insert((el.id, attr_idx));
                    needs_ref = true;
                }
            }
            if needs_ref {
                data.node_needs_ref.insert(el.id);
            }
            walk_fragment(&el.fragment, component, data);
        }
        Node::IfBlock(block) => {
            if expr_references_rune(&block.id, data) {
                data.dynamic_nodes.insert(block.id);
            }
            walk_fragment(&block.consequent, component, data);
            if let Some(alt) = &block.alternate {
                walk_fragment(alt, component, data);
            }
        }
        Node::EachBlock(block) => {
            if expr_references_rune(&block.id, data) {
                data.dynamic_nodes.insert(block.id);
            }
            walk_fragment(&block.body, component, data);
            if let Some(fb) = &block.fallback {
                walk_fragment(fb, component, data);
            }
        }
        Node::Text(_) | Node::Comment(_) => {}
    }
}

fn expr_references_rune(node_id: &NodeId, data: &AnalysisData) -> bool {
    if let Some(info) = data.expressions.get(node_id) {
        return info
            .references
            .iter()
            .any(|r| data.symbol_by_name.get(&r.name).is_some_and(|id| data.runes.contains_key(id)));
    }
    false
}

fn attr_references_rune(attr: &Attribute, attr_idx: usize, el_id: NodeId, data: &AnalysisData) -> bool {
    if matches!(attr, Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)) {
        return false;
    }
    if let Some(info) = data.attr_expressions.get(&(el_id, attr_idx)) {
        return info
            .references
            .iter()
            .any(|r| data.symbol_by_name.get(&r.name).is_some_and(|id| data.runes.contains_key(id)));
    }
    false
}
