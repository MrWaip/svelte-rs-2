use svelte_ast::{Component, Fragment, Node};

use crate::data::{ContentType, FragmentItem, FragmentKey, AnalysisData};

pub fn classify_content(component: &Component, data: &mut AnalysisData) {
    classify_by_key(FragmentKey::Root, data);
    // Classify snippet bodies
    for node in &component.fragment.nodes {
        if let Node::SnippetBlock(block) = node {
            classify_by_key(FragmentKey::SnippetBody(block.id), data);
            descend_fragment(&block.body, data);
        }
    }
    descend_fragment(&component.fragment, data);
}

fn descend_fragment(fragment: &Fragment, data: &mut AnalysisData) {
    for node in &fragment.nodes {
        match node {
            Node::Element(el) => {
                classify_by_key(FragmentKey::Element(el.id), data);
                descend_fragment(&el.fragment, data);
            }
            Node::ComponentNode(cn) => {
                classify_by_key(FragmentKey::ComponentNode(cn.id), data);
                descend_fragment(&cn.fragment, data);
            }
            Node::IfBlock(block) => {
                classify_by_key(FragmentKey::IfConsequent(block.id), data);
                classify_by_key(FragmentKey::IfAlternate(block.id), data);
                descend_fragment(&block.consequent, data);
                if let Some(alt) = &block.alternate {
                    descend_fragment(alt, data);
                }
            }
            Node::EachBlock(block) => {
                classify_by_key(FragmentKey::EachBody(block.id), data);
                if block.fallback.is_some() {
                    classify_by_key(FragmentKey::EachFallback(block.id), data);
                }
                descend_fragment(&block.body, data);
                if let Some(fb) = &block.fallback {
                    descend_fragment(fb, data);
                }
            }
            Node::SnippetBlock(block) => {
                descend_fragment(&block.body, data);
            }
            Node::Text(_) | Node::ExpressionTag(_) | Node::Comment(_) | Node::RenderTag(_) => {}
        }
    }
}

fn classify_by_key(key: FragmentKey, data: &mut AnalysisData) {
    let content_type = match data.lowered_fragments.get(&key) {
        Some(lf) => classify_items(&lf.items),
        None => ContentType::Empty,
    };
    data.content_types.insert(key, content_type);
}

fn classify_items(items: &[FragmentItem]) -> ContentType {
    if items.is_empty() {
        return ContentType::Empty;
    }

    if items.len() == 1 {
        match &items[0] {
            FragmentItem::Element(_) => return ContentType::SingleElement,
            FragmentItem::ComponentNode(_) | FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) | FragmentItem::RenderTag(_) => return ContentType::SingleBlock,
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
            FragmentItem::ComponentNode(_) | FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) | FragmentItem::RenderTag(_) => has_block = true,
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
