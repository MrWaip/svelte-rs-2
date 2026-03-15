use crate::data::{AnalysisData, ConcatPart, ContentType, FragmentItem};

/// Classify all fragments and mark which ones have dynamic children.
/// Single pass over `lowered_fragments` instead of two separate traversals.
pub fn classify_and_mark_dynamic(data: &mut AnalysisData) {
    let dynamic_nodes = &data.dynamic_nodes;
    let results: Vec<_> = data
        .lowered_fragments
        .iter()
        .map(|(&key, lf)| {
            let ct = classify_items(&lf.items);
            let has_dynamic = lf.items.iter().any(|item| item_is_dynamic(item, dynamic_nodes));
            (key, ct, has_dynamic)
        })
        .collect();

    for (key, ct, has_dynamic) in results {
        data.content_types.insert(key, ct);
        if has_dynamic {
            data.fragment_has_dynamic_children.insert(key);
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
        | FragmentItem::KeyBlock(id) => dynamic_nodes.contains(id),
    }
}

fn classify_items(items: &[FragmentItem]) -> ContentType {
    if items.is_empty() {
        return ContentType::Empty;
    }

    if items.len() == 1 {
        match &items[0] {
            FragmentItem::Element(_) => return ContentType::SingleElement,
            FragmentItem::ComponentNode(_)
            | FragmentItem::IfBlock(_)
            | FragmentItem::EachBlock(_)
            | FragmentItem::RenderTag(_)
            | FragmentItem::HtmlTag(_)
            | FragmentItem::KeyBlock(_) => return ContentType::SingleBlock,
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
            | FragmentItem::KeyBlock(_) => has_block = true,
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
        return ContentType::Mixed;
    }

    if has_dynamic_text {
        ContentType::DynamicText
    } else if has_static_text {
        ContentType::StaticText
    } else {
        ContentType::Empty
    }
}
