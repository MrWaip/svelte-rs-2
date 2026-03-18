use std::borrow::Cow;

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
    // Collect ConstTag node IDs for this fragment
    let const_ids: Vec<_> = fragment.nodes.iter()
        .filter_map(|n| n.as_const_tag().map(|ct| ct.id))
        .collect();
    if !const_ids.is_empty() {
        data.const_tags.by_fragment.insert(key, const_ids);
    }

    let items = build_items(fragment, component);
    data.fragments.lowered.insert(key, LoweredFragment { items });

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
            Node::KeyBlock(block) => {
                lower_fragment(&block.fragment, FragmentKey::KeyBlockBody(block.id), component, data);
            }
            Node::SvelteHead(head) => {
                lower_fragment(&head.fragment, FragmentKey::SvelteHeadBody(head.id), component, data);
            }
            Node::SvelteElement(el) => {
                lower_fragment(&el.fragment, FragmentKey::SvelteElementBody(el.id), component, data);
            }
            Node::SvelteWindow(_) | Node::SvelteDocument(_) => {}
            Node::Text(_) | Node::Comment(_) | Node::ExpressionTag(_) | Node::RenderTag(_) | Node::HtmlTag(_) | Node::ConstTag(_) | Node::Error(_) => {}
        }
    }
}

/// Build the lowered item list for a fragment.
///
/// Follows the Svelte reference compiler's `clean_nodes` algorithm:
/// 1. Filter out comments and hoisted nodes (SnippetBlock)
/// 2. Strip leading whitespace-only Text nodes, then trim leading ws from first Text
/// 3. Strip trailing whitespace-only Text nodes, then trim trailing ws from last Text
/// 4. For internal Text nodes: collapse boundary whitespace to single space,
///    but preserve whitespace adjacent to ExpressionTag
/// 5. Group consecutive Text + ExpressionTag into TextConcat
fn build_items(fragment: &Fragment, component: &Component) -> Vec<FragmentItem> {
    // Step 1: collect regular nodes (skip comments and snippets)
    let mut regular: Vec<&Node> = Vec::new();
    for node in &fragment.nodes {
        match node {
            Node::Comment(_) | Node::SnippetBlock(_) | Node::ConstTag(_) | Node::SvelteHead(_) | Node::SvelteWindow(_) | Node::Error(_) => continue,
            _ => regular.push(node),
        }
    }

    // Step 2: strip leading whitespace-only Text nodes (index-based to avoid O(n) shifts)
    let mut start = 0;
    while let Some(Node::Text(t)) = regular.get(start) {
        if !is_ws_only(t.value(&component.source)) {
            break;
        }
        start += 1;
    }

    // Step 3: strip trailing whitespace-only Text nodes
    let mut end = regular.len();
    while end > start {
        if let Some(Node::Text(t)) = regular.get(end - 1) {
            if is_ws_only(t.value(&component.source)) {
                end -= 1;
                continue;
            }
        }
        break;
    }

    let regular = &regular[start..end];

    if regular.is_empty() {
        return vec![];
    }

    // Sequential trimming pass + grouping into FragmentItems.
    // Process nodes in order, tracking whether the previous trimmed text
    // ends with whitespace to avoid double spaces.
    let len = regular.len();
    let mut items: Vec<FragmentItem> = Vec::new();
    let mut concat: Vec<ConcatPart> = Vec::new();
    let mut prev_text_ends_ws = false;

    let flush = |concat: &mut Vec<ConcatPart>, items: &mut Vec<FragmentItem>| {
        if !concat.is_empty() {
            let parts = std::mem::take(concat);
            let has_expr = parts.iter().any(|p| matches!(p, ConcatPart::Expr(_)));
            items.push(FragmentItem::TextConcat { parts, has_expr });
        }
    };

    for i in 0..len {
        match regular[i] {
            Node::Text(text) => {
                let raw = text.value(&component.source);
                let is_first = i == 0;
                let is_last = i == len - 1;
                let prev = i.checked_sub(1).map(|j| regular[j]);
                let next = regular.get(i + 1).copied();

                let data = trim_text(raw, is_first, is_last, prev, next, prev_text_ends_ws);

                prev_text_ends_ws = ends_with_ws(&data);

                if !data.is_empty() {
                    concat.push(ConcatPart::Text(data.into_owned()));
                }
            }
            Node::ExpressionTag(tag) => {
                prev_text_ends_ws = false;
                concat.push(ConcatPart::Expr(tag.id));
            }
            other => {
                prev_text_ends_ws = false;
                flush(&mut concat, &mut items);
                match other {
                    Node::Element(el) => items.push(FragmentItem::Element(el.id)),
                    Node::ComponentNode(cn) => items.push(FragmentItem::ComponentNode(cn.id)),
                    Node::IfBlock(block) => items.push(FragmentItem::IfBlock(block.id)),
                    Node::EachBlock(block) => items.push(FragmentItem::EachBlock(block.id)),
                    Node::RenderTag(tag) => items.push(FragmentItem::RenderTag(tag.id)),
                    Node::HtmlTag(tag) => items.push(FragmentItem::HtmlTag(tag.id)),
                    Node::KeyBlock(block) => items.push(FragmentItem::KeyBlock(block.id)),
                    Node::SvelteElement(el) => items.push(FragmentItem::SvelteElement(el.id)),
                    _ => {}
                }
            }
        }
    }

    flush(&mut concat, &mut items);
    items
}

