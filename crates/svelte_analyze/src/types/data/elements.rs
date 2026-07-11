use super::*;
use oxc_syntax::node::NodeId as OxcNodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassDirectiveInfo {
    pub id: NodeId,
    pub name: String,
    pub has_expression: bool,
    pub expr_id: OxcNodeId,
}

#[derive(Clone)]
pub struct ComponentPropInfo {
    pub kind: ComponentPropKind,
}

#[derive(Clone)]
pub enum ComponentPropKind {
    String {
        name: String,
        value_span: Span,
    },
    Boolean {
        name: String,
    },
    Expression {
        name: String,
        attr_id: NodeId,

        expr_id: OxcNodeId,
        shorthand: bool,
        needs_memo: bool,
    },
    Concatenation {
        name: String,
        attr_id: NodeId,
        parts: Vec<ConcatPart>,
    },
    BindThis {
        bind_id: NodeId,
        expr_id: OxcNodeId,
    },
    Bind {
        name: String,
        bind_id: NodeId,
        expr_id: OxcNodeId,
        mode: ComponentBindMode,

        expr_name: Option<String>,

        requires_ownership_emit: bool,
    },
    Spread {
        attr_id: NodeId,
        expr_id: OxcNodeId,
    },
    Attach {
        attr_id: NodeId,
        expr_id: OxcNodeId,
    },

    Event {
        name: String,
        attr_id: NodeId,

        expr_id: Option<OxcNodeId>,
        has_expression: bool,
        has_once_modifier: bool,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum ComponentBindMode {
    PropSource,
    Rune,
    Plain,

    StoreSub,
}

#[derive(Debug, Clone, Copy)]
pub enum EventHandlerMode {
    Delegated { passive: bool },
    Direct { capture: bool, passive: bool },
}

#[derive(Debug, Clone)]
pub enum SvelteElementTag {
    Known(String),
    Dynamic(OxcNodeId),
}

pub struct ElementFlags {
    pub(crate) needs_input_defaults: NodeBitSet,
    pub(crate) needs_var: NodeBitSet,
    pub(crate) needs_ref: NodeBitSet,
    pub(crate) bound_contenteditable: NodeBitSet,
    pub(crate) has_use_directive: NodeBitSet,
    pub(crate) expression_shorthand: NodeBitSet,
    pub(crate) component_props: NodeTable<Vec<ComponentPropInfo>>,

    pub(crate) components_with_css_props: NodeBitSet,
    pub(crate) event_handler_mode: NodeTable<EventHandlerMode>,

    pub(crate) needs_textarea_value_lowering: NodeBitSet,

    pub(crate) needs_textarea_content_reset: NodeBitSet,

    pub(crate) option_synthetic_value_expr: NodeTable<NodeId>,

    pub(crate) customizable_select: NodeBitSet,

    pub(crate) is_selectedcontent: NodeBitSet,

    pub(crate) svelte_fragment_slots: NodeBitSet,

    pub(crate) hydration_attribute_changed_ignored: NodeBitSet,

    pub(crate) svelte_element_tag: NodeTable<SvelteElementTag>,
}

impl ElementFlags {
    pub fn new(node_count: u32) -> Self {
        Self {
            needs_input_defaults: NodeBitSet::new(node_count),
            needs_var: NodeBitSet::new(node_count),
            needs_ref: NodeBitSet::new(node_count),
            bound_contenteditable: NodeBitSet::new(node_count),
            has_use_directive: NodeBitSet::new(node_count),
            expression_shorthand: NodeBitSet::new(node_count),
            component_props: NodeTable::new(node_count),
            components_with_css_props: NodeBitSet::new(node_count),
            event_handler_mode: NodeTable::new(node_count),
            needs_textarea_value_lowering: NodeBitSet::new(node_count),
            needs_textarea_content_reset: NodeBitSet::new(node_count),
            option_synthetic_value_expr: NodeTable::new(node_count),
            customizable_select: NodeBitSet::new(node_count),
            is_selectedcontent: NodeBitSet::new(node_count),
            svelte_fragment_slots: NodeBitSet::new(node_count),
            hydration_attribute_changed_ignored: NodeBitSet::new(node_count),
            svelte_element_tag: NodeTable::new(node_count),
        }
    }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool {
        self.needs_input_defaults.contains(&id)
    }
    pub fn needs_var(&self, id: NodeId) -> bool {
        self.needs_var.contains(&id)
    }
    pub fn needs_ref(&self, id: NodeId) -> bool {
        self.needs_ref.contains(&id)
    }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool {
        self.bound_contenteditable.contains(&id)
    }
    pub fn has_use_directive(&self, id: NodeId) -> bool {
        self.has_use_directive.contains(&id)
    }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool {
        self.expression_shorthand.contains(&id)
    }
    pub fn component_props(&self, id: NodeId) -> &[ComponentPropInfo] {
        self.component_props.get(id).map_or(&[], |v| v.as_slice())
    }
    pub fn has_component_css_props(&self, id: NodeId) -> bool {
        self.components_with_css_props.contains(&id)
    }
    pub fn event_handler_mode(&self, attr_id: NodeId) -> Option<EventHandlerMode> {
        self.event_handler_mode.get(attr_id).copied()
    }
    pub fn needs_textarea_value_lowering(&self, id: NodeId) -> bool {
        self.needs_textarea_value_lowering.contains(&id)
    }
    pub fn needs_textarea_content_reset(&self, id: NodeId) -> bool {
        self.needs_textarea_content_reset.contains(&id)
    }
    pub fn option_synthetic_value_expr(&self, id: NodeId) -> Option<NodeId> {
        self.option_synthetic_value_expr.get(id).copied()
    }
    pub fn is_customizable_select(&self, id: NodeId) -> bool {
        self.customizable_select.contains(&id)
    }
    pub fn is_selectedcontent(&self, id: NodeId) -> bool {
        self.is_selectedcontent.contains(&id)
    }
    pub fn is_svelte_fragment_slot(&self, id: NodeId) -> bool {
        self.svelte_fragment_slots.contains(&id)
    }
    pub fn hydration_attribute_changed_ignored(&self, id: NodeId) -> bool {
        self.hydration_attribute_changed_ignored.contains(&id)
    }
    pub fn svelte_element_tag(&self, id: NodeId) -> Option<&SvelteElementTag> {
        self.svelte_element_tag.get(id)
    }
}
