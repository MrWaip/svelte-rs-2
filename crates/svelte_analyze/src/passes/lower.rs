use std::borrow::Cow;

use smallvec::SmallVec;

use svelte_ast::{
    is_mathml, is_svg, is_whitespace_removable_parent, AstStore, Attribute, Component, Fragment,
    Namespace, Node, NodeId,
};
use svelte_span::Span;

use crate::types::data::{
    AnalysisData, FragmentItem, FragmentKey, LoweredFragment, LoweredTextPart,
};

/// Check if a direct component child carries a static `slot="name"` attribute.
/// Returns the child NodeId when present so lower/codegen can keep named-slot
/// grouping aligned across elements, fragments, and child components.
fn determine_slot(node: &Node) -> Option<NodeId> {
    let (id, attrs) = match node {
        Node::Element(el) => (el.id, &el.attributes),
        Node::SvelteFragmentLegacy(el) => (el.id, &el.attributes),
        Node::ComponentNode(cn) => (cn.id, &cn.attributes),
        _ => return None,
    };
    let has_slot = attrs
        .iter()
        .any(|attr| matches!(attr, Attribute::StringAttribute(sa) if sa.name == "slot"));
    has_slot.then_some(id)
}

fn legacy_slot_name<'a>(attrs: &'a [Attribute], source: &'a str) -> &'a str {
    for attr in attrs {
        if let Attribute::StringAttribute(attr) = attr {
            if attr.name == "name" {
                return attr.value_span.source_text(source);
            }
        }
    }

    "default"
}

fn record_custom_element_slot_name(data: &mut AnalysisData, attrs: &[Attribute], source: &str) {
    if !data.output.custom_element {
        return;
    }

    let slot_name = legacy_slot_name(attrs, source);
    if data
        .output
        .custom_element_slot_names
        .iter()
        .any(|existing| existing == slot_name)
    {
        return;
    }

    data.output
        .custom_element_slot_names
        .push(slot_name.to_string());
}

pub fn lower(component: &Component, data: &mut AnalysisData) {
    let initial_in_svg = component
        .options
        .as_ref()
        .and_then(|o| o.namespace.as_ref())
        == Some(&Namespace::Svg);
    let initial_in_mathml = component
        .options
        .as_ref()
        .and_then(|o| o.namespace.as_ref())
        == Some(&Namespace::Mathml);
    let preserve_whitespace = data.script.preserve_whitespace;
    lower_fragment(
        &component.fragment,
        FragmentKey::Root,
        component,
        data,
        &component.store,
        initial_in_svg,
        initial_in_svg,
        initial_in_mathml,
        preserve_whitespace,
    );
}

/// Collect `const_tags.by_fragment` before lower runs so subsequent passes
/// (const-tag sym population, reactivity compute, dynamism) can key off it.
pub(crate) fn collect_const_tag_fragments(component: &Component, data: &mut AnalysisData) {
    collect_const_tags_in(
        &component.fragment,
        FragmentKey::Root,
        data,
        &component.store,
    );
}

