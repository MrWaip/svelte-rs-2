use svelte_ast::{Component, Fragment, Node};

use crate::data::{AnalysisData, FragmentItem, FragmentKey};

/// Pre-compute which IfBlocks have an elseif as their alternate.
pub fn detect_elseif(component: &Component, data: &mut AnalysisData) {
    walk_fragment(&component.fragment, data);
}

fn walk_fragment(fragment: &Fragment, data: &mut AnalysisData) {
    for node in &fragment.nodes {
        match node {
            Node::Element(el) => walk_fragment(&el.fragment, data),
            Node::IfBlock(b) => {
                walk_fragment(&b.consequent, data);
                if let Some(alt) = &b.alternate {
                    walk_fragment(alt, data);

                    let alt_key = FragmentKey::IfAlternate(b.id);
                    let is_elseif = data.lowered_fragments.get(&alt_key).is_some_and(|lf| {
                        lf.items.len() == 1
                            && matches!(&lf.items[0], FragmentItem::IfBlock(id)
                                if alt.nodes.iter().any(|n| matches!(n, Node::IfBlock(ib) if ib.id == *id && ib.elseif)))
                    });
                    if is_elseif {
                        data.alt_is_elseif.insert(b.id);
                    }
                }
            }
            Node::EachBlock(b) => {
                walk_fragment(&b.body, data);
                if let Some(fb) = &b.fallback {
                    walk_fragment(fb, data);
                }
            }
            _ => {}
        }
    }
}
