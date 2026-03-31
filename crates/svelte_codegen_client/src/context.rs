use std::ops::{Deref, DerefMut};

use rustc_hash::{FxHashMap, FxHashSet};

use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_analyze::{
    AnalysisData, ClassDirectiveInfo, CodegenView, ComponentPropInfo, ContentStrategy,
    EventHandlerMode, ExpressionInfo, FragmentKey, IdentGen, LoweredFragment, ParserResult,
};
use svelte_ast::{
    AwaitBlock, Component, ComponentNode, DebugTag, EachBlock, Element, IfBlock, KeyBlock, NodeId,
    RenderTag, SnippetBlock, SvelteBody, SvelteBoundary, SvelteDocument, SvelteElement,
    SvelteWindow,
};
use svelte_transform::TransformData;

use crate::builder::Builder;

/// Read-only codegen query context.
///
/// Owns immutable access to component AST, analysis-derived view and compile metadata.
pub struct CodegenQuery<'a> {
    pub component: &'a Component,
    pub view: CodegenView<'a>,
    pub name: &'a str,
    pub source: &'a str,
    pub filename: &'a str,
    pub experimental_async: bool,
    pub dev: bool,
}

impl<'a> CodegenQuery<'a> {
    pub fn new(
        component: &'a Component,
        analysis: &'a AnalysisData,
        name: &'a str,
        source: &'a str,
        filename: &'a str,
        experimental_async: bool,
        dev: bool,
    ) -> Self {
        Self {
            component,
            view: CodegenView::new(analysis),
            name,
            source,
            filename,
            experimental_async,
            dev,
        }
    }

    pub fn raw(&self) -> &'a AnalysisData {
        self.view.raw()
    }

    // -- Node lookups (O(1)) --

    pub fn element(&self, id: NodeId) -> &'a Element {
        self.component.store.element(id)
    }
    pub fn component_node(&self, id: NodeId) -> &'a ComponentNode {
        self.component.store.component_node(id)
    }
    pub fn if_block(&self, id: NodeId) -> &'a IfBlock {
        self.component.store.if_block(id)
    }
    pub fn each_block(&self, id: NodeId) -> &'a EachBlock {
        self.component.store.each_block(id)
    }
    pub fn snippet_block(&self, id: NodeId) -> &'a SnippetBlock {
        self.component.store.snippet_block(id)
    }
    pub fn render_tag(&self, id: NodeId) -> &'a RenderTag {
        self.component.store.render_tag(id)
    }
    pub fn key_block(&self, id: NodeId) -> &'a KeyBlock {
        self.component.store.key_block(id)
    }
    pub fn svelte_element(&self, id: NodeId) -> &'a SvelteElement {
        self.component.store.svelte_element(id)
    }
    pub fn svelte_boundary(&self, id: NodeId) -> &'a SvelteBoundary {
        self.component.store.svelte_boundary(id)
    }
    pub fn await_block(&self, id: NodeId) -> &'a AwaitBlock {
        self.component.store.await_block(id)
    }
    pub fn svelte_window(&self, id: NodeId) -> &'a SvelteWindow {
        self.component.store.svelte_window(id)
    }
    pub fn svelte_document(&self, id: NodeId) -> &'a SvelteDocument {
        self.component.store.svelte_document(id)
    }
    pub fn svelte_body(&self, id: NodeId) -> &'a SvelteBody {
        self.component.store.svelte_body(id)
    }
    pub fn debug_tag(&self, id: NodeId) -> &'a DebugTag {
        self.component.store.debug_tag(id)
    }

    pub fn lowered_fragment(&self, key: &FragmentKey) -> &LoweredFragment {
        self.view
            .lowered_fragment(key)
            .unwrap_or_else(|| panic!("lowered fragment {:?} not found", key))
    }

    pub fn known_value(&self, name: &str) -> Option<&str> {
        self.raw().known_value(name)
    }

    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        self.view.expr_has_blockers(id)
    }

    pub fn expression_blockers(&self, id: NodeId) -> Vec<u32> {
        self.view.expression_blockers(id).into_iter().collect()
    }

    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.raw().expression(id)
    }
}

