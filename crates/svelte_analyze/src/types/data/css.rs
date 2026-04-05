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
}

impl CssAnalysis {
    pub fn empty(node_count: u32) -> Self {
        Self {
            hash: String::new(),
            scoped_elements: NodeBitSet::new(node_count),
            inject_styles: false,
        }
    }
}
