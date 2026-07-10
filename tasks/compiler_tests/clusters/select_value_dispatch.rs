use super::*;

compiler_case!(
    legacy_prop_select,
    "select_value_dispatch/legacy_prop_select",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_prop_select_multiple,
    "select_value_dispatch/legacy_prop_select_multiple",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_prop_input,
    "select_value_dispatch/legacy_prop_input",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_local_select,
    "select_value_dispatch/legacy_local_select",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_bindable_select,
    "select_value_dispatch/runes_bindable_select",
    [prod, dev, ssr, ssr_dev]
);
