use crate::{AttributeId, NodeId, OwnerId};

#[derive(Debug)]
pub struct Element<'hir> {
    pub node_id: NodeId,
    pub owner_id: OwnerId,
    pub name: &'hir str,
    pub attributes: Vec<AttributeId>,
    pub node_ids: Vec<NodeId>,
    pub self_closing: bool,
    pub kind: ElementKind,
}

impl<'hir> Element<'hir> {
    pub fn is_noscript(&self) -> bool {
        return matches!(self.kind, ElementKind::Noscript);
    }
}

#[derive(Debug)]
pub enum ElementKind {
    Noscript,
    Unknown,
}

impl ElementKind {
    pub fn from_str(name: &str) -> Self {
        match name {
            "noscript" => Self::Noscript,
            _ => Self::Unknown,
        }
    }
}
