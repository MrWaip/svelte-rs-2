use super::*;

pub struct ClassDirectiveInfo {
    pub id: NodeId,
    pub name: String,
    pub has_expression: bool,
    pub expr_id: oxc_syntax::node::NodeId,
}

#[derive(Clone)]
pub struct ComponentPropInfo {
    pub kind: ComponentPropKind,
    pub is_dynamic: bool,
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

        expr_id: oxc_syntax::node::NodeId,
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
        expr_id: oxc_syntax::node::NodeId,
    },
    Bind {
        name: String,
        bind_id: NodeId,
        expr_id: oxc_syntax::node::NodeId,
        mode: ComponentBindMode,

        expr_name: Option<String>,
    },
    Spread {
        attr_id: NodeId,
        expr_id: oxc_syntax::node::NodeId,
    },
    Attach {
        attr_id: NodeId,
        expr_id: oxc_syntax::node::NodeId,
    },

    Event {
        name: String,
        attr_id: NodeId,

        expr_id: Option<oxc_syntax::node::NodeId>,
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

pub struct ElementFlags {
    pub(crate) class_attr_id: NodeTable<NodeId>,
    pub(crate) class_directive_info: NodeTable<Vec<ClassDirectiveInfo>>,
    pub(crate) needs_clsx: NodeBitSet,
    pub(crate) static_class: NodeTable<String>,
    pub(crate) style_directives: NodeTable<Vec<StyleDirective>>,
    pub(crate) static_style: NodeTable<String>,
    pub(crate) needs_input_defaults: NodeBitSet,
    pub(crate) needs_var: NodeBitSet,
    pub(crate) needs_ref: NodeBitSet,
    pub(crate) bound_contenteditable: NodeBitSet,
    pub(crate) has_use_directive: NodeBitSet,
    pub(crate) has_dynamic_class_directives: NodeBitSet,
    pub(crate) expression_shorthand: NodeBitSet,
    pub(crate) component_props: NodeTable<Vec<ComponentPropInfo>>,
    pub(crate) component_binding_sym: NodeTable<SymbolId>,

    pub(crate) component_css_props: NodeTable<Vec<(String, NodeId, oxc_syntax::node::NodeId)>>,
    pub(crate) event_handler_mode: NodeTable<EventHandlerMode>,

    pub(crate) needs_textarea_value_lowering: NodeBitSet,

    pub(crate) option_synthetic_value_expr: NodeTable<NodeId>,

    pub(crate) customizable_select: NodeBitSet,

    pub(crate) is_selectedcontent: NodeBitSet,

    pub(crate) svelte_fragment_slots: NodeBitSet,

    pub(crate) is_svelte_self: NodeBitSet,
}

impl ElementFlags {
    pub fn new(node_count: u32) -> Self {
        Self {
            class_attr_id: NodeTable::new(node_count),
            class_directive_info: NodeTable::new(node_count),
            needs_clsx: NodeBitSet::new(node_count),
            static_class: NodeTable::new(node_count),
            style_directives: NodeTable::new(node_count),
            static_style: NodeTable::new(node_count),
            needs_input_defaults: NodeBitSet::new(node_count),
            needs_var: NodeBitSet::new(node_count),
            needs_ref: NodeBitSet::new(node_count),
            bound_contenteditable: NodeBitSet::new(node_count),
            has_use_directive: NodeBitSet::new(node_count),
            has_dynamic_class_directives: NodeBitSet::new(node_count),
            expression_shorthand: NodeBitSet::new(node_count),
            component_props: NodeTable::new(node_count),
            component_binding_sym: NodeTable::new(node_count),
            component_css_props: NodeTable::new(node_count),
            event_handler_mode: NodeTable::new(node_count),
            needs_textarea_value_lowering: NodeBitSet::new(node_count),
            option_synthetic_value_expr: NodeTable::new(node_count),
            customizable_select: NodeBitSet::new(node_count),
            is_selectedcontent: NodeBitSet::new(node_count),
            svelte_fragment_slots: NodeBitSet::new(node_count),
            is_svelte_self: NodeBitSet::new(node_count),
        }
    }
    pub fn has_class_directives(&self, id: NodeId) -> bool {
        self.class_directive_info.contains_key(id)
    }
    pub fn has_class_attribute(&self, id: NodeId) -> bool {
        self.class_attr_id.contains_key(id)
    }
    pub fn class_attr_id(&self, id: NodeId) -> Option<NodeId> {
        self.class_attr_id.get(id).copied()
    }
    pub fn class_directive_info(&self, id: NodeId) -> Option<&[ClassDirectiveInfo]> {
        self.class_directive_info.get(id).map(|v| v.as_slice())
    }
    pub fn needs_clsx(&self, id: NodeId) -> bool {
        self.needs_clsx.contains(&id)
    }
    pub fn has_style_directives(&self, id: NodeId) -> bool {
        self.style_directives.contains_key(id)
    }
    pub fn style_directives(&self, id: NodeId) -> &[StyleDirective] {
        self.style_directives.get(id).map_or(&[], |v| v.as_slice())
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
    pub fn static_class(&self, id: NodeId) -> Option<&str> {
        self.static_class.get(id).map(|s| s.as_str())
    }
    pub fn static_style(&self, id: NodeId) -> Option<&str> {
        self.static_style.get(id).map(|s| s.as_str())
    }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool {
        self.bound_contenteditable.contains(&id)
    }
    pub fn has_use_directive(&self, id: NodeId) -> bool {
        self.has_use_directive.contains(&id)
    }
    pub fn has_dynamic_class_directives(&self, id: NodeId) -> bool {
        self.has_dynamic_class_directives.contains(&id)
    }

    pub fn is_expression_shorthand(&self, id: NodeId) -> bool {
        self.expression_shorthand.contains(&id)
    }
    pub fn component_props(&self, id: NodeId) -> &[ComponentPropInfo] {
        self.component_props.get(id).map_or(&[], |v| v.as_slice())
    }
    pub fn component_binding_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.component_binding_sym.get(id).copied()
    }
    pub fn component_css_props(&self, id: NodeId) -> &[(String, NodeId, oxc_syntax::node::NodeId)] {
        self.component_css_props
            .get(id)
            .map_or(&[], |v| v.as_slice())
    }
    pub fn has_component_css_props(&self, id: NodeId) -> bool {
        self.component_css_props.contains_key(id)
    }
    pub fn event_handler_mode(&self, attr_id: NodeId) -> Option<EventHandlerMode> {
        self.event_handler_mode.get(attr_id).copied()
    }
    pub fn needs_textarea_value_lowering(&self, id: NodeId) -> bool {
        self.needs_textarea_value_lowering.contains(&id)
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
    pub fn is_svelte_self(&self, id: NodeId) -> bool {
        self.is_svelte_self.contains(&id)
    }
}
