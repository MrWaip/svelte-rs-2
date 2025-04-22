use oxc_syntax::scope::ScopeId;

use crate::{EachBlock, Element, HirStore, IfBlock, NodeId, OwnerId, Template};

#[derive(Debug)]
pub enum OwnerNode<'hir> {
    Element(&'hir Element<'hir>),
    Template(&'hir Template),
    IfBlock(&'hir IfBlock),
    EachBlock(&'hir EachBlock),
    Phantom,
}

impl OwnerNode<'_> {
    pub fn is_if_block(&self) -> bool {
        matches!(self, OwnerNode::IfBlock(_))
    }

    pub fn as_element(&self) -> Option<&Element> {
        match self {
            OwnerNode::Element(element) => Some(element),
            _ => None,
        }
    }

    pub fn as_template(&self) -> Option<&Template> {
        match self {
            OwnerNode::Template(template) => Some(template),
            _ => None,
        }
    }

    pub fn as_if_block(&self) -> Option<&IfBlock> {
        match self {
            OwnerNode::IfBlock(if_block) => Some(if_block),
            _ => None,
        }
    }

    pub fn first(&self) -> Option<&NodeId> {
        match self {
            OwnerNode::Element(it) => it.node_ids.first(),
            OwnerNode::Template(it) => it.node_ids.first(),
            OwnerNode::IfBlock(it) => it.consequent.first(),
            OwnerNode::EachBlock(it) => it.node_ids.first(),
            OwnerNode::Phantom => todo!(),
        }
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            OwnerNode::Element(it) => it.node_id,
            OwnerNode::Template(_) => HirStore::TEMPLATE_NODE_ID,
            OwnerNode::IfBlock(it) => it.node_id,
            OwnerNode::EachBlock(it) => it.node_id,
            OwnerNode::Phantom => todo!(),
        }
    }

    pub fn iter_nodes_rev(&self) -> Box<dyn Iterator<Item = &NodeId> + '_> {
        match self {
            OwnerNode::Element(it) => Box::new(it.node_ids.iter().rev()),
            OwnerNode::Template(it) => Box::new(it.node_ids.iter().rev()),
            OwnerNode::EachBlock(it) => Box::new(it.node_ids.iter().rev()),
            OwnerNode::IfBlock(it) => {
                if let Some(alternate) = &it.alternate {
                    return Box::new(alternate.iter().rev().chain(it.consequent.iter().rev()));
                }

                Box::new(it.consequent.iter().rev())
            }
            _ => todo!(),
        }
    }

    /// if a component/snippet/each block starts with text,
    /// we need to add an anchor comment so that its text node doesn't get fused with its surroundings
    pub fn is_require_next(&self) -> bool {
        // (parent.type === 'Fragment' ||
        // 		parent.type === 'SnippetBlock' ||
        // 		parent.type === 'EachBlock' ||
        // 		parent.type === 'SvelteComponent' ||
        // 		parent.type === 'SvelteBoundary' ||
        // 		parent.type === 'Component' ||
        // 		parent.type === 'SvelteSelf') &&

        matches!(self, OwnerNode::EachBlock(_) | OwnerNode::Template(_))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, OwnerNode::Element(_))
    }

    pub fn scope_id(&self) -> Option<ScopeId> {
        match self {
            OwnerNode::Element(it) => it.scope_id.get(),
            OwnerNode::Template(it) => it.scope_id.get(),
            OwnerNode::IfBlock(it) => it.scope_id.get(),
            OwnerNode::EachBlock(it) => it.scope_id.get(),
            OwnerNode::Phantom => todo!(),
        }
    }
}
