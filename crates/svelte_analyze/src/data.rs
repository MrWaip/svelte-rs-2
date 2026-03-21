use oxc_ast::ast::Expression;
use rustc_hash::{FxHashMap, FxHashSet};
use svelte_ast::{ConcatPart, NodeId, StyleDirective};
use svelte_js::{ExpressionInfo, ScriptInfo};
use svelte_span::Span;

use crate::scope::{ComponentScoping, SymbolId};

// ---------------------------------------------------------------------------
// ParsedExprs — parsed JS expression ASTs in a shared OXC allocator
// ---------------------------------------------------------------------------

/// Parsed JS expression ASTs, stored in a shared OXC allocator.
/// Separate from AnalysisData to avoid lifetime propagation.
pub struct ParsedExprs<'a> {
    /// Template expressions: ExpressionTag, IfBlock test, EachBlock expr, RenderTag, HtmlTag.
    pub exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Attribute expressions, keyed by attribute NodeId.
    pub attr_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// ConcatenationAttribute dynamic parts: (attr_id, part_index).
    pub concat_part_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,
    /// EachBlock key expressions: keyed by EachBlock NodeId.
    pub key_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Pre-parsed script Program AST. Consumed by codegen via `Option::take()`.
    pub script_program: Option<oxc_ast::ast::Program<'a>>,
    /// DebugTag identifier expressions: (debug_tag_id, identifier_index) → transformed expression.
    pub debug_tag_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,
    /// Pre-parsed custom element `extend` expression. Consumed by codegen via `Option::take()`.
    pub ce_extend_expr: Option<Expression<'a>>,
    /// Pre-parsed prop default expressions, indexed by prop position in PropsDeclaration.
    /// Consumed by codegen via clone/take.
    pub prop_default_exprs: Vec<Option<Expression<'a>>>,
    /// Pre-parsed each-block destructuring context bindings, keyed by EachBlock NodeId.
    /// Consumed by codegen via `remove()`.
    pub each_context_bindings: FxHashMap<NodeId, svelte_js::EachContextBinding<'a>>,
    /// Pre-parsed directive name expressions (use:, transition:, animate:).
    /// Keyed by directive NodeId. Consumed by codegen via `remove()`.
    pub directive_name_exprs: FxHashMap<NodeId, Expression<'a>>,
}

