use crate::{Element, ExpressionFlags, IfBlock, Interpolation, VirtualConcatenation};

#[derive(Debug, Clone, Copy)]
pub struct NodeMetadata {
    pub dynamic: bool,
    pub expression_flags: Option<ExpressionFlags>,
}

impl Default for NodeMetadata {
    fn default() -> Self {
        Self {
            dynamic: false,
            expression_flags: None,
        }
    }
}

impl NodeMetadata {
    pub fn mark_dynamic(&mut self) {
        self.dynamic = true;
    }
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

impl<'a> WithMetadata for IfBlock<'a> {
    fn get_metadata(&self) -> NodeMetadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.metadata = Some(metadata);
    }
}
