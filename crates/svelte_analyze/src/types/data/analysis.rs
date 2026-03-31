use super::*;

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
    pub element_flags: ElementFlags,
    pub fragments: FragmentData,
    pub snippets: SnippetData,
    pub const_tags: ConstTagData,
    pub debug_tags: DebugTagData,
    pub title_elements: TitleElementData,
    pub each_blocks: EachBlockData,
    pub(crate) render_tag_callee_sym: NodeTable<SymbolId>,
    pub(crate) render_tag_is_chain: NodeBitSet,
    pub render_tag_plans: NodeTable<RenderTagPlan>,
    pub node_expr_handles: NodeTable<ExprHandle>,
    pub attr_expr_handles: NodeTable<ExprHandle>,
    pub const_tag_stmt_handles: NodeTable<StmtHandle>,
    pub snippet_stmt_handles: NodeTable<StmtHandle>,
    pub each_context_stmt_handles: NodeTable<StmtHandle>,
    pub each_index_stmt_handles: NodeTable<StmtHandle>,
    pub await_value_stmt_handles: NodeTable<StmtHandle>,
    pub await_error_stmt_handles: NodeTable<StmtHandle>,
    pub await_bindings: AwaitBindingData,
    pub bind_semantics: BindSemanticsData,
    pub import_syms: FxHashSet<SymbolId>,
    pub custom_element: bool,
    pub runtime_plan: RuntimePlan,
    pub ce_config: Option<svelte_parser::ParsedCeConfig>,
    pub(crate) proxy_state_inits: FxHashMap<compact_str::CompactString, bool>,
    pub(crate) has_store_member_mutations: bool,
    pub(crate) blocker_data: BlockerData,
    pub(crate) script_rune_call_kinds: FxHashMap<u32, crate::types::script::RuneKind>,
    pub(crate) pickled_await_offsets: FxHashSet<u32>,
    pub ignore_data: IgnoreData,
}

impl AnalysisData {
    pub(crate) fn new_empty(node_count: u32) -> Self {
        Self {
            expressions: NodeTable::new(node_count),
            attr_expressions: NodeTable::new(node_count),
            script: None,
            scoping: ComponentScoping::new(None),
            dynamic_nodes: NodeBitSet::new(node_count),
            alt_is_elseif: NodeBitSet::new(node_count),
            props: None,
            props_id: None,
            exports: Vec::new(),
            needs_context: false,
            has_class_state_fields: false,
            element_flags: ElementFlags::new(node_count),
            fragments: FragmentData::with_capacity(node_count as usize / 3),
            snippets: SnippetData::new(node_count),
            const_tags: ConstTagData::new(node_count),
            debug_tags: DebugTagData::new(),
            title_elements: TitleElementData::new(),
            each_blocks: EachBlockData::new(node_count),
            render_tag_callee_sym: NodeTable::new(node_count),
            render_tag_is_chain: NodeBitSet::new(node_count),
            render_tag_plans: NodeTable::new(node_count),
            node_expr_handles: NodeTable::new(node_count),
            attr_expr_handles: NodeTable::new(node_count),
            const_tag_stmt_handles: NodeTable::new(node_count),
            snippet_stmt_handles: NodeTable::new(node_count),
            each_context_stmt_handles: NodeTable::new(node_count),
            each_index_stmt_handles: NodeTable::new(node_count),
            await_value_stmt_handles: NodeTable::new(node_count),
            await_error_stmt_handles: NodeTable::new(node_count),
            await_bindings: AwaitBindingData::new(node_count),
            bind_semantics: BindSemanticsData::new(node_count),
            import_syms: FxHashSet::default(),
            custom_element: false,
            runtime_plan: RuntimePlan::default(),
            ce_config: None,
            proxy_state_inits: FxHashMap::default(),
            has_store_member_mutations: false,
            blocker_data: BlockerData::default(),
            script_rune_call_kinds: FxHashMap::default(),
            pickled_await_offsets: FxHashSet::default(),
            ignore_data: IgnoreData::new(),
        }
    }
}

impl AnalysisData {
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
    pub fn attr_expression(&self, id: NodeId) -> Option<&ExpressionInfo> {
        self.attr_expressions.get(id)
    }
    pub fn node_expr_handle(&self, id: NodeId) -> ExprHandle {
        *self
            .node_expr_handles
            .get(id)
            .unwrap_or_else(|| panic!("no expr handle for node {:?}", id))
    }
    pub fn attr_expr_handle(&self, id: NodeId) -> ExprHandle {
        *self
            .attr_expr_handles
            .get(id)
            .unwrap_or_else(|| panic!("no expr handle for attr {:?}", id))
    }
    pub fn const_tag_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.const_tag_stmt_handles.get(id).copied()
    }
    pub fn snippet_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.snippet_stmt_handles.get(id).copied()
    }
    pub fn each_context_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.each_context_stmt_handles.get(id).copied()
    }
    pub fn each_index_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.each_index_stmt_handles.get(id).copied()
    }
    pub fn await_value_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.await_value_stmt_handles.get(id).copied()
    }
    pub fn await_error_stmt_handle(&self, id: NodeId) -> Option<StmtHandle> {
        self.await_error_stmt_handles.get(id).copied()
    }
    pub fn attr_is_import(&self, attr_id: NodeId) -> bool {
        self.attr_expressions
            .get(attr_id)
            .and_then(|info| info.ref_symbols.first())
            .is_some_and(|&sym| self.import_syms.contains(&sym))
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
                        || self.fragment_references_any_symbol(&FragmentKey::IfConsequent(*id), syms)
                        || self.fragment_references_any_symbol(&FragmentKey::IfAlternate(*id), syms)
                    {
                        return true;
                    }
                }
                FragmentItem::EachBlock(id) => {
                    if self.node_expr_references_syms(*id, syms)
                        || self.fragment_references_any_symbol(&FragmentKey::EachBody(*id), syms)
                        || self.fragment_references_any_symbol(&FragmentKey::EachFallback(*id), syms)
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
                        || self.fragment_references_any_symbol(&FragmentKey::KeyBlockBody(*id), syms)
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
                    if self.fragment_references_any_symbol(
                        &FragmentKey::SvelteBoundaryBody(*id),
                        syms,
                    ) {
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
