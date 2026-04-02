use svelte_ast::{Attribute, Element};

use crate::types::data::{
    AnalysisData, ContentStrategy, FragmentItem, FragmentKey, LoweredTextPart,
};
use crate::walker::TemplateVisitor;

/// Visitor that computes content_type + has_dynamic_children + needs_var
/// for element fragments during the main composite walk.
///
/// Must be LAST in the composite tuple: reads `dynamic_nodes` and `needs_ref`
/// populated by ReactivityVisitor earlier in the same walk.
pub(crate) struct ContentAndVarVisitor<'s> {
    pub source: &'s str,
}

impl TemplateVisitor for ContentAndVarVisitor<'_> {
    fn leave_element(&mut self, el: &Element, ctx: &mut crate::walker::VisitContext<'_>) {
        let key = FragmentKey::Element(el.id);

        // Compute content_type + has_dynamic for this element's fragment
        if let Some(lf) = ctx.data.fragments.lowered.get(&key) {
            let cs = classify_items(&lf.items, self.source);
            let has_dynamic = lf
                .items
                .iter()
                .any(|item| item_is_dynamic(item, &ctx.data.dynamic_nodes));
            ctx.data.fragments.content_types.insert(key, cs);
            if has_dynamic {
                ctx.data.fragments.has_dynamic_children.insert(key);
            }
        }

        // Compute needs_var (same logic as former NeedsVarVisitor)
        if element_needs_var(el, ctx.data) {
            ctx.data.element_flags.needs_var.insert(el.id);
        }
    }
}

/// Classify non-element fragments after the composite walk completes.
/// Element fragments are already classified by ContentAndVarVisitor::leave_element.
pub fn classify_remaining_fragments(data: &mut AnalysisData, source: &str) {
    let keys: Vec<_> = data
        .fragments
        .lowered
        .keys()
        .filter(|k| !matches!(k, FragmentKey::Element(_)))
        .copied()
        .collect();
    for key in keys {
        let lf = &data.fragments.lowered[&key];
        let cs = classify_items(&lf.items, source);
        let has_dynamic = lf
            .items
            .iter()
            .any(|item| item_is_dynamic(item, &data.dynamic_nodes));
        data.fragments.content_types.insert(key, cs);
        if has_dynamic {
            data.fragments.has_dynamic_children.insert(key);
        }
    }
}

fn element_needs_var(el: &Element, data: &AnalysisData) -> bool {
    let id = el.id;

    if data.element_flags.needs_ref.contains(&id) {
        return true;
    }

    let has_runtime_attrs = el.attributes.iter().any(|a| {
        !matches!(
            a,
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_)
        )
    });
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
        ContentStrategy::DynamicText => true,
        ContentStrategy::SingleBlock(_) => true,
        ContentStrategy::SingleElement(_) | ContentStrategy::Mixed { .. } => {
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
            // Bottom-up: child elements' needs_var already computed via leave_element
            data.element_flags.needs_var.contains(id)
        }
        FragmentItem::ComponentNode(_)
        | FragmentItem::IfBlock(_)
        | FragmentItem::EachBlock(_)
        | FragmentItem::RenderTag(_)
        | FragmentItem::HtmlTag(_)
        | FragmentItem::KeyBlock(_)
        | FragmentItem::SvelteElement(_)
        | FragmentItem::SvelteBoundary(_)
        | FragmentItem::AwaitBlock(_) => true,
    }
}

fn item_is_dynamic(
    item: &FragmentItem,
    dynamic_nodes: &crate::types::node_table::NodeBitSet,
) -> bool {
    match item {
        FragmentItem::TextConcat { parts, .. } => parts
            .iter()
            .any(|p| matches!(p, LoweredTextPart::Expr(id) if dynamic_nodes.contains(id))),
        FragmentItem::Element(id)
        | FragmentItem::ComponentNode(id)
        | FragmentItem::IfBlock(id)
        | FragmentItem::EachBlock(id)
        | FragmentItem::RenderTag(id)
        | FragmentItem::HtmlTag(id)
        | FragmentItem::KeyBlock(id)
        | FragmentItem::SvelteElement(id)
        | FragmentItem::SvelteBoundary(id)
        | FragmentItem::AwaitBlock(id) => dynamic_nodes.contains(id),
    }
}

fn classify_items(items: &[FragmentItem], source: &str) -> ContentStrategy {
    if items.is_empty() {
        return ContentStrategy::Empty;
    }

    if items.len() == 1 {
        match &items[0] {
            FragmentItem::Element(id) => return ContentStrategy::SingleElement(*id),
            FragmentItem::TextConcat { .. } => {}
            item => return ContentStrategy::SingleBlock(item.clone()),
        }
    }

    let mut has_element = false;
    let mut has_block = false;
    let mut has_static_text = false;
    let mut has_dynamic_text = false;

    for item in items {
        match item {
            FragmentItem::Element(_) => has_element = true,
            FragmentItem::ComponentNode(_)
            | FragmentItem::IfBlock(_)
            | FragmentItem::EachBlock(_)
            | FragmentItem::RenderTag(_)
            | FragmentItem::HtmlTag(_)
            | FragmentItem::KeyBlock(_)
            | FragmentItem::SvelteElement(_)
            | FragmentItem::SvelteBoundary(_)
            | FragmentItem::AwaitBlock(_) => has_block = true,
            FragmentItem::TextConcat { has_expr, .. } => {
                if *has_expr {
                    has_dynamic_text = true;
                } else {
                    has_static_text = true;
                }
            }
        }
    }

    if has_element || has_block {
        let has_text = has_static_text || has_dynamic_text;
        return ContentStrategy::Mixed {
            has_elements: has_element,
            has_blocks: has_block,
            has_text,
        };
    }

    if has_dynamic_text {
        ContentStrategy::DynamicText
    } else if has_static_text {
        let text = if let FragmentItem::TextConcat { parts, .. } = &items[0] {
            parts
                .iter()
                .map(|p| match p {
                    LoweredTextPart::TextSpan(span) => span.source_text(&source),
                    LoweredTextPart::TextOwned(t) => t.as_str(),
                    LoweredTextPart::Expr(_) => "",
                })
                .collect::<String>()
        } else {
            String::new()
        };
        ContentStrategy::Static(text)
    } else {
        ContentStrategy::Empty
    }
}
