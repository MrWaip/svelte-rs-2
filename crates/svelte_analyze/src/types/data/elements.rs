use super::*;

pub struct ClassDirectiveInfo {
    pub id: NodeId,
    pub name: String,
    pub has_expression: bool,
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
    },
    Bind {
        name: String,
        bind_id: NodeId,
        mode: ComponentBindMode,
        /// For store-sub binds: the expression identifier (e.g., `"$count"`).
        /// `None` for non-store binds where `name` is used directly.
        expr_name: Option<String>,
    },
    Spread {
        attr_id: NodeId,
    },
    Attach {
        attr_id: NodeId,
    },
    /// LEGACY(svelte4): `on:event` directive on component tags → `$$events`.
    Event {
        name: String,
        attr_id: NodeId,
        has_expression: bool,
        has_once_modifier: bool,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum ComponentBindMode {
    PropSource,
    Rune,
    Plain,
    /// Bind expression targets a store subscription (e.g. `bind:value={$count}`).
    /// Codegen emits `$.mark_store_binding()` in the getter.
    StoreSub,
}

#[derive(Debug, Clone, Copy)]
pub enum EventHandlerMode {
    Delegated { passive: bool },
    Direct { capture: bool, passive: bool },
}

pub struct ElementFlags {
    // Shared normalized attribute facts live in ElementFacts; this table stays limited
    // to downstream derived flags that are specific to later analyze/codegen consumers.
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
    /// `--*` attributes on a component — routed through `<svelte-css-wrapper>` /
    /// `<g>` + `$.css_props(...)` instead of being passed as ordinary props.
    /// Each entry stores the full attribute name (with `--` prefix) and the
    /// attribute NodeId for retrieving the parsed expression.
    pub(crate) component_css_props: NodeTable<Vec<(String, NodeId)>>,
    pub(crate) event_handler_mode: NodeTable<EventHandlerMode>,
    /// `<textarea>` with expression children and no explicit `value` attribute —
    /// codegen emits `$.remove_textarea_child` + `$.set_value` instead of textContent.
    pub(crate) needs_textarea_value_lowering: NodeBitSet,
    /// `<option>` with a single ExpressionTag child and no explicit `value` attribute.
    /// Maps option element NodeId → ExpressionTag NodeId for `__value` synthesis.
    pub(crate) option_synthetic_value_expr: NodeTable<NodeId>,
    /// `<select>`, `<optgroup>`, `<option>` elements with rich DOM content requiring `$.customizable_select`.
    pub(crate) customizable_select: NodeBitSet,
    /// `<selectedcontent>` elements — require a JS var for `$.selectedcontent(el, setter)`.
    pub(crate) is_selectedcontent: NodeBitSet,
    /// Named slot elements that are `<svelte:fragment slot="name">` wrappers.
    /// Keyed by the slot element NodeId. Used in codegen to consume the extra
    /// `root` identifier that the reference compiler allocates for the wrapper.
    pub(crate) svelte_fragment_slots: NodeBitSet,
    /// `<svelte:self>` component nodes — needs `$.comment()` anchor in non-root context.
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
    /// `class_needs_state` moved to `AnalysisData::class_needs_state` because
    /// it reads from `DynamismData`, which is not owned by `ElementFlags`.
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool {
        self.expression_shorthand.contains(&id)
    }
    pub fn component_props(&self, id: NodeId) -> &[ComponentPropInfo] {
        self.component_props.get(id).map_or(&[], |v| v.as_slice())
    }
    pub fn component_binding_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.component_binding_sym.get(id).copied()
    }
    pub fn component_css_props(&self, id: NodeId) -> &[(String, NodeId)] {
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