impl<'a> Deref for CodegenQuery<'a> {
    type Target = CodegenView<'a>;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

/// Mutable runtime/codegen state used during emission.
pub struct CodegenState<'a> {
    pub b: Builder<'a>,
    // Compatibility fields while call-sites migrate from `ctx.*` to `ctx.query.*`.
    pub component: &'a Component,
    pub name: &'a str,
    pub source: &'a str,
    pub filename: &'a str,
    pub experimental_async: bool,
    pub dev: bool,
    /// Data produced by the transform phase (e.g. tmp names for destructured const tags).
    pub transform_data: TransformData,
    /// Pre-parsed and pre-transformed expression ASTs (mutable for ownership transfer via remove).
    pub parsed: &'a mut ParserResult<'a>,
    /// Shared unique identifier generator.
    ident_gen: &'a mut IdentGen,
    /// Template declarations from nested fragments, hoisted to module scope.
    pub module_hoisted: Vec<Statement<'a>>,

    // -- Bind group --
    pub needs_binding_group: bool,
    /// Generated $$index names for each blocks with contains_group_binding
    /// (populated during each_block codegen, consumed by bind_group codegen).
    pub group_index_names: FxHashMap<NodeId, String>,
    /// Set while processing children of a contenteditable element with bind:innerHTML/innerText/textContent.
    /// Text nodes use `nodeValue=` init instead of `$.set_text()` update.
    pub bound_contenteditable: bool,
    /// Snippet param names for the currently generating snippet body.
    pub snippet_param_names: Vec<String>,

    /// Event names that use delegation (e.g., "click" from `onclick={handler}`).
    /// Ordered Vec for deterministic output + HashSet for O(1) dedup.
    pub delegated_events: Vec<String>,
    delegated_events_set: FxHashSet<String>,

    /// Whether any $inspect.trace() was found in template expressions (triggers tracing import).
    pub has_tracing: bool,
    /// Const-tag blocker propagation (experimental.async).
    /// Maps SymbolId of a const-tag binding → (promises_var_name, thunk_index).
    /// Populated by `emit_const_tags` when `$.run()` mode is used.
    /// Consumed by template effect emission to add `promises[M]` blockers.
    pub(crate) const_tag_blockers: FxHashMap<SymbolId, (String, usize)>,
    /// Accumulated const-tag blocker expressions from child traversal.
    /// Collected during text expression codegen, consumed by `emit_template_effect_with_blockers`.
    pub(crate) pending_const_blockers: Vec<Expression<'a>>,
}

impl<'a> CodegenState<'a> {
    fn new(
        allocator: &'a oxc_allocator::Allocator,
        component: &'a Component,
        name: &'a str,
        source: &'a str,
        filename: &'a str,
        experimental_async: bool,
        dev: bool,
        parsed: &'a mut ParserResult<'a>,
        ident_gen: &'a mut IdentGen,
        transform_data: TransformData,
    ) -> Self {
        Self {
            b: Builder::new(allocator),
            component,
            name,
            source,
            filename,
            experimental_async,
            dev,
            transform_data,
            parsed,
            ident_gen,
            module_hoisted: Vec::new(),
            needs_binding_group: false,
            group_index_names: FxHashMap::default(),
            bound_contenteditable: false,
            snippet_param_names: Vec::new(),
            delegated_events: Vec::new(),
            delegated_events_set: FxHashSet::default(),
            has_tracing: false,
            const_tag_blockers: FxHashMap::default(),
            pending_const_blockers: Vec::new(),
        }
    }

    pub fn gen_ident(&mut self, prefix: &str) -> String {
        self.ident_gen.gen(prefix)
    }

    pub fn add_delegated_event(&mut self, event_name: String) {
        if self.delegated_events_set.insert(event_name.clone()) {
            self.delegated_events.push(event_name);
        }
    }
}

/// Compatibility wrapper during query/state split migration.
///
/// Migration note:
/// - Temporary Ctx proxy methods are all methods that forward to `self.query.*` or `self.state.*`.
/// - Migrate template modules in this order:
///   1) replace read sites with `ctx.query.*` (node lookup + analysis queries),
///   2) replace write sites with `ctx.state.*`,
///   3) once module has no `Ctx` proxy calls, delete those specific proxy methods.
/// - Final cleanup step is removing `Ctx` deref-based compatibility and using `CodegenQuery/CodegenState`
///   explicitly across all call-sites.
pub struct Ctx<'a> {
    pub query: CodegenQuery<'a>,
    pub state: CodegenState<'a>,
}

