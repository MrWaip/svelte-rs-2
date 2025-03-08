use std::{iter::Chain, slice::Iter};

use crate::{Element, HirStore, IfBlock, NodeId, Template};

#[derive(Debug)]
pub enum OwnerNode<'hir> {
    Element(&'hir Element<'hir>),
    Template(&'hir Template),
    IfBlock(&'hir IfBlock),
    EachBlock,
    Phantom,
}

impl<'hir> OwnerNode<'hir> {
    pub fn is_if_block(&self) -> bool {
        return matches!(self, OwnerNode::IfBlock(_));
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
        return match self {
            OwnerNode::Element(it) => it.node_ids.first(),
            OwnerNode::Template(it) => it.node_ids.first(),
            OwnerNode::IfBlock(it) => it.consequent.first(),
            OwnerNode::EachBlock => todo!(),
            OwnerNode::Phantom => todo!(),
        };
    }

    pub fn node_id(&self) -> NodeId {
        match self {
            OwnerNode::Element(it) => it.node_id,
            OwnerNode::Template(_) => HirStore::TEMPLATE_NODE_ID,
            OwnerNode::IfBlock(it) => it.node_id,
            OwnerNode::EachBlock => todo!(),
            OwnerNode::Phantom => todo!(),
        }
    }

    pub fn iter_nodes_rev(&self) -> Box<dyn Iterator<Item = &NodeId> + '_> {
        return match self {
            OwnerNode::Element(it) => Box::new(it.node_ids.iter().rev()),
            OwnerNode::Template(it) => Box::new(it.node_ids.iter().rev()),
            OwnerNode::IfBlock(it) => {
                if let Some(alternate) = &it.alternate {
                    return Box::new(alternate.iter().rev().chain(it.consequent.iter().rev()));
                }

                return Box::new(it.consequent.iter().rev());
            }
            _ => todo!(),
        };
    }
}
