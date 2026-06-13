use oxc_semantic::SymbolId;
use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

pub struct TransformData {
    pub const_tag_tmp_names: FxHashMap<NodeId, String>,

    pub each_synthetic_index_names_legacy: FxHashMap<SymbolId, String>,
}

impl Default for TransformData {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformData {
    pub fn new() -> Self {
        Self {
            const_tag_tmp_names: FxHashMap::default(),
            each_synthetic_index_names_legacy: FxHashMap::default(),
        }
    }
}