fn collect_const_tags_in(
    fragment: &Fragment,
    key: FragmentKey,
    data: &mut AnalysisData,
    store: &AstStore,
) {
    let mut const_ids: Option<Vec<svelte_ast::NodeId>> = None;
    for &id in &fragment.nodes {
        if let Some(ct) = store.get(id).as_const_tag() {
            const_ids.get_or_insert_with(Vec::new).push(ct.id);
        }
    }
    if let Some(ids) = const_ids {
        data.template.const_tags.by_fragment.insert(key, ids);
    }
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::Element(el) => {
                collect_const_tags_in(&el.fragment, FragmentKey::Element(el.id), data, store);
            }
            Node::ComponentNode(cn) => {
                collect_const_tags_in(&cn.fragment, FragmentKey::ComponentNode(cn.id), data, store);
            }
            Node::IfBlock(block) => {
                collect_const_tags_in(
                    &block.consequent,
                    FragmentKey::IfConsequent(block.id),
                    data,
                    store,
                );
                if let Some(alt) = &block.alternate {
                    collect_const_tags_in(alt, FragmentKey::IfAlternate(block.id), data, store);
                }
            }
            Node::EachBlock(block) => {
                collect_const_tags_in(&block.body, FragmentKey::EachBody(block.id), data, store);
                if let Some(fb) = &block.fallback {
                    collect_const_tags_in(fb, FragmentKey::EachFallback(block.id), data, store);
                }
            }
            Node::SnippetBlock(block) => {
                collect_const_tags_in(&block.body, FragmentKey::SnippetBody(block.id), data, store);
            }
            Node::KeyBlock(block) => {
                collect_const_tags_in(
                    &block.fragment,
                    FragmentKey::KeyBlockBody(block.id),
                    data,
                    store,
                );
            }
            Node::SvelteHead(head) => {
                collect_const_tags_in(
                    &head.fragment,
                    FragmentKey::SvelteHeadBody(head.id),
                    data,
                    store,
                );
            }
            Node::SvelteElement(el) => {
                collect_const_tags_in(
                    &el.fragment,
                    FragmentKey::SvelteElementBody(el.id),
                    data,
                    store,
                );
            }
            Node::SvelteBoundary(b) => {
                collect_const_tags_in(
                    &b.fragment,
                    FragmentKey::SvelteBoundaryBody(b.id),
                    data,
                    store,
                );
            }
            Node::AwaitBlock(block) => {
                if let Some(ref p) = block.pending {
                    collect_const_tags_in(p, FragmentKey::AwaitPending(block.id), data, store);
                }
                if let Some(ref t) = block.then {
                    collect_const_tags_in(t, FragmentKey::AwaitThen(block.id), data, store);
                }
                if let Some(ref c) = block.catch {
                    collect_const_tags_in(c, FragmentKey::AwaitCatch(block.id), data, store);
                }
            }
            _ => {}
        }
    }
}

fn lower_fragment(
    fragment: &Fragment,
    key: FragmentKey,
    component: &Component,
    data: &mut AnalysisData,
    store: &AstStore,
    in_svg_non_text: bool, // whitespace removal: true inside SVG non-text and table-like elements
    in_svg_ns: bool,       // actual SVG namespace: true only when inside a real SVG subtree
    in_mathml: bool,
    preserve_whitespace: bool,
) {
    lower_nodes(
        &fragment.nodes,
        key,
        component,
        data,
        store,
        in_svg_non_text,
        in_svg_ns,
        in_mathml,
        preserve_whitespace,
    );
}

