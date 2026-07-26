use svelte_span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChildPropMode {
    #[default]
    In,
    InOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegacySlotSanitization {
    #[default]
    Unneeded,
    Needed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComponentBindOwnership {
    #[default]
    Untracked,
    Tracked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComponentFrame {
    #[default]
    Frameless,
    Scoped,
    Exposed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropAccessors {
    #[default]
    Hidden,
    Exposed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropsInput {
    #[default]
    Ignored,
    Consumed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StoreBindings {
    #[default]
    Absent,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContextScope {
    #[default]
    Direct,
    Wrapped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContentProjection {
    #[default]
    Unused,
    RenderTags,
    LegacySlots {
        first_slot_syntax: Span,
    },
    Mixed {
        first_slot_syntax: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RuntimeSemantics {
    pub child_prop_mode: ChildPropMode,
    pub frame: ComponentFrame,
    pub prop_accessors: PropAccessors,
    pub props_input: PropsInput,
    pub stores: StoreBindings,
    pub legacy_init: LegacyInit,
    pub sanitized_legacy_slots: LegacySlotSanitization,
    pub component_bind_ownership: ComponentBindOwnership,
    pub context_ssr: ContextScope,
    pub props_input_ssr: PropsInput,
    pub content_projection: ContentProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegacyInit {
    #[default]
    None,
    Plain,
    Immutable,
}

#[derive(Default)]
pub struct RuntimeSemanticsStore {
    semantics: RuntimeSemantics,
}

impl RuntimeSemanticsStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn record(&mut self, semantics: RuntimeSemantics) {
        self.semantics = semantics;
    }

    pub fn query(&self) -> RuntimeSemantics {
        self.semantics
    }
}
