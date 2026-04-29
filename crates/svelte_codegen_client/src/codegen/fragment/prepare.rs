use std::borrow::Cow;

use smallvec::SmallVec;
use svelte_ast::{AstStore, Node, NodeId};
use svelte_span::Span;

use crate::codegen::{
    data_structures::{ConcatPart, FragmentCtx},
    fragment::types::{BufItem, Child, ChildrenFlags, ContentStrategy, HoistedBucket, HoistedKind},
};

pub(super) fn prepare<'a>(
    raw: &[NodeId],
    store: &'a AstStore,
    ctx: &FragmentCtx<'a>,
    bucket: &mut HoistedBucket,
) -> (Vec<Child>, ContentStrategy) {
    let exclude_slotted = ctx.role == svelte_ast::FragmentRole::ComponentChildren;
    let mut filtered: SmallVec<[&Node; 8]> = SmallVec::with_capacity(raw.len());
    for &id in raw {
        let node = store.get(id);
        if exclude_slotted && node_has_slot_attribute(node) {
            continue;
        }
        match hoisted_kind(node, ctx.inside_head) {
            HoistedKind::Comment | HoistedKind::Error => continue,
            HoistedKind::Snippet => {
                bucket.snippets.push(node.node_id());
                continue;
            }
            HoistedKind::ConstTag => {
                bucket.const_tags.push(node.node_id());
                continue;
            }
            HoistedKind::DebugTag => {
                bucket.debug_tags.push(node.node_id());
                continue;
            }
            HoistedKind::SvelteHead => {
                bucket.svelte_head.push(node.node_id());
                continue;
            }
            HoistedKind::SvelteWindow => {
                bucket.svelte_window.push(node.node_id());
                continue;
            }
            HoistedKind::SvelteDocument => {
                bucket.svelte_document.push(node.node_id());
                continue;
            }
            HoistedKind::SvelteBody => {
                bucket.svelte_body.push(node.node_id());
                continue;
            }
            HoistedKind::TitleInsideHead => {
                bucket.titles.push(node.node_id());
                continue;
            }
            HoistedKind::NotHoisted => {
                filtered.push(node);
            }
        }
    }

    let preserve = ctx.preserve_whitespace || ctx.is_pre || ctx.is_textarea;
    let filtered_slice: &[&Node] = if preserve {
        let mut end = filtered.len();
        while end > 0 {
            if let Node::Text(t) = filtered[end - 1]
                && is_ws_only(t.value(ctx.source))
            {
                end -= 1;
                continue;
            }
            break;
        }
        &filtered[..end]
    } else {
        let mut start = 0;
        while start < filtered.len() {
            if let Node::Text(t) = filtered[start]
                && is_ws_only(t.value(ctx.source))
            {
                start += 1;
                continue;
            }
            break;
        }
        let mut end = filtered.len();
        while end > start {
            if let Node::Text(t) = filtered[end - 1]
                && is_ws_only(t.value(ctx.source))
            {
                end -= 1;
                continue;
            }
            break;
        }
        &filtered[start..end]
    };

    let len = filtered_slice.len();
    if len == 0 {
        return (Vec::new(), ContentStrategy::Empty);
    }

    let mut children: Vec<Child> = Vec::with_capacity(len);
    let mut buf: SmallVec<[BufItem; 4]> = SmallVec::new();
    let mut flags = ChildrenFlags::empty();
    let mut prev_text_ends_ws = false;

    for fi in 0..len {
        let node = filtered_slice[fi];
        match node {
            Node::Text(text) => {
                let raw_value = text.value(ctx.source);
                let is_first = fi == 0;
                let is_last = fi == len - 1;
                let prev_is_expr =
                    fi > 0 && matches!(filtered_slice[fi - 1], Node::ExpressionTag(_));
                let next_is_expr =
                    fi + 1 < len && matches!(filtered_slice[fi + 1], Node::ExpressionTag(_));

                let trimmed = if preserve {
                    Cow::Borrowed(raw_value)
                } else {
                    trim_text(
                        raw_value,
                        is_first,
                        is_last,
                        prev_is_expr,
                        next_is_expr,
                        prev_text_ends_ws,
                    )
                };

                prev_text_ends_ws = ends_with_ws(&trimmed);

                if ctx.can_remove_entirely && trimmed.as_ref() == " " {
                    continue;
                }
                if trimmed.is_empty() {
                    continue;
                }

                let part = match trimmed {
                    Cow::Borrowed(s) if text.decoded.is_none() && is_slice_of(ctx.source, s) => {
                        let source_ptr = ctx.source.as_ptr() as usize;
                        let offset = s.as_ptr() as usize - source_ptr;
                        ConcatPart::Static(Span::new(offset as u32, (offset + s.len()) as u32))
                    }
                    Cow::Borrowed(s) => ConcatPart::StaticOwned(s.to_string()),
                    Cow::Owned(s) => ConcatPart::StaticOwned(s),
                };
                buf.push(BufItem::Text(part));
            }
            Node::ExpressionTag(tag) => {
                prev_text_ends_ws = false;
                buf.push(BufItem::Expr(tag.id));
            }
            _ => {
                prev_text_ends_ws = false;
                flush_buf(&mut buf, &mut children, &mut flags);
                children.push(Child::Node(node.node_id()));
                mark_node_flag(&mut flags, node);
            }
        }
    }
    flush_buf(&mut buf, &mut children, &mut flags);

    let strategy = classify(flags, &children, store);
    (children, strategy)
}