/// Trim a text node's whitespace following the Svelte reference algorithm.
///
/// Returns `Cow::Borrowed` when no modification is needed (zero alloc),
/// `Cow::Owned` only when whitespace actually changes.
fn trim_text<'a>(
    raw: &'a str,
    is_first: bool,
    is_last: bool,
    prev: Option<&Node>,
    next: Option<&Node>,
    prev_text_ends_ws: bool,
) -> Cow<'a, str> {
    let mut data: Cow<'a, str> = Cow::Borrowed(raw);

    // Leading whitespace
    if is_first {
        data = strip_leading_ws(data);
    } else if !matches!(prev, Some(Node::ExpressionTag(_))) {
        let replacement = if prev_text_ends_ws { "" } else { " " };
        data = replace_leading_ws(data, replacement);
    }

    // Trailing whitespace
    if is_last {
        data = strip_trailing_ws(data);
    } else if !matches!(next, Some(Node::ExpressionTag(_))) {
        data = replace_trailing_ws(data, " ");
    }

    data
}

// ---------------------------------------------------------------------------
// Whitespace utilities — operate on [ \t\r\n] to match Svelte's patterns.
// Return Cow to avoid allocation when the string is unchanged.
// ---------------------------------------------------------------------------

fn is_ws_char(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn is_ws_only(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_ws_char)
}

fn ends_with_ws(s: &str) -> bool {
    s.ends_with(is_ws_char)
}

/// Count leading `[ \t\r\n]` bytes.
fn leading_ws_len(s: &str) -> usize {
    s.len() - s.trim_start_matches(is_ws_char).len()
}

/// Count trailing `[ \t\r\n]` bytes.
fn trailing_ws_len(s: &str) -> usize {
    s.len() - s.trim_end_matches(is_ws_char).len()
}

/// Strip leading `[ \t\r\n]+`. Borrows when no leading ws.
fn strip_leading_ws(s: Cow<str>) -> Cow<str> {
    let ws = leading_ws_len(&s);
    if ws == 0 {
        return s;
    }
    match s {
        Cow::Borrowed(b) => Cow::Borrowed(&b[ws..]),
        Cow::Owned(o) => Cow::Owned(o[ws..].to_string()),
    }
}

/// Strip trailing `[ \t\r\n]+`. Borrows when no trailing ws.
fn strip_trailing_ws(s: Cow<str>) -> Cow<str> {
    let ws = trailing_ws_len(&s);
    if ws == 0 {
        return s;
    }
    match s {
        Cow::Borrowed(b) => Cow::Borrowed(&b[..b.len() - ws]),
        Cow::Owned(mut o) => {
            o.truncate(o.len() - ws);
            Cow::Owned(o)
        }
    }
}

