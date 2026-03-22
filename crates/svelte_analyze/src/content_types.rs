use crate::data::{AnalysisData, LoweredTextPart, ContentStrategy, FragmentItem};

/// Classify all fragments and mark which ones have dynamic children.
pub fn classify_and_mark_dynamic(data: &mut AnalysisData) {
    let dynamic_nodes = &data.dynamic_nodes;
    let keys: Vec<_> = data.fragments.lowered.keys().copied().collect();
    for key in keys {
        let lf = &data.fragments.lowered[&key];
        let cs = classify_items(&lf.items);
        let has_dynamic = lf.items.iter().any(|item| item_is_dynamic(item, dynamic_nodes));
        data.fragments.content_types.insert(key, cs);
        if has_dynamic {
            data.fragments.has_dynamic_children.insert(key);
        }
    }
}

fn item_is_dynamic(
    item: &FragmentItem,
    dynamic_nodes: &crate::node_table::NodeBitSet,
) -> bool {
    match item {
        FragmentItem::TextConcat { parts, .. } => parts.iter().any(|p| {
            matches!(p, LoweredTextPart::Expr(id) if dynamic_nodes.contains(id))
        }),
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

fn classify_items(items: &[FragmentItem]) -> ContentStrategy {
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
        return ContentStrategy::Mixed { has_elements: has_element, has_blocks: has_block, has_text };
    }

    if has_dynamic_text {
        ContentStrategy::DynamicText
    } else if has_static_text {
        // Extract text from the single TextConcat item (all parts are static)
        let text = if let FragmentItem::TextConcat { parts, .. } = &items[0] {
            parts.iter().map(|p| match p {
                LoweredTextPart::Text(t) => t.as_str(),
                LoweredTextPart::Expr(_) => "", // unreachable for static text
            }).collect::<String>()
        } else {
            String::new()
        };
        ContentStrategy::Static(text)
    } else {
        ContentStrategy::Empty
    }
}
