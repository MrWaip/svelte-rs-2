use compact_str::CompactString;
use rustc_hash::FxHashSet;
use svelte_css::CssNodeId;

use super::NodeBitSet;

pub struct CssAnalysis {
    pub hash: String,

    pub scoped_elements: NodeBitSet,

    pub inject_styles: bool,

    pub keyframes: Vec<CompactString>,

    pub used_selectors: FxHashSet<CssNodeId>,
}

impl CssAnalysis {
    pub fn empty(node_count: u32) -> Self {
        Self {
            hash: String::new(),
            scoped_elements: NodeBitSet::new(node_count),
            inject_styles: false,
            keyframes: Vec::new(),
            used_selectors: FxHashSet::default(),
        }
    }
}
