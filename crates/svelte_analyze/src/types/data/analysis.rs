use super::*;
use svelte_ast::{Attribute, BindDirective, ExpressionAttribute, StringAttribute};

pub struct AnalysisData {
    pub expressions: NodeTable<ExpressionInfo>,
    pub attr_expressions: NodeTable<ExpressionInfo>,
    pub script: Option<ScriptInfo>,
    pub scoping: ComponentScoping,
    pub dynamic_nodes: NodeBitSet,
    pub alt_is_elseif: NodeBitSet,
    pub props: Option<PropsAnalysis>,
    pub props_id: Option<String>,
    pub exports: Vec<ExportInfo>,
    pub needs_context: bool,
    pub has_class_state_fields: bool,
    element_facts: ElementFacts,
    pub element_flags: ElementFlags,
    pub fragment_facts: FragmentFacts,
    pub rich_content_facts: RichContentFacts,
    pub fragments: FragmentData,
    pub snippets: SnippetData,
    pub const_tags: ConstTagData,
    pub debug_tags: DebugTagData,
    pub title_elements: TitleElementData,
    pub each_context: EachContextIndex,
    pub template_topology: TemplateTopology,
    pub template_elements: TemplateElementIndex,
    pub template_semantics: TemplateSemanticsData,
    pub(crate) render_tag_callee_sym: NodeTable<SymbolId>,
    pub(crate) render_tag_is_chain: NodeBitSet,
    pub render_tag_plans: NodeTable<RenderTagPlan>,
    pub(crate) html_tag_in_svg: NodeBitSet,
    pub(crate) html_tag_in_mathml: NodeBitSet,
    pub await_bindings: AwaitBindingData,
    pub bind_semantics: BindSemanticsData,
    pub import_syms: FxHashSet<SymbolId>,
    pub runes: bool,
    pub custom_element: bool,
    pub experimental_async: bool,
    pub runtime_plan: RuntimePlan,
    pub ce_config: Option<svelte_parser::ParsedCeConfig>,
    pub(crate) proxy_state_inits: FxHashMap<compact_str::CompactString, bool>,
    pub(crate) has_store_member_mutations: bool,
    pub(crate) blocker_data: BlockerData,
    pub(crate) script_rune_call_kinds: FxHashMap<u32, crate::types::script::RuneKind>,
    pub(crate) pickled_await_offsets: FxHashSet<u32>,
    pub ignore_data: IgnoreData,
    /// CSS-scoping metadata: hash, scoped elements, transformed stylesheet.
    pub css: CssAnalysis,
}

impl AnalysisData {
    pub(crate) fn new_empty(node_count: u32) -> Self {
        Self {
            expressions: NodeTable::new(node_count),
            attr_expressions: NodeTable::new(node_count),
            script: None,
            scoping: ComponentScoping::new_empty(),
            dynamic_nodes: NodeBitSet::new(node_count),
            alt_is_elseif: NodeBitSet::new(node_count),
            props: None,
            props_id: None,
            exports: Vec::new(),
            needs_context: false,
            has_class_state_fields: false,
            element_facts: ElementFacts::new(node_count),
            element_flags: ElementFlags::new(node_count),
            fragment_facts: FragmentFacts::new(),
            rich_content_facts: RichContentFacts::new(),
            fragments: FragmentData::with_capacity(node_count as usize / 3),
            snippets: SnippetData::new(node_count),
            const_tags: ConstTagData::new(node_count),
            debug_tags: DebugTagData::new(),
            title_elements: TitleElementData::new(),
            each_context: EachContextIndex::new(node_count),
            template_topology: TemplateTopology::new(node_count),
            template_elements: TemplateElementIndex::new(node_count),
            template_semantics: TemplateSemanticsData::new(node_count),
            render_tag_callee_sym: NodeTable::new(node_count),
            render_tag_is_chain: NodeBitSet::new(node_count),
            render_tag_plans: NodeTable::new(node_count),
            html_tag_in_svg: NodeBitSet::new(node_count),
            html_tag_in_mathml: NodeBitSet::new(node_count),
            await_bindings: AwaitBindingData::new(node_count),
            bind_semantics: BindSemanticsData::new(node_count),
            import_syms: FxHashSet::default(),
            runes: true,
            custom_element: false,
            experimental_async: false,
            runtime_plan: RuntimePlan::default(),
            ce_config: None,
            proxy_state_inits: FxHashMap::default(),
            has_store_member_mutations: false,
            blocker_data: BlockerData::default(),
            script_rune_call_kinds: FxHashMap::default(),
            pickled_await_offsets: FxHashSet::default(),
            ignore_data: IgnoreData::new(),
            css: CssAnalysis::empty(node_count),
        }
    }
}

