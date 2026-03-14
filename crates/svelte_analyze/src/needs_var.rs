//! Compute which elements need a DOM variable during traversal.
//!
//! Uses `leave_element` for bottom-up order: children are processed before parents,
//! so `elements_needing_var` for nested elements is ready when checking outer ones.
//!
//! Runs after: reactivity, content_types, lower.

use oxc_semantic::ScopeId;
use svelte_ast::{Attribute, Element};

use crate::data::{AnalysisData, ContentType, FragmentItem, FragmentKey};
use crate::walker::TemplateVisitor;

pub(crate) struct NeedsVarVisitor;

impl TemplateVisitor for NeedsVarVisitor {
    fn leave_element(&mut self, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        if element_needs_var(el, data) {
            data.elements_needing_var.insert(el.id);
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
        FragmentItem::TextConcat { has_expr, .. } => *has_expr,
        FragmentItem::Element(id) => {
            // Already computed: leave_element processes children before parents
            data.elements_needing_var.contains(id)
        }
        FragmentItem::ComponentNode(_) | FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) | FragmentItem::RenderTag(_) => true,
    }
}
