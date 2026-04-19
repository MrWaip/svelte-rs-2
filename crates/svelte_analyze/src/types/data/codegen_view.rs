use super::*;

#[derive(Clone, Copy)]
pub struct CodegenView<'d, 'a> {
    data: &'d AnalysisData<'a>,
}

impl<'d, 'a> CodegenView<'d, 'a> {
    pub fn new(data: &'d AnalysisData<'a>) -> Self {
        Self { data }
    }

    pub fn custom_element(&self) -> bool {
        self.data.output.custom_element
    }
    pub fn runes(&self) -> bool {
        self.data.uses_runes()
    }
    pub fn accessors(&self) -> bool {
        self.data.script.accessors
    }
    pub fn immutable(&self) -> bool {
        self.data.script.immutable
    }
    pub fn preserve_whitespace(&self) -> bool {
        self.data.script.preserve_whitespace
    }
    pub fn runtime_plan(&self) -> RuntimePlan {
        self.data.output.runtime_plan
    }
    pub fn component_name(&self) -> &str {
        self.data.component_name()
    }
    pub fn is_dynamic(&self, id: NodeId) -> bool {
        self.data.dynamism.is_dynamic_node(id)
    }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool {
        self.data.is_elseif_alt(id)
    }
    pub fn exports(&self) -> &[ExportInfo] {
        &self.data.script.exports
    }
    pub fn props(&self) -> Option<&PropsAnalysis> {
        self.data.script.props.as_ref()
    }
    pub fn needs_context(&self) -> bool {
        self.data.output.needs_context
    }
    pub fn needs_sanitized_legacy_slots(&self) -> bool {
        self.data.output.needs_sanitized_legacy_slots
    }
    pub fn custom_element_slot_names(&self) -> &[String] {
        self.data.custom_element_slot_names()
    }
    pub fn props_id(&self) -> Option<&str> {
        self.data.script.props_id.as_deref()
    }
    pub fn scoping(&self) -> &ComponentScoping<'a> {
        &self.data.scoping
    }
    pub fn iter_store_declarations(
        &self,
    ) -> impl Iterator<
        Item = (
            svelte_component_semantics::OxcNodeId,
            StoreDeclarationSemantics,
        ),
    > + '_ {
        self.data.iter_store_declarations()
    }
    pub fn ce_config(&self) -> Option<&svelte_parser::ParsedCeConfig> {
        self.data.script.ce_config.as_ref()
    }
    pub fn script_rune_calls(&self) -> &ScriptRuneCalls {
        self.data.script_rune_calls()
    }
    pub fn instance_script_node_id_offset(&self) -> u32 {
        self.data.script.instance_node_id_offset
    }
    pub fn module_script_node_id_offset(&self) -> u32 {
        self.data.script.module_node_id_offset
    }
    pub fn symbol_name(&self, sym: SymbolId) -> &str {
        self.data.scoping.symbol_name(sym)
    }
    pub fn find_binding(&self, scope_id: oxc_semantic::ScopeId, name: &str) -> Option<SymbolId> {
        self.data.scoping.find_binding(scope_id, name)
    }
    pub fn fragment_scope(&self, key: &FragmentKey) -> Option<oxc_semantic::ScopeId> {
        self.data.scoping.fragment_scope(key)
    }
    pub fn blocker_data(&self) -> &BlockerData {
        self.data.blocker_data()
    }
    pub fn symbol_blocker(&self, sym: SymbolId) -> Option<u32> {
        self.data.blocker_data().symbol_blocker(sym)
    }
    pub fn is_pickled_await(&self, offset: u32) -> bool {
        self.data.is_pickled_await(offset)
    }
    pub fn is_ignored(&self, node_id: NodeId, code: &str) -> bool {
        self.data.output.ignore_data.is_ignored(node_id, code)
    }
    pub fn is_ignored_at_span(&self, span_start: u32, code: &str) -> bool {
        self.data
            .output
            .ignore_data
            .is_ignored_at_span(span_start, code)
    }
    pub fn await_value_binding(&self, id: NodeId) -> Option<&AwaitBindingInfo> {
        self.data.template.await_bindings.value(id)
    }
    pub fn await_error_binding(&self, id: NodeId) -> Option<&AwaitBindingInfo> {
        self.data.template.await_bindings.error(id)
    }
    pub fn expr_deps(&self, site: ExprSite) -> Option<ExprDeps<'_>> {
        self.data.expr_deps(site)
    }
    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        self.data.expr_has_blockers(id)
    }
    pub fn expression_blockers(&self, id: NodeId) -> SmallVec<[u32; 2]> {
        self.data.expression_blockers(id)
    }
    pub fn attr_expression_blockers(&self, id: NodeId) -> SmallVec<[u32; 2]> {
        self.data.attr_expression_blockers(id)
    }
    pub fn needs_expr_memoization(&self, id: NodeId) -> bool {
        self.data.needs_expr_memoization(id)
    }
    pub fn expr_is_async(&self, id: NodeId) -> bool {
        self.data.expr_is_async(id)
    }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.data.attr_expression(id)
    }
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.data.attr_is_import(attr_id)
    }
    pub fn attr_is_function(&self, attr_id: NodeId) -> bool {
        self.data.attr_is_function(attr_id)
    }
    pub fn node_expr_handle(&self, id: NodeId) -> ExprHandle {
        self.data.node_expr_handle(id)
    }
    pub fn attr_expr_handle(&self, id: NodeId) -> ExprHandle {
        self.data.attr_expr_handle(id)
    }
    /// Non-panicking attribute expression handle lookup. Useful for directives
    /// that may lack an expression in rare cases (e.g. function bindings).
    pub fn attr_expr_handle_opt(&self, id: NodeId) -> Option<ExprHandle> {
        self.data
            .template
            .template_semantics
            .attr_expr_handles
            .get(id)
            .copied()
    }
    pub fn const_tag_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.data.const_tag_stmt_handle(id)
    }
    pub fn let_directive_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.data.let_directive_stmt_handle(id)
    }
    pub fn snippet_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.data.snippet_stmt_handle(id)
    }
    pub fn await_value_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.data.await_value_stmt_handle(id)
    }
    pub fn await_error_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.data.await_error_stmt_handle(id)
    }
    pub fn node_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.data.node_ref_symbols(id)
    }
    pub fn stmt_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.data.stmt_ref_symbols(id)
    }
    pub fn snippet_param_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.data.snippet_param_ref_symbols(id)
    }
    pub fn shorthand_symbol(&self, id: NodeId) -> Option<SymbolId> {
        self.data.shorthand_symbol(id)
    }
    pub fn bind_target_symbol(&self, id: NodeId) -> Option<SymbolId> {
        self.data.bind_target_symbol(id)
    }
    pub fn symbol_for_reference(
        &self,
        ref_id: svelte_component_semantics::ReferenceId,
    ) -> Option<SymbolId> {
        self.data.symbol_for_reference(ref_id)
    }
    pub fn symbol_for_identifier_reference(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<SymbolId> {
        self.data.symbol_for_identifier_reference(id)
    }
    pub fn binding_origin_key(&self, sym: SymbolId) -> Option<&str> {
        self.data.binding_origin_key(sym)
    }
    pub fn binding_origin_key_for_reference(
        &self,
        ref_id: svelte_component_semantics::ReferenceId,
    ) -> Option<&str> {
        self.data.binding_origin_key_for_reference(ref_id)
    }
    pub fn binding_origin_key_for_identifier_reference(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<&str> {
        self.data.binding_origin_key_for_identifier_reference(id)
    }
    pub fn declaration_root(&self, sym: SymbolId) -> Option<svelte_component_semantics::OxcNodeId> {
        self.data.declaration_root_for_symbol(sym)
    }
    pub fn declaration_semantics(
        &self,
        node_id: svelte_component_semantics::OxcNodeId,
    ) -> DeclarationSemantics {
        self.data.declaration_semantics(node_id)
    }
    pub fn reference_semantics(
        &self,
        ref_id: svelte_component_semantics::ReferenceId,
    ) -> ReferenceSemantics {
        self.data.reference_semantics(ref_id)
    }
    pub fn lowered_fragment(&self, key: &FragmentKey) -> Option<&LoweredFragment> {
        self.data.template.fragments.lowered(key)
    }
    pub fn fragment_blockers(&self, key: &FragmentKey) -> &[u32] {
        self.data.template.fragments.fragment_blockers(key)
    }
    pub fn fragment_references_any_symbol(
        &self,
        key: &FragmentKey,
        syms: &FxHashSet<SymbolId>,
    ) -> bool {
        self.data.fragment_references_any_symbol(key, syms)
    }
    pub fn content_type(&self, key: &FragmentKey) -> ContentStrategy {
        self.data.template.fragments.content_type(key)
    }
    pub fn has_dynamic_children(&self, key: &FragmentKey) -> bool {
        self.data.template.fragments.has_dynamic_children(key)
    }
    pub fn string_attribute<'b>(
        &self,
        id: NodeId,
        attrs: &'b [svelte_ast::Attribute],
        name: &str,
    ) -> Option<&'b svelte_ast::StringAttribute> {
        self.data.string_attribute(id, attrs, name)
    }
    pub fn attr_index(&self, id: NodeId) -> Option<&AttrIndex> {
        self.data.attr_index(id)
    }
    pub fn has_spread(&self, id: NodeId) -> bool {
        self.data.has_spread(id)
    }
    pub fn namespace(&self, id: NodeId) -> Option<NamespaceKind> {
        self.data.namespace(id)
    }
    pub fn creation_namespace(&self, id: NodeId) -> Option<svelte_ast::Namespace> {
        self.data.creation_namespace(id)
    }
    pub fn is_void(&self, id: NodeId) -> bool {
        self.data.is_void(id)
    }
    pub fn is_custom_element(&self, id: NodeId) -> bool {
        self.data.is_custom_element(id)
    }
    pub fn event_modifiers(&self, id: NodeId) -> EventModifier {
        self.data.event_modifiers(id)
    }
    pub fn has_class_directives(&self, id: NodeId) -> bool {
        self.data.elements.flags.has_class_directives(id)
    }
    pub fn has_class_attribute(&self, id: NodeId) -> bool {
        self.data.elements.flags.has_class_attribute(id)
    }
    pub fn needs_clsx(&self, id: NodeId) -> bool {
        self.data.elements.flags.needs_clsx(id)
    }
    pub fn has_style_directives(&self, id: NodeId) -> bool {
        self.data.elements.flags.has_style_directives(id)
    }
    pub fn style_directives(&self, id: NodeId) -> &[StyleDirective] {
        self.data.elements.flags.style_directives(id)
    }
    pub fn needs_input_defaults(&self, id: NodeId) -> bool {
        self.data.elements.flags.needs_input_defaults(id)
    }
    pub fn needs_var(&self, id: NodeId) -> bool {
        self.data.elements.flags.needs_var(id)
    }
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool {
        self.data.dynamism.is_dynamic_attr(id)
    }
    pub fn has_state_attr(&self, id: NodeId) -> bool {
        self.data.dynamism.has_state_attr(id)
    }
    pub fn static_class(&self, id: NodeId) -> Option<&str> {
        self.data.elements.flags.static_class(id)
    }
    pub fn static_style(&self, id: NodeId) -> Option<&str> {
        self.data.elements.flags.static_style(id)
    }
    pub fn is_bound_contenteditable(&self, id: NodeId) -> bool {
        self.data.elements.flags.is_bound_contenteditable(id)
    }
    pub fn has_use_directive(&self, id: NodeId) -> bool {
        self.data.elements.flags.has_use_directive(id)
    }
    pub fn has_dynamic_class_directives(&self, id: NodeId) -> bool {
        self.data.elements.flags.has_dynamic_class_directives(id)
    }
    pub fn class_needs_state(&self, id: NodeId) -> bool {
        self.data.class_needs_state(id)
    }
    pub fn class_attr_id(&self, id: NodeId) -> Option<NodeId> {
        self.data.elements.flags.class_attr_id(id)
    }
    pub fn class_directive_info(&self, id: NodeId) -> Option<&[ClassDirectiveInfo]> {
        self.data.elements.flags.class_directive_info(id)
    }
    pub fn is_expression_shorthand(&self, id: NodeId) -> bool {
        self.data.elements.flags.is_expression_shorthand(id)
    }
    pub fn component_props(&self, id: NodeId) -> &[ComponentPropInfo] {
        self.data.elements.flags.component_props(id)
    }
    pub fn component_binding_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.data.elements.flags.component_binding_sym(id)
    }
    pub fn component_css_props(&self, id: NodeId) -> &[(String, NodeId)] {
        self.data.elements.flags.component_css_props(id)
    }
    pub fn has_component_css_props(&self, id: NodeId) -> bool {
        self.data.elements.flags.has_component_css_props(id)
    }
    pub fn component_snippets(&self, id: NodeId) -> &[NodeId] {
        self.data.template.snippets.component_snippets(id)
    }
    pub fn component_named_slots(&self, id: NodeId) -> &[(NodeId, FragmentKey)] {
        self.data.template.snippets.component_named_slots(id)
    }
    pub fn is_dynamic_component(&self, id: NodeId) -> bool {
        self.data.dynamism.is_dynamic_component(id)
    }
    pub fn is_snippet_hoistable(&self, id: NodeId) -> bool {
        self.data.template.snippets.is_hoistable(id)
    }
    pub fn event_handler_mode(&self, id: NodeId) -> Option<EventHandlerMode> {
        self.data.elements.flags.event_handler_mode(id)
    }
    pub fn needs_textarea_value_lowering(&self, id: NodeId) -> bool {
        self.data.elements.flags.needs_textarea_value_lowering(id)
    }
    pub fn option_synthetic_value_expr(&self, id: NodeId) -> Option<NodeId> {
        self.data.elements.flags.option_synthetic_value_expr(id)
    }
    pub fn is_customizable_select(&self, id: NodeId) -> bool {
        self.data.elements.flags.is_customizable_select(id)
    }
    pub fn is_selectedcontent(&self, id: NodeId) -> bool {
        self.data.elements.flags.is_selectedcontent(id)
    }
    pub fn is_svelte_fragment_slot(&self, id: NodeId) -> bool {
        self.data.elements.flags.is_svelte_fragment_slot(id)
    }
    pub fn is_svelte_self(&self, id: NodeId) -> bool {
        self.data.elements.flags.is_svelte_self(id)
    }
    pub fn render_tag_plan(&self, id: NodeId) -> Option<&RenderTagPlan> {
        self.data.render_tag_plan(id)
    }
    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.data.template.bind_semantics.has_bind_group(id)
    }
    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.data.template.bind_semantics.bind_group_value_attr(id)
    }
    pub fn parent_each_blocks(&self, id: NodeId) -> SmallVec<[NodeId; 4]> {
        self.data.parent_each_blocks(id)
    }
    #[deprecated(note = "use block_semantics(id) — matches!(sem.flavor, EachFlavor::BindGroup)")]
    pub fn contains_group_binding(&self, id: NodeId) -> bool {
        self.data.contains_group_binding(id)
    }
    pub fn bind_blockers(&self, id: NodeId) -> &[u32] {
        self.data.template.bind_semantics.bind_blockers(id)
    }
    pub fn bind_target_semantics(&self, id: NodeId) -> Option<BindTargetSemantics> {
        self.data.bind_target_semantics(id).copied()
    }
    pub fn bind_each_context(&self, id: NodeId) -> Option<&[SymbolId]> {
        self.data.bind_each_context(id)
    }
    pub fn const_tag_names(&self, id: NodeId) -> Option<&Vec<String>> {
        self.data.template.const_tags.names(id)
    }
    pub fn const_tag_syms(&self, id: NodeId) -> Option<&[SymbolId]> {
        self.data.const_tag_syms(id)
    }
    pub fn const_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.data.template.const_tags.by_fragment(key)
    }
    pub fn debug_tags_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.data.template.debug_tags.by_fragment(key)
    }
    pub fn title_elements_for_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.data.template.title_elements.by_fragment(key)
    }
    #[deprecated(note = "use block_semantics(id)")]
    pub fn each_index_name(&self, id: NodeId) -> Option<&str> {
        self.data
            .each_index_sym(id)
            .map(|sym| self.data.scoping.symbol_name(sym))
    }
    /// Returns true when the given symbol is the `index` binding of some
    /// `{#each}` block. Each-block indices are always `number`, so an
    /// interpolation referencing one never needs a `?? ""` fallback.
    #[deprecated(note = "use block_semantics_store.block_for_each_index_sym(sym).is_some()")]
    pub fn is_each_index_sym(&self, sym: SymbolId) -> bool {
        self.data.each_block_for_index_sym(sym).is_some()
    }
    #[deprecated(note = "use block_semantics(id)")]
    pub fn each_key_uses_index(&self, id: NodeId) -> bool {
        self.data.each_key_uses_index(id)
    }
    #[deprecated(note = "use block_semantics(id)")]
    pub fn each_body_uses_index(&self, id: NodeId) -> bool {
        self.data.each_body_uses_index(id)
    }
    #[deprecated(note = "use block_semantics(id)")]
    pub fn each_key_is_item(&self, id: NodeId) -> bool {
        self.data.each_key_is_item(id)
    }
    #[deprecated(note = "use block_semantics(id).shadows_outer")]
    pub fn each_needs_collection_id(&self, id: NodeId) -> bool {
        self.data.each_needs_collection_id(id)
    }
    #[deprecated(note = "use block_semantics(id)")]
    pub fn each_has_animate(&self, id: NodeId) -> bool {
        self.data.each_has_animate(id)
    }
    #[deprecated(note = "read the name from the source via block.context_span")]
    pub fn each_context_name(&self, id: NodeId) -> &str {
        self.data.each_context_name(id)
    }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.data.expression(id)
    }
    pub fn known_value(&self, name: &str) -> Option<&str> {
        self.data.known_value(name)
    }
    pub fn html_tag_in_svg(&self, id: NodeId) -> bool {
        self.data.html_tag_in_svg(id)
    }
    pub fn html_tag_in_mathml(&self, id: NodeId) -> bool {
        self.data.html_tag_in_mathml(id)
    }
    pub fn nearest_element(&self, id: NodeId) -> Option<NodeId> {
        self.data.nearest_element(id)
    }
    pub fn template_element_parent(&self, id: NodeId) -> Option<NodeId> {
        self.data.template_element_parent(id)
    }
    /// The scoping class for this component, e.g. `"svelte-1a7i8ec"`.
    /// Returns an empty string when no `<style>` block is present.
    pub fn css_hash(&self) -> &str {
        self.data.css_hash()
    }
    /// Whether the element should receive the scoped CSS class.
    pub fn is_css_scoped(&self, id: NodeId) -> bool {
        self.data.is_css_scoped(id)
    }
    /// Whether CSS should be injected at runtime via `$.append_styles()`.
    pub fn inject_styles(&self) -> bool {
        self.data.inject_styles()
    }
}
