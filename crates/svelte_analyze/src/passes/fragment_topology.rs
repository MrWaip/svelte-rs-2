use smallvec::SmallVec;

use svelte_ast::{AstStore, Attribute, Component, FragmentRole, Node, NodeId};

use crate::types::data::AnalysisData;

pub(crate) fn build(component: &Component, data: &mut AnalysisData) {
    visit_fragment(component.root, component, data);
}

fn visit_fragment(
    fragment_id: svelte_ast::FragmentId,
    component: &Component,
    data: &mut AnalysisData,
) {
    let store = &component.store;
    let fragment = store.fragment(fragment_id);
    let inside_head = fragment.role == FragmentRole::SvelteHeadBody;

    if data.script.blocker_data.has_async {
        let mut blockers = SmallVec::<[u32; 2]>::new();
        for &id in &fragment.nodes {
            if matches!(store.get(id), Node::ExpressionTag(_)) {
                for idx in data.expression_blockers(id) {
                    blockers.push(idx);
                }
            }
        }
        if !blockers.is_empty() {
            blockers.sort_unstable();
            blockers.dedup();
            data.template
                .insert_fragment_blockers_by_id(fragment_id, blockers);
        }
    }

    let mut debug_ids: Option<Vec<NodeId>> = None;
    let mut title_ids: Option<Vec<NodeId>> = None;

    let nodes = fragment.nodes.clone();
    for id in nodes {
        match store.get(id) {
            Node::DebugTag(tag) => {
                debug_ids.get_or_insert_with(Vec::new).push(tag.id);
            }
            Node::Element(el) => {
                if inside_head && el.name == "title" {
                    title_ids.get_or_insert_with(Vec::new).push(el.id);
                }
                visit_fragment(el.fragment, component, data);
            }
            Node::SlotElementLegacy(el) => {
                record_custom_element_slot_name(data, &el.attributes, &component.source);
                visit_fragment(el.fragment, component, data);
            }
            Node::ComponentNode(cn) => {
                let snippets: Vec<_> = store
                    .fragment(cn.fragment)
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

                let cn_id = cn.id;
                let cn_fragment = cn.fragment;
                let legacy_slots: Vec<_> = cn
                    .legacy_slots
                    .iter()
                    .map(|s| {
                        let wrapper_id = store.fragment_nodes(s.fragment)[0];
                        (s.fragment, wrapper_id)
                    })
                    .collect();

                visit_fragment(cn_fragment, component, data);

                for (slot_fid, wrapper_id) in legacy_slots {
                    let slot_items: SmallVec<[NodeId; 4]> = match store.get(wrapper_id) {
                        Node::SvelteFragmentLegacy(el) => {
                            data.elements.flags.svelte_fragment_slots.insert(wrapper_id);
                            store.fragment_nodes(el.fragment).iter().copied().collect()
                        }
                        _ => {
                            let mut v = SmallVec::new();
                            v.push(wrapper_id);
                            v
                        }
                    };
                    visit_slot(slot_fid, &slot_items, cn_id, component, data);
                }
            }
            Node::SvelteFragmentLegacy(el) => {
                visit_fragment(el.fragment, component, data);
            }
            Node::IfBlock(block) => {
                visit_fragment(block.consequent, component, data);
                if let Some(alt) = block.alternate {
                    visit_fragment(alt, component, data);
                }
            }
            Node::EachBlock(block) => {
                visit_fragment(block.body, component, data);
                if let Some(fb) = block.fallback {
                    visit_fragment(fb, component, data);
                }
            }
            Node::SnippetBlock(block) => {
                visit_fragment(block.body, component, data);
            }
            Node::KeyBlock(block) => {
                visit_fragment(block.fragment, component, data);
            }
            Node::SvelteHead(head) => {
                visit_fragment(head.fragment, component, data);
            }
            Node::SvelteElement(el) => {
                visit_fragment(el.fragment, component, data);
            }
            Node::SvelteBoundary(b) => {
                visit_fragment(b.fragment, component, data);
            }
            Node::AwaitBlock(block) => {
                if let Some(p) = block.pending {
                    visit_fragment(p, component, data);
                }
                if let Some(t) = block.then {
                    visit_fragment(t, component, data);
                }
                if let Some(c) = block.catch {
                    visit_fragment(c, component, data);
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
        data.template
            .debug_tags
            .by_fragment
            .insert(fragment_id, ids);
    }
    if let Some(ids) = title_ids {
        data.template
            .title_elements
            .by_fragment
            .insert(fragment_id, ids);
    }
}

fn visit_slot(
    slot_fid: svelte_ast::FragmentId,
    items: &[NodeId],
    component_id: NodeId,
    component: &Component,
    data: &mut AnalysisData,
) {
    let store = &component.store;

    if data.script.blocker_data.has_async {
        let mut blockers = SmallVec::<[u32; 2]>::new();
        for &id in items {
            if matches!(store.get(id), Node::ExpressionTag(_)) {
                for idx in data.expression_blockers(id) {
                    blockers.push(idx);
                }
            }
        }
        if !blockers.is_empty() {
            blockers.sort_unstable();
            blockers.dedup();
            data.template
                .insert_fragment_blockers_by_id(slot_fid, blockers);
        }
    }

    let mut debug_ids: Option<Vec<NodeId>> = None;

    for &id in items {
        match store.get(id) {
            Node::DebugTag(tag) => {
                debug_ids.get_or_insert_with(Vec::new).push(tag.id);
            }
            Node::Element(el) => visit_fragment(el.fragment, component, data),
            Node::SlotElementLegacy(el) => {
                record_custom_element_slot_name(data, &el.attributes, &component.source);
                visit_fragment(el.fragment, component, data);
            }
            Node::ComponentNode(cn) => visit_fragment(cn.fragment, component, data),
            Node::SvelteFragmentLegacy(el) => visit_fragment(el.fragment, component, data),
            _ => {}
        }
    }

    let _ = component_id;

    if let Some(ids) = debug_ids {
        data.template.debug_tags.by_fragment.insert(slot_fid, ids);
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

pub fn fragment_items(store: &AstStore, fragment_id: svelte_ast::FragmentId) -> Vec<NodeId> {
    let fragment = store.fragment(fragment_id);
    let inside_head = fragment.role == FragmentRole::SvelteHeadBody;
    let nodes: &[NodeId] = if fragment.role == FragmentRole::NamedSlot
        && fragment.nodes.len() == 1
        && matches!(store.get(fragment.nodes[0]), Node::SvelteFragmentLegacy(_))
    {
        let Node::SvelteFragmentLegacy(el) = store.get(fragment.nodes[0]) else {
            unreachable!()
        };
        &store.fragment(el.fragment).nodes
    } else {
        &fragment.nodes
    };
    nodes
        .iter()
        .copied()
        .filter(|&id| match store.get(id) {
            Node::SnippetBlock(_) => false,
            Node::ConstTag(_) | Node::DebugTag(_) => false,
            Node::Element(el) if inside_head && el.name == "title" => false,
            _ => true,
        })
        .collect()
}
