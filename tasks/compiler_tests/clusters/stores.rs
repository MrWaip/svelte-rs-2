use super::*;

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
