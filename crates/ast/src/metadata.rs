use crate::{Element, IfBlock, Interpolation, Node, ScriptTag, VirtualConcatenation};

#[derive(Debug, Default, Clone, Copy)]
pub struct NodeMetadata {
    pub dynamic: bool,
}

pub trait WithMetadata {
    fn get_metadata(&self) -> NodeMetadata;

    fn set_metadata(&mut self, metadata: NodeMetadata);
}

impl<'a> WithMetadata for Element<'a> {
    fn get_metadata(&self) -> NodeMetadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.metadata = Some(metadata);
    }
}

impl<'a> WithMetadata for Interpolation<'a> {
    fn get_metadata(&self) -> NodeMetadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.metadata = Some(metadata);
    }
}

impl<'a> WithMetadata for VirtualConcatenation<'a> {
    fn get_metadata(&self) -> NodeMetadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.metadata = Some(metadata);
    }
}

impl<'a> WithMetadata for ScriptTag<'a> {
    fn get_metadata(&self) -> NodeMetadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.metadata = Some(metadata);
    }
}

impl<'a> WithMetadata for IfBlock<'a> {
    fn get_metadata(&self) -> NodeMetadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.metadata = Some(metadata);
    }
}

impl<'a> WithMetadata for Node<'a> {
    fn get_metadata(&self) -> NodeMetadata {
        match self {
            Node::Element(element) => element.get_metadata(),
            Node::Text(_) => unreachable!(),
            Node::Interpolation(interpolation) => interpolation.get_metadata(),
            Node::IfBlock(if_block) => if_block.get_metadata(),
            Node::VirtualConcatenation(virtual_concatenation) => {
                virtual_concatenation.get_metadata()
            }
            Node::ScriptTag(script_tag) => script_tag.get_metadata(),
        }
    }

    fn set_metadata(&mut self, metadata: NodeMetadata) {
        match self {
            Node::Element(node) => node.set_metadata(metadata),
            Node::Text(_) => unreachable!(),
            Node::Interpolation(node) => node.set_metadata(metadata),
            Node::IfBlock(node) => node.set_metadata(metadata),
            Node::VirtualConcatenation(node) => node.set_metadata(metadata),
            Node::ScriptTag(node) => node.set_metadata(metadata),
        }
    }
}
