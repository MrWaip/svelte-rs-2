use super::*;

compiler_case!(
    state_member_derived,
    "legacy_select_indirect/state_member_derived",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(prop_member_each, "legacy_select_indirect/prop_member_each");
compiler_case!(
    function_collection_dep,
    "legacy_select_indirect/function_collection_dep"
);
