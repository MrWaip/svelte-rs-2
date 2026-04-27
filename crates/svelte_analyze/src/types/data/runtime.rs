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
    /// LEGACY(svelte4): which `$.init(...)` call (if any) to emit before the
    /// template body. Codegen matches and emits one statement.
    pub legacy_init: LegacyInit,
}

/// LEGACY(svelte4): which `$.init(...)` form the legacy runtime needs.
/// Deprecated in Svelte 5, remove in Svelte 6.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LegacyInit {
    /// No `$.init(...)` call — runes mode, accessors-only legacy, or no legacy
    /// runtime gate triggered.
    #[default]
    None,
    /// `$.init()` — legacy member-mutated bindable props or `$$props` reads.
    Plain,
    /// `$.init(true)` — `<svelte:options immutable={true} />` legacy mode.
    Immutable,
}
