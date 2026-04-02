use super::*;

pub struct TemplateSemanticsData {
    pub(crate) node_expr_handles: NodeTable<ExprHandle>,
    pub(crate) attr_expr_handles: NodeTable<ExprHandle>,
    pub(crate) const_tag_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) snippet_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) each_context_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) each_index_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) await_value_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) await_error_stmt_handles: NodeTable<StmtHandle>,
    pub(crate) node_ref_symbols: NodeTable<SmallVec<[SymbolId; 2]>>,
    pub(crate) stmt_ref_symbols: NodeTable<SmallVec<[SymbolId; 2]>>,
}

impl TemplateSemanticsData {
    pub fn new(node_count: u32) -> Self {
        Self {
            node_expr_handles: NodeTable::new(node_count),
            attr_expr_handles: NodeTable::new(node_count),
            const_tag_stmt_handles: NodeTable::new(node_count),
            snippet_stmt_handles: NodeTable::new(node_count),
            each_context_stmt_handles: NodeTable::new(node_count),
            each_index_stmt_handles: NodeTable::new(node_count),
            await_value_stmt_handles: NodeTable::new(node_count),
            await_error_stmt_handles: NodeTable::new(node_count),
            node_ref_symbols: NodeTable::new(node_count),
            stmt_ref_symbols: NodeTable::new(node_count),
        }
    }

    pub fn node_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.node_ref_symbols.get(id).map_or(&[], |v| v.as_slice())
    }

    pub fn stmt_ref_symbols(&self, id: NodeId) -> &[SymbolId] {
        self.stmt_ref_symbols.get(id).map_or(&[], |v| v.as_slice())
    }
}

pub struct SnippetData {
    pub(crate) hoistable: NodeBitSet,
    pub(crate) component_snippets: NodeTable<Vec<NodeId>>,
}

impl SnippetData {
    pub fn new(node_count: u32) -> Self {
        Self {
            hoistable: NodeBitSet::new(node_count),
            component_snippets: NodeTable::new(node_count),
        }
    }

    pub fn is_hoistable(&self, id: NodeId) -> bool {
        self.hoistable.contains(&id)
    }
    pub fn component_snippets(&self, id: NodeId) -> &[NodeId] {
        self.component_snippets
            .get(id)
            .map_or(&[], |v| v.as_slice())
    }
}

