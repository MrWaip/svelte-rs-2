use super::*;

compiler_case!(
    dynamic_this,
    "svelte_element/named_slot/dynamic_this",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    static_this,
    "svelte_element/named_slot/static_this",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    default_slot_guard,
    "svelte_element/named_slot/default_slot_guard",
    [prod, dev, ssr, ssr_dev]
);
