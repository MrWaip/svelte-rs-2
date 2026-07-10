use super::*;

compiler_case!(
    legacy_each_unrelated_prop,
    "bind_group/legacy/each_unrelated_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_each_item_no_index,
    "bind_group/legacy/each_item_no_index",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_each_user_index,
    "bind_group/legacy/each_user_index",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_nested_multi_level,
    "bind_group/legacy/nested_multi_level",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_distinct_keypath_two_groups,
    "bind_group/legacy/distinct_keypath_two_groups",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_same_keypath_guard,
    "bind_group/legacy/same_keypath_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_distinct_base_guard,
    "bind_group/legacy/distinct_base_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_transitive_collection_two_each,
    "bind_group/runes/transitive_collection_two_each",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_transitive_collection_keyed_outer,
    "bind_group/runes/transitive_collection_keyed_outer",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_referenced_in_if,
    "bind_group/legacy/referenced_in_if",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_referenced_plain_guard,
    "bind_group/legacy/referenced_plain_guard",
    [prod, dev, ssr, ssr_dev]
);
