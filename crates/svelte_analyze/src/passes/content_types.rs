use svelte_ast::{AstStore, Attribute, Element, Node, NodeId};

use crate::types::data::{AnalysisData, FragmentKey};
use crate::walker::TemplateVisitor;

/// Visitor that computes `needs_var` for element fragments during the main
/// composite walk.
///
/// Must be LAST in the composite tuple: reads `dynamic_nodes` and `needs_ref`
/// populated by ReactivityVisitor earlier in the same walk.
pub(crate) struct ContentAndVarVisitor;

impl TemplateVisitor for ContentAndVarVisitor {
    fn leave_element(&mut self, el: &Element, ctx: &mut crate::walker::VisitContext<'_, '_>) {
        if element_needs_var(el, ctx.data, ctx.store) {
            ctx.data.elements.flags.needs_var.insert(el.id);
        }
    }
}

fn element_needs_var(el: &Element, data: &AnalysisData, store: &AstStore) -> bool {
    let id = el.id;

    if data.elements.flags.needs_ref.contains(&id) {
        return true;
    }

    if data.elements.flags.is_customizable_select(id) {
        return true;
    }

    if data.elements.flags.is_selectedcontent(id) {
        return true;
    }

    if data.has_runtime_attrs(id) {
        return true;
    }

    if el.name == "option"
        && el
            .attributes
            .iter()
            .any(|a| matches!(a, Attribute::StringAttribute(s) if s.name == "value"))
    {
        return true;
    }

    let Some(lf) = data.template.lowered_fragment(&FragmentKey::Element(id)) else {
        return false;
    };
    lf.iter()
        .any(|&item_id| item_needs_var(item_id, data, store))
}

fn item_needs_var(id: NodeId, data: &AnalysisData, store: &AstStore) -> bool {
    match store.get(id) {
        Node::Text(_) => false,
        Node::ExpressionTag(_) => true,
        Node::Element(_) => data.elements.flags.needs_var.contains(&id),
        Node::SlotElementLegacy(_) => true,
        Node::SvelteFragmentLegacy(_) => data
            .template
            .lowered_fragment(&FragmentKey::Element(id))
            .is_some_and(|fragment| {
                fragment
                    .iter()
                    .any(|&inner| item_needs_var(inner, data, store))
            }),
        Node::ComponentNode(_)
        | Node::IfBlock(_)
        | Node::EachBlock(_)
        | Node::RenderTag(_)
        | Node::HtmlTag(_)
        | Node::KeyBlock(_)
        | Node::SvelteElement(_)
        | Node::SvelteBoundary(_)
        | Node::AwaitBlock(_) => true,
        _ => false,
    }
}
