use compact_str::CompactString;
use rustc_hash::FxHashSet;
use svelte_css::CssNodeId;

use super::NodeBitSet;

/// CSS-scoping metadata computed by the CSS analysis pass.
pub struct CssAnalysis {
    /// Deterministic scoping class, e.g. `"svelte-1a7i8ec"`.
    /// Empty string when the component has no `<style>` block.
    pub hash: String,
    /// Template element NodeIds that should receive the scoped class attribute.
    pub scoped_elements: NodeBitSet,
    /// Whether CSS should be injected at runtime via `$.append_styles()` instead of
    /// returned in `CompileResult.css`. Set when `css:"injected"` is active.
    pub inject_styles: bool,
    /// Locally-scoped `@keyframes` names (those without `-global-` prefix).
    /// Used by the CSS transform to prefix these names with the component hash
    /// and rewrite matching `animation`/`animation-name` values.
    pub keyframes: Vec<CompactString>,
    /// Set of `ComplexSelector` CssNodeIds that match at least one template element.
    /// Selectors not in this set are considered unused (emit `css_unused_selector` warning).
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
