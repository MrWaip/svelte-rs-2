use rustc_hash::FxHashMap;
use svelte_ast::NodeId;

/// Data produced by the transform phase.
/// Separate from AnalysisData so the transform pass does not mutate analysis side tables.
pub struct TransformData {
    /// Generated tmp variable names for destructured const tags.
    /// Keys are NodeIds of ConstTag nodes with destructured patterns (len > 1 binding).
    pub const_tag_tmp_names: FxHashMap<NodeId, String>,
}

impl TransformData {
    pub fn new() -> Self {
        Self {
            const_tag_tmp_names: FxHashMap::default(),
        }
    }
}
