use std::ops::{Deref, DerefMut};

use rustc_hash::{FxHashMap, FxHashSet};

use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_analyze::{
    AnalysisData, BindTargetSemantics, ClassDirectiveInfo, CodegenView, ComponentPropInfo,
    ContentStrategy, EventHandlerMode, ExprDeps, ExprSite, ExpressionInfo, FragmentKey, IdentGen,
    LoweredFragment, ParserResult, RenderTagPlan, RuntimePlan,
};
use svelte_ast::{
    AwaitBlock, Component, ComponentNode, DebugTag, EachBlock, Element, IfBlock, KeyBlock, NodeId,
    RenderTag, SvelteBody, SvelteBoundary, SvelteDocument, SvelteElement, SvelteWindow,
};
use svelte_transform::TransformData;

use svelte_ast_builder::Builder;

/// Read-only codegen query context.
///
/// Owns immutable access to component AST, analysis-derived view and compile metadata.
pub struct CodegenQuery<'a> {
    pub component: &'a Component,
    pub view: CodegenView<'a, 'a>,
    pub analysis: &'a AnalysisData<'a>,
}

impl<'a> CodegenQuery<'a> {
    pub fn new(component: &'a Component, analysis: &'a AnalysisData<'a>) -> Self {
        Self {
            component,
            view: CodegenView::new(analysis),
            analysis,
        }
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

    pub fn maybe_lowered_fragment(&self, key: &FragmentKey) -> Option<&LoweredFragment> {
        self.view.lowered_fragment(key)
    }

    pub fn known_value(&self, name: &str) -> Option<&str> {
        self.view.known_value(name)
    }

    #[allow(dead_code)]
    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        self.view.expr_has_blockers(id)
    }

    pub fn expression_blockers(&self, id: NodeId) -> Vec<u32> {
        self.view.expression_blockers(id).into_iter().collect()
    }

    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.view.expression(id)
    }
    pub fn expr_deps(&self, site: ExprSite) -> Option<ExprDeps<'_>> {
        self.view.expr_deps(site)
    }

    pub fn runtime_plan(&self) -> RuntimePlan {
        self.view.runtime_plan()
    }
}

impl<'a> Deref for CodegenQuery<'a> {
    type Target = CodegenView<'a, 'a>;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

/// Mutable runtime/codegen state used during emission.
pub struct CodegenState<'a> {
    pub b: Builder<'a>,
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

    /// Event names that use delegation (e.g., "click" from `onclick={handler}`).
    /// Ordered Vec for deterministic output + HashSet for O(1) dedup.
    pub delegated_events: Vec<String>,
    delegated_events_set: FxHashSet<String>,

