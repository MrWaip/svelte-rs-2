use svelte_ast::{AstStore, Component, Fragment, Node};

use crate::types::data::{AnalysisData, FragmentKey};

/// Collect `const_tags.by_fragment` before the reactivity fix-point runs so
/// subsequent passes (const-tag sym population, reactivity compute, dynamism)
/// can key off it.
pub(crate) fn collect(component: &Component, data: &mut AnalysisData) {
    walk(
        &component.fragment,
        FragmentKey::Root,
        data,
        &component.store,
    );
}

fn walk(fragment: &Fragment, key: FragmentKey, data: &mut AnalysisData, store: &AstStore) {
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
                walk(&el.fragment, FragmentKey::Element(el.id), data, store);
            }
            Node::ComponentNode(cn) => {
                walk(&cn.fragment, FragmentKey::ComponentNode(cn.id), data, store);
            }
            Node::IfBlock(block) => {
                walk(
                    &block.consequent,
                    FragmentKey::IfConsequent(block.id),
                    data,
                    store,
                );
                if let Some(alt) = &block.alternate {
                    walk(alt, FragmentKey::IfAlternate(block.id), data, store);
                }
            }
            Node::EachBlock(block) => {
                walk(&block.body, FragmentKey::EachBody(block.id), data, store);
                if let Some(fb) = &block.fallback {
                    walk(fb, FragmentKey::EachFallback(block.id), data, store);
                }
            }
            Node::SnippetBlock(block) => {
                walk(&block.body, FragmentKey::SnippetBody(block.id), data, store);
            }
            Node::KeyBlock(block) => {
                walk(
                    &block.fragment,
                    FragmentKey::KeyBlockBody(block.id),
                    data,
                    store,
                );
            }
            Node::SvelteHead(head) => {
                walk(
                    &head.fragment,
                    FragmentKey::SvelteHeadBody(head.id),
                    data,
                    store,
                );
            }
            Node::SvelteElement(el) => {
                walk(
                    &el.fragment,
                    FragmentKey::SvelteElementBody(el.id),
                    data,
                    store,
                );
            }
            Node::SvelteBoundary(b) => {
                walk(
                    &b.fragment,
                    FragmentKey::SvelteBoundaryBody(b.id),
                    data,
                    store,
                );
            }
            Node::AwaitBlock(block) => {
                if let Some(ref p) = block.pending {
                    walk(p, FragmentKey::AwaitPending(block.id), data, store);
                }
                if let Some(ref t) = block.then {
                    walk(t, FragmentKey::AwaitThen(block.id), data, store);
                }
                if let Some(ref c) = block.catch {
                    walk(c, FragmentKey::AwaitCatch(block.id), data, store);
                }
            }
            _ => {}
        }
    }
}
