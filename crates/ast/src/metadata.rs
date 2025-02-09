use crate::Element;

#[derive(Debug, Clone, Copy)]
pub struct ElementMetadata {
    pub has_dynamic_nodes: bool,
}

impl Default for ElementMetadata {
    fn default() -> Self {
        Self {
            has_dynamic_nodes: false,
        }
    }
}

impl ElementMetadata {}

pub trait WithMetadata {
    fn get_metadata(&self) -> ElementMetadata;

    fn set_metadata(&mut self, metadata: ElementMetadata);
}

impl<'a> WithMetadata for Element<'a> {
    fn get_metadata(&self) -> ElementMetadata {
        self.metadata.unwrap()
    }

    fn set_metadata(&mut self, metadata: ElementMetadata) {
        self.metadata = Some(metadata);
    }
}
