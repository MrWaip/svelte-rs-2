use super::*;

compiler_case!(
    component_prop,
    "async_construct_wrap/component_prop",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    component_prop_standalone,
    "async_construct_wrap/component_prop_standalone",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    component_prop_multiple,
    "async_construct_wrap/component_prop_multiple",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    component_prop_sync_and_async,
    "async_construct_wrap/component_prop_sync_and_async",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    component_prop_blocker_only,
    "async_construct_wrap/component_prop_blocker_only",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    component_prop_blocker_and_await,
    "async_construct_wrap/component_prop_blocker_and_await",
    ignore =
        "blocker identity is the promise index, not the declaration, so one index cannot repeat"
);
compiler_case!(
    component_spread,
    "async_construct_wrap/component_spread",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    component_bind,
    "async_construct_wrap/component_bind",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    component_css_props,
    "async_construct_wrap/component_css_props",
    ignore = "custom css props memoize outside the component's async value numbering"
);
compiler_case!(
    component_dynamic,
    "async_construct_wrap/component_dynamic",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    component_self,
    "async_construct_wrap/component_self",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    slot_prop,
    "async_construct_wrap/slot_prop",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    slot_prop_fallback,
    "async_construct_wrap/slot_prop_fallback",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    element_spread,
    "async_construct_wrap/element_spread",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    element_spread_sync_and_async,
    "async_construct_wrap/element_spread_sync_and_async",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    element_spread_blocker_only,
    "async_construct_wrap/element_spread_blocker_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_spread_class_directive,
    "async_construct_wrap/element_spread_class_directive",
    [prod, dev, ssr_todo, ssr_dev_todo]
);

compiler_case!(
    component_prop_sync_guard,
    "async_construct_wrap/component_prop_sync_guard"
);
compiler_case!(
    slot_prop_sync_guard,
    "async_construct_wrap/slot_prop_sync_guard"
);
compiler_case!(
    element_spread_sync_guard,
    "async_construct_wrap/element_spread_sync_guard"
);
