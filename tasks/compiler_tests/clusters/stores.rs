use super::*;

compiler_case!(
    assignment_alias,
    "stores/assignment/alias",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_array_hole,
    "stores/assignment/array_hole",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_array_of_objects,
    "stores/assignment/array_of_objects",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_array_rest,
    "stores/assignment/array_rest",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_array_rest_nested,
    "stores/assignment/array_rest_nested",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_computed_key,
    "stores/assignment/computed_key",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_default_intermediate_array,
    "stores/assignment/default_intermediate_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_default_intermediate_object,
    "stores/assignment/default_intermediate_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_default_leaf_array,
    "stores/assignment/default_leaf_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_default_leaf_object,
    "stores/assignment/default_leaf_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_flat_object,
    "stores/assignment/flat_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_member_target,
    "stores/assignment/member_target",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_nested_array,
    "stores/assignment/nested_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_nested_object,
    "stores/assignment/nested_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_object_in_array_in_object,
    "stores/assignment/object_in_array_in_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_object_of_arrays,
    "stores/assignment/object_of_arrays",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_object_rest,
    "stores/assignment/object_rest",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_single_array,
    "stores/assignment/single_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    assignment_string_key,
    "stores/assignment/string_key",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    runes_sub_state_held_unsub,
    "stores/runes_sub/state_held_unsub",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_prop_update,
    "stores/runes_sub/prop_update",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_runelike_prop_name,
    "stores/runes_sub/runelike_prop_name",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_state_reassign_unsub,
    "stores/runes_sub/state_reassign_unsub",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_plain_import_guard,
    "stores/runes_sub/plain_import_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_rune_state_no_store_guard,
    "stores/runes_sub/rune_state_no_store_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_derived_store_import_guard,
    "stores/runes_sub/derived_store_import_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_rune_shadowed_by_module_binding,
    "stores/runes_sub/rune_shadowed_by_module_binding",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_rune_shadowed_by_local_binding,
    "stores/runes_sub/rune_shadowed_by_local_binding",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_no_binding_guard,
    "stores/runes_sub/no_binding_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_self_init_guard,
    "stores/runes_sub/self_init_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_sole_rune_shadowed_legacy_guard,
    "stores/runes_sub/sole_rune_shadowed_legacy_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_sole_rune_shadowed_by_local_import,
    "stores/runes_sub/sole_rune_shadowed_by_local_import",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_rune_shadowed_by_local_import_live_rune,
    "stores/runes_sub/rune_shadowed_by_local_import_live_rune",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_all_runes_shadowed_by_local_import,
    "stores/runes_sub/all_runes_shadowed_by_local_import",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_sub_unrelated_local_import_guard,
    "stores/runes_sub/unrelated_local_import_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_module_case!(
    runes_sub_module_rune_shadowed_by_import,
    "stores/runes_sub/module_rune_shadowed_by_import"
);
compiler_module_case!(
    runes_sub_module_rune_no_shadow_guard,
    "stores/runes_sub/module_rune_no_shadow_guard"
);
