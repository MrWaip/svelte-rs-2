use oxc_semantic::SymbolId;
use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

pub enum RestExcludeKey {
    Str(String),
    Num(f64),
}

pub struct RestExcludes {
    pub name: String,
    pub keys: Vec<RestExcludeKey>,
}

pub struct TransformData {
    pub rest_excludes: Vec<RestExcludes>,

    pub const_tag_tmp_names: FxHashMap<NodeId, String>,

    pub each_index_internal_names: FxHashMap<NodeId, String>,

    pub each_index_block_by_item: FxHashMap<SymbolId, NodeId>,

    pub each_collection_internal_names_legacy: FxHashMap<NodeId, String>,

    pub each_collection_block_by_item_legacy: FxHashMap<SymbolId, NodeId>,

    pub destructure_default_simple: FxHashMap<SymbolId, Vec<bool>>,
}

impl Default for TransformData {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformData {
    pub fn new() -> Self {
        Self {
            rest_excludes: Vec::new(),
            const_tag_tmp_names: FxHashMap::default(),
            each_index_internal_names: FxHashMap::default(),
            each_index_block_by_item: FxHashMap::default(),
            each_collection_internal_names_legacy: FxHashMap::default(),
            each_collection_block_by_item_legacy: FxHashMap::default(),
            destructure_default_simple: FxHashMap::default(),
        }
    }
}
