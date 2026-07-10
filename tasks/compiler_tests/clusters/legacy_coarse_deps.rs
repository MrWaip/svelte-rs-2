use super::*;

compiler_case!(
    coarse_deps_dep_order_props_first,
    "legacy/coarse_deps/dep_order_props_first",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_props_last,
    "legacy/coarse_deps/dep_order_props_last",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_props_middle,
    "legacy/coarse_deps/dep_order_props_middle",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_restprops_first,
    "legacy/coarse_deps/dep_order_restprops_first",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_props_restprops_mixed,
    "legacy/coarse_deps/dep_order_props_restprops_mixed",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    coarse_deps_reactive_api_export_fn,
    "legacy/coarse_deps/reactive_api_export_fn",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_reactive_api_export_const,
    "legacy/coarse_deps/reactive_api_export_const",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_reactive_api_export_class,
    "legacy/coarse_deps/reactive_api_export_class",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_reactive_local_fn,
    "legacy/coarse_deps/reactive_local_fn",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_reactive_local_const,
    "legacy/coarse_deps/reactive_local_const",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_reactive_prop,
    "legacy/coarse_deps/reactive_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_reactive_import,
    "legacy/coarse_deps/reactive_import",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_reactive_state,
    "legacy/coarse_deps/reactive_state",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    coarse_deps_state_store_subscribed,
    "legacy/coarse_deps/state_store_subscribed",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_each_index_keyed,
    "legacy/coarse_deps/each_index_keyed",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_state_plain,
    "legacy/coarse_deps/state_plain",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    coarse_deps_module_mutated_read_template,
    "legacy/coarse_deps/module_mutated_read_template",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_module_mutated_read_reactive_stmt,
    "legacy/coarse_deps/module_mutated_read_reactive_stmt",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_module_unmutated_read_template,
    "legacy/coarse_deps/module_unmutated_read_template",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_instance_mutated_read_template,
    "legacy/coarse_deps/instance_mutated_read_template",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_each_index_unkeyed,
    "legacy/coarse_deps/each_index_unkeyed",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_const_tag_dep_sibling,
    "legacy/coarse_deps/const_tag_dep_sibling",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_dollar_slots_bare,
    "legacy/coarse_deps/if_dollar_slots_bare",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_dollar_props_bare,
    "legacy/coarse_deps/if_dollar_props_bare",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_dollar_restprops_bare,
    "legacy/coarse_deps/if_dollar_restprops_bare",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_attr_dollar_slots_bare,
    "legacy/coarse_deps/attr_dollar_slots_bare",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_prop_bare,
    "legacy/coarse_deps/if_prop_bare",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_state_bare,
    "legacy/coarse_deps/if_state_bare",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_member,
    "legacy/coarse_deps/if_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_call,
    "legacy/coarse_deps/if_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_slots_member,
    "legacy/coarse_deps/if_slots_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_if_props_member,
    "legacy/coarse_deps/if_props_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_text_dollar_slots_concat,
    "legacy/coarse_deps/text_dollar_slots_concat",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_prop_slots_member,
    "legacy/coarse_deps/prop_slots_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_prop_global_member,
    "legacy/coarse_deps/prop_global_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_prop_const_inline_bare,
    "legacy/coarse_deps/prop_const_inline_bare",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_store_member_lhs_read,
    "legacy/coarse_deps/dep_order_store_member_lhs_read",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_store_member_lhs_only,
    "legacy/coarse_deps/dep_order_store_member_lhs_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_nested_member_lhs,
    "legacy/coarse_deps/dep_order_nested_member_lhs",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_plain_member_lhs_read,
    "legacy/coarse_deps/dep_order_plain_member_lhs_read",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_ident_lhs_reassign_read,
    "legacy/coarse_deps/dep_order_ident_lhs_reassign_read",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_ident_lhs_only,
    "legacy/coarse_deps/dep_order_ident_lhs_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_compound_assign,
    "legacy/coarse_deps/dep_order_compound_assign",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    coarse_deps_dep_order_readonly_block,
    "legacy/coarse_deps/dep_order_readonly_block",
    [prod, dev, ssr, ssr_dev]
);
