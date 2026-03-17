use crate::data::{AnalysisData, ConcatPart, ContentStrategy, FragmentItem};

/// Classify all fragments and mark which ones have dynamic children.
/// Single pass over `lowered_fragments` instead of two separate traversals.
pub fn classify_and_mark_dynamic(data: &mut AnalysisData) {
    let dynamic_nodes = &data.dynamic_nodes;
    let results: Vec<_> = data
        .fragments
        .lowered
        .iter()
        .map(|(&key, lf)| {
            let cs = classify_items(&lf.items);
            let has_dynamic = lf.items.iter().any(|item| item_is_dynamic(item, dynamic_nodes));
            (key, cs, has_dynamic)
        })
        .collect();

    for (key, cs, has_dynamic) in results {
        data.fragments.content_types.insert(key, cs);
        if has_dynamic {
            data.fragments.has_dynamic_children.insert(key);
        }
    }
}

fn item_is_dynamic(
    item: &FragmentItem,
    dynamic_nodes: &rustc_hash::FxHashSet<svelte_ast::NodeId>,
) -> bool {
    match item {
        FragmentItem::TextConcat { parts, .. } => parts.iter().any(|p| {
            matches!(p, ConcatPart::Expr(id) if dynamic_nodes.contains(id))
        }),
        FragmentItem::Element(id)
        | FragmentItem::ComponentNode(id)
        | FragmentItem::IfBlock(id)
        | FragmentItem::EachBlock(id)
        | FragmentItem::RenderTag(id)
        | FragmentItem::HtmlTag(id)
        | FragmentItem::KeyBlock(id)
        | FragmentItem::SvelteElement(id) => dynamic_nodes.contains(id),
    }
}

fn classify_items(items: &[FragmentItem]) -> ContentStrategy {
    if items.is_empty() {
        return ContentStrategy::Empty;
    }

    if items.len() == 1 {
        match &items[0] {
            FragmentItem::Element(id) => return ContentStrategy::SingleElement(*id),
            FragmentItem::ComponentNode(id)
            | FragmentItem::IfBlock(id)
            | FragmentItem::EachBlock(id)
            | FragmentItem::RenderTag(id)
            | FragmentItem::HtmlTag(id)
            | FragmentItem::KeyBlock(id)
            | FragmentItem::SvelteElement(id) => return ContentStrategy::SingleBlock(*id),
            FragmentItem::TextConcat { .. } => {}
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
            | FragmentItem::SvelteElement(_) => has_block = true,
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
        return ContentStrategy::Dynamic { has_elements: has_element, has_blocks: has_block, has_text };
    }

    if has_dynamic_text {
        ContentStrategy::Dynamic { has_elements: false, has_blocks: false, has_text: true }
    } else if has_static_text {
        // Extract text from the single TextConcat item (all parts are static)
        let text = if let FragmentItem::TextConcat { parts, .. } = &items[0] {
            parts.iter().map(|p| match p {
                ConcatPart::Text(t) => t.as_str(),
                ConcatPart::Expr(_) => "", // unreachable for static text
            }).collect::<String>()
        } else {
            String::new()
        };
        ContentStrategy::Static(text)
    } else {
        ContentStrategy::Empty
    }
}
