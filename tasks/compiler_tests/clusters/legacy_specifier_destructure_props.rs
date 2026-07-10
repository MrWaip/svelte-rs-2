use super::*;

compiler_case!(
    array_props,
    "legacy_specifier_destructure_props/array_props",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_const_destructure_export,
    "legacy_specifier_destructure_props/guard_const_destructure_export",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_inline_export_destructure,
    "legacy_specifier_destructure_props/guard_inline_export_destructure",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_simple_specifier,
    "legacy_specifier_destructure_props/guard_simple_specifier",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    nested,
    "legacy_specifier_destructure_props/nested",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    object_all_props,
    "legacy_specifier_destructure_props/object_all_props",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    object_prop_plain,
    "legacy_specifier_destructure_props/object_prop_plain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    object_prop_plain_nomut,
    "legacy_specifier_destructure_props/object_prop_plain_nomut",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    object_prop_state_leaf,
    "legacy_specifier_destructure_props/object_prop_state_leaf",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    object_prop_store,
    "legacy_specifier_destructure_props/object_prop_store",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    object_prop_store_nomut,
    "legacy_specifier_destructure_props/object_prop_store_nomut",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    object_rest,
    "legacy_specifier_destructure_props/object_rest",
    [prod, dev, ssr, ssr_dev]
);
