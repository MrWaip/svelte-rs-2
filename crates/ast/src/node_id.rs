use oxc_syntax::node::NodeId;

use crate::{Element, Fragment};

impl Element<'_> {
    pub fn node_id(&self) -> NodeId {
        return self.node_id.unwrap();
    }

    pub fn set_node_id(&mut self, node_id: NodeId) {
        self.node_id = Some(node_id);
    }
}

impl Fragment<'_> {
    pub fn node_id(&self) -> NodeId {
        return self.node_id.unwrap();
    }

    pub fn set_node_id(&mut self, node_id: NodeId) {
        self.node_id = Some(node_id);
    }
}