impl<'a> Deref for Ctx<'a> {
    type Target = CodegenState<'a>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<'a> DerefMut for Ctx<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<'a> Ctx<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        allocator: &'a oxc_allocator::Allocator,
        component: &'a Component,
        analysis: &'a AnalysisData,
        parsed: &'a mut ParserResult<'a>,
        ident_gen: &'a mut IdentGen,
        transform_data: TransformData,
        name: &str,
        dev: bool,
        source: &'a str,
        filename: &str,
        experimental_async: bool,
    ) -> Self {
        let name = allocator.alloc_str(name);
        let filename = allocator.alloc_str(filename);

        Self {
            query: CodegenQuery::new(
                component,
                analysis,
                name,
                source,
                filename,
                experimental_async,
                dev,
            ),
            state: CodegenState::new(
                allocator,
                component,
                name,
                source,
                filename,
                experimental_async,
                dev,
                parsed,
                ident_gen,
                transform_data,
            ),
        }
    }

    // -- Node lookups (O(1)) --

    pub fn element(&self, id: NodeId) -> &'a Element {
        self.query.element(id)
    }
    pub fn component_node(&self, id: NodeId) -> &'a ComponentNode {
        self.query.component_node(id)
    }
    pub fn if_block(&self, id: NodeId) -> &'a IfBlock {
        self.query.if_block(id)
    }
    pub fn each_block(&self, id: NodeId) -> &'a EachBlock {
        self.query.each_block(id)
    }
    pub fn snippet_block(&self, id: NodeId) -> &'a SnippetBlock {
        self.query.snippet_block(id)
    }
    pub fn render_tag(&self, id: NodeId) -> &'a RenderTag {
        self.query.render_tag(id)
    }
    pub fn key_block(&self, id: NodeId) -> &'a KeyBlock {
        self.query.key_block(id)
    }
    pub fn svelte_element(&self, id: NodeId) -> &'a SvelteElement {
        self.query.svelte_element(id)
    }
    pub fn svelte_boundary(&self, id: NodeId) -> &'a SvelteBoundary {
        self.query.svelte_boundary(id)
    }
    pub fn await_block(&self, id: NodeId) -> &'a AwaitBlock {
        self.query.await_block(id)
    }
    pub fn svelte_window(&self, id: NodeId) -> &'a SvelteWindow {
        self.query.svelte_window(id)
    }
    pub fn svelte_document(&self, id: NodeId) -> &'a SvelteDocument {
        self.query.svelte_document(id)
    }
    pub fn svelte_body(&self, id: NodeId) -> &'a SvelteBody {
        self.query.svelte_body(id)
    }

    pub fn lowered_fragment(&self, key: &FragmentKey) -> &LoweredFragment {
        self.query.lowered_fragment(key)
    }

    /// Temporary migration path for still-unported call-sites.
    pub fn analysis(&self) -> &'a AnalysisData {
        self.query.raw()
    }

    // -- Identifiers --

    /// Generate a unique identifier like `root`, `root_1`, `root_2`, …
    pub fn gen_ident(&mut self, prefix: &str) -> String {
        self.state.gen_ident(prefix)
    }

    // -- Analysis shortcuts --

    pub fn is_dynamic(&self, id: NodeId) -> bool {
        self.analysis().is_dynamic(id)
    }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool {
        self.analysis().is_elseif_alt(id)
    }
    pub fn is_mutable_rune_target(&self, id: NodeId) -> bool {
        self.analysis().bind_semantics.is_mutable_rune_target(id)
    }
    pub fn is_prop_source_node(&self, id: NodeId) -> bool {
        self.analysis().bind_semantics.is_prop_source(id)
    }
    pub fn bind_each_context(&self, id: NodeId) -> Option<&Vec<String>> {
        self.analysis().bind_semantics.each_context(id)
    }
    pub fn each_index_name(&self, id: NodeId) -> Option<String> {
        self.analysis()
            .each_blocks
            .index_sym(id)
            .map(|sym| self.analysis().scoping.symbol_name(sym).to_string())
    }
    pub fn await_value_binding(&self, id: NodeId) -> Option<&svelte_analyze::AwaitBindingInfo> {
        self.analysis().await_bindings.value(id)
    }
    pub fn await_error_binding(&self, id: NodeId) -> Option<&svelte_analyze::AwaitBindingInfo> {
        self.analysis().await_bindings.error(id)
    }
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.analysis().attr_is_import(attr_id)
    }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.query.expression(id)
    }
    pub fn const_tag_symbol_blocker_expr(&self, sym: SymbolId) -> Option<Expression<'a>> {
        let (name, idx) = self.const_tag_blockers.get(&sym)?;
        Some(
            self.b
                .computed_member_expr(self.b.rid_expr(name), self.b.num_expr(*idx as f64)),
        )
    }
    pub fn known_value(&self, name: &str) -> Option<&str> {
        self.query.known_value(name)
    }

    /// Check if expression for node has `has_await`.
    pub fn expr_has_await(&self, id: NodeId) -> bool {
        self.expression(id).is_some_and(|info| info.has_await)
    }

    /// Check if expression has blockers (references bindings with blocker metadata).
    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        self.query.expr_has_blockers(id)
    }

    /// Build `promises_var[index]` expressions for const-tag symbols referenced by a node's expression.
    /// Returns one expression per referenced const-tag binding that has a blocker.
    pub fn const_tag_blocker_exprs(&mut self, id: NodeId) -> Vec<Expression<'a>> {
        if self.const_tag_blockers.is_empty() {
            return Vec::new();
        }
        let Some(info) = self.expression(id) else {
            return Vec::new();
        };
        let ref_symbols = info.ref_symbols.clone();
        let mut result = Vec::new();
        for sym in &ref_symbols {
            if let Some(expr) = self.const_tag_symbol_blocker_expr(*sym) {
                result.push(expr);
            }
        }
        result
    }

    /// Build `[$$promises[i], ...]` blockers array from expression's blocker indices.
    pub fn build_blockers_array(&mut self, id: NodeId) -> oxc_ast::ast::Expression<'a> {
        let indices = self.query.expression_blockers(id);
        if indices.is_empty() {
            return self.b.empty_array_expr();
        }
        self.b.promises_array(&indices)
    }

    /// Build `$.async(anchor, blockers, async_values, callback)` statement.
    /// Common pattern used by if/each/key/html blocks when expression is async.
    pub(crate) fn gen_async_block(
        &mut self,
        id: NodeId,
        anchor: oxc_ast::ast::Expression<'a>,
        has_await: bool,
        async_thunk: Option<oxc_ast::ast::Expression<'a>>,
        param_name: &str,
        inner_stmts: Vec<Statement<'a>>,
    ) -> Statement<'a> {
        let blockers = self.build_blockers_array(id);
        let async_values = if has_await {
            crate::builder::Arg::Expr(self.b.array_expr([async_thunk.unwrap()]))
        } else {
            crate::builder::Arg::Expr(self.b.void_zero_expr())
        };
        let callback_params = if has_await {
            self.b.params(["node", param_name])
        } else {
            self.b.params(["node"])
        };
        let callback = self.b.arrow_block_expr(callback_params, inner_stmts);
        self.b.call_stmt(
            "$.async",
            [
                crate::builder::Arg::Expr(anchor),
                crate::builder::Arg::Expr(blockers),
                async_values,
                crate::builder::Arg::Expr(callback),
            ],
        )
    }

    // -- Fragment shortcuts --

    pub fn content_type(&self, key: &FragmentKey) -> ContentStrategy {
        self.query.view.content_type(key)
    }
    pub fn has_dynamic_children(&self, key: &FragmentKey) -> bool {
        self.query.view.has_dynamic_children(key)
    }

    // -- Element flag shortcuts --

    pub fn has_spread(&self, id: NodeId) -> bool {
        self.query.view.has_spread(id)
    }
    pub fn has_class_directives(&self, id: NodeId) -> bool {
        self.query.view.has_class_directives(id)
    }
    pub fn has_class_attribute(&self, id: NodeId) -> bool {
        self.query.view.has_class_attribute(id)
    }
    pub fn needs_clsx(&self, id: NodeId) -> bool {
        self.query.view.needs_clsx(id)
    }
    pub fn has_style_directives(&self, id: NodeId) -> bool {
        self.query.view.has_style_directives(id)
    }
    pub fn style_directives(&self, id: NodeId) -> &[svelte_ast::StyleDirective] {
        self.query.view.style_directives(id)
    }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool {
        self.query.view.needs_input_defaults(id)
    }
    pub fn needs_var(&self, id: NodeId) -> bool {
        self.query.view.needs_var(id)
    }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool {
        self.query.view.is_dynamic_attr(id)
    }
    pub fn static_class(&self, id: NodeId) -> Option<&str> {
        self.query.view.static_class(id)
    }
    pub fn static_style(&self, id: NodeId) -> Option<&str> {
        self.query.view.static_style(id)
    }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool {
        self.query.view.is_bound_contenteditable(id)
    }
    pub fn has_use_directive(&self, id: NodeId) -> bool {
        self.query.view.has_use_directive(id)
    }
    #[allow(dead_code)]
    pub fn has_dynamic_class_directives(&self, id: NodeId) -> bool {
        self.query.view.has_dynamic_class_directives(id)
    }
    pub fn class_needs_state(&self, id: NodeId) -> bool {
        self.query.view.class_needs_state(id)
    }
    pub fn class_attr_id(&self, id: NodeId) -> Option<NodeId> {
        self.query.view.class_attr_id(id)
    }
    pub fn class_directive_info(&self, id: NodeId) -> Option<&[ClassDirectiveInfo]> {
        self.query.view.class_directive_info(id)
    }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool {
        self.query.view.is_expression_shorthand(id)
    }
    pub fn component_props(&self, id: NodeId) -> &[ComponentPropInfo] {
        self.query.view.component_props(id)
    }
    pub fn component_snippets(&self, id: NodeId) -> &[NodeId] {
        self.query.view.component_snippets(id)
    }
    pub fn event_handler_mode(&self, attr_id: NodeId) -> Option<EventHandlerMode> {
        self.query.view.event_handler_mode(attr_id)
    }
    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.analysis().bind_semantics.has_bind_group(id)
    }
    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.analysis().bind_semantics.bind_group_value_attr(id)
    }
    pub fn parent_each_blocks(&self, id: NodeId) -> Option<&Vec<NodeId>> {
        self.analysis().bind_semantics.parent_each_blocks(id)
    }
    pub fn contains_group_binding(&self, id: NodeId) -> bool {
        self.analysis().bind_semantics.contains_group_binding(id)
    }

    // -- Snippet shortcuts --

    pub fn is_snippet_hoistable(&self, id: NodeId) -> bool {
        self.analysis().snippets.is_hoistable(id)
    }

    // -- ConstTag shortcuts --

    pub fn const_tag_names(&self, id: NodeId) -> Option<&Vec<String>> {
        self.analysis().const_tags.names(id)
    }
    pub fn const_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.analysis().const_tags.by_fragment(key)
    }

    // -- DebugTag shortcuts --

    pub fn debug_tag(&self, id: NodeId) -> &'a DebugTag {
        self.query.debug_tag(id)
    }
    pub fn debug_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.analysis().debug_tags.by_fragment(key)
    }

    // -- EachBlock shortcuts --

    pub fn each_key_uses_index(&self, id: NodeId) -> bool {
        self.analysis().each_blocks.key_uses_index(id)
    }
    pub fn each_body_uses_index(&self, id: NodeId) -> bool {
        self.analysis().each_blocks.body_uses_index(id)
    }
    pub fn each_key_is_item(&self, id: NodeId) -> bool {
        self.analysis().each_blocks.key_is_item(id)
    }
    pub fn each_has_animate(&self, id: NodeId) -> bool {
        self.analysis().each_blocks.has_animate(id)
    }

    // -- Expression offset lookups (for offset-keyed ParserResult) --

    pub fn node_expr_offset(&self, node_id: NodeId) -> u32 {
        self.analysis().node_expr_offset(node_id)
    }

    pub fn attr_expr_offset(&self, attr_id: NodeId) -> u32 {
        self.analysis().attr_expr_offset(attr_id)
    }

    // -- Delegated events --

    /// Register a delegated event name (deduplicates via O(1) HashSet lookup).
    pub fn add_delegated_event(&mut self, event_name: String) {
        self.state.add_delegated_event(event_name);
    }
}
