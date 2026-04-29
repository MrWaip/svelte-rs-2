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

    pub legacy_init: LegacyInit,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LegacyInit {
    #[default]
    None,

    Plain,

    Immutable,
}