fn classify(flags: ChildrenFlags, children: &[Child], store: &AstStore) -> ContentStrategy {
    if children.is_empty() {
        return ContentStrategy::Empty;
    }
    if children.len() > 1 {
        let first_is_text_like = matches!(
            children.first(),
            Some(Child::Text(_) | Child::Expr(_) | Child::Concat(_))
        );
        let first_is_block = match children.first() {
            Some(Child::Node(nid)) => !matches!(store.get(*nid), Node::Element(_)),
            _ => false,
        };
        return ContentStrategy::Multi {
            count: children.len() as u16,
            has_elements: flags.contains(ChildrenFlags::HAS_ELEMENT),
            has_blocks: flags.contains(ChildrenFlags::HAS_BLOCK),
            has_text: flags.contains(ChildrenFlags::HAS_TEXT)
                || flags.contains(ChildrenFlags::HAS_EXPR)
                || flags.contains(ChildrenFlags::HAS_CONCAT),
            first_is_block,
            first_is_text_like,
        };
    }
    match &children[0] {
        Child::Text(_) => ContentStrategy::SingleStatic,
        Child::Expr(id) => ContentStrategy::SingleExpr(*id),
        Child::Concat(_) => ContentStrategy::SingleConcat,
        Child::Node(id) => {
            if flags.contains(ChildrenFlags::HAS_BLOCK) {
                ContentStrategy::SingleBlock(*id)
            } else {
                ContentStrategy::SingleElement(*id)
            }
        }
    }
}

fn node_has_slot_attribute(node: &Node) -> bool {
    use svelte_ast::Attribute;
    let attrs: &[Attribute] = match node {
        Node::Element(el) => &el.attributes,
        Node::SvelteFragmentLegacy(el) => &el.attributes,
        Node::ComponentNode(cn) => &cn.attributes,
        _ => return false,
    };
    attrs
        .iter()
        .any(|attr| matches!(attr, Attribute::StringAttribute(sa) if sa.name == "slot"))
}

fn hoisted_kind(node: &Node, inside_head: bool) -> HoistedKind {
    match node {
        Node::Comment(_) => HoistedKind::Comment,
        Node::Error(_) => HoistedKind::Error,
        Node::SnippetBlock(_) => HoistedKind::Snippet,
        Node::ConstTag(_) => HoistedKind::ConstTag,
        Node::DebugTag(_) => HoistedKind::DebugTag,
        Node::SvelteHead(_) => HoistedKind::SvelteHead,
        Node::SvelteWindow(_) => HoistedKind::SvelteWindow,
        Node::SvelteDocument(_) => HoistedKind::SvelteDocument,
        Node::SvelteBody(_) => HoistedKind::SvelteBody,
        Node::Element(el) if inside_head && el.name == "title" => HoistedKind::TitleInsideHead,
        _ => HoistedKind::NotHoisted,
    }
}

fn flush_buf(
    buf: &mut SmallVec<[BufItem; 4]>,
    children: &mut Vec<Child>,
    flags: &mut ChildrenFlags,
) {
    if buf.is_empty() {
        return;
    }
    if buf.len() == 1 {
        let Some(only) = buf.pop() else {
            return;
        };
        match only {
            BufItem::Text(part) => {
                children.push(Child::Text(part));
                flags.insert(ChildrenFlags::HAS_TEXT);
            }
            BufItem::Expr(id) => {
                children.push(Child::Expr(id));
                flags.insert(ChildrenFlags::HAS_EXPR);
            }
        }
    } else {
        let parts: SmallVec<[ConcatPart; 4]> = buf
            .drain(..)
            .map(|item| match item {
                BufItem::Text(part) => part,
                BufItem::Expr(id) => ConcatPart::Expr(id),
            })
            .collect();
        children.push(Child::Concat(parts));
        flags.insert(ChildrenFlags::HAS_CONCAT);
    }
}

