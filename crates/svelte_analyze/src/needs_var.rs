//! Compute which elements need a DOM variable during traversal.
//!
//! Uses `leave_element` for bottom-up order: children are processed before parents,
//! so `elements_needing_var` for nested elements is ready when checking outer ones.
//!
//! Runs after: reactivity, content_types, lower.

use oxc_semantic::ScopeId;
use svelte_ast::{Attribute, Element};

use crate::data::{AnalysisData, ContentStrategy, FragmentItem, FragmentKey};
use crate::walker::TemplateVisitor;

pub(crate) struct NeedsVarVisitor;

impl TemplateVisitor for NeedsVarVisitor {
    fn leave_element(&mut self, el: &Element, _scope: ScopeId, data: &mut AnalysisData) {
        if element_needs_var(el, data) {
            data.element_flags.needs_var.insert(el.id);
        }
    }
}

fn element_needs_var(el: &Element, data: &AnalysisData) -> bool {
    let id = el.id;

    if data.element_flags.needs_ref.contains(&id) {
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
        .fragments
        .content_types
        .get(&key)
        .cloned()
        .unwrap_or(ContentStrategy::Empty);
    match ct {
        ContentStrategy::Empty | ContentStrategy::Static(_) => false,
        ContentStrategy::Dynamic { has_elements: false, has_blocks: false, .. } => true, // DynamicText
        ContentStrategy::SingleBlock(_) => true,
        ContentStrategy::SingleElement(_) | ContentStrategy::Dynamic { .. } => {
            let Some(lf) = data.fragments.lowered.get(&key) else {
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
            data.element_flags.needs_var.contains(id)
        }
        FragmentItem::ComponentNode(_) | FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) | FragmentItem::RenderTag(_) | FragmentItem::HtmlTag(_) | FragmentItem::KeyBlock(_) | FragmentItem::SvelteElement(_) | FragmentItem::SvelteBoundary(_) | FragmentItem::AwaitBlock(_) | FragmentItem::TitleElement(_) => true,
    }
}
