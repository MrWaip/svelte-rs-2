use svelte_ast::{Component, Fragment, Node};

use crate::data::{ConcatPart, FragmentItem, FragmentKey, LoweredFragment, AnalysisData};

pub fn lower(component: &Component, data: &mut AnalysisData) {
    lower_fragment(&component.fragment, FragmentKey::Root, component, data);
}

fn lower_fragment(
    fragment: &Fragment,
    key: FragmentKey,
    component: &Component,
    data: &mut AnalysisData,
) {
    let items = build_items(fragment, component);
    data.lowered_fragments.insert(key, LoweredFragment { items });

    for node in &fragment.nodes {
        match node {
            Node::Element(el) => {
                lower_fragment(&el.fragment, FragmentKey::Element(el.id), component, data);
            }
            Node::ComponentNode(cn) => {
                lower_fragment(&cn.fragment, FragmentKey::ComponentNode(cn.id), component, data);
            }
            Node::IfBlock(block) => {
                lower_fragment(&block.consequent, FragmentKey::IfConsequent(block.id), component, data);
                if let Some(alt) = &block.alternate {
                    lower_fragment(alt, FragmentKey::IfAlternate(block.id), component, data);
                }
            }
            Node::EachBlock(block) => {
                lower_fragment(&block.body, FragmentKey::EachBody(block.id), component, data);
                if let Some(fb) = &block.fallback {
                    lower_fragment(fb, FragmentKey::EachFallback(block.id), component, data);
                }
            }
            Node::SnippetBlock(block) => {
                lower_fragment(&block.body, FragmentKey::SnippetBody(block.id), component, data);
            }
            Node::Text(_) | Node::Comment(_) | Node::ExpressionTag(_) | Node::RenderTag(_) => {}
        }
    }
}

/// Build the lowered item list for a fragment:
/// - Skip whitespace-only text at the leading and trailing boundaries
/// - Group consecutive Text + ExpressionTag into TextConcat
/// - Elements and blocks become standalone items
/// - Comments are dropped
fn build_items(fragment: &Fragment, component: &Component) -> Vec<FragmentItem> {
    let nodes = &fragment.nodes;

    // Find the slice bounds: trim whitespace-only text at boundaries
    let start = nodes.iter().position(|n| !is_removable_text(n, component)).unwrap_or(nodes.len());
    let end = nodes
        .iter()
        .rposition(|n| !is_removable_text(n, component))
        .map(|i| i + 1)
        .unwrap_or(0);

    if start >= end {
        return vec![];
    }

    let significant = &nodes[start..end];
    let mut items: Vec<FragmentItem> = Vec::new();
    let mut concat: Vec<ConcatPart> = Vec::new();

    let flush = |concat: &mut Vec<ConcatPart>, items: &mut Vec<FragmentItem>| {
        if !concat.is_empty() {
            let parts = std::mem::take(concat);
            items.push(FragmentItem::TextConcat { parts });
        }
    };

    for (i, node) in significant.iter().enumerate() {
        match node {
            Node::Text(text) => {
                let raw = text.value(&component.source);
                let content = trim_text(raw, i, significant, component);
                if !content.is_empty() {
                    // Collapse consecutive whitespace-only text parts
                    // (happens when comments between elements are dropped)
                    if content.trim().is_empty() {
                        if let Some(ConcatPart::Text(prev)) = concat.last() {
                            if prev.trim().is_empty() {
                                continue;
                            }
                        }
                    }
                    concat.push(ConcatPart::Text(content));
                }
            }
            Node::ExpressionTag(tag) => {
                concat.push(ConcatPart::Expr(tag.id));
            }
            Node::Element(el) => {
                flush(&mut concat, &mut items);
                items.push(FragmentItem::Element(el.id));
            }
            Node::ComponentNode(cn) => {
                flush(&mut concat, &mut items);
                items.push(FragmentItem::ComponentNode(cn.id));
            }
            Node::IfBlock(block) => {
                flush(&mut concat, &mut items);
                items.push(FragmentItem::IfBlock(block.id));
            }
            Node::EachBlock(block) => {
                flush(&mut concat, &mut items);
                items.push(FragmentItem::EachBlock(block.id));
            }
            Node::SnippetBlock(_) => {
                // Snippets don't produce DOM at declaration site
            }
            Node::RenderTag(tag) => {
                flush(&mut concat, &mut items);
                items.push(FragmentItem::RenderTag(tag.id));
            }
            Node::Comment(_) => {
                // Drop comments
            }
        }
    }

    flush(&mut concat, &mut items);
    items
}

fn is_removable_text(node: &Node, component: &Component) -> bool {
    match node {
        Node::Text(text) => text.value(&component.source).trim().is_empty(),
        // SnippetBlock produces no DOM at declaration site
        Node::SnippetBlock(_) => true,
        _ => false,
    }
}

/// Trim a text node's content based on its neighbours.
///
/// Rules:
/// - Collapse internal runs of whitespace to a single space.
/// - For pure-whitespace content: keep a single " " if either neighbour is an
///   ExpressionTag; drop it otherwise.
/// - For mixed content (non-whitespace characters present): preserve a leading
///   space when the original starts with whitespace, and a trailing space when
///   the original ends with whitespace.
fn trim_text(raw: &str, idx: usize, siblings: &[Node], _component: &Component) -> String {
    let prev_exists = prev_significant(idx, siblings).is_some();
    let next_exists = next_significant(idx, siblings).is_some();

    let starts_ws = raw.starts_with(|c: char| c.is_ascii_whitespace());
    let ends_ws = raw.ends_with(|c: char| c.is_ascii_whitespace());

    // Collapse internal whitespace sequences.
    let inner: String = raw.split_whitespace().collect::<Vec<_>>().join(" ");

    if inner.is_empty() {
        // Pure-whitespace node: keep a separator space between two significant siblings.
        if prev_exists && next_exists {
            return " ".to_string();
        }
        return String::new();
    }

    let mut s = String::new();
    if starts_ws && prev_exists {
        s.push(' ');
    }
    s.push_str(&inner);
    if ends_ws && next_exists {
        s.push(' ');
    }
    s
}

fn is_insignificant(n: &Node) -> bool {
    n.is_comment() || n.is_snippet_block()
}

fn prev_significant(idx: usize, siblings: &[Node]) -> Option<&Node> {
    siblings[..idx].iter().rev().find(|n| !is_insignificant(n))
}

fn next_significant(idx: usize, siblings: &[Node]) -> Option<&Node> {
    siblings[idx + 1..].iter().find(|n| !is_insignificant(n))
}
