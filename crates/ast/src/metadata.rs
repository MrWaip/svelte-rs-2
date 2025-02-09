use crate::{Element, Interpolation, VirtualConcatenation};

#[derive(Debug, Clone, Copy, Default)]
pub struct ElementMetadata {
    pub has_dynamic_nodes: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InterpolationMetadata {
    pub has_reactivity: bool,
    pub has_call_expression: bool,
}

impl InterpolationMetadata {
    pub fn add(&mut self, other: InterpolationMetadata) {
        self.has_call_expression = self.has_call_expression || other.has_call_expression;
        self.has_reactivity = self.has_reactivity || other.has_reactivity;
    }
}

pub trait WithMetadata {
    type Metadata;

    fn get_metadata(&self) -> Self::Metadata;

    fn set_metadata(&mut self, metadata: Self::Metadata);
}

impl<'a> WithMetadata for Element<'a> {
    type Metadata = ElementMetadata;

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: Self::Metadata) {
        self.metadata = Some(metadata);
    }
}

impl<'a> WithMetadata for Interpolation<'a> {
    type Metadata = InterpolationMetadata;

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: Self::Metadata) {
        self.metadata = Some(metadata);
    }
}

impl<'a> WithMetadata for VirtualConcatenation<'a> {
    type Metadata = InterpolationMetadata;

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata
    }

    fn set_metadata(&mut self, metadata: Self::Metadata) {
        self.metadata = metadata;
    }
}
