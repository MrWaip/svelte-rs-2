use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node};

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
                if attr_references_rune(attr, component, data) {
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

fn expr_references_rune(node_id: &svelte_ast::NodeId, data: &AnalysisData) -> bool {
    if let Some(info) = data.expressions.get(node_id) {
        return info
            .references
            .iter()
            .any(|r| data.symbol_by_name.get(&r.name).is_some_and(|&idx| data.runes.contains_key(&idx)));
    }
    false
}

fn attr_references_rune(attr: &Attribute, component: &Component, data: &AnalysisData) -> bool {
    match attr {
        Attribute::ExpressionAttribute(a) => {
            let source = component.source_text(a.expression_span);
            let offset = a.expression_span.start as u32;
            analyze_refs(source, offset, data)
        }
        Attribute::ConcatenationAttribute(a) => {
            for part in &a.parts {
                if let ConcatPart::Dynamic(span) = part {
                    let source = component.source_text(*span);
                    let offset = span.start as u32;
                    if analyze_refs(source, offset, data) {
                        return true;
                    }
                }
            }
            false
        }
        Attribute::ClassDirective(a) => {
            if let Some(span) = a.expression_span {
                let source = component.source_text(span);
                let offset = span.start as u32;
                return analyze_refs(source, offset, data);
            }
            false
        }
        Attribute::BindDirective(a) => {
            if let Some(span) = a.expression_span {
                let source = component.source_text(span);
                let offset = span.start as u32;
                return analyze_refs(source, offset, data);
            }
            false
        }
        Attribute::ShorthandOrSpread(a) => {
            let source = component.source_text(a.expression_span);
            let offset = a.expression_span.start as u32;
            analyze_refs(source, offset, data)
        }
        Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => false,
    }
}

fn analyze_refs(source: &str, offset: u32, data: &AnalysisData) -> bool {
    let Ok(info) = svelte_js::analyze_expression(source, offset) else {
        return false;
    };
    info.references
        .iter()
        .any(|r| data.symbol_by_name.get(&r.name).is_some_and(|&idx| data.runes.contains_key(&idx)))
}