fn lower_nodes(
    nodes: &[NodeId],
    key: FragmentKey,
    component: &Component,
    data: &mut AnalysisData,
    store: &AstStore,
    in_svg_non_text: bool,
    in_svg_ns: bool,
    in_mathml: bool,
    preserve_whitespace: bool,
) {
    for &id in nodes {
        data.template
            .template_topology
            .record_node_fragment(id, key);
    }

    let inside_head = matches!(key, FragmentKey::SvelteHeadBody(_));

    let items = build_items_from_nodes(
        nodes,
        component,
        inside_head,
        in_svg_non_text,
        preserve_whitespace,
        store,
    );

    // Pre-compute fragment blocker indices (experimental.async)
    if data.script.blocker_data.has_async {
        let mut blockers = SmallVec::<[u32; 2]>::new();
        for item in &items {
            if let FragmentItem::TextConcat { parts, .. } = item {
                for part in parts {
                    if let LoweredTextPart::Expr(id) = part {
                        for idx in data.expression_blockers(*id) {
                            blockers.push(idx);
                        }
                    }
                }
            }
        }
        if !blockers.is_empty() {
            blockers.sort_unstable();
            blockers.dedup();
            data.template
                .fragments
                .fragment_blockers
                .insert(key, blockers);
        }
    }

    data.template
        .fragments
        .lowered
        .insert(key, LoweredFragment { items });

    let mut debug_ids: Option<Vec<NodeId>> = None;
    let mut title_ids: Option<Vec<NodeId>> = None;

    for &id in nodes {
        match store.get(id) {
            Node::DebugTag(tag) => {
                debug_ids.get_or_insert_with(Vec::new).push(tag.id);
            }
            Node::Element(el) => {
                if inside_head && el.name == "title" {
                    title_ids.get_or_insert_with(Vec::new).push(el.id);
                }
                // SVG elements (except <text>) propagate can_remove_entirely to children.
                // <foreignObject> resets to HTML context.
                let child_svg = if el.name == "foreignObject" {
                    false
                } else if is_svg(&el.name) && el.name != "text" {
                    true
                } else if in_svg_non_text && el.name != "text" {
                    // Inherit SVG context from ancestor (e.g. <g> inside <svg>)
                    true
                } else {
                    is_whitespace_removable_parent(&el.name)
                };
                // SVG namespace propagates through all elements except foreignObject.
                // Unlike whitespace tracking, <text> does NOT reset namespace.
                let child_svg_ns = el.name != "foreignObject" && (is_svg(&el.name) || in_svg_ns);
                // annotation-xml resets from MathML to HTML context
                let child_mathml =
                    is_mathml(&el.name) || (in_mathml && el.name != "annotation-xml");
                lower_nodes(
                    &el.fragment.nodes,
                    FragmentKey::Element(el.id),
                    component,
                    data,
                    store,
                    child_svg,
                    child_svg_ns,
                    child_mathml,
                    preserve_whitespace,
                );
            }
            Node::SlotElementLegacy(el) => {
                record_custom_element_slot_name(data, &el.attributes, &component.source);
                lower_nodes(
                    &el.fragment.nodes,
                    FragmentKey::Element(el.id),
                    component,
                    data,
                    store,
                    inside_head,
                    in_svg_ns,
                    in_mathml,
                    preserve_whitespace,
                );
            }
            Node::ComponentNode(cn) => {
                let snippets: Vec<_> = cn
                    .fragment
                    .nodes
                    .iter()
                    .filter_map(|&nid| {
                        if let Node::SnippetBlock(s) = store.get(nid) {
                            Some(s.id)
                        } else {
                            None
                        }
                    })
                    .collect();
                if !snippets.is_empty() {
                    data.template
                        .snippets
                        .component_snippets
                        .insert(cn.id, snippets);
                }

                // Partition children by slot="name" attribute
                let mut default_nodes = SmallVec::<[NodeId; 8]>::new();
                let mut named_groups = SmallVec::<[(NodeId, NodeId); 4]>::new();

                for &child_id in &cn.fragment.nodes {
                    if let Some(slot_el_id) = determine_slot(store.get(child_id)) {
                        named_groups.push((slot_el_id, child_id));
                    } else {
                        default_nodes.push(child_id);
                    }
                }

                lower_nodes(
                    default_nodes.as_slice(),
                    FragmentKey::ComponentNode(cn.id),
                    component,
                    data,
                    store,
                    false,
                    false,
                    false,
                    preserve_whitespace,
                );

                let mut slot_mappings: Vec<(NodeId, FragmentKey)> = Vec::new();
                for (slot_el_id, child_id) in named_groups {
                    let frag_key = FragmentKey::NamedSlot(cn.id, slot_el_id);
                    // <svelte:fragment slot="name"> wraps the real slot content — lower
                    // its children instead of the wrapper element itself.
                    let slot_nodes: SmallVec<[NodeId; 4]> = match store.get(child_id) {
                        Node::SvelteFragmentLegacy(el) => {
                            data.elements.flags.svelte_fragment_slots.insert(slot_el_id);
                            el.fragment.nodes.iter().copied().collect()
                        }
                        _ => {
                            let mut v = SmallVec::new();
                            v.push(child_id);
                            v
                        }
                    };
                    lower_nodes(
                        &slot_nodes,
                        frag_key,
                        component,
                        data,
                        store,
                        false,
                        false,
                        false,
                        preserve_whitespace,
                    );
                    slot_mappings.push((slot_el_id, frag_key));
                }
                if !slot_mappings.is_empty() {
                    data.template
                        .snippets
                        .component_named_slots
                        .insert(cn.id, slot_mappings);
                }
            }
            Node::SvelteFragmentLegacy(el) => {
                lower_nodes(
                    &el.fragment.nodes,
                    FragmentKey::Element(el.id),
                    component,
                    data,
                    store,
                    inside_head,
                    in_svg_ns,
                    in_mathml,
                    preserve_whitespace,
                );
            }
            Node::IfBlock(block) => {
                lower_nodes(
                    &block.consequent.nodes,
                    FragmentKey::IfConsequent(block.id),
                    component,
                    data,
                    store,
                    in_svg_non_text,
                    in_svg_ns,
                    in_mathml,
                    preserve_whitespace,
                );
                if let Some(alt) = &block.alternate {
                    let alt_key = FragmentKey::IfAlternate(block.id);
                    lower_nodes(
                        &alt.nodes,
                        alt_key,
                        component,
                        data,
                        store,
                        in_svg_non_text,
                        in_svg_ns,
                        in_mathml,
                        preserve_whitespace,
                    );
                    // Detect elseif: alternate has a single IfBlock child marked as elseif
                    let is_elseif = data.template.fragments.lowered.get(&alt_key).is_some_and(|lf| {
                        lf.items.len() == 1
                            && matches!(&lf.items[0], FragmentItem::IfBlock(id)
                                if alt.nodes.iter().any(|&nid| matches!(store.get(nid), Node::IfBlock(ib) if ib.id == *id && ib.elseif)))
                    });
                    if is_elseif {
                        data.output.alt_is_elseif.insert(block.id);
                    }
                }
            }
            Node::EachBlock(block) => {
                lower_nodes(
                    &block.body.nodes,
                    FragmentKey::EachBody(block.id),
                    component,
                    data,
                    store,
                    in_svg_non_text,
                    in_svg_ns,
                    in_mathml,
                    preserve_whitespace,
                );
                if let Some(fb) = &block.fallback {
                    lower_nodes(
                        &fb.nodes,
                        FragmentKey::EachFallback(block.id),
                        component,
                        data,
                        store,
                        in_svg_non_text,
                        in_svg_ns,
                        in_mathml,
                        preserve_whitespace,
                    );
                }
            }
            Node::SnippetBlock(block) => {
                lower_nodes(
                    &block.body.nodes,
                    FragmentKey::SnippetBody(block.id),
                    component,
                    data,
                    store,
                    false,
                    false,
                    false,
                    preserve_whitespace,
                );
            }
            Node::KeyBlock(block) => {
                lower_nodes(
                    &block.fragment.nodes,
                    FragmentKey::KeyBlockBody(block.id),
                    component,
                    data,
                    store,
                    in_svg_non_text,
                    in_svg_ns,
                    in_mathml,
                    preserve_whitespace,
                );
            }
            Node::SvelteHead(head) => {
                lower_nodes(
                    &head.fragment.nodes,
                    FragmentKey::SvelteHeadBody(head.id),
                    component,
                    data,
                    store,
                    false,
                    false,
                    false,
                    preserve_whitespace,
                );
            }
            Node::SvelteElement(el) => {
                lower_nodes(
                    &el.fragment.nodes,
                    FragmentKey::SvelteElementBody(el.id),
                    component,
                    data,
                    store,
                    in_svg_non_text,
                    in_svg_ns,
                    in_mathml,
                    preserve_whitespace,
                );
            }
            Node::SvelteBoundary(b) => {
                lower_nodes(
                    &b.fragment.nodes,
                    FragmentKey::SvelteBoundaryBody(b.id),
                    component,
                    data,
                    store,
                    false,
                    false,
                    false,
                    preserve_whitespace,
                );
            }
            Node::AwaitBlock(block) => {
                if let Some(ref p) = block.pending {
                    lower_nodes(
                        &p.nodes,
                        FragmentKey::AwaitPending(block.id),
                        component,
                        data,
                        store,
                        in_svg_non_text,
                        in_svg_ns,
                        in_mathml,
                        preserve_whitespace,
                    );
                }
                if let Some(ref t) = block.then {
                    lower_nodes(
                        &t.nodes,
                        FragmentKey::AwaitThen(block.id),
                        component,
                        data,
                        store,
                        in_svg_non_text,
                        in_svg_ns,
                        in_mathml,
                        preserve_whitespace,
                    );
                }
                if let Some(ref c) = block.catch {
                    lower_nodes(
                        &c.nodes,
                        FragmentKey::AwaitCatch(block.id),
                        component,
                        data,
                        store,
                        in_svg_non_text,
                        in_svg_ns,
                        in_mathml,
                        preserve_whitespace,
                    );
                }
            }
            Node::SvelteWindow(_) | Node::SvelteDocument(_) | Node::SvelteBody(_) => {}
            Node::HtmlTag(tag) => {
                if in_svg_ns {
                    data.elements.html_tag_in_svg.insert(tag.id);
                } else if in_mathml {
                    data.elements.html_tag_in_mathml.insert(tag.id);
                }
            }
            Node::Text(_)
            | Node::Comment(_)
            | Node::ExpressionTag(_)
            | Node::RenderTag(_)
            | Node::ConstTag(_)
            | Node::Error(_) => {}
        }
    }

    if let Some(ids) = debug_ids {
        data.template.debug_tags.by_fragment.insert(key, ids);
    }
    if let Some(ids) = title_ids {
        data.template.title_elements.by_fragment.insert(key, ids);
    }
}

