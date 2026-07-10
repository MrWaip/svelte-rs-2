use super::*;

compiler_case!(
    legacy_store_readonly_guard,
    "each_item_writeback/legacy_store_readonly_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_store_bind,
    "each_item_writeback/legacy_store_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_store_assign,
    "each_item_writeback/legacy_store_assign",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_array_bind,
    "each_item_writeback/legacy_reactive_array_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_store_destructure_bind,
    "each_item_writeback/legacy_store_destructure_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_store_assign_noindex,
    "each_item_writeback/legacy_store_assign_noindex",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_store_bind_noindex,
    "each_item_writeback/legacy_store_bind_noindex",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_array_bind_noindex,
    "each_item_writeback/legacy_reactive_array_bind_noindex",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_store_destructure_bind_noindex,
    "each_item_writeback/legacy_store_destructure_bind_noindex",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_array_item_assign,
    "each_item_writeback/legacy_reactive_array_item_assign",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_array_item_mutate,
    "each_item_writeback/legacy_reactive_array_item_mutate",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_element_bind_member,
    "each_item_writeback/legacy_reactive_element_bind_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_destructure_element_bind,
    "each_item_writeback/legacy_reactive_destructure_element_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_destructure_default_bind,
    "each_item_writeback/legacy_destructure_default_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_keyed_index_bind,
    "each_item_writeback/legacy_keyed_index_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reassigned_collection_expr,
    "each_item_writeback/legacy_reassigned_collection_expr",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_filtered_collection_mutate,
    "each_item_writeback/legacy_reactive_filtered_collection_mutate",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_keyed_element_bind_member,
    "each_item_writeback/legacy_reactive_keyed_element_bind_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_collection_reassigned_bind,
    "each_item_writeback/legacy_reactive_collection_reassigned_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_await_rooted_collection_mutate,
    "each_item_writeback/legacy_reactive_await_rooted_collection_mutate",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_array_item_mutate_guard,
    "each_item_writeback/runes_array_item_mutate_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_derived_collection_bind,
    "each_item_writeback/legacy_reactive_derived_collection_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_derived_chain_collection_bind,
    "each_item_writeback/legacy_reactive_derived_chain_collection_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_derived_collection_no_writeback,
    "each_item_writeback/legacy_reactive_derived_collection_no_writeback",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_derived_collection_bind,
    "each_item_writeback/runes_derived_collection_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_function_in_collection_expr,
    "each_item_writeback/legacy_reactive_function_in_collection_expr",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_shadow_item_bind,
    "each_item_writeback/legacy_shadow_item_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_shadow_item_member_bind,
    "each_item_writeback/legacy_shadow_item_member_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_shadow_item_readonly_mutated,
    "each_item_writeback/legacy_shadow_item_readonly_mutated",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_noshadow_item_bind_guard,
    "each_item_writeback/legacy_noshadow_item_bind_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_shadow_keyed_item_member_bind,
    "each_item_writeback/legacy_shadow_keyed_item_member_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_shadow_destructure_member_bind,
    "each_item_writeback/legacy_shadow_destructure_member_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_nested_parent_collection_mutate,
    "each_item_writeback/legacy_reactive_nested_parent_collection_mutate",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_nested_destructure_parent_collection,
    "each_item_writeback/legacy_reactive_nested_destructure_parent_collection",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_reactive_nested_member_arg_value,
    "each_item_writeback/legacy_reactive_nested_member_arg_value",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_prop_item_member_read_assign,
    "each_item_writeback/legacy_prop_item_member_read_assign",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_local_item_member_read_assign,
    "each_item_writeback/legacy_local_item_member_read_assign",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_prop_item_member_read_only,
    "each_item_writeback/legacy_prop_item_member_read_only",
    [prod, dev, ssr, ssr_dev]
);