pub struct ConstTagData {
    pub(crate) names: NodeTable<Vec<String>>,
    pub(crate) syms: NodeTable<Vec<SymbolId>>,
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl ConstTagData {
    pub fn new(node_count: u32) -> Self {
        Self {
            names: NodeTable::new(node_count),
            syms: NodeTable::new(node_count),
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn names(&self, id: NodeId) -> Option<&Vec<String>> {
        self.names.get(id)
    }
    pub fn syms(&self, id: NodeId) -> Option<&Vec<SymbolId>> {
        self.syms.get(id)
    }
    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

pub struct DebugTagData {
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl DebugTagData {
    pub fn new() -> Self {
        Self {
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

pub struct TitleElementData {
    pub(crate) by_fragment: FxHashMap<FragmentKey, Vec<NodeId>>,
}

impl TitleElementData {
    pub fn new() -> Self {
        Self {
            by_fragment: FxHashMap::default(),
        }
    }

    pub fn by_fragment(&self, key: &FragmentKey) -> Option<&Vec<NodeId>> {
        self.by_fragment.get(key)
    }
}

pub struct EachBlockData {
    pub(crate) index_syms: NodeTable<SymbolId>,
    pub(crate) index_sym_to_block: FxHashMap<SymbolId, NodeId>,
    pub(crate) key_node_ids: NodeTable<NodeId>,
    pub(crate) key_uses_index: NodeBitSet,
    pub(crate) is_destructured: NodeBitSet,
    pub(crate) body_uses_index: NodeBitSet,
    pub(crate) key_is_item: NodeBitSet,
    pub(crate) has_animate: NodeBitSet,
    pub(crate) context_names: NodeTable<String>,
}

impl EachBlockData {
    pub fn new(node_count: u32) -> Self {
        Self {
            index_syms: NodeTable::new(node_count),
            index_sym_to_block: FxHashMap::default(),
            key_node_ids: NodeTable::new(node_count),
            key_uses_index: NodeBitSet::new(node_count),
            is_destructured: NodeBitSet::new(node_count),
            body_uses_index: NodeBitSet::new(node_count),
            key_is_item: NodeBitSet::new(node_count),
            has_animate: NodeBitSet::new(node_count),
            context_names: NodeTable::new(node_count),
        }
    }

    pub(crate) fn build_index_lookup(&mut self) {
        self.index_sym_to_block = self
            .index_syms
            .iter()
            .map(|(block_id, &sym)| (sym, block_id))
            .collect();
    }

    pub fn index_sym(&self, id: NodeId) -> Option<SymbolId> {
        self.index_syms.get(id).copied()
    }
    pub fn key_uses_index(&self, id: NodeId) -> bool {
        self.key_uses_index.contains(&id)
    }
    pub fn is_destructured(&self, id: NodeId) -> bool {
        self.is_destructured.contains(&id)
    }
    pub fn body_uses_index(&self, id: NodeId) -> bool {
        self.body_uses_index.contains(&id)
    }
    pub fn key_is_item(&self, id: NodeId) -> bool {
        self.key_is_item.contains(&id)
    }
    pub fn has_animate(&self, id: NodeId) -> bool {
        self.has_animate.contains(&id)
    }
    pub fn context_name(&self, id: NodeId) -> &str {
        self.context_names.get(id).map_or("$$item", |s| s.as_str())
    }
}

pub struct AwaitBindingData {
    pub(crate) values: NodeTable<AwaitBindingInfo>,
    pub(crate) errors: NodeTable<AwaitBindingInfo>,
}

impl AwaitBindingData {
    pub fn new(node_count: u32) -> Self {
        Self {
            values: NodeTable::new(node_count),
            errors: NodeTable::new(node_count),
        }
    }

    pub fn value(&self, id: NodeId) -> Option<&AwaitBindingInfo> {
        self.values.get(id)
    }
    pub fn error(&self, id: NodeId) -> Option<&AwaitBindingInfo> {
        self.errors.get(id)
    }
}

pub struct BindSemanticsData {
    pub(crate) mutable_rune_targets: NodeBitSet,
    pub(crate) prop_source_nodes: NodeBitSet,
    pub(crate) bind_each_context: NodeTable<Vec<String>>,
    pub(crate) has_bind_group: NodeBitSet,
    pub(crate) bind_group_value_attr: NodeTable<NodeId>,
    pub(crate) parent_each_blocks: NodeTable<Vec<NodeId>>,
    pub(crate) contains_group_binding: NodeBitSet,
    pub(crate) bind_blockers: NodeTable<SmallVec<[u32; 2]>>,
}

impl BindSemanticsData {
    pub fn new(node_count: u32) -> Self {
        Self {
            mutable_rune_targets: NodeBitSet::new(node_count),
            prop_source_nodes: NodeBitSet::new(node_count),
            bind_each_context: NodeTable::new(node_count),
            has_bind_group: NodeBitSet::new(node_count),
            bind_group_value_attr: NodeTable::new(node_count),
            parent_each_blocks: NodeTable::new(node_count),
            contains_group_binding: NodeBitSet::new(node_count),
            bind_blockers: NodeTable::new(node_count),
        }
    }

    pub fn is_mutable_rune_target(&self, id: NodeId) -> bool {
        self.mutable_rune_targets.contains(&id)
    }
    pub fn is_prop_source(&self, id: NodeId) -> bool {
        self.prop_source_nodes.contains(&id)
    }
    pub fn each_context(&self, id: NodeId) -> Option<&Vec<String>> {
        self.bind_each_context.get(id)
    }
    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.has_bind_group.contains(&id)
    }
    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.bind_group_value_attr.get(id).copied()
    }
    pub fn parent_each_blocks(&self, id: NodeId) -> Option<&Vec<NodeId>> {
        self.parent_each_blocks.get(id)
    }
    pub fn contains_group_binding(&self, id: NodeId) -> bool {
        self.contains_group_binding.contains(&id)
    }
    pub fn bind_blockers(&self, id: NodeId) -> &[u32] {
        self.bind_blockers.get(id).map_or(&[], |v| v.as_slice())
    }
}