impl AnalysisData {
    pub(crate) fn record_element_facts(&mut self, id: NodeId, attrs: &[Attribute]) {
        self.element_facts.record(id, attrs);
    }
    pub fn html_tag_in_svg(&self, id: NodeId) -> bool {
        self.html_tag_in_svg.contains(&id)
    }
    pub fn html_tag_in_mathml(&self, id: NodeId) -> bool {
        self.html_tag_in_mathml.contains(&id)
    }
    pub fn blocker_data(&self) -> &BlockerData {
        &self.blocker_data
    }
    pub fn script_rune_call_kinds(&self) -> &FxHashMap<u32, crate::types::script::RuneKind> {
        &self.script_rune_call_kinds
    }
    pub fn script_rune_call_kind(&self, offset: u32) -> Option<crate::types::script::RuneKind> {
        self.script_rune_call_kinds.get(&offset).copied()
    }
    pub fn is_pickled_await(&self, offset: u32) -> bool {
        self.pickled_await_offsets.contains(&offset)
    }
    pub fn is_dynamic(&self, id: NodeId) -> bool {
        self.dynamic_nodes.contains(&id)
    }
    pub fn is_elseif_alt(&self, id: NodeId) -> bool {
        self.alt_is_elseif.contains(&id)
    }
    pub fn expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.expressions.get(id)
    }
    pub fn fragment_facts(&self, key: &FragmentKey) -> Option<&FragmentFactsEntry> {
        self.fragment_facts.entry(key)
    }
    pub fn fragment_has_children(&self, key: &FragmentKey) -> bool {
        self.fragment_facts.has_children(key)
    }
    pub fn fragment_child_count(&self, key: &FragmentKey) -> u32 {
        self.fragment_facts.child_count(key)
    }
    pub fn fragment_single_child(&self, key: &FragmentKey) -> Option<NodeId> {
        self.fragment_facts.single_child(key)
    }
    pub fn fragment_non_trivial_child_count(&self, key: &FragmentKey) -> u32 {
        self.fragment_facts.non_trivial_child_count(key)
    }
    pub fn fragment_has_non_trivial_children(&self, key: &FragmentKey) -> bool {
        self.fragment_facts.has_non_trivial_children(key)
    }
    pub fn fragment_has_expression_child(&self, key: &FragmentKey) -> bool {
        self.fragment_facts.has_expression_child(key)
    }
    pub fn fragment_single_expression_child(&self, key: &FragmentKey) -> Option<NodeId> {
        self.fragment_facts.single_expression_child(key)
    }
    pub fn fragment_single_non_trivial_child(&self, key: &FragmentKey) -> Option<NodeId> {
        self.fragment_facts.single_non_trivial_child(key)
    }
    pub fn fragment_has_direct_animate_child(&self, key: &FragmentKey) -> bool {
        self.fragment_facts.has_direct_animate_child(key)
    }
    pub fn fragment_has_rich_content(
        &self,
        key: &FragmentKey,
        parent: RichContentParentKind,
    ) -> bool {
        self.rich_content_facts.has_rich_content(key, parent)
    }
    pub fn element_facts(&self, id: NodeId) -> Option<&ElementFactsEntry> {
        self.element_facts.entry(id)
    }
    fn attr_index(&self, id: NodeId) -> Option<&AttrIndex> {
        self.element_facts.attr_index(id)
    }
    pub fn has_attribute(&self, id: NodeId, name: &str) -> bool {
        self.attr_index(id).is_some_and(|index| index.has(name))
    }
    pub fn attribute<'a>(
        &self,
        id: NodeId,
        attrs: &'a [Attribute],
        name: &str,
    ) -> Option<&'a Attribute> {
        self.attr_index(id)?.first(attrs, name)
    }
    pub fn string_attribute<'a>(
        &self,
        id: NodeId,
        attrs: &'a [Attribute],
        name: &str,
    ) -> Option<&'a svelte_ast::StringAttribute> {
        self.attribute(id, attrs, name).and_then(|attr| match attr {
            Attribute::StringAttribute(attr) => Some(attr),
            _ => None,
        })
    }
    pub fn expression_attribute<'a>(
        &self,
        id: NodeId,
        attrs: &'a [Attribute],
        name: &str,
    ) -> Option<&'a ExpressionAttribute> {
        self.attribute(id, attrs, name).and_then(|attr| match attr {
            Attribute::ExpressionAttribute(attr) => Some(attr),
            _ => None,
        })
    }
    pub fn bind_directive<'a>(
        &self,
        id: NodeId,
        attrs: &'a [Attribute],
        name: &str,
    ) -> Option<&'a BindDirective> {
        self.attribute(id, attrs, name).and_then(|attr| match attr {
            Attribute::BindDirective(attr) => Some(attr),
            _ => None,
        })
    }
    pub fn static_text_attribute_value<'a>(
        &self,
        id: NodeId,
        attrs: &'a [Attribute],
        name: &str,
        source: &'a str,
    ) -> Option<&'a str> {
        self.string_attribute(id, attrs, name)
            .map(|attr: &'a StringAttribute| attr.value_span.source_text(source))
    }
    pub fn has_true_boolean_attribute(
        &self,
        id: NodeId,
        attrs: &[Attribute],
        name: &str,
        source: &str,
    ) -> bool {
        self.attribute(id, attrs, name)
            .is_some_and(|attr| match attr {
                Attribute::BooleanAttribute(_) => true,
                Attribute::StringAttribute(attr) => {
                    attr.value_span.source_text(source).trim() == "true"
                }
                _ => false,
            })
    }
    pub fn has_spread(&self, id: NodeId) -> bool {
        self.element_facts.has_spread(id)
    }
    pub fn has_runtime_attrs(&self, id: NodeId) -> bool {
        self.element_facts.has_runtime_attrs(id)
    }
    pub fn parent(&self, id: NodeId) -> Option<ParentRef> {
        self.template_topology.parent(id)
    }
    pub fn expr_parent(&self, id: NodeId) -> Option<ParentRef> {
        self.template_topology.expr_parent(id)
    }
    pub fn ancestors(&self, id: NodeId) -> crate::types::data::template_topology::Ancestors<'_> {
        self.template_topology.ancestors(id)
    }
    pub fn expr_ancestors(
        &self,
        id: NodeId,
    ) -> crate::types::data::template_topology::Ancestors<'_> {
        self.template_topology.expr_ancestors(id)
    }
    pub fn nearest_element(&self, id: NodeId) -> Option<NodeId> {
        self.template_topology.nearest_element(id)
    }
    pub fn nearest_element_for_expr(&self, id: NodeId) -> Option<NodeId> {
        self.template_topology.nearest_element_for_expr(id)
    }
    pub fn template_element(&self, id: NodeId) -> Option<&TemplateElementEntry> {
        self.template_elements.entry(id)
    }
    pub fn template_elements(&self) -> &[NodeId] {
        self.template_elements.all_elements()
    }
    pub fn template_elements_with_tag(&self, tag_name: &str) -> &[NodeId] {
        self.template_elements.elements_with_tag(tag_name)
    }
    pub fn template_element_parent(&self, id: NodeId) -> Option<NodeId> {
        self.template_elements.parent_element(id)
    }
    pub fn template_element_previous_sibling(&self, id: NodeId) -> Option<NodeId> {
        self.template_elements.previous_sibling(id)
    }
    pub fn template_element_next_sibling(&self, id: NodeId) -> Option<NodeId> {
        self.template_elements.next_sibling(id)
    }
    pub fn template_element_tag_name(&self, id: NodeId) -> Option<&str> {
        self.template_elements.tag_name(id)
    }
    pub fn template_element_static_id(&self, id: NodeId) -> Option<&str> {
        self.template_elements.static_id(id)
    }
    pub fn template_element_has_static_class(&self, id: NodeId, class_name: &str) -> bool {
        self.template_elements.has_static_class(id, class_name)
    }
    pub fn template_element_may_match_class(&self, id: NodeId) -> bool {
        self.template_elements.may_match_class(id)
    }
    pub fn template_element_may_match_id(&self, id: NodeId) -> bool {
        self.template_elements.may_match_id(id)
    }
    pub fn template_elements_for_class(&self, class_name: &str) -> SmallVec<[NodeId; 8]> {
        self.template_elements.class_candidates(class_name)
    }
    pub fn template_elements_for_id(&self, id_name: &str) -> SmallVec<[NodeId; 8]> {
        self.template_elements.id_candidates(id_name)
    }
    pub fn template_element_previous_siblings(&self, id: NodeId) -> SmallVec<[NodeId; 8]> {
        self.template_elements.previous_siblings(id)
    }
    pub fn each_index_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.each_context.index_sym(id)
    }
    pub fn each_block_for_index_sym(&self, sym: SymbolId) -> Option<NodeId> {
        self.each_context.block_for_index_sym(sym)
    }
    pub fn each_key_node_id(&self, id: NodeId) -> Option<NodeId> {
        self.each_context.key_node_id(id)
    }
    pub fn each_key_uses_index(&self, id: NodeId) -> bool {
        self.each_context.key_uses_index(id)
    }
    pub fn each_is_destructured(&self, id: NodeId) -> bool {
        self.each_context.is_destructured(id)
    }
    pub fn each_body_uses_index(&self, id: NodeId) -> bool {
        self.each_context.body_uses_index(id)
    }
    pub fn each_key_is_item(&self, id: NodeId) -> bool {
        self.each_context.key_is_item(id)
    }
    pub fn each_has_animate(&self, id: NodeId) -> bool {
        self.each_context.has_animate(id)
    }
    pub fn each_context_name(&self, id: NodeId) -> &str {
        self.each_context.context_name(id)
    }
    pub fn bind_each_context(&self, id: NodeId) -> Option<&Vec<String>> {
        self.each_context.bind_this_context(id)
    }
    pub fn parent_each_blocks(&self, id: NodeId) -> Option<&Vec<NodeId>> {
        self.each_context.parent_each_blocks(id)
    }
    pub fn contains_group_binding(&self, id: NodeId) -> bool {
        self.each_context.contains_group_binding(id)
    }
    pub fn css_hash(&self) -> &str {
        &self.css.hash
    }
    pub fn is_css_scoped(&self, id: NodeId) -> bool {
        self.css.scoped_elements.contains(&id)
    }
    pub fn inject_styles(&self) -> bool {
        self.css.inject_styles
    }
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.attr_expressions.get(id)
    }
    pub fn node_expr_handle(&self, id: NodeId) -> ExprHandle {
        *self
            .template_semantics
            .node_expr_handles
            .get(id)
            .unwrap_or_else(|| panic!("no expr handle for node {:?}", id))
    }
    pub fn attr_expr_handle(&self, id: NodeId) -> ExprHandle {
        *self
            .template_semantics
            .attr_expr_handles
            .get(id)
            .unwrap_or_else(|| panic!("no expr handle for attr {:?}", id))
    }
    pub fn const_tag_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.template_semantics
            .const_tag_stmt_handles
            .get(id)
            .copied()
    }
    pub fn snippet_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.template_semantics
            .snippet_stmt_handles
            .get(id)
            .copied()
    }
    pub fn each_context_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.template_semantics
            .each_context_stmt_handles
            .get(id)
            .copied()
    }
    pub fn each_index_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.template_semantics
            .each_index_stmt_handles
            .get(id)
            .copied()
    }
    pub fn await_value_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.template_semantics
            .await_value_stmt_handles
            .get(id)
            .copied()
    }
    pub fn await_error_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.template_semantics
            .await_error_stmt_handles
            .get(id)
            .copied()
    }
    pub fn node_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.template_semantics.node_ref_symbols(id)
    }
    pub fn stmt_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.template_semantics.stmt_ref_symbols(id)
    }
    pub fn snippet_param_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.template_semantics.stmt_ref_symbols(id)
    }
    pub fn shorthand_symbol(&self, id: NodeId) -> Option<SymbolId> {
        self.template_semantics
            .node_ref_symbols(id)
            .first()
            .copied()
    }
    pub fn bind_target_symbol(&self, id: NodeId) -> Option<SymbolId> {
        self.attr_expressions
            .get(id)
            .and_then(|info| match info.kind {
                ExpressionKind::Identifier(_) => info.ref_symbols.first().copied(),
                _ => None,
            })
            .or_else(|| self.shorthand_symbol(id))
    }
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(attr_id).is_some_and(|info| {
            info.ref_symbols
                .first()
                .copied()
                .or_else(|| match &info.kind {
                    ExpressionKind::Identifier(name) => self
                        .scoping
                        .find_binding(self.scoping.root_scope_id(), name.as_str()),
                    _ => None,
                })
                .is_some_and(|sym| self.import_syms.contains(&sym))
        })
    }
    pub fn render_tag_plan(&self, id: NodeId) -> Option<&RenderTagPlan> {
        self.render_tag_plans.get(id)
    }
    pub fn const_tag_syms(&self, id: NodeId) -> Option<&[SymbolId]> {
        self.const_tags.syms(id).map(|syms| syms.as_slice())
    }
    pub fn expr_deps(&self, site: ExprSite) -> Option<ExprDeps<'_>> {
        match site {
            ExprSite::Node(id) => {
                let info = self.expressions.get(id)?;
                let blockers = self.collect_blockers(&info.ref_symbols);
                Some(ExprDeps {
                    info,
                    blockers,
                    needs_memo: (info.has_call || info.has_await) && !info.ref_symbols.is_empty(),
                })
            }
            ExprSite::Attr(id) => {
                let info = self.attr_expressions.get(id)?;
                let blockers = self.collect_blockers(&info.ref_symbols);
                Some(ExprDeps {
                    info,
                    blockers,
                    needs_memo: self.element_flags.is_dynamic_attr(id)
                        && (info.has_call || info.has_await),
                })
            }
        }
    }
    pub fn component_attr_needs_memo(&self, attr_id: NodeId) -> bool {
        self.attr_expressions.get(attr_id).is_some_and(|e| {
            e.has_call || (!e.kind.is_simple() && self.element_flags.is_dynamic_attr(attr_id))
        })
    }
    pub fn needs_expr_memoization(&self, id: NodeId) -> bool {
        self.expr_deps(ExprSite::Node(id))
            .is_some_and(|deps| deps.needs_memo)
    }
    pub fn expr_has_blockers(&self, id: NodeId) -> bool {
        self.expr_deps(ExprSite::Node(id))
            .is_some_and(|deps| deps.has_blockers())
    }
    pub fn expression_blockers(&self, id: NodeId) -> SmallVec<[u32; 2]> {
        self.expr_deps(ExprSite::Node(id))
            .map(|deps| deps.blockers)
            .unwrap_or_default()
    }
    pub fn attr_expression_blockers(&self, id: NodeId) -> SmallVec<[u32; 2]> {
        self.expr_deps(ExprSite::Attr(id))
            .map(|deps| deps.blockers)
            .unwrap_or_default()
    }
    fn collect_blockers(&self, ref_symbols: &[SymbolId]) -> SmallVec<[u32; 2]> {
        let mut result = SmallVec::new();
        if !self.blocker_data.has_async {
            return result;
        }
        for sym in ref_symbols {
            if let Some(&idx) = self.blocker_data.symbol_blockers.get(sym) {
                if !result.contains(&idx) {
                    result.push(idx);
                }
            }
        }
        result.sort_unstable();
        result
    }
    pub fn known_value(&self, name: &str) -> Option<&str> {
        let root = self.scoping.root_scope_id();
        let sym_id = self.scoping.find_binding(root, name)?;
        self.scoping.known_value_by_sym(sym_id)
    }
    fn effective_fragment_scope(
        &self,
        key: FragmentKey,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.scoping.fragment_scope(&key).unwrap_or(parent_scope)
    }
    pub fn if_consequent_scope(
        &self,
        block_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::IfConsequent(block_id), parent_scope)
    }
    pub fn if_alternate_scope(
        &self,
        block_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::IfAlternate(block_id), parent_scope)
    }
    pub fn each_body_scope(
        &self,
        block_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::EachBody(block_id), parent_scope)
    }
    pub fn snippet_body_scope(
        &self,
        block_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::SnippetBody(block_id), parent_scope)
    }
    pub fn key_block_body_scope(
        &self,
        block_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::KeyBlockBody(block_id), parent_scope)
    }
    pub fn svelte_head_body_scope(
        &self,
        head_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::SvelteHeadBody(head_id), parent_scope)
    }
    pub fn svelte_element_body_scope(
        &self,
        el_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::SvelteElementBody(el_id), parent_scope)
    }
    pub fn svelte_boundary_body_scope(
        &self,
        boundary_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::SvelteBoundaryBody(boundary_id), parent_scope)
    }
    pub fn await_pending_scope(
        &self,
        block_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::AwaitPending(block_id), parent_scope)
    }
    pub fn await_then_scope(
        &self,
        block_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::AwaitThen(block_id), parent_scope)
    }
    pub fn await_catch_scope(
        &self,
        block_id: NodeId,
        parent_scope: oxc_semantic::ScopeId,
    ) -> oxc_semantic::ScopeId {
        self.effective_fragment_scope(FragmentKey::AwaitCatch(block_id), parent_scope)
    }
    pub fn fragment_references_any_symbol(
        &self,
        key: &FragmentKey,
        syms: &FxHashSet<SymbolId>,
    ) -> bool {
        if syms.is_empty() {
            return false;
        }
        let Some(fragment) = self.fragments.lowered(key) else {
            return false;
        };
        for item in &fragment.items {
            match item {
                FragmentItem::TextConcat { parts, .. } => {
                    for part in parts {
                        if let LoweredTextPart::Expr(id) = part {
                            if self.expressions.get(*id).is_some_and(|info| {
                                info.ref_symbols.iter().any(|s| syms.contains(s))
                            }) {
                                return true;
                            }
                        }
                    }
                }
                FragmentItem::Element(el_id) => {
                    if self.fragment_references_any_symbol(&FragmentKey::Element(*el_id), syms) {
                        return true;
                    }
                }
                FragmentItem::IfBlock(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self
                            .fragment_references_any_symbol(&FragmentKey::IfConsequent(*id), syms)
                        || self.fragment_references_any_symbol(&FragmentKey::IfAlternate(*id), syms)
                    {
                        return true;
                    }
                }
                FragmentItem::EachBlock(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self.fragment_references_any_symbol(&FragmentKey::EachBody(*id), syms)
                        || self
                            .fragment_references_any_symbol(&FragmentKey::EachFallback(*id), syms)
                    {
                        return true;
                    }
                }
                FragmentItem::RenderTag(id) | FragmentItem::HtmlTag(id) => {
                    if self.node_expr_references_syms(*id, syms) {
                        return true;
                    }
                }
                FragmentItem::KeyBlock(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self
                            .fragment_references_any_symbol(&FragmentKey::KeyBlockBody(*id), syms)
                    {
                        return true;
                    }
                }
                FragmentItem::SvelteElement(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self.fragment_references_any_symbol(
                            &FragmentKey::SvelteElementBody(*id),
                            syms,
                        )
                    {
                        return true;
                    }
                }
                FragmentItem::SvelteBoundary(id) => {
                    if self
                        .fragment_references_any_symbol(&FragmentKey::SvelteBoundaryBody(*id), syms)
                    {
                        return true;
                    }
                }
                FragmentItem::ComponentNode(id) => {
                    if self.fragment_references_any_symbol(&FragmentKey::ComponentNode(*id), syms) {
                        return true;
                    }
                }
                FragmentItem::AwaitBlock(id) => {
                    if self.node_expr_references_syms(*id, syms) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn node_expr_references_syms(&self, id: NodeId, syms: &FxHashSet<SymbolId>) -> bool {
        self.expressions
            .get(id)
            .is_some_and(|info| info.ref_symbols.iter().any(|s| syms.contains(s)))
    }
}
