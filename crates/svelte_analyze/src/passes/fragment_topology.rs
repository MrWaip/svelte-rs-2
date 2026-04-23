use smallvec::SmallVec;

use svelte_ast::{AstStore, Attribute, Component, Fragment, FragmentRole, Node, NodeId};

use crate::types::data::{AnalysisData, FragmentLayout};

pub(crate) fn build(component: &Component, data: &mut AnalysisData) {
    visit(&component.fragment, None, component, data, &component.store);
}

fn visit(
    fragment: &Fragment,
    owner: Option<NodeId>,
    component: &Component,
    data: &mut AnalysisData,
    store: &AstStore,
) {
    for &id in &fragment.nodes {
        data.template
            .template_topology
            .record_node_fragment(id, fragment.id);
    }

    let inside_head = fragment.role == FragmentRole::SvelteHeadBody;
    let items = build_items(&fragment.nodes, inside_head, store);

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
            data.template
                .insert_fragment_blockers_by_id(fragment.id, blockers);
        }
    }

    let mut debug_ids: Option<Vec<NodeId>> = None;
    let mut title_ids: Option<Vec<NodeId>> = None;

    for &id in &fragment.nodes {
        match store.get(id) {
            Node::DebugTag(tag) => {
                debug_ids.get_or_insert_with(Vec::new).push(tag.id);
            }
            Node::Element(el) => {
                if inside_head && el.name == "title" {
                    title_ids.get_or_insert_with(Vec::new).push(el.id);
                }
                visit(&el.fragment, Some(el.id), component, data, store);
            }
            Node::SlotElementLegacy(el) => {
                record_custom_element_slot_name(data, &el.attributes, &component.source);
                visit(&el.fragment, Some(el.id), component, data, store);
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

                visit(&cn.fragment, Some(cn.id), component, data, store);

                for slot in &cn.legacy_slots {
                    let wrapper_id = slot.fragment.nodes[0];
                    let slot_items: SmallVec<[NodeId; 4]> = match store.get(wrapper_id) {
                        Node::SvelteFragmentLegacy(el) => {
                            data.elements.flags.svelte_fragment_slots.insert(wrapper_id);
                            el.fragment.nodes.iter().copied().collect()
                        }
                        _ => {
                            let mut v = SmallVec::new();
                            v.push(wrapper_id);
                            v
                        }
                    };
                    visit_slot(slot, &slot_items, cn.id, component, data, store);
                }
            }
            Node::SvelteFragmentLegacy(el) => {
                visit(&el.fragment, Some(el.id), component, data, store);
            }
            Node::IfBlock(block) => {
                visit(&block.consequent, Some(block.id), component, data, store);
                if let Some(alt) = &block.alternate {
                    visit(alt, Some(block.id), component, data, store);
                }
            }
            Node::EachBlock(block) => {
                visit(&block.body, Some(block.id), component, data, store);
                if let Some(fb) = &block.fallback {
                    visit(fb, Some(block.id), component, data, store);
                }
            }
            Node::SnippetBlock(block) => {
                visit(&block.body, Some(block.id), component, data, store);
            }
            Node::KeyBlock(block) => {
                visit(&block.fragment, Some(block.id), component, data, store);
            }
            Node::SvelteHead(head) => {
                visit(&head.fragment, Some(head.id), component, data, store);
            }
            Node::SvelteElement(el) => {
                visit(&el.fragment, Some(el.id), component, data, store);
            }
            Node::SvelteBoundary(b) => {
                visit(&b.fragment, Some(b.id), component, data, store);
            }
            Node::AwaitBlock(block) => {
                if let Some(p) = &block.pending {
                    visit(p, Some(block.id), component, data, store);
                }
                if let Some(t) = &block.then {
                    visit(t, Some(block.id), component, data, store);
                }
                if let Some(c) = &block.catch {
                    visit(c, Some(block.id), component, data, store);
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

    data.template.insert_fragment_layout(
        fragment.id,
        FragmentLayout {
            items,
            owner,
            role: fragment.role,
        },
    );

    if let Some(ids) = debug_ids {
        data.template
            .debug_tags
            .by_fragment
            .insert(fragment.id, ids);
    }
    if let Some(ids) = title_ids {
        data.template
            .title_elements
            .by_fragment
            .insert(fragment.id, ids);
    }
}

fn visit_slot(
    slot: &svelte_ast::LegacySlot,
    items: &[NodeId],
    component_id: NodeId,
    component: &Component,
    data: &mut AnalysisData,
    store: &AstStore,
) {
    for &id in items {
        data.template
            .template_topology
            .record_node_fragment(id, slot.fragment.id);
    }

    let lowered = build_items(items, false, store);

    if data.script.blocker_data.has_async {
        let mut blockers = SmallVec::<[u32; 2]>::new();
        for &item_id in &lowered {
            if matches!(store.get(item_id), Node::ExpressionTag(_)) {
                for idx in data.expression_blockers(item_id) {
                    blockers.push(idx);
                }
            }
        }
        if !blockers.is_empty() {
            blockers.sort_unstable();
            blockers.dedup();
            data.template
                .insert_fragment_blockers_by_id(slot.fragment.id, blockers);
        }
    }

    let mut debug_ids: Option<Vec<NodeId>> = None;

    for &id in items {
        match store.get(id) {
            Node::DebugTag(tag) => {
                debug_ids.get_or_insert_with(Vec::new).push(tag.id);
            }
            Node::Element(el) => visit(&el.fragment, Some(el.id), component, data, store),
            Node::SlotElementLegacy(el) => {
                record_custom_element_slot_name(data, &el.attributes, &component.source);
                visit(&el.fragment, Some(el.id), component, data, store);
            }
            Node::ComponentNode(cn) => visit(&cn.fragment, Some(cn.id), component, data, store),
            Node::SvelteFragmentLegacy(el) => {
                visit(&el.fragment, Some(el.id), component, data, store)
            }
            _ => {}
        }
    }

    data.template.insert_fragment_layout(
        slot.fragment.id,
        FragmentLayout {
            items: lowered,
            owner: Some(component_id),
            role: slot.fragment.role,
        },
    );

    if let Some(ids) = debug_ids {
        data.template
            .debug_tags
            .by_fragment
            .insert(slot.fragment.id, ids);
    }
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

fn build_items(nodes: &[NodeId], inside_head: bool, store: &AstStore) -> Vec<NodeId> {
    let mut out = Vec::with_capacity(nodes.len());
    for &id in nodes {
        match store.get(id) {
            Node::SnippetBlock(_) => {}
            Node::ConstTag(_) | Node::DebugTag(_) => {}
            Node::Element(el) if inside_head && el.name == "title" => {}
            _ => out.push(id),
        }
    }
    out
}
