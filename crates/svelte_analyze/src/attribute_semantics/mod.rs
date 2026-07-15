pub(crate) mod builder;
pub mod data;

pub use builder::{BindingGroupTable, build};
pub use data::{
    AttributeSemantics, BoundaryPropSemantics, ClassSemantics, ComponentAttachEmit,
    ComponentAttachSemantics, ComponentBindKind, ComponentBindSemantics, ComponentBindTarget,
    ComponentCssPropValue, ComponentPropConcatSemantics, ComponentPropExpressionSemantics,
    ComponentPropMemo, ComponentPropSemantics, ComponentSpreadEmit, ComponentSpreadSemantics,
    ConcatPartEmit, DefaultAttrKind, DefaultAttrSemantics, DocumentBindSemantics,
    ElementBindPropertyKind, ElementBindSemantics, EventHandler, EventSemantics, GroupBindValue,
    GroupReflection, HandlerEffect, HtmlBindKind, HtmlConcatPart, HtmlConcatSemantics, SkipCause,
    SpecialValueKind, SpecialValueSemantics, StyleSemantics, SvelteComponentThisSemantics,
    TemplateEffect, WindowBindSemantics, is_component_css_property,
};

use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

#[derive(Debug, Default, Clone)]
pub struct AttributeSemanticsStore {
    entries: FxHashMap<u32, AttributeSemantics>,
}

impl AttributeSemanticsStore {
    pub(crate) fn new(_node_count: u32) -> Self {
        Self {
            entries: FxHashMap::default(),
        }
    }

    pub fn get(&self, id: NodeId) -> &AttributeSemantics {
        self.entries
            .get(&id.0)
            .unwrap_or(&AttributeSemantics::NonSpecial)
    }

    #[allow(dead_code)]
    pub(crate) fn set(&mut self, id: NodeId, value: AttributeSemantics) {
        self.entries.insert(id.0, value);
    }
}
