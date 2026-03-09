use svelte_ast::{Attribute, Component, Fragment, Node, NodeId};

use crate::data::AnalysisData;

pub fn mark_reactivity(component: &Component, data: &mut AnalysisData) {
    let mut each_vars = Vec::new();
    walk_fragment(&component.fragment, component, data, &mut each_vars);
}

fn walk_fragment(
    fragment: &Fragment,
    component: &Component,
    data: &mut AnalysisData,
    each_vars: &mut Vec<String>,
) {
    for node in &fragment.nodes {
        walk_node(node, component, data, each_vars);
    }
}

fn walk_node(
    node: &Node,
    component: &Component,
    data: &mut AnalysisData,
    each_vars: &mut Vec<String>,
) {
    match node {
        Node::ExpressionTag(tag) => {
            if expr_is_dynamic(&tag.id, data, each_vars) {
                data.dynamic_nodes.insert(tag.id);
            }
        }
        Node::Element(el) => {
            let mut needs_ref = false;
            for (attr_idx, attr) in el.attributes.iter().enumerate() {
                if attr_is_dynamic(attr, attr_idx, el.id, data, each_vars) {
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
            walk_fragment(&el.fragment, component, data, each_vars);
        }
        Node::IfBlock(block) => {
            if expr_is_dynamic(&block.id, data, each_vars) {
                data.dynamic_nodes.insert(block.id);
            }
            walk_fragment(&block.consequent, component, data, each_vars);
            if let Some(alt) = &block.alternate {
                walk_fragment(alt, component, data, each_vars);
            }
        }
        Node::EachBlock(block) => {
            if expr_is_dynamic(&block.id, data, each_vars) {
                data.dynamic_nodes.insert(block.id);
            }
            // Each context and index variables are dynamic inside the body
            let context_name = component.source_text(block.context_span).to_string();
            each_vars.push(context_name);
            let index_name = block.index_span.map(|s| component.source_text(s).to_string());
            if let Some(ref idx) = index_name {
                each_vars.push(idx.clone());
            }
            walk_fragment(&block.body, component, data, each_vars);
            if index_name.is_some() {
                each_vars.pop();
            }
            each_vars.pop();
            if let Some(fb) = &block.fallback {
                walk_fragment(fb, component, data, each_vars);
            }
        }
        Node::Text(_) | Node::Comment(_) => {}
    }
}

fn expr_is_dynamic(node_id: &NodeId, data: &AnalysisData, each_vars: &[String]) -> bool {
    if let Some(info) = data.expressions.get(node_id) {
        return info.references.iter().any(|r| {
            data.symbol_by_name
                .get(&r.name)
                .is_some_and(|id| data.runes.contains_key(id))
                || each_vars.contains(&r.name)
        });
    }
    false
}

fn attr_is_dynamic(
    attr: &Attribute,
    attr_idx: usize,
    el_id: NodeId,
    data: &AnalysisData,
    each_vars: &[String],
) -> bool {
    if matches!(
        attr,
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
    ) {
        return false;
    }
    if let Some(info) = data.attr_expressions.get(&(el_id, attr_idx)) {
        return info.references.iter().any(|r| {
            data.symbol_by_name
                .get(&r.name)
                .is_some_and(|id| data.runes.contains_key(id))
                || each_vars.contains(&r.name)
        });
    }
    false
}
