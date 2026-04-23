use svelte_ast::{AstStore, Component, Fragment, Node};

use crate::types::data::AnalysisData;

pub(crate) fn collect(component: &Component, data: &mut AnalysisData) {
    walk(&component.fragment, data, &component.store);
}

fn walk(fragment: &Fragment, data: &mut AnalysisData, store: &AstStore) {
    let mut const_ids: Option<Vec<svelte_ast::NodeId>> = None;
    for &id in &fragment.nodes {
        if let Some(ct) = store.get(id).as_const_tag() {
            const_ids.get_or_insert_with(Vec::new).push(ct.id);
        }
    }
    if let Some(ids) = const_ids {
        data.template
            .const_tags
            .by_fragment
            .insert(fragment.id, ids);
    }
    for &id in &fragment.nodes {
        match store.get(id) {
            Node::Element(el) => walk(&el.fragment, data, store),
            Node::ComponentNode(cn) => {
                walk(&cn.fragment, data, store);
                for slot in &cn.legacy_slots {
                    walk(&slot.fragment, data, store);
                }
            }
            Node::IfBlock(block) => {
                walk(&block.consequent, data, store);
                if let Some(alt) = &block.alternate {
                    walk(alt, data, store);
                }
            }
            Node::EachBlock(block) => {
                walk(&block.body, data, store);
                if let Some(fb) = &block.fallback {
                    walk(fb, data, store);
                }
            }
            Node::SnippetBlock(block) => walk(&block.body, data, store),
            Node::KeyBlock(block) => walk(&block.fragment, data, store),
            Node::SvelteHead(head) => walk(&head.fragment, data, store),
            Node::SvelteElement(el) => walk(&el.fragment, data, store),
            Node::SvelteBoundary(b) => walk(&b.fragment, data, store),
            Node::AwaitBlock(block) => {
                if let Some(p) = &block.pending {
                    walk(p, data, store);
                }
                if let Some(t) = &block.then {
                    walk(t, data, store);
                }
                if let Some(c) = &block.catch {
                    walk(c, data, store);
                }
            }
            _ => {}
        }
    }
}