#[derive(Clone, Copy)]
struct FilteredNode<'a> {
    node: &'a Node,
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
///
/// Returns true for nodes that are filtered out of lowered representation.
#[inline]
fn is_skipped_node(node: &Node, inside_head: bool) -> bool {
    match node {
        Node::Comment(_)
        | Node::SnippetBlock(_)
        | Node::ConstTag(_)
        | Node::DebugTag(_)
        | Node::SvelteHead(_)
        | Node::SvelteWindow(_)
        | Node::SvelteDocument(_)
        | Node::SvelteBody(_)
        | Node::Error(_) => true,
        Node::Element(el) if inside_head && el.name == "title" => true,
        _ => false,
    }
}

#[cfg(test)]
fn build_items(
    fragment: &Fragment,
    component: &Component,
    inside_head: bool,
    can_remove_entirely: bool,
    preserve_whitespace: bool,
    store: &AstStore,
) -> Vec<FragmentItem> {
    build_items_from_nodes(
        &fragment.nodes,
        component,
        inside_head,
        can_remove_entirely,
        preserve_whitespace,
        store,
    )
}

fn build_items_from_nodes(
    nodes: &[NodeId],
    component: &Component,
    inside_head: bool,
    can_remove_entirely: bool,
    preserve_whitespace: bool,
    store: &AstStore,
) -> Vec<FragmentItem> {
    let source = &component.source;
    let source_ptr = source.as_ptr() as usize;

    let mut filtered = SmallVec::<[FilteredNode<'_>; 8]>::with_capacity(nodes.len());
    for &id in nodes {
        let node = store.get(id);
        if !is_skipped_node(node, inside_head) {
            filtered.push(FilteredNode { node });
        }
    }

    let filtered = if preserve_whitespace {
        let mut end = filtered.len();
        while end > 0 {
            if let Node::Text(t) = filtered[end - 1].node {
                if is_ws_only(t.value(source)) {
                    end -= 1;
                    continue;
                }
            }
            break;
        }
        &filtered[..end]
    } else {
        let mut start = 0;
        while start < filtered.len() {
            if let Node::Text(t) = filtered[start].node {
                if is_ws_only(t.value(source)) {
                    start += 1;
                    continue;
                }
            }
            break;
        }

        let mut end = filtered.len();
        while end > start {
            if let Node::Text(t) = filtered[end - 1].node {
                if is_ws_only(t.value(source)) {
                    end -= 1;
                    continue;
                }
            }
            break;
        }

        &filtered[start..end]
    };
    let len = filtered.len();

    if len == 0 {
        return vec![];
    }

    let mut items: Vec<FragmentItem> = Vec::with_capacity(len);
    let mut concat = SmallVec::<[LoweredTextPart; 4]>::new();
    let mut concat_has_expr = false;
    let mut prev_text_ends_ws = false;

    let flush = |concat: &mut SmallVec<[LoweredTextPart; 4]>,
                 concat_has_expr: &mut bool,
                 items: &mut Vec<FragmentItem>| {
        if !concat.is_empty() {
            let parts = std::mem::take(concat).into_vec();
            items.push(FragmentItem::TextConcat {
                parts,
                has_expr: std::mem::take(concat_has_expr),
            });
        }
    };

    for fi in 0..len {
        let entry = filtered[fi];
        match entry.node {
            Node::Text(text) => {
                let value = text.value(source);
                let is_first = fi == 0;
                let is_last = fi == len - 1;
                let prev = if fi > 0 {
                    Some(filtered[fi - 1].node)
                } else {
                    None
                };
                let next = if fi + 1 < len {
                    Some(filtered[fi + 1].node)
                } else {
                    None
                };

                let trimmed = if preserve_whitespace {
                    Cow::Borrowed(value)
                } else {
                    trim_text(value, is_first, is_last, prev, next, prev_text_ends_ws)
                };

                prev_text_ends_ws = ends_with_ws(&trimmed);

                // SVG/table context: drop text nodes that collapse to a single space
                if can_remove_entirely && trimmed.as_ref() == " " {
                    // Don't add — whitespace between elements is insignificant
                } else if !trimmed.is_empty() {
                    let part = match trimmed {
                        Cow::Borrowed(s) if text.decoded.is_none() => {
                            // Compute span from pointer offset into source
                            let offset = s.as_ptr() as usize - source_ptr;
                            LoweredTextPart::TextSpan(Span::new(
                                offset as u32,
                                (offset + s.len()) as u32,
                            ))
                        }
                        Cow::Borrowed(s) => LoweredTextPart::TextOwned(s.to_string()),
                        Cow::Owned(s) => LoweredTextPart::TextOwned(s),
                    };
                    concat.push(part);
                }
            }
            Node::ExpressionTag(tag) => {
                prev_text_ends_ws = false;
                concat.push(LoweredTextPart::Expr(tag.id));
                concat_has_expr = true;
            }
            other => {
                prev_text_ends_ws = false;
                flush(&mut concat, &mut concat_has_expr, &mut items);
                match other {
                    Node::Element(el) => items.push(FragmentItem::Element(el.id)),
                    Node::SlotElementLegacy(el) => {
                        items.push(FragmentItem::SlotElementLegacy(el.id))
                    }
                    Node::SvelteFragmentLegacy(el) => {
                        items.push(FragmentItem::SvelteFragmentLegacy(el.id))
                    }
                    Node::ComponentNode(cn) => items.push(FragmentItem::ComponentNode(cn.id)),
                    Node::IfBlock(block) => items.push(FragmentItem::IfBlock(block.id)),
                    Node::EachBlock(block) => items.push(FragmentItem::EachBlock(block.id)),
                    Node::RenderTag(tag) => items.push(FragmentItem::RenderTag(tag.id)),
                    Node::HtmlTag(tag) => items.push(FragmentItem::HtmlTag(tag.id)),
                    Node::KeyBlock(block) => items.push(FragmentItem::KeyBlock(block.id)),
                    Node::SvelteElement(el) => items.push(FragmentItem::SvelteElement(el.id)),
                    Node::SvelteBoundary(b) => items.push(FragmentItem::SvelteBoundary(b.id)),
                    Node::AwaitBlock(block) => items.push(FragmentItem::AwaitBlock(block.id)),
                    _ => {}
                }
            }
        }
    }

    flush(&mut concat, &mut concat_has_expr, &mut items);
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
// Whitespace utilities — operate on ASCII [ \t\r\n] to match Svelte's patterns.
// Return Cow to avoid allocation when the string is unchanged.
// ---------------------------------------------------------------------------

fn is_ws_byte(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n')
}

fn is_ws_only(s: &str) -> bool {
    !s.is_empty() && s.as_bytes().iter().copied().all(is_ws_byte)
}

fn ends_with_ws(s: &str) -> bool {
    s.as_bytes().last().copied().is_some_and(is_ws_byte)
}

/// Count leading `[ \t\r\n]` bytes.
fn leading_ws_len(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .take_while(|&&byte| is_ws_byte(byte))
        .count()
}

/// Count trailing `[ \t\r\n]` bytes.
fn trailing_ws_len(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .rev()
        .take_while(|&&byte| is_ws_byte(byte))
        .count()
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
    use svelte_ast::{AstStore, ExpressionTag, NodeId, Text};
    use svelte_span::Span;

    fn text_node(start: u32, end: u32) -> Node {
        Node::Text(Text {
            id: NodeId(0), // overwritten by store.push
            span: Span { start, end },
            decoded: None,
        })
    }

    fn expr_node() -> Node {
        Node::ExpressionTag(ExpressionTag {
            id: NodeId(0), // overwritten by store.push
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
        assert_eq!(
            &*replace_leading_ws(Cow::Borrowed("\n  hello"), " "),
            " hello"
        );
        assert_eq!(
            &*replace_leading_ws(Cow::Borrowed("\n  hello"), ""),
            "hello"
        );
        assert_eq!(&*replace_leading_ws(Cow::Borrowed("hello"), " "), "hello");
    }

    #[test]
    fn test_replace_trailing_ws() {
        assert_eq!(
            &*replace_trailing_ws(Cow::Borrowed("hello  \n"), " "),
            "hello "
        );
        assert_eq!(
            &*replace_trailing_ws(Cow::Borrowed("hello  \n"), ""),
            "hello"
        );
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
        let mut store = AstStore::new();
        let ids: Vec<NodeId> = nodes.into_iter().map(|n| store.push(n)).collect();
        Component::new(
            source.to_string(),
            Fragment::new(ids),
            store,
            None,
            None,
            None,
        )
    }

    fn collect_text_parts(items: &[FragmentItem], source: &str) -> Vec<String> {
        let mut out = Vec::new();
        for item in items {
            if let FragmentItem::TextConcat { parts, .. } = item {
                for part in parts {
                    if let Some(s) = part.text_value(source) {
                        out.push(s.to_string());
                    }
                }
            }
        }
        out
    }

    #[test]
    fn trim_leading_and_trailing() {
        let src = "\n  hello  \n";
        let comp = make_component(src, vec![text_node(0, src.len() as u32)]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert_eq!(texts, vec!["hello"]);
    }

    #[test]
    fn preserve_internal_newlines() {
        let src = "hello\n  world";
        let comp = make_component(src, vec![text_node(0, src.len() as u32)]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert_eq!(texts, vec!["hello\n  world"]);
    }

    #[test]
    fn tabs_and_crlf() {
        let src = "\r\n\thello\r\n";
        let comp = make_component(src, vec![text_node(0, src.len() as u32)]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert_eq!(texts, vec!["hello"]);
    }

    #[test]
    fn pure_whitespace_only_is_removed() {
        let src = "  \n\t  ";
        let comp = make_component(src, vec![text_node(0, src.len() as u32)]);
        let items = build_items(&comp.fragment, &comp, false, false, false, &comp.store);
        assert!(items.is_empty());
    }

    #[test]
    fn expr_neighbor_preserves_ws() {
        let src = "{expr}\n  hello";
        let comp = make_component(src, vec![expr_node(), text_node(6, src.len() as u32)]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert_eq!(texts, vec!["\n  hello"]);
    }

    #[test]
    fn expr_next_preserves_trailing_ws() {
        let src = "hello  \n{expr}";
        let comp = make_component(src, vec![text_node(0, 8), expr_node()]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert_eq!(texts, vec!["hello  \n"]);
    }

    #[test]
    fn adjacent_text_nodes_no_double_space() {
        let src = "\n\n";
        let comp = make_component(src, vec![text_node(0, 1), text_node(1, 2)]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert!(texts.is_empty());
    }

    #[test]
    fn ws_between_non_expr_nodes_collapses_to_space() {
        let src = "hello\n\n  world";
        let comp = make_component(src, vec![text_node(0, src.len() as u32)]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert_eq!(texts, vec!["hello\n\n  world"]);
    }

    #[test]
    fn text_between_expressions_preserves_all_ws() {
        let src = "{a}\n  \n{b}";
        let comp = make_component(src, vec![expr_node(), text_node(3, 7), expr_node()]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert_eq!(texts, vec!["\n  \n"]);
    }

    #[test]
    fn text_after_expr_before_non_expr() {
        let src = "{expr}\n  hello\n  x";
        let comp = make_component(src, vec![expr_node(), text_node(6, 16), text_node(16, 18)]);
        let texts = collect_text_parts(
            &build_items(&comp.fragment, &comp, false, false, false, &comp.store),
            &comp.source,
        );
        assert_eq!(texts, vec!["\n  hello ", "x"]);
    }

    #[test]
    fn no_alloc_when_text_unchanged() {
        // Text without any boundary whitespace: should stay Cow::Borrowed
        let raw = "hello";
        let result = trim_text(
            raw,
            false,
            false,
            Some(&expr_node()),
            Some(&expr_node()),
            false,
        );
        assert!(matches!(result, Cow::Borrowed("hello")));
    }

    #[test]
    fn no_alloc_for_first_without_leading_ws() {
        let raw = "hello world";
        let result = trim_text(raw, true, true, None, None, false);
        assert!(matches!(result, Cow::Borrowed("hello world")));
    }

    #[test]
    fn unchanged_text_still_uses_text_span_after_source_hoisting() {
        let src = "hello";
        let comp = make_component(src, vec![text_node(0, src.len() as u32)]);
        let items = build_items(&comp.fragment, &comp, false, false, false, &comp.store);
        let FragmentItem::TextConcat { parts, has_expr } = &items[0] else {
            panic!("expected TextConcat");
        };
        assert!(!has_expr);
        assert!(
            matches!(parts.as_slice(), [LoweredTextPart::TextSpan(span)] if *span == Span::new(0, 5))
        );
    }

    #[test]
    fn svg_context_removes_collapsed_whitespace() {
        // In SVG, whitespace between elements collapses to " " then is dropped
        use svelte_ast::{Element, Fragment};

        let src = "<line />\n\t<rect />";
        // Simulate: [Element, Text("\n\t"), Element]
        let el1 = Node::Element(Element {
            id: NodeId(0),
            span: Span::new(0, 8),
            name: "line".into(),
            attributes: vec![],
            fragment: Fragment::new(vec![]),
            self_closing: true,
        });
        let el2 = Node::Element(Element {
            id: NodeId(0),
            span: Span::new(10, 17),
            name: "rect".into(),
            attributes: vec![],
            fragment: Fragment::new(vec![]),
            self_closing: true,
        });
        let text = text_node(8, 10);
        let comp = make_component(src, vec![el1, text, el2]);

        // Without can_remove_entirely: whitespace collapses to " " and is kept
        let items_html = build_items(&comp.fragment, &comp, false, false, false, &comp.store);
        let texts_html = collect_text_parts(&items_html, &comp.source);
        assert_eq!(texts_html, vec![" "]);

        // With can_remove_entirely: whitespace is dropped entirely
        let items_svg = build_items(&comp.fragment, &comp, false, true, false, &comp.store);
        let texts_svg = collect_text_parts(&items_svg, &comp.source);
        assert!(
            texts_svg.is_empty(),
            "SVG context should drop inter-element whitespace"
        );
    }
}
