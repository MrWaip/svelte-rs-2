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
    /// Named slots for component children: maps component NodeId → vec of (slot_element_id, fragment_key).
    pub(crate) component_named_slots: NodeTable<Vec<(NodeId, FragmentKey)>>,
}

impl SnippetData {
    pub fn new(node_count: u32) -> Self {
        Self {
            hoistable: NodeBitSet::new(node_count),
            component_snippets: NodeTable::new(node_count),
            component_named_slots: NodeTable::new(node_count),
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
    pub fn component_named_slots(&self, id: NodeId) -> &[(NodeId, FragmentKey)] {
        self.component_named_slots
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
    pub(crate) has_bind_group: NodeBitSet,
    pub(crate) bind_group_value_attr: NodeTable<NodeId>,
    pub(crate) bind_blockers: NodeTable<SmallVec<[u32; 2]>>,
    pub(crate) bind_this_each_context: NodeTable<SmallVec<[SymbolId; 4]>>,
}

impl BindSemanticsData {
    pub fn new(node_count: u32) -> Self {
        Self {
            mutable_rune_targets: NodeBitSet::new(node_count),
            prop_source_nodes: NodeBitSet::new(node_count),
            has_bind_group: NodeBitSet::new(node_count),
            bind_group_value_attr: NodeTable::new(node_count),
            bind_blockers: NodeTable::new(node_count),
            bind_this_each_context: NodeTable::new(node_count),
        }
    }

    pub fn is_mutable_rune_target(&self, id: NodeId) -> bool {
        self.mutable_rune_targets.contains(&id)
    }
    pub fn is_prop_source(&self, id: NodeId) -> bool {
        self.prop_source_nodes.contains(&id)
    }
    pub fn has_bind_group(&self, id: NodeId) -> bool {
        self.has_bind_group.contains(&id)
    }
    pub fn bind_group_value_attr(&self, id: NodeId) -> Option<NodeId> {
        self.bind_group_value_attr.get(id).copied()
    }
    pub fn bind_blockers(&self, id: NodeId) -> &[u32] {
        self.bind_blockers.get(id).map_or(&[], |v| v.as_slice())
    }
    pub fn bind_this_each_context(&self, id: NodeId) -> Option<&[SymbolId]> {
        self.bind_this_each_context
            .get(id)
            .map(|syms| syms.as_slice())
    }
}
