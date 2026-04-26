#[derive(Debug, Clone, Copy, Default)]
pub struct RuntimePlan {
    pub needs_push: bool,
    pub has_component_exports: bool,
    pub has_exports: bool,
    pub has_bindable: bool,
    pub has_stores: bool,
    pub has_ce_props: bool,
    pub needs_props_param: bool,
    pub needs_pop_with_return: bool,
    /// LEGACY(svelte4): emit `$.init()` (or `$.init(true)` for immutable) before the
    /// template body when the legacy runtime needs an init step — i.e. immutable mode,
    /// member-mutated bindable props, or `$$props` / `$$restProps` reads.
    /// Accessors-only paths skip init.
    pub has_legacy_runtime_init: bool,
}
