use super::*;

compiler_case!(
    derived_undefined_store_read,
    "stores/derived_undefined_store_read"
);
compiler_case!(assignment_alias, "stores/assignment/alias");
compiler_case!(assignment_array_hole, "stores/assignment/array_hole");
compiler_case!(
    assignment_array_of_objects,
    "stores/assignment/array_of_objects"
);
compiler_case!(assignment_array_rest, "stores/assignment/array_rest");
compiler_case!(
    assignment_array_rest_nested,
    "stores/assignment/array_rest_nested"
);
compiler_case!(assignment_computed_key, "stores/assignment/computed_key");
compiler_case!(
    assignment_default_intermediate_array,
    "stores/assignment/default_intermediate_array"
);
compiler_case!(
    assignment_default_intermediate_object,
    "stores/assignment/default_intermediate_object"
);
compiler_case!(
    assignment_default_leaf_array,
    "stores/assignment/default_leaf_array"
);
compiler_case!(
    assignment_default_leaf_object,
    "stores/assignment/default_leaf_object"
);
compiler_case!(assignment_flat_object, "stores/assignment/flat_object");
compiler_case!(assignment_member_target, "stores/assignment/member_target");
compiler_case!(assignment_nested_array, "stores/assignment/nested_array");
compiler_case!(assignment_nested_object, "stores/assignment/nested_object");
compiler_case!(
    assignment_object_in_array_in_object,
    "stores/assignment/object_in_array_in_object"
);
compiler_case!(
    assignment_object_of_arrays,
    "stores/assignment/object_of_arrays"
);
compiler_case!(assignment_object_rest, "stores/assignment/object_rest");
compiler_case!(assignment_single_array, "stores/assignment/single_array");
compiler_case!(assignment_string_key, "stores/assignment/string_key");

compiler_case!(store_target_array, "stores/assignment/store_target_array");
compiler_case!(
    store_target_object_seq,
    "stores/assignment/store_target_object_seq"
);
compiler_case!(
    store_target_object_iife,
    "stores/assignment/store_target_object_iife"
);
compiler_case!(
    store_target_mixed_leaves,
    "stores/assignment/store_target_mixed_leaves"
);
compiler_case!(
    store_target_value_position,
    "stores/assignment/store_target_value_position"
);
compiler_case!(store_target_nested, "stores/assignment/store_target_nested");
compiler_case!(
    store_target_default_leaf,
    "stores/assignment/store_target_default_leaf"
);
compiler_case!(
    store_target_array_rest,
    "stores/assignment/store_target_array_rest"
);
compiler_case!(
    store_target_object_rest,
    "stores/assignment/store_target_object_rest"
);
compiler_case!(
    store_target_computed_key,
    "stores/assignment/store_target_computed_key"
);
compiler_case!(
    store_target_none_guard,
    "stores/assignment/store_target_none_guard"
);
compiler_case!(
    store_single_assign_guard,
    "stores/assignment/store_single_assign_guard"
);

compiler_case!(
    runes_sub_state_held_unsub,
    "stores/runes_sub/state_held_unsub"
);
compiler_case!(runes_sub_prop_update, "stores/runes_sub/prop_update");
compiler_case!(
    runes_sub_runelike_prop_name,
    "stores/runes_sub/runelike_prop_name"
);
compiler_case!(
    runes_sub_state_reassign_unsub,
    "stores/runes_sub/state_reassign_unsub"
);
compiler_case!(
    runes_sub_plain_import_guard,
    "stores/runes_sub/plain_import_guard"
);
compiler_case!(
    runes_sub_rune_state_no_store_guard,
    "stores/runes_sub/rune_state_no_store_guard"
);
compiler_case!(
    runes_sub_derived_store_import_guard,
    "stores/runes_sub/derived_store_import_guard"
);
compiler_case!(
    runes_sub_rune_shadowed_by_module_binding,
    "stores/runes_sub/rune_shadowed_by_module_binding"
);
compiler_case!(
    runes_sub_rune_shadowed_by_local_binding,
    "stores/runes_sub/rune_shadowed_by_local_binding"
);
compiler_case!(
    runes_sub_no_binding_guard,
    "stores/runes_sub/no_binding_guard"
);
compiler_case!(
    runes_sub_self_init_guard,
    "stores/runes_sub/self_init_guard"
);
compiler_case!(
    runes_sub_sole_rune_shadowed_legacy_guard,
    "stores/runes_sub/sole_rune_shadowed_legacy_guard"
);
compiler_case!(
    runes_sub_sole_rune_shadowed_by_local_import,
    "stores/runes_sub/sole_rune_shadowed_by_local_import"
);
compiler_case!(
    runes_sub_rune_shadowed_by_local_import_live_rune,
    "stores/runes_sub/rune_shadowed_by_local_import_live_rune"
);
compiler_case!(
    runes_sub_all_runes_shadowed_by_local_import,
    "stores/runes_sub/all_runes_shadowed_by_local_import"
);
compiler_case!(
    runes_sub_unrelated_local_import_guard,
    "stores/runes_sub/unrelated_local_import_guard"
);
compiler_module_case!(
    runes_sub_module_rune_shadowed_by_import,
    "stores/runes_sub/module_rune_shadowed_by_import"
);
compiler_module_case!(
    runes_sub_module_rune_no_shadow_guard,
    "stores/runes_sub/module_rune_no_shadow_guard"
);

compiler_case!(
    runes_sub_sole_held_store_read,
    "stores/runes_sub/sole_held_store_read"
);
compiler_case!(
    runes_sub_held_store_assign,
    "stores/runes_sub/held_store_assign"
);
compiler_case!(
    runes_sub_store_mutate_callback_param,
    "stores/runes_sub/store_mutate_callback_param",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    runes_sub_held_store_member_read_guard,
    "stores/runes_sub/held_store_member_read_guard"
);
compiler_case!(
    runes_sub_held_store_bare_read_guard,
    "stores/runes_sub/held_store_bare_read_guard"
);
compiler_case!(
    runes_sub_held_store_mutate_guard,
    "stores/runes_sub/held_store_mutate_guard"
);
compiler_case!(
    runes_sub_sole_real_store_read_guard,
    "stores/runes_sub/sole_real_store_read_guard"
);

compiler_case!(
    bind_store_textarea_contenteditable,
    "stores/bind_store_textarea_contenteditable"
);

compiler_case!(
    server_member_bind_mutate_bare_base,
    "stores/server_member_bind_mutate_bare_base",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    server_bind_pair_name_order,
    "stores/server_bind_pair_name_order",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    store_base_source_prop_default,
    "stores/store_base_source_prop_default"
);
