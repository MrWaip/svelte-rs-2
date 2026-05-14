pub(crate) mod builder;
pub mod data;

pub use builder::{build, BindingGroupTable};
pub use data::{
    AttributeSemantics, BoundaryPropEmit, BoundaryPropSemantics, ComponentAttachEmit,
    ComponentAttachSemantics, ComponentBindKind, ComponentBindSemantics, ComponentBindTarget,
    ComponentPropConcatSemantics, ComponentPropExpressionSemantics, ComponentPropMemo,
    ComponentPropSemantics, ComponentSpreadEmit, ComponentSpreadSemantics, ConcatPartEmit,
    DocumentBindSemantics, HtmlConcatPart, HtmlConcatSemantics, TemplateEffect,
    ElementBindPropertyKind, ElementBindSemantics, EventEmit, EventSemantics, HandlerEmit,
    HtmlBindKind, WindowBindSemantics,
};

use svelte_ast::NodeId;

#[derive(Debug, Default, Clone)]
pub struct AttributeSemanticsStore {
    entries: Vec<AttributeSemantics>,
}

impl AttributeSemanticsStore {
    pub(crate) fn new(node_count: u32) -> Self {
        let mut entries = Vec::with_capacity(node_count as usize);
        entries.resize_with(node_count as usize, AttributeSemantics::default);
        Self { entries }
    }

    pub fn get(&self, id: NodeId) -> &AttributeSemantics {
        self.entries
            .get(id.0 as usize)
            .unwrap_or(&AttributeSemantics::NonSpecial)
    }

    #[allow(dead_code)]
    pub(crate) fn set(&mut self, id: NodeId, value: AttributeSemantics) {
        let idx = id.0 as usize;
        if idx >= self.entries.len() {
            self.entries
                .resize_with(idx + 1, AttributeSemantics::default);
        }
        self.entries[idx] = value;
    }
}
