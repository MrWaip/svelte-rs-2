use svelte_ast::{Component, Fragment, Node, NodeId};

use crate::data::{
    alternate_id, ContentType, FragmentItem, AnalysisData, ROOT_FRAGMENT_ID,
};

pub fn classify_content(component: &Component, data: &mut AnalysisData) {
    classify_by_id(ROOT_FRAGMENT_ID, data);
    descend_fragment(&component.fragment, data);
}

fn descend_fragment(fragment: &Fragment, data: &mut AnalysisData) {
    for node in &fragment.nodes {
        match node {
            Node::Element(el) => {
                classify_by_id(el.id, data);
                descend_fragment(&el.fragment, data);
            }
            Node::IfBlock(block) => {
                classify_by_id(block.id, data);
                classify_by_id(alternate_id(block.id), data);
                descend_fragment(&block.consequent, data);
                if let Some(alt) = &block.alternate {
                    descend_fragment(alt, data);
                }
            }
            Node::EachBlock(block) => {
                classify_by_id(block.id, data);
                descend_fragment(&block.body, data);
            }
            Node::Text(_) | Node::ExpressionTag(_) | Node::Comment(_) => {}
        }
    }
}

fn classify_by_id(owner_id: NodeId, data: &mut AnalysisData) {
    let content_type = match data.lowered_fragments.get(&owner_id) {
        Some(lf) => classify_items(&lf.items),
        None => ContentType::Empty,
    };
    data.content_types.insert(owner_id, content_type);
}

fn classify_items(items: &[FragmentItem]) -> ContentType {
    if items.is_empty() {
        return ContentType::Empty;
    }

    // Check: exactly one Element and nothing else → SingleElement
    if items.len() == 1 {
        if let FragmentItem::Element(_) = &items[0] {
            return ContentType::SingleElement;
        }
    }

    let mut has_element = false;
    let mut has_block = false;
    let mut has_static_text = false;
    let mut has_dynamic_text = false;

    for item in items {
        match item {
            FragmentItem::Element(_) => has_element = true,
            FragmentItem::Block(_) => has_block = true,
            FragmentItem::TextConcat { parts } => {
                let has_expr = parts.iter().any(|p| matches!(p, crate::data::ConcatPart::Expr(_)));
                if has_expr {
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
