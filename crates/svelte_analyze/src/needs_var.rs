//! Compute which elements need a DOM variable during traversal.
//!
//! Runs after: reactivity, content_types, lower.

use svelte_ast::{Attribute, Component, Element, Fragment, Node};

use crate::data::{AnalysisData, ConcatPart, ContentType, FragmentItem, FragmentKey};

/// Populate `data.elements_needing_var` for all elements in the component.
pub fn compute_elements_needing_var(component: &Component, data: &mut AnalysisData) {
    walk_fragment(&component.fragment, data);
}

fn walk_fragment(fragment: &Fragment, data: &mut AnalysisData) {
    for node in &fragment.nodes {
        match node {
            Node::Element(el) => {
                // Walk children first so nested elements are already computed
                walk_fragment(&el.fragment, data);
                if element_needs_var(el, data) {
                    data.elements_needing_var.insert(el.id);
                }
            }
            Node::IfBlock(b) => {
                walk_fragment(&b.consequent, data);
                if let Some(alt) = &b.alternate {
                    walk_fragment(alt, data);
                }
            }
            Node::EachBlock(b) => {
                walk_fragment(&b.body, data);
                if let Some(fb) = &b.fallback {
                    walk_fragment(fb, data);
                }
            }
            Node::SnippetBlock(b) => {
                walk_fragment(&b.body, data);
            }
            _ => {}
        }
    }
}

fn element_needs_var(el: &Element, data: &AnalysisData) -> bool {
    let id = el.id;

    if data.node_needs_ref.contains(&id) {
        return true;
    }

    let has_runtime_attrs = el
        .attributes
        .iter()
        .any(|a| !matches!(a, Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)));
    if has_runtime_attrs {
        return true;
    }

    let key = FragmentKey::Element(id);
    let ct = data
        .content_types
        .get(&key)
        .copied()
        .unwrap_or(ContentType::Empty);
    match ct {
        ContentType::Empty | ContentType::StaticText => false,
        ContentType::DynamicText | ContentType::SingleBlock => true,
        ContentType::SingleElement | ContentType::Mixed => {
            let Some(lf) = data.lowered_fragments.get(&key) else {
                return false;
            };
            lf.items.iter().any(|item| item_needs_var(item, data))
        }
    }
}

fn item_needs_var(item: &FragmentItem, data: &AnalysisData) -> bool {
    match item {
        FragmentItem::TextConcat { parts } => {
            parts.iter().any(|p| matches!(p, ConcatPart::Expr(_)))
        }
        FragmentItem::Element(id) => {
            // Already computed: children are walked before parents
            data.elements_needing_var.contains(id)
        }
        FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) | FragmentItem::RenderTag(_) => true,
    }
}
