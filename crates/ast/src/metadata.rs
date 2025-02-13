use crate::{
    ClassDirective, Concatenation, Element, ExpressionAttribute, ExpressionAttributeValue,
    Fragment, Interpolation, VirtualConcatenation,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct ElementMetadata {
    pub has_dynamic_nodes: bool,
    pub need_reset: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InterpolationMetadata {
    pub has_reactivity: bool,
    pub has_call_expression: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AttributeMetadata {
    pub has_reactivity: bool,
}

impl InterpolationMetadata {
    pub fn add(&mut self, other: InterpolationMetadata) {
        self.has_call_expression = self.has_call_expression || other.has_call_expression;
        self.has_reactivity = self.has_reactivity || other.has_reactivity;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FragmentMetadata {
    pub need_start_with_next: bool,
    pub anchor: FragmentAnchor,
    pub is_empty: bool
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FragmentAnchor {
    #[default]
    Fragment,
    Text,
    TextInline,
    Element,
    Comment,
}

impl FragmentAnchor {
    pub fn is_text(&self) -> bool {
        return matches!(self, FragmentAnchor::Text);
    }

    pub fn is_element(&self) -> bool {
        return matches!(self, FragmentAnchor::Element);
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

impl<'a> WithMetadata for ClassDirective<'a> {
    type Metadata = AttributeMetadata;

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: Self::Metadata) {
        self.metadata = Some(metadata)
    }
}

impl<'a> WithMetadata for Concatenation<'a> {
    type Metadata = AttributeMetadata;

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: Self::Metadata) {
        self.metadata = Some(metadata)
    }
}

impl<'a> WithMetadata for ExpressionAttributeValue<'a> {
    type Metadata = AttributeMetadata;

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: Self::Metadata) {
        self.metadata = Some(metadata)
    }
}

impl<'a> WithMetadata for ExpressionAttribute<'a> {
    type Metadata = AttributeMetadata;

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: Self::Metadata) {
        self.metadata = Some(metadata)
    }
}

impl<'a> WithMetadata for Fragment<'a> {
    type Metadata = FragmentMetadata;

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: Self::Metadata) {
        self.metadata = Some(metadata)
    }
}