fn mark_node_flag(flags: &mut ChildrenFlags, node: &Node) {
    match node {
        Node::Element(_) | Node::SlotElementLegacy(_) | Node::SvelteFragmentLegacy(_) => {
            flags.insert(ChildrenFlags::HAS_ELEMENT);
        }
        _ => {
            flags.insert(ChildrenFlags::HAS_BLOCK);
        }
    }
}

fn is_slice_of(haystack: &str, needle: &str) -> bool {
    let h_start = haystack.as_ptr() as usize;
    let h_end = h_start + haystack.len();
    let n_start = needle.as_ptr() as usize;
    let n_end = n_start + needle.len();
    n_start >= h_start && n_end <= h_end
}

fn is_ws_byte(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n')
}

fn is_ws_only(s: &str) -> bool {
    !s.is_empty() && s.as_bytes().iter().copied().all(is_ws_byte)
}

fn ends_with_ws(s: &str) -> bool {
    s.as_bytes().last().copied().is_some_and(is_ws_byte)
}

fn leading_ws_len(s: &str) -> usize {
    s.as_bytes().iter().take_while(|&&b| is_ws_byte(b)).count()
}

fn trailing_ws_len(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .rev()
        .take_while(|&&b| is_ws_byte(b))
        .count()
}

fn strip_leading_ws(s: Cow<str>) -> Cow<str> {
    let ws = leading_ws_len(&s);
    if ws == 0 {
        return s;
    }
    match s {
        Cow::Borrowed(b) => Cow::Borrowed(&b[ws..]),
        Cow::Owned(mut o) => {
            o.drain(..ws);
            Cow::Owned(o)
        }
    }
}

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

fn replace_leading_ws<'a>(s: Cow<'a, str>, replacement: &str) -> Cow<'a, str> {
    let ws = leading_ws_len(&s);
    if ws == 0 {
        return s;
    }
    if replacement.is_empty() {
        return strip_leading_ws_prefix(s, ws);
    }
    if ws == s.len() && replacement == " " {
        return Cow::Borrowed(" ");
    }
    let rest = &s[ws..];
    let mut out = String::with_capacity(replacement.len() + rest.len());
    out.push_str(replacement);
    out.push_str(rest);
    Cow::Owned(out)
}

fn strip_leading_ws_prefix(s: Cow<str>, ws: usize) -> Cow<str> {
    match s {
        Cow::Borrowed(b) => Cow::Borrowed(&b[ws..]),
        Cow::Owned(mut o) => {
            o.drain(..ws);
            Cow::Owned(o)
        }
    }
}

fn replace_trailing_ws<'a>(s: Cow<'a, str>, replacement: &str) -> Cow<'a, str> {
    let ws = trailing_ws_len(&s);
    if ws == 0 {
        return s;
    }
    let prefix_end = s.len() - ws;
    if prefix_end == 0 && replacement == " " {
        return Cow::Borrowed(" ");
    }
    if replacement.is_empty() {
        return strip_trailing_ws_suffix(s, ws);
    }
    let prefix = &s[..prefix_end];
    let mut out = String::with_capacity(prefix.len() + replacement.len());
    out.push_str(prefix);
    out.push_str(replacement);
    Cow::Owned(out)
}

fn strip_trailing_ws_suffix(s: Cow<str>, ws: usize) -> Cow<str> {
    match s {
        Cow::Borrowed(b) => Cow::Borrowed(&b[..b.len() - ws]),
        Cow::Owned(mut o) => {
            o.truncate(o.len() - ws);
            Cow::Owned(o)
        }
    }
}

fn trim_text<'a>(
    raw: &'a str,
    is_first: bool,
    is_last: bool,
    prev_is_expr: bool,
    next_is_expr: bool,
    prev_text_ends_ws: bool,
) -> Cow<'a, str> {
    let mut data: Cow<'a, str> = Cow::Borrowed(raw);

    if is_first {
        data = strip_leading_ws(data);
    } else if !prev_is_expr {
        let replacement = if prev_text_ends_ws { "" } else { " " };
        data = replace_leading_ws(data, replacement);
    }

    if is_last {
        data = strip_trailing_ws(data);
    } else if !next_is_expr {
        data = replace_trailing_ws(data, " ");
    }

    data
}
