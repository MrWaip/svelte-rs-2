use super::*;

compiler_case!(
    declaration_legacy_alias,
    "const_tag/declaration/legacy/alias",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_array_hole,
    "const_tag/declaration/legacy/array_hole",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_array_of_objects,
    "const_tag/declaration/legacy/array_of_objects",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_array_rest,
    "const_tag/declaration/legacy/array_rest",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_array_rest_nested,
    "const_tag/declaration/legacy/array_rest_nested",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_computed_key,
    "const_tag/declaration/legacy/computed_key",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_computed_key_self_ref,
    "const_tag/declaration/legacy/computed_key_self_ref",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_default_intermediate_array,
    "const_tag/declaration/legacy/default_intermediate_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_default_intermediate_object,
    "const_tag/declaration/legacy/default_intermediate_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_default_leaf_array,
    "const_tag/declaration/legacy/default_leaf_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_default_leaf_object,
    "const_tag/declaration/legacy/default_leaf_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_flat_object,
    "const_tag/declaration/legacy/flat_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_nested_array,
    "const_tag/declaration/legacy/nested_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_nested_object,
    "const_tag/declaration/legacy/nested_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_object_in_array_in_object,
    "const_tag/declaration/legacy/object_in_array_in_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_object_of_arrays,
    "const_tag/declaration/legacy/object_of_arrays",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_object_rest,
    "const_tag/declaration/legacy/object_rest",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_single_array,
    "const_tag/declaration/legacy/single_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_legacy_string_key,
    "const_tag/declaration/legacy/string_key",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_alias,
    "const_tag/declaration/runes/alias",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_array_hole,
    "const_tag/declaration/runes/array_hole",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_array_of_objects,
    "const_tag/declaration/runes/array_of_objects",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_array_rest,
    "const_tag/declaration/runes/array_rest",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_array_rest_nested,
    "const_tag/declaration/runes/array_rest_nested",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_computed_key,
    "const_tag/declaration/runes/computed_key",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_default_intermediate_array,
    "const_tag/declaration/runes/default_intermediate_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_default_intermediate_object,
    "const_tag/declaration/runes/default_intermediate_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_default_leaf_array,
    "const_tag/declaration/runes/default_leaf_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_default_leaf_object,
    "const_tag/declaration/runes/default_leaf_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_flat_object,
    "const_tag/declaration/runes/flat_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_nested_array,
    "const_tag/declaration/runes/nested_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_nested_object,
    "const_tag/declaration/runes/nested_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_object_in_array_in_object,
    "const_tag/declaration/runes/object_in_array_in_object",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_object_of_arrays,
    "const_tag/declaration/runes/object_of_arrays",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_object_rest,
    "const_tag/declaration/runes/object_rest",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_single_array,
    "const_tag/declaration/runes/single_array",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_runes_string_key,
    "const_tag/declaration/runes/string_key",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    order_legacy_forward_dependency,
    "const_tag/order/legacy/forward_dependency",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    order_legacy_deep_chain,
    "const_tag/order/legacy/deep_chain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    order_legacy_forward_reactive,
    "const_tag/order/legacy/forward_reactive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    order_legacy_in_order_guard,
    "const_tag/order/legacy/in_order_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    order_legacy_independent_guard,
    "const_tag/order/legacy/independent_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    order_legacy_single_guard,
    "const_tag/order/legacy/single_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    order_runes_in_order_guard,
    "const_tag/order/runes/in_order_guard",
    [prod, dev, ssr, ssr_dev]
);