    /// Transformed and serialized CSS text for injected mode (`$$css.code`).
    /// `Some` when `css:"injected"` is active and the stylesheet was successfully transformed.
    pub css_text: Option<&'a str>,
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
        name: &'a str,
        source: &'a str,
        filename: &'a str,
        experimental_async: bool,
        dev: bool,
        parsed: &'a mut ParserResult<'a>,
        ident_gen: &'a mut IdentGen,
        transform_data: TransformData,
        css_text: Option<&'a str>,
    ) -> Self {
        Self {
            b: Builder::new(allocator),
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
            delegated_events: Vec::new(),
            delegated_events_set: FxHashSet::default(),
            css_text,
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

/// Thin orchestration wrapper over immutable query data and mutable codegen state.
///
/// `Ctx` is kept for helpers that genuinely combine query access, builder utilities and
/// mutable emission state. Pure read paths should prefer `ctx.query.*`, and mutable
/// generation state should live under `ctx.state.*`.
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
        css_text: Option<&str>,
        name: &str,
        dev: bool,
        source: &'a str,
        filename: &str,
        experimental_async: bool,
    ) -> Self {
        let name = allocator.alloc_str(name);
        let filename = allocator.alloc_str(filename);
        let css_text = css_text.map(|t| allocator.alloc_str(t) as &str);

        Self {
            query: CodegenQuery::new(component, analysis),
            state: CodegenState::new(
                allocator,
                name,
                source,
                filename,
                experimental_async,
                dev,
                parsed,
                ident_gen,
                transform_data,
                css_text,
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

    // -- Identifiers --

    /// Generate a unique identifier like `root`, `root_1`, `root_2`, …
    pub fn gen_ident(&mut self, prefix: &str) -> String {
        self.state.gen_ident(prefix)
    }

    // -- Analysis shortcuts --

    pub fn is_dynamic(&self, id: NodeId) -> bool {
        self.query.view.is_dynamic(id)
    }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool {
        self.query.view.is_elseif_alt(id)
    }
    /// `ReferenceId` of the root identifier in a directive's expression.
    ///
    /// After the shorthand rework, all directive expressions (bind, class,
    /// style, each-collection, attr `{foo}`) are real `Expression::Identifier`
    /// / `MemberExpression` trees parsed into `ParserResult`. The root
    /// identifier carries a `ReferenceId` written by `JsSemanticVisitor`
    /// during the builder pass — this is the single identity the consumer
    /// needs to ask `reference_semantics(ref_id)` for the operation-level
    /// answer.
    pub fn directive_root_ref_id(
        &self,
        dir_id: NodeId,
    ) -> Option<oxc_syntax::reference::ReferenceId> {
        let handle = self.query.view.attr_expr_handle_opt(dir_id)?;
        let expr = self.state.parsed.expr(handle)?;
        let mut current = expr;
        loop {
            match current {
                oxc_ast::ast::Expression::StaticMemberExpression(m) => current = &m.object,
                oxc_ast::ast::Expression::ComputedMemberExpression(m) => current = &m.object,
                oxc_ast::ast::Expression::Identifier(id) => return id.reference_id.get(),
                _ => return None,
            }
        }
    }

    /// Convenience helper: `reference_semantics` of a directive's root
    /// identifier. Returns `NonReactive` when the directive has no resolvable
    /// root identifier (e.g. function-expression `bind:value={(get,set)}`).
    pub fn directive_root_reference_semantics(
        &self,
        dir_id: NodeId,
    ) -> svelte_analyze::ReferenceSemantics {
        match self.directive_root_ref_id(dir_id) {
            Some(ref_id) => self.query.view.reference_semantics(ref_id),
            None => svelte_analyze::ReferenceSemantics::NonReactive,
        }
    }

    pub fn bind_each_context(&self, id: NodeId) -> Option<&[SymbolId]> {
        self.query.view.bind_each_context(id)
    }
    pub fn attr_index(&self, id: NodeId) -> Option<&svelte_analyze::AttrIndex> {
        self.query.view.attr_index(id)
    }
    pub fn each_index_name(&self, id: NodeId) -> Option<String> {
        self.query.view.each_index_name(id).map(str::to_string)
    }
    pub fn is_each_index_sym(&self, sym: SymbolId) -> bool {
        self.query.view.is_each_index_sym(sym)
    }
    pub fn nearest_element(&self, id: NodeId) -> Option<NodeId> {
        self.query.view.nearest_element(id)
    }
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.query.view.attr_is_import(attr_id)
    }
    pub fn attr_is_function(&self, attr_id: NodeId) -> bool {
        self.query.view.attr_is_function(attr_id)
    }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.query.expression(id)
    }
    pub fn expr_deps(&self, site: ExprSite) -> Option<ExprDeps<'_>> {
        self.query.expr_deps(site)
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

    /// Check if expression for node is analyzer-classified as async.
    pub fn expr_has_await(&self, id: NodeId) -> bool {
        self.query.view.expr_is_async(id)
    }

    pub fn runtime_plan(&self) -> RuntimePlan {
        self.query.runtime_plan()
    }

    /// Check if expression has blockers (references bindings with blocker metadata).
    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        self.expr_deps(ExprSite::Node(id))
            .is_some_and(|deps| deps.has_blockers())
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
        let ref_symbols = info.ref_symbols().to_vec();
        let mut result = Vec::new();
        for sym in &ref_symbols {
            if let Some(expr) = self.const_tag_symbol_blocker_expr(*sym) {
                result.push(expr);
            }
        }
        result
    }

    // -- Fragment shortcuts --

    pub fn content_type(&self, key: &FragmentKey) -> ContentStrategy {
        self.query.view.content_type(key)
    }
    pub fn fragment_blockers(&self, key: &FragmentKey) -> &[u32] {
        self.query.view.fragment_blockers(key)
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
    pub fn needs_textarea_value_lowering(&self, id: NodeId) -> bool {
        self.query.view.needs_textarea_value_lowering(id)
    }
    pub fn option_synthetic_value_expr(&self, id: NodeId) -> Option<NodeId> {
        self.query.view.option_synthetic_value_expr(id)
    }
    pub fn is_customizable_select(&self, id: NodeId) -> bool {
        self.query.view.is_customizable_select(id)
    }
    pub fn is_selectedcontent(&self, id: NodeId) -> bool {
        self.query.view.is_selectedcontent(id)
    }
    pub fn is_svelte_fragment_slot(&self, id: NodeId) -> bool {
        self.query.view.is_svelte_fragment_slot(id)
    }
    pub fn is_svelte_self(&self, id: NodeId) -> bool {
        self.query.view.is_svelte_self(id)
    }
    pub fn needs_var(&self, id: NodeId) -> bool {
        self.query.view.needs_var(id)
    }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool {
        self.query.view.is_dynamic_attr(id)
    }
    pub fn has_state_attr(&self, id: NodeId) -> bool {
        self.query.view.has_state_attr(id)
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
    pub fn component_binding_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.query.view.component_binding_sym(id)
    }
    pub fn component_css_props(&self, id: NodeId) -> &[(String, NodeId)] {
        self.query.view.component_css_props(id)
    }
    pub fn has_component_css_props(&self, id: NodeId) -> bool {
        self.query.view.has_component_css_props(id)
    }
    pub fn component_snippets(&self, id: NodeId) -> &[NodeId] {
        self.query.view.component_snippets(id)
    }
    pub fn component_named_slots(&self, id: NodeId) -> &[(NodeId, FragmentKey)] {
        self.query.view.component_named_slots(id)
    }
    pub fn is_dynamic_component(&self, id: NodeId) -> bool {
        self.query.view.is_dynamic_component(id)
    }
    pub fn event_handler_mode(&self, attr_id: NodeId) -> Option<EventHandlerMode> {
        self.query.view.event_handler_mode(attr_id)
    }
    pub fn ce_config(&self) -> Option<&svelte_parser::ParsedCeConfig> {
        self.query.view.ce_config()
    }
    pub fn script_rune_calls(&self) -> &svelte_analyze::ScriptRuneCalls {
        self.query.view.script_rune_calls()
    }
    pub fn instance_script_node_id_offset(&self) -> u32 {
        self.query.view.instance_script_node_id_offset()
    }
    pub fn symbol_name(&self, sym: SymbolId) -> &str {
        self.query.view.symbol_name(sym)
    }
    pub fn render_tag_plan(&self, id: NodeId) -> Option<&RenderTagPlan> {
        self.query.view.render_tag_plan(id)
    }
    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.query.view.has_bind_group(id)
    }
    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.query.view.bind_group_value_attr(id)
    }
    pub fn parent_each_blocks(&self, id: NodeId) -> Vec<NodeId> {
        self.query.view.parent_each_blocks(id).into_iter().collect()
    }
    pub fn bind_blockers(&self, id: NodeId) -> &[u32] {
        self.query.view.bind_blockers(id)
    }
    pub fn bind_target_semantics(&self, id: NodeId) -> Option<BindTargetSemantics> {
        self.query.view.bind_target_semantics(id)
    }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.query.view.attr_expression(id)
    }
    #[allow(dead_code)]
    pub fn attr_expression_blockers(&self, id: NodeId) -> Vec<u32> {
        self.expr_deps(ExprSite::Attr(id))
            .map(|deps| deps.blockers.into_iter().collect())
            .unwrap_or_default()
    }
    #[allow(dead_code)]
    pub fn needs_expr_memoization(&self, id: NodeId) -> bool {
        self.expr_deps(ExprSite::Node(id))
            .is_some_and(|deps| deps.needs_memo)
    }
    pub fn symbol_blocker(&self, sym: SymbolId) -> Option<u32> {
        self.query.view.symbol_blocker(sym)
    }

    // -- Snippet shortcuts --

    // -- ConstTag shortcuts --

    pub fn const_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.query.view.const_tags_for_fragment(key)
    }

    // -- DebugTag shortcuts --

    pub fn debug_tag(&self, id: NodeId) -> &'a DebugTag {
        self.query.debug_tag(id)
    }
    pub fn debug_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.query.view.debug_tags_for_fragment(key)
    }
    pub fn title_elements_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.query.view.title_elements_for_fragment(key)
    }

    // -- Parsed template JS handles --

    pub fn node_expr_handle(&self, node_id: NodeId) -> svelte_analyze::ExprHandle {
        self.query.view.node_expr_handle(node_id)
    }

    pub fn attr_expr_handle(&self, attr_id: NodeId) -> svelte_analyze::ExprHandle {
        self.query.view.attr_expr_handle(attr_id)
    }

    pub fn const_tag_stmt_handle(&self, id: NodeId) -> Option<svelte_analyze::StmtHandle> {
        self.query.view.const_tag_stmt_handle(id)
    }

    pub fn let_directive_stmt_handle(&self, id: NodeId) -> Option<svelte_analyze::StmtHandle> {
        self.query.view.let_directive_stmt_handle(id)
    }

    pub fn fragment_references_any_symbol(
        &self,
        key: &FragmentKey,
        syms: &rustc_hash::FxHashSet<SymbolId>,
    ) -> bool {
        self.query.view.fragment_references_any_symbol(key, syms)
    }

    // -- Delegated events --

    /// Register a delegated event name (deduplicates via O(1) HashSet lookup).
    pub fn add_delegated_event(&mut self, event_name: String) {
        self.state.add_delegated_event(event_name);
    }

    // -- CSS scoping --

    /// Scoping class for this component, e.g. `"svelte-1a7i8ec"`.
    /// Returns an empty string when no `<style>` block is present.
    pub fn css_hash(&self) -> &str {
        self.query.view.css_hash()
    }

    /// Whether this element should receive the scoped CSS class attribute.
    pub fn is_css_scoped(&self, id: NodeId) -> bool {
        self.query.view.is_css_scoped(id)
    }
}