/// Replace leading `[ \t\r\n]+` with `replacement`. Borrows when no leading ws.
fn replace_leading_ws<'a>(s: Cow<'a, str>, replacement: &str) -> Cow<'a, str> {
    let ws = leading_ws_len(&s);
    if ws == 0 {
        return s;
    }
    let rest = &s[ws..];
    let mut out = String::with_capacity(replacement.len() + rest.len());
    out.push_str(replacement);
    out.push_str(rest);
    Cow::Owned(out)
}

/// Replace trailing `[ \t\r\n]+` with `replacement`. Borrows when no trailing ws.
fn replace_trailing_ws<'a>(s: Cow<'a, str>, replacement: &str) -> Cow<'a, str> {
    let ws = trailing_ws_len(&s);
    if ws == 0 {
        return s;
    }
    let prefix = &s[..s.len() - ws];
    let mut out = String::with_capacity(prefix.len() + replacement.len());
    out.push_str(prefix);
    out.push_str(replacement);
    Cow::Owned(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use svelte_ast::{ExpressionTag, NodeId, Text};
    use svelte_span::Span;

    fn text_node(id: u32, start: u32, end: u32) -> Node {
        Node::Text(Text {
            id: NodeId(id),
            span: Span { start, end },
        })
    }

    fn expr_node(id: u32) -> Node {
        Node::ExpressionTag(ExpressionTag {
            id: NodeId(id),
            span: Span { start: 0, end: 0 },
            expression_span: Span { start: 0, end: 0 },
        })
    }

    // -- Whitespace utility tests --

    #[test]
    fn test_is_ws_only() {
        assert!(is_ws_only(" \t\r\n"));
        assert!(is_ws_only("   "));
        assert!(!is_ws_only("  hello  "));
        assert!(!is_ws_only(""));
    }

    #[test]
    fn test_strip_leading_ws() {
        assert_eq!(&*strip_leading_ws(Cow::Borrowed("\n  hello")), "hello");
        assert_eq!(&*strip_leading_ws(Cow::Borrowed("hello")), "hello");
        assert_eq!(&*strip_leading_ws(Cow::Borrowed("\t\r\n  ")), "");
    }

    #[test]
    fn test_strip_trailing_ws() {
        assert_eq!(&*strip_trailing_ws(Cow::Borrowed("hello  \n")), "hello");
        assert_eq!(&*strip_trailing_ws(Cow::Borrowed("hello")), "hello");
        assert_eq!(&*strip_trailing_ws(Cow::Borrowed("  \t\r\n")), "");
    }

    #[test]
    fn test_replace_leading_ws() {
        assert_eq!(&*replace_leading_ws(Cow::Borrowed("\n  hello"), " "), " hello");
        assert_eq!(&*replace_leading_ws(Cow::Borrowed("\n  hello"), ""), "hello");
        assert_eq!(&*replace_leading_ws(Cow::Borrowed("hello"), " "), "hello");
    }

    #[test]
    fn test_replace_trailing_ws() {
        assert_eq!(&*replace_trailing_ws(Cow::Borrowed("hello  \n"), " "), "hello ");
        assert_eq!(&*replace_trailing_ws(Cow::Borrowed("hello  \n"), ""), "hello");
        assert_eq!(&*replace_trailing_ws(Cow::Borrowed("hello"), " "), "hello");
    }

    #[test]
    fn strip_borrows_when_no_ws() {
        let s = Cow::Borrowed("hello");
        let result = strip_leading_ws(s);
        assert!(matches!(result, Cow::Borrowed(_)));

        let s = Cow::Borrowed("hello");
        let result = strip_trailing_ws(s);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn strip_leading_borrows_slice() {
        let s = Cow::Borrowed("\n  hello");
        let result = strip_leading_ws(s);
        assert!(matches!(result, Cow::Borrowed("hello")));
    }

    #[test]
    fn strip_trailing_borrows_slice() {
        let s = Cow::Borrowed("hello  \n");
        let result = strip_trailing_ws(s);
        assert!(matches!(result, Cow::Borrowed("hello")));
    }

    // -- build_items integration tests --

    fn make_component(source: &str, nodes: Vec<Node>) -> Component {
        Component::new(
            source.to_string(),
            Fragment::new(nodes),
            None,
            None,
        )
    }

    fn collect_text_parts(items: &[FragmentItem]) -> Vec<String> {
        let mut out = Vec::new();
        for item in items {
            if let FragmentItem::TextConcat { parts, .. } = item {
                for part in parts {
                    if let ConcatPart::Text(s) = part {
                        out.push(s.clone());
                    }
                }
            }
        }
        out
    }

    #[test]
    fn trim_leading_and_trailing() {
        let src = "\n  hello  \n";
        let comp = make_component(src, vec![text_node(1, 0, src.len() as u32)]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert_eq!(texts, vec!["hello"]);
    }

    #[test]
    fn preserve_internal_newlines() {
        let src = "hello\n  world";
        let comp = make_component(src, vec![text_node(1, 0, src.len() as u32)]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert_eq!(texts, vec!["hello\n  world"]);
    }

    #[test]
    fn tabs_and_crlf() {
        let src = "\r\n\thello\r\n";
        let comp = make_component(src, vec![text_node(1, 0, src.len() as u32)]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert_eq!(texts, vec!["hello"]);
    }

    #[test]
    fn pure_whitespace_only_is_removed() {
        let src = "  \n\t  ";
        let comp = make_component(src, vec![text_node(1, 0, src.len() as u32)]);
        let items = build_items(&comp.fragment, &comp);
        assert!(items.is_empty());
    }

    #[test]
    fn expr_neighbor_preserves_ws() {
        let src = "{expr}\n  hello";
        let comp = make_component(src, vec![
            expr_node(1),
            text_node(2, 6, src.len() as u32),
        ]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert_eq!(texts, vec!["\n  hello"]);
    }

    #[test]
    fn expr_next_preserves_trailing_ws() {
        let src = "hello  \n{expr}";
        let comp = make_component(src, vec![
            text_node(1, 0, 8),
            expr_node(2),
        ]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert_eq!(texts, vec!["hello  \n"]);
    }

    #[test]
    fn adjacent_text_nodes_no_double_space() {
        let src = "\n\n";
        let comp = make_component(src, vec![
            text_node(1, 0, 1),
            text_node(2, 1, 2),
        ]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert!(texts.is_empty());
    }

    #[test]
    fn ws_between_non_expr_nodes_collapses_to_space() {
        let src = "hello\n\n  world";
        let comp = make_component(src, vec![text_node(1, 0, src.len() as u32)]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert_eq!(texts, vec!["hello\n\n  world"]);
    }

    #[test]
    fn text_between_expressions_preserves_all_ws() {
        let src = "{a}\n  \n{b}";
        let comp = make_component(src, vec![
            expr_node(1),
            text_node(2, 3, 7),
            expr_node(3),
        ]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert_eq!(texts, vec!["\n  \n"]);
    }

    #[test]
    fn text_after_expr_before_non_expr() {
        let src = "{expr}\n  hello\n  x";
        let comp = make_component(src, vec![
            expr_node(1),
            text_node(2, 6, 16),
            text_node(3, 16, 18),
        ]);
        let texts = collect_text_parts(&build_items(&comp.fragment, &comp));
        assert_eq!(texts, vec!["\n  hello ", "x"]);
    }

    #[test]
    fn no_alloc_when_text_unchanged() {
        // Text without any boundary whitespace: should stay Cow::Borrowed
        let raw = "hello";
        let result = trim_text(raw, false, false, Some(&expr_node(1)), Some(&expr_node(2)), false);
        assert!(matches!(result, Cow::Borrowed("hello")));
    }

    #[test]
    fn no_alloc_for_first_without_leading_ws() {
        let raw = "hello world";
        let result = trim_text(raw, true, true, None, None, false);
        assert!(matches!(result, Cow::Borrowed("hello world")));
    }
}