impl<'a> ParsedExprs<'a> {
    pub fn new() -> Self {
        Self {
            exprs: FxHashMap::default(),
            attr_exprs: FxHashMap::default(),
            concat_part_exprs: FxHashMap::default(),
            key_exprs: FxHashMap::default(),
            script_program: None,
            debug_tag_exprs: FxHashMap::default(),
            ce_extend_expr: None,
            prop_default_exprs: Vec::new(),
            each_context_bindings: FxHashMap::default(),
            directive_name_exprs: FxHashMap::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// FragmentKey — typed key for lowered_fragments and content_types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentKey {
    Root,
    Element(NodeId),
    ComponentNode(NodeId),
    IfConsequent(NodeId),
    IfAlternate(NodeId),
    EachBody(NodeId),
    EachFallback(NodeId),
    SnippetBody(NodeId),
    KeyBlockBody(NodeId),
    SvelteHeadBody(NodeId),
    SvelteElementBody(NodeId),
    SvelteBoundaryBody(NodeId),
    AwaitPending(NodeId),
    AwaitThen(NodeId),
    AwaitCatch(NodeId),
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Grouped sub-structures
// ---------------------------------------------------------------------------

/// Pre-collected class directive data for codegen (avoids re-traversal).
pub struct ClassDirectiveInfo {
    pub id: NodeId,
    pub name: String,
    pub has_expression: bool,
}

/// Pre-classified component attribute for codegen consumption.
#[derive(Clone)]
pub struct ComponentPropInfo {
    pub kind: ComponentPropKind,
    pub is_dynamic: bool,
}

#[derive(Clone)]
pub enum ComponentPropKind {
    /// `name="value"` — static string
    String { name: String, value_span: Span },
    /// `name` — boolean attribute
    Boolean { name: String },
    /// `name={expr}` — expression (possibly shorthand, possibly memoized)
    Expression { name: String, attr_id: NodeId, shorthand: bool, needs_memo: bool },
    /// `name="text{expr}text"` — template concatenation
    Concatenation { name: String, attr_id: NodeId, parts: Vec<ConcatPart> },
    /// `{name}` — shorthand
    Shorthand { attr_id: NodeId, name: String },
    /// `bind:this={expr}`
    BindThis { bind_id: NodeId },
    /// `{...spread}` — tracked but skipped in prop building
    Spread,
}

/// Pre-computed event handler delegation vs direct binding decision.
#[derive(Debug, Clone, Copy)]
pub enum EventHandlerMode {
    /// Event uses delegation: `$.delegated(name, el, handler, [void 0, true])`
    Delegated { passive: bool },
    /// Event uses direct binding: `$.event(name, el, handler, [capture?, passive?])`
    Direct { capture: bool, passive: bool },
}

/// Per-element flags populated by ElementFlagsVisitor, reactivity, and needs_var passes.
pub struct ElementFlags {
    pub(crate) has_spread: FxHashSet<NodeId>,
    pub(crate) class_attr_id: FxHashMap<NodeId, NodeId>,
    pub(crate) class_directive_info: FxHashMap<NodeId, Vec<ClassDirectiveInfo>>,
    pub(crate) needs_clsx: FxHashSet<NodeId>,
    /// Pre-extracted static class attribute value (avoids span→string conversion in codegen).
    pub(crate) static_class: FxHashMap<NodeId, String>,
    pub(crate) style_directives: FxHashMap<NodeId, Vec<StyleDirective>>,
    /// Pre-extracted static style attribute value (avoids span→string conversion in codegen).
    pub(crate) static_style: FxHashMap<NodeId, String>,
    pub(crate) needs_input_defaults: FxHashSet<NodeId>,
    pub(crate) needs_var: FxHashSet<NodeId>,
    pub(crate) needs_ref: FxHashSet<NodeId>,
    pub(crate) dynamic_attrs: FxHashSet<NodeId>,
    /// Elements with both `contenteditable="true"` and `bind:innerHTML|innerText|textContent`.
    /// Text children use `nodeValue=` init instead of `$.set_text()` update.
    pub(crate) bound_contenteditable: FxHashSet<NodeId>,
    pub(crate) has_use_directive: FxHashSet<NodeId>,
    pub(crate) has_dynamic_class_directives: FxHashSet<NodeId>,
    /// Attribute/directive whose expression is a simple identifier matching the name
    /// (e.g., `class:foo={foo}`, `style:color={color}`). Enables property shorthand in output.
    pub(crate) expression_shorthand: FxHashSet<NodeId>,
    /// Pre-classified component attributes for codegen (avoids two-pass pattern).
    pub(crate) component_props: FxHashMap<NodeId, Vec<ComponentPropInfo>>,
    /// Pre-computed event handler delegation routing (avoids on-the-fly decision in codegen).
    pub(crate) event_handler_mode: FxHashMap<NodeId, EventHandlerMode>,
}

impl ElementFlags {
    pub fn new() -> Self {
        Self {
            has_spread: FxHashSet::default(),
            class_attr_id: FxHashMap::default(),
            class_directive_info: FxHashMap::default(),
            needs_clsx: FxHashSet::default(),
            static_class: FxHashMap::default(),
            style_directives: FxHashMap::default(),
            static_style: FxHashMap::default(),
            needs_input_defaults: FxHashSet::default(),
            needs_var: FxHashSet::default(),
            needs_ref: FxHashSet::default(),
            dynamic_attrs: FxHashSet::default(),
            bound_contenteditable: FxHashSet::default(),
            has_use_directive: FxHashSet::default(),
            has_dynamic_class_directives: FxHashSet::default(),
            expression_shorthand: FxHashSet::default(),
            component_props: FxHashMap::default(),
            event_handler_mode: FxHashMap::default(),
        }
    }

    pub fn has_spread(&self, id: NodeId) -> bool { self.has_spread.contains(&id) }
    pub fn has_class_directives(&self, id: NodeId) -> bool { self.class_directive_info.contains_key(&id) }
    pub fn has_class_attribute(&self, id: NodeId) -> bool { self.class_attr_id.contains_key(&id) }
    pub fn class_attr_id(&self, id: NodeId) -> Option<NodeId> { self.class_attr_id.get(&id).copied() }
    pub fn class_directive_info(&self, id: NodeId) -> Option<&[ClassDirectiveInfo]> { self.class_directive_info.get(&id).map(|v| v.as_slice()) }
    pub fn needs_clsx(&self, id: NodeId) -> bool { self.needs_clsx.contains(&id) }
    pub fn has_style_directives(&self, id: NodeId) -> bool { self.style_directives.contains_key(&id) }
    pub fn style_directives(&self, id: NodeId) -> &[StyleDirective] { self.style_directives.get(&id).map_or(&[], |v| v.as_slice()) }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool { self.needs_input_defaults.contains(&id) }
    pub fn needs_var(&self, id: NodeId) -> bool { self.needs_var.contains(&id) }
    pub fn needs_ref(&self, id: NodeId) -> bool { self.needs_ref.contains(&id) }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool { self.dynamic_attrs.contains(&id) }
    pub fn static_class(&self, id: NodeId) -> Option<&str> { self.static_class.get(&id).map(|s| s.as_str()) }
    pub fn static_style(&self, id: NodeId) -> Option<&str> { self.static_style.get(&id).map(|s| s.as_str()) }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool { self.bound_contenteditable.contains(&id) }
    pub fn has_use_directive(&self, id: NodeId) -> bool { self.has_use_directive.contains(&id) }
    pub fn has_dynamic_class_directives(&self, id: NodeId) -> bool { self.has_dynamic_class_directives.contains(&id) }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool { self.expression_shorthand.contains(&id) }
    pub fn component_props(&self, id: NodeId) -> &[ComponentPropInfo] {
        self.component_props.get(&id).map_or(&[], |v| v.as_slice())
    }
    pub fn event_handler_mode(&self, attr_id: NodeId) -> Option<EventHandlerMode> {
        self.event_handler_mode.get(&attr_id).copied()
    }
}

/// Fragment lowering results and content classification.
pub struct FragmentData {
    pub(crate) lowered: FxHashMap<FragmentKey, LoweredFragment>,
    pub(crate) content_types: FxHashMap<FragmentKey, ContentStrategy>,
    pub(crate) has_dynamic_children: FxHashSet<FragmentKey>,
}

impl FragmentData {
    pub fn new() -> Self {
        Self {
            lowered: FxHashMap::default(),
            content_types: FxHashMap::default(),
            has_dynamic_children: FxHashSet::default(),
        }
    }

    pub fn content_type(&self, key: &FragmentKey) -> ContentStrategy {
        self.content_types.get(key).cloned().unwrap_or(ContentStrategy::Empty)
    }

    pub fn has_dynamic_children(&self, key: &FragmentKey) -> bool {
        self.has_dynamic_children.contains(key)
    }

    pub fn lowered(&self, key: &FragmentKey) -> Option<&LoweredFragment> {
        self.lowered.get(key)
    }
}

/// Snippet analysis: parameter names and hoistability.
pub struct SnippetData {
    pub(crate) params: FxHashMap<NodeId, Vec<String>>,
    pub(crate) hoistable: FxHashSet<NodeId>,
}

impl SnippetData {
    pub fn new() -> Self {
        Self {
            params: FxHashMap::default(),
            hoistable: FxHashSet::default(),
        }
    }

    pub fn params(&self, id: NodeId) -> Option<&Vec<String>> { self.params.get(&id) }
    pub fn is_hoistable(&self, id: NodeId) -> bool { self.hoistable.contains(&id) }
}

/// ConstTag analysis: declared names and per-fragment grouping.
pub struct ConstTagData {
    pub(crate) names: FxHashMap<NodeId, Vec<String>>,
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl ConstTagData {
    pub fn new() -> Self {
        Self {
            names: FxHashMap::default(),
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn names(&self, id: NodeId) -> Option<&Vec<String>> { self.names.get(&id) }
    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.by_fragment.get(key) }
}

/// DebugTag per-fragment grouping.
pub struct DebugTagData {
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl DebugTagData {
    pub fn new() -> Self {
        Self {
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> { self.by_fragment.get(key) }
}

/// Each-block context/index names, extracted from source text during scope building.
pub struct EachBlockData {
    pub(crate) context_names: FxHashMap<NodeId, String>,
    pub(crate) index_names: FxHashMap<NodeId, String>,
    /// Key expression references the index variable (needs index param in key arrow).
    pub(crate) key_uses_index: FxHashSet<NodeId>,
    /// Context is a destructuring pattern (`{ name, value }` or `[a, b]`).
    pub(crate) is_destructured: FxHashSet<NodeId>,
    /// Body expressions reference the index variable (needs index param in render fn).
    pub(crate) body_uses_index: FxHashSet<NodeId>,
    /// Key expression is the same simple identifier as the context variable.
    pub(crate) key_is_item: FxHashSet<NodeId>,
    /// Body contains an element with an `animate:` directive.
    pub(crate) has_animate: FxHashSet<NodeId>,
}

impl EachBlockData {
    pub fn new() -> Self {
        Self {
            context_names: FxHashMap::default(),
            index_names: FxHashMap::default(),
            key_uses_index: FxHashSet::default(),
            is_destructured: FxHashSet::default(),
            body_uses_index: FxHashSet::default(),
            key_is_item: FxHashSet::default(),
            has_animate: FxHashSet::default(),
        }
    }

    pub fn context_name(&self, id: NodeId) -> Option<&str> {
        self.context_names.get(&id).map(|s| s.as_str())
    }

    pub fn index_name(&self, id: NodeId) -> Option<&str> {
        self.index_names.get(&id).map(|s| s.as_str())
    }

    pub fn key_uses_index(&self, id: NodeId) -> bool { self.key_uses_index.contains(&id) }
    pub fn is_destructured(&self, id: NodeId) -> bool { self.is_destructured.contains(&id) }
    pub fn body_uses_index(&self, id: NodeId) -> bool { self.body_uses_index.contains(&id) }
    pub fn key_is_item(&self, id: NodeId) -> bool { self.key_is_item.contains(&id) }
    pub fn has_animate(&self, id: NodeId) -> bool { self.has_animate.contains(&id) }
}

/// Await block binding patterns, parsed via OXC in the `parse_js` pass.
pub struct AwaitBindingData {
    /// Then binding info, keyed by AwaitBlock NodeId.
    pub(crate) values: FxHashMap<NodeId, svelte_js::AwaitBindingInfo>,
    /// Catch binding info, keyed by AwaitBlock NodeId.
    pub(crate) errors: FxHashMap<NodeId, svelte_js::AwaitBindingInfo>,
}

impl AwaitBindingData {
    pub fn new() -> Self {
        Self {
            values: FxHashMap::default(),
            errors: FxHashMap::default(),
        }
    }

    pub fn value(&self, id: NodeId) -> Option<&svelte_js::AwaitBindingInfo> { self.values.get(&id) }
    pub fn error(&self, id: NodeId) -> Option<&svelte_js::AwaitBindingInfo> { self.errors.get(&id) }
}

/// Pre-computed bind/directive semantics for codegen.
///
/// Eliminates string-based symbol re-resolution in codegen: instead of
/// `source_text(span) → find_binding → is_rune && is_mutated`, codegen
/// reads pre-computed flags keyed by NodeId.
pub struct BindSemanticsData {
    /// Bind directives / class directives / style directives whose target
    /// is a mutable rune (needs `$.get()`/`$.set()` instead of plain access).
    /// Key: directive NodeId.
    pub(crate) mutable_rune_targets: FxHashSet<NodeId>,
    /// Nodes whose expression resolves to a prop source
    /// (each_block collection, render_tag argument identifiers).
    /// Key: EachBlock NodeId or RenderTag argument NodeId.
    pub(crate) prop_source_nodes: FxHashSet<NodeId>,
    /// Pre-computed each-block variable names referenced in bind:this expressions.
    /// Key: BindDirective NodeId. Value: names of each-block vars used in the expression.
    pub(crate) bind_each_context: FxHashMap<NodeId, Vec<String>>,
    /// Elements that have a `bind:group` directive.
    /// Their `value` attribute uses the `__value` pattern instead of `$.set_value`.
    pub(crate) has_bind_group: FxHashSet<NodeId>,
    /// bind:group directive → NodeId of the value attribute on the same element (if any).
    /// Used to build the getter thunk that evaluates the value expression.
    pub(crate) bind_group_value_attr: FxHashMap<NodeId, NodeId>,
    /// bind:group directive → ancestor each block NodeIds whose context vars
    /// appear in the binding expression (inner-to-outer order).
    pub(crate) parent_each_blocks: FxHashMap<NodeId, Vec<NodeId>>,
    /// Each blocks that need a generated `$$index` parameter for group binding.
    pub(crate) contains_group_binding: FxHashSet<NodeId>,
}

impl BindSemanticsData {
    pub fn new() -> Self {
        Self {
            mutable_rune_targets: FxHashSet::default(),
            prop_source_nodes: FxHashSet::default(),
            bind_each_context: FxHashMap::default(),
            has_bind_group: FxHashSet::default(),
            bind_group_value_attr: FxHashMap::default(),
            parent_each_blocks: FxHashMap::default(),
            contains_group_binding: FxHashSet::default(),
        }
    }

    pub fn is_mutable_rune_target(&self, id: NodeId) -> bool {
        self.mutable_rune_targets.contains(&id)
    }

    pub fn is_prop_source(&self, id: NodeId) -> bool {
        self.prop_source_nodes.contains(&id)
    }

    pub fn each_context(&self, id: NodeId) -> Option<&Vec<String>> {
        self.bind_each_context.get(&id)
    }

    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.has_bind_group.contains(&id)
    }

    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.bind_group_value_attr.get(&id).copied()
    }

    pub fn parent_each_blocks(&self, id: NodeId) -> Option<&Vec<NodeId>> {
        self.parent_each_blocks.get(&id)
    }

    pub fn contains_group_binding(&self, id: NodeId) -> bool {
        self.contains_group_binding.contains(&id)
    }
}

// ---------------------------------------------------------------------------
// AnalysisData — side tables populated by all passes
// ---------------------------------------------------------------------------

pub struct AnalysisData {
    /// Parsed JS metadata for ExpressionTag nodes (and IfBlock/EachBlock test expressions).
    pub expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Parsed JS metadata for attribute expressions, keyed by attribute NodeId.
    pub attr_expressions: FxHashMap<NodeId, ExpressionInfo>,
    /// Parsed script block declarations.
    pub script: Option<ScriptInfo>,
    /// Unified scope tree for script + template (oxc-based).
    pub scoping: ComponentScoping,
    /// Nodes (ExpressionTag / IfBlock / EachBlock) that reference rune symbols.
    pub dynamic_nodes: FxHashSet<NodeId>,
    /// NodeIds of IfBlocks whose alternate is an elseif (single IfBlock with elseif: true).
    pub alt_is_elseif: FxHashSet<NodeId>,
    /// Props analysis (from $props() destructuring).
    pub props: Option<PropsAnalysis>,
    /// Binding name from `const id = $props.id()`.
    pub props_id: Option<String>,
    /// Exported names from `export const/function/class` or `export { ... }`.
    pub exports: Vec<svelte_js::ExportInfo>,
    /// Component needs runtime context (`$.push`/`$.pop`), e.g. has `$effect` calls.
    pub needs_context: bool,
    /// Script contains class declarations with `$state`/`$state.raw` fields.
    /// When true, member access on local bindings is treated as dynamic.
    pub has_class_state_fields: bool,

    /// Per-element flags (spread, class/style directives, needs_var, etc.).
    pub element_flags: ElementFlags,
    /// Fragment lowering and content classification.
    pub fragments: FragmentData,
    /// Snippet parameters and hoistability.
    pub snippets: SnippetData,
    /// ConstTag declared names and per-fragment grouping.
    pub const_tags: ConstTagData,
    /// DebugTag per-fragment grouping.
    pub debug_tags: DebugTagData,
    /// Each-block context/index names.
    pub each_blocks: EachBlockData,
    /// Per-argument `has_call` flags for render tag expressions (keyed by RenderTag NodeId).
    pub render_tag_arg_has_call: FxHashMap<NodeId, Vec<bool>>,
    /// Intermediate: per-argument identifier name (if the arg is a plain identifier).
    /// Consumed by `resolve_render_tag_prop_sources` after props analysis.
    pub(crate) render_tag_arg_idents: FxHashMap<NodeId, Vec<Option<String>>>,
    /// Per-argument prop-source SymbolId for render tags.
    /// Some(sym) = prop-source arg (pass getter directly), None = not a prop-source.
    pub render_tag_prop_sources: FxHashMap<NodeId, Vec<Option<SymbolId>>>,
    /// Callee identifier name for render tags (only set when callee is an Identifier).
    pub(crate) render_tag_callee_name: FxHashMap<NodeId, String>,
    /// Callee SymbolId for render tags (resolved during resolve_references).
    pub(crate) render_tag_callee_sym: FxHashMap<NodeId, SymbolId>,
    /// Render tags whose expression was a ChainExpression (`{@render fn?.()}`).
    pub render_tag_is_chain: FxHashSet<NodeId>,
    /// Dynamic render tags — callee is a non-normal binding (prop, state, snippet param, etc.).
    pub render_tag_dynamic: FxHashSet<NodeId>,
    /// Render tags whose callee is a getter function (prop-source or snippet param).
    /// These pass the callee directly to `$.snippet` instead of wrapping in a thunk.
    pub render_tag_callee_is_getter: FxHashSet<NodeId>,
    /// Await block binding patterns (then/catch), parsed via OXC.
    pub await_bindings: AwaitBindingData,
    /// Pre-computed bind/directive semantics (mutable rune targets, prop sources).
    pub bind_semantics: BindSemanticsData,
    /// Pre-computed import SymbolIds from root scope (O(1) lookup in codegen).
    pub import_syms: FxHashSet<SymbolId>,
    /// Whether this component is compiled as a custom element.
    /// When true, all props become prop sources with getter/setter exports.
    pub custom_element: bool,
    /// Parsed custom element config (from object expression form).
    pub ce_config: Option<svelte_js::ParsedCeConfig>,
}

impl AnalysisData {
    pub fn new() -> Self {
        Self {
            expressions: FxHashMap::default(),
            attr_expressions: FxHashMap::default(),
            script: None,
            scoping: ComponentScoping::empty(),
            dynamic_nodes: FxHashSet::default(),
            alt_is_elseif: FxHashSet::default(),
            props: None,
            props_id: None,
            exports: Vec::new(),
            needs_context: false,
            has_class_state_fields: false,
            element_flags: ElementFlags::new(),
            fragments: FragmentData::new(),
            snippets: SnippetData::new(),
            const_tags: ConstTagData::new(),
            debug_tags: DebugTagData::new(),
            each_blocks: EachBlockData::new(),
            render_tag_arg_has_call: FxHashMap::default(),
            render_tag_arg_idents: FxHashMap::default(),
            render_tag_prop_sources: FxHashMap::default(),
            render_tag_callee_name: FxHashMap::default(),
            render_tag_callee_sym: FxHashMap::default(),
            render_tag_is_chain: FxHashSet::default(),
            render_tag_dynamic: FxHashSet::default(),
            render_tag_callee_is_getter: FxHashSet::default(),
            await_bindings: AwaitBindingData::new(),
            bind_semantics: BindSemanticsData::new(),
            import_syms: FxHashSet::default(),
            custom_element: false,
            ce_config: None,
        }
    }
}

impl AnalysisData {
    pub fn is_dynamic(&self, id: NodeId) -> bool { self.dynamic_nodes.contains(&id) }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool { self.alt_is_elseif.contains(&id) }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> { self.expressions.get(&id) }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> { self.attr_expressions.get(&id) }
    pub fn render_tag_arg_has_call(&self, id: NodeId) -> Option<&[bool]> { self.render_tag_arg_has_call.get(&id).map(|v| v.as_slice()) }
    pub fn render_tag_prop_sources(&self, id: NodeId) -> Option<&[Option<SymbolId>]> { self.render_tag_prop_sources.get(&id).map(|v| v.as_slice()) }
    pub fn render_tag_is_chain(&self, id: NodeId) -> bool { self.render_tag_is_chain.contains(&id) }
    pub fn render_tag_is_dynamic(&self, id: NodeId) -> bool { self.render_tag_dynamic.contains(&id) }
    pub fn render_tag_callee_is_getter(&self, id: NodeId) -> bool { self.render_tag_callee_is_getter.contains(&id) }

    /// Component attribute needs `$.derived()` memoization:
    /// has a function call, OR is a non-simple dynamic expression.
    pub fn component_attr_needs_memo(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(&attr_id).is_some_and(|e|
            e.has_call || (!e.kind.is_simple() && self.element_flags.is_dynamic_attr(attr_id))
        )
    }

    /// Expression has a function call AND references to resolved bindings — needs `$.derived` wrapping.
    pub fn needs_expr_memoization(&self, id: NodeId) -> bool {
        self.expressions.get(&id).is_some_and(|e|
            e.has_call && e.references.iter().any(|r| r.symbol_id.is_some())
        )
    }

    /// Known compile-time value for a name at root scope (looks up SymbolId internally).
    pub fn known_value(&self, name: &str) -> Option<&str> {
        let root = self.scoping.root_scope_id();
        let sym_id = self.scoping.find_binding(root, name)?;
        self.scoping.known_value_by_sym(sym_id)
    }
}

// ---------------------------------------------------------------------------
// LoweredFragment — trimmed + grouped representation of a fragment
// ---------------------------------------------------------------------------

pub struct LoweredFragment {
    pub items: Vec<FragmentItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FragmentItem {
    /// A standalone element node.
    Element(NodeId),
    /// A component instantiation.
    ComponentNode(NodeId),
    /// An IfBlock (has its own sub-fragments in lowered_fragments).
    IfBlock(NodeId),
    /// An EachBlock (has its own sub-fragments in lowered_fragments).
    EachBlock(NodeId),
    /// A RenderTag ({@render snippet(args)}).
    RenderTag(NodeId),
    /// An HtmlTag ({@html expr}).
    HtmlTag(NodeId),
    /// A KeyBlock ({#key expr}...{/key}).
    KeyBlock(NodeId),
    /// A SvelteElement (<svelte:element this={tag}>).
    SvelteElement(NodeId),
    /// A SvelteBoundary (<svelte:boundary>).
    SvelteBoundary(NodeId),
    /// An AwaitBlock ({#await expr}...{/await}).
    AwaitBlock(NodeId),
    /// A <title> element inside <svelte:head>, special-cased to assign document.title.
    TitleElement(NodeId),
    /// Adjacent text nodes and expression tags grouped together.
    TextConcat { parts: Vec<LoweredTextPart>, has_expr: bool },
}

impl FragmentItem {
    /// Returns `true` if this is a standalone `{expression}` with no surrounding text.
    /// Used by codegen to pass `is_text: true` to `$.child()` / `$.sibling()`.
    pub fn is_standalone_expr(&self) -> bool {
        matches!(self, FragmentItem::TextConcat { parts, .. }
            if parts.len() == 1 && matches!(parts[0], LoweredTextPart::Expr(_)))
    }

    /// Extract the `NodeId` from any single-node variant.
    /// Panics on `TextConcat` (which has no single id).
    pub fn node_id(&self) -> NodeId {
        match self {
            FragmentItem::Element(id)
            | FragmentItem::ComponentNode(id)
            | FragmentItem::IfBlock(id)
            | FragmentItem::EachBlock(id)
            | FragmentItem::RenderTag(id)
            | FragmentItem::HtmlTag(id)
            | FragmentItem::KeyBlock(id)
            | FragmentItem::SvelteElement(id)
            | FragmentItem::SvelteBoundary(id)
            | FragmentItem::AwaitBlock(id)
            | FragmentItem::TitleElement(id) => *id,
            FragmentItem::TextConcat { .. } => panic!("TextConcat has no single NodeId"),
        }
    }
}

impl LoweredFragment {
    /// Get the first item's NodeId if it is an Element.
    pub fn first_element_id(&self) -> Option<NodeId> {
        match self.items.first()? {
            FragmentItem::Element(id) => Some(*id),
            _ => None,
        }
    }

    /// Get the first item's NodeId if it is an IfBlock.
    pub fn first_if_block_id(&self) -> Option<NodeId> {
        match self.items.first()? {
            FragmentItem::IfBlock(id) => Some(*id),
            _ => None,
        }
    }

    /// Get the first item's NodeId if it is an EachBlock.
    pub fn first_each_block_id(&self) -> Option<NodeId> {
        match self.items.first()? {
            FragmentItem::EachBlock(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredTextPart {
    /// Static text content (possibly trimmed).
    Text(String),
    /// Expression tag node id.
    Expr(NodeId),
}

// ---------------------------------------------------------------------------
// PropsAnalysis — analysis of $props() destructuring
// ---------------------------------------------------------------------------

pub struct PropsAnalysis {
    pub props: Vec<PropAnalysis>,
    pub has_bindable: bool,
}

pub struct PropAnalysis {
    pub local_name: String,
    pub prop_name: String,
    pub default_span: Option<svelte_span::Span>,
    pub default_text: Option<String>,
    pub is_bindable: bool,
    pub is_rest: bool,
    /// Default value requires lazy evaluation (`() => expr`).
    /// True when `default_text` is present and is not a simple expression.
    pub is_lazy_default: bool,
    /// Prop needs `$.prop()` source (has default, is mutated, or custom element).
    pub is_prop_source: bool,
    /// Prop's rune symbol is mutated (reassigned somewhere in script/template).
    pub is_mutated: bool,
    /// Prop name starts with `$$` — reserved, excluded from CE accessors.
    pub is_reserved: bool,
}

// ---------------------------------------------------------------------------
// ContentStrategy — classification of what a fragment contains, with embedded data
// ---------------------------------------------------------------------------

/// Describes the content of a fragment. Carries item data so codegen does not
/// need to re-inspect the lowered fragment for common decisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentStrategy {
    Empty,
    /// Only static text, no expressions. Contains the pre-extracted text value.
    Static(String),
    /// Exactly one element node. Contains its NodeId.
    SingleElement(NodeId),
    /// Exactly one block node (IfBlock, EachBlock, etc.). Stores the FragmentItem directly.
    SingleBlock(FragmentItem),
    /// Text with expressions (no elements or blocks).
    DynamicText,
    /// Mix of elements, blocks, and/or text.
    Mixed { has_elements: bool, has_blocks: bool, has_text: bool },
}
