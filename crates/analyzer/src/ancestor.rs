use oxc_semantic::NodeId;

#[derive(Debug, Clone)]
pub enum Ancestor<'b> {
    Template(NodeId),
    IfBlock(NodeId),
    Element(NodeId),
    Nope(&'b str),
}

impl<'a> Ancestor<'a> {
    pub fn is_template(&self) -> bool {
        return matches!(self, Ancestor::Template(_));
    }

    pub fn is_fragment_owner(&self) -> bool {
        return matches!(self, Ancestor::Template(_) | Ancestor::IfBlock(_));
    }

    pub fn get_node_id(&self) -> NodeId {
        return match self {
            Ancestor::Template(node_id) => *node_id,
            Ancestor::IfBlock(node_id) => *node_id,
            Ancestor::Element(node_id) => *node_id,
            Ancestor::Nope(_) => unreachable!(),
        };
    }
}
