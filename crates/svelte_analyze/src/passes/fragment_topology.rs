use smallvec::SmallVec;

use svelte_ast::{AstStore, Attribute, Component, Node, NodeId};

use crate::types::data::{AnalysisData, FragmentKey};

/// Build per-fragment side tables:
/// - `template_topology.node_fragment(id)` — parent fragment for every node
/// - `lowered_fragment(key)` — ordered `NodeId`s with hoisted children filtered out
/// - `fragment_blockers(key)` — async blocker indices per fragment
/// - `debug_tags.by_fragment` / `title_elements.by_fragment`
/// - `snippets.component_snippets` / `snippets.component_named_slots`
/// - `elements.flags.svelte_fragment_slots`
/// - `output.custom_element_slot_names`
pub(crate) fn build(component: &Component, data: &mut AnalysisData) {
    visit_nodes(
        &component.fragment.nodes,
        FragmentKey::Root,
        component,
        data,
        &component.store,
    );
}

fn visit_nodes(
    nodes: &[NodeId],
    key: FragmentKey,
    component: &Component,
    data: &mut AnalysisData,
    store: &AstStore,
) {
    for &id in nodes {
        data.template
            .template_topology
            .record_node_fragment(id, key);
    }

    let inside_head = matches!(key, FragmentKey::SvelteHeadBody(_));

    let items = build_items(nodes, inside_head, store);

    if data.script.blocker_data.has_async {
        let mut blockers = SmallVec::<[u32; 2]>::new();
        for &item_id in &items {
            if matches!(store.get(item_id), Node::ExpressionTag(_)) {
                for idx in data.expression_blockers(item_id) {
                    blockers.push(idx);
                }
            }
        }
        if !blockers.is_empty() {
            blockers.sort_unstable();
            blockers.dedup();
            data.template.fragment_blockers.insert(key, blockers);
        }
    }

    data.template.lowered_fragments.insert(key, items);

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
                visit_nodes(
                    &el.fragment.nodes,
                    FragmentKey::Element(el.id),
                    component,
                    data,
                    store,
                );
            }
            Node::SlotElementLegacy(el) => {
                record_custom_element_slot_name(data, &el.attributes, &component.source);
                visit_nodes(
                    &el.fragment.nodes,
                    FragmentKey::Element(el.id),
                    component,
                    data,
                    store,
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

                let mut default_nodes = SmallVec::<[NodeId; 8]>::new();
                let mut named_groups = SmallVec::<[(NodeId, NodeId); 4]>::new();
                for &child_id in &cn.fragment.nodes {
                    if let Some(slot_el_id) = determine_slot(store.get(child_id)) {
                        named_groups.push((slot_el_id, child_id));
                    } else {
                        default_nodes.push(child_id);
                    }
                }

                visit_nodes(
                    default_nodes.as_slice(),
                    FragmentKey::ComponentNode(cn.id),
                    component,
                    data,
                    store,
                );

                let mut slot_mappings: Vec<(NodeId, FragmentKey)> = Vec::new();
                for (slot_el_id, child_id) in named_groups {
                    let frag_key = FragmentKey::NamedSlot(cn.id, slot_el_id);
                    // `<svelte:fragment slot="name">` unwraps: its children
                    // populate the slot, the wrapper itself is discarded.
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
                    visit_nodes(&slot_nodes, frag_key, component, data, store);
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
                visit_nodes(
                    &el.fragment.nodes,
                    FragmentKey::Element(el.id),
                    component,
                    data,
                    store,
                );
            }
            Node::IfBlock(block) => {
                visit_nodes(
                    &block.consequent.nodes,
                    FragmentKey::IfConsequent(block.id),
                    component,
                    data,
                    store,
                );
                if let Some(alt) = &block.alternate {
                    visit_nodes(
                        &alt.nodes,
                        FragmentKey::IfAlternate(block.id),
                        component,
                        data,
                        store,
                    );
                }
            }
            Node::EachBlock(block) => {
                visit_nodes(
                    &block.body.nodes,
                    FragmentKey::EachBody(block.id),
                    component,
                    data,
                    store,
                );
                if let Some(fb) = &block.fallback {
                    visit_nodes(
                        &fb.nodes,
                        FragmentKey::EachFallback(block.id),
                        component,
                        data,
                        store,
                    );
                }
            }
            Node::SnippetBlock(block) => {
                visit_nodes(
                    &block.body.nodes,
                    FragmentKey::SnippetBody(block.id),
                    component,
                    data,
                    store,
                );
            }
            Node::KeyBlock(block) => {
                visit_nodes(
                    &block.fragment.nodes,
                    FragmentKey::KeyBlockBody(block.id),
                    component,
                    data,
                    store,
                );
            }
            Node::SvelteHead(head) => {
                visit_nodes(
                    &head.fragment.nodes,
                    FragmentKey::SvelteHeadBody(head.id),
                    component,
                    data,
                    store,
                );
            }
            Node::SvelteElement(el) => {
                visit_nodes(
                    &el.fragment.nodes,
                    FragmentKey::SvelteElementBody(el.id),
                    component,
                    data,
                    store,
                );
            }
            Node::SvelteBoundary(b) => {
                visit_nodes(
                    &b.fragment.nodes,
                    FragmentKey::SvelteBoundaryBody(b.id),
                    component,
                    data,
                    store,
                );
            }
            Node::AwaitBlock(block) => {
                if let Some(ref p) = block.pending {
                    visit_nodes(
                        &p.nodes,
                        FragmentKey::AwaitPending(block.id),
                        component,
                        data,
                        store,
                    );
                }
                if let Some(ref t) = block.then {
                    visit_nodes(
                        &t.nodes,
                        FragmentKey::AwaitThen(block.id),
                        component,
                        data,
                        store,
                    );
                }
                if let Some(ref c) = block.catch {
                    visit_nodes(
                        &c.nodes,
                        FragmentKey::AwaitCatch(block.id),
                        component,
                        data,
                        store,
                    );
                }
            }
            Node::SvelteWindow(_)
            | Node::SvelteDocument(_)
            | Node::SvelteBody(_)
            | Node::HtmlTag(_)
            | Node::Text(_)
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

/// Emit filtered `NodeId`s for a fragment: drop hoisted/error nodes, keep
/// original order. Boundary-whitespace trimming is codegen's concern and
/// lives in `codegen/fragment/prepare`; analyze consumers (css_prune,
/// needs_var, symbol-ref walks) treat `Text` uniformly regardless of content,
/// so pre-trimming here would be wasted work.
fn build_items(nodes: &[NodeId], inside_head: bool, store: &AstStore) -> Vec<NodeId> {
    let mut out = Vec::with_capacity(nodes.len());
    for &id in nodes {
        if !is_hoisted(store.get(id), inside_head) {
            out.push(id);
        }
    }
    out
}

#[inline]
fn is_hoisted(node: &Node, inside_head: bool) -> bool {
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
