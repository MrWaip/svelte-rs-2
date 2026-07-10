use super::*;

compiler_case!(
    state_literal_shorthand,
    "component_props/state_literal_shorthand",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    state_reassigned_getter,
    "component_props/state_reassigned_getter",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_call_getter,
    "component_props/const_call_getter",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    derived_getter,
    "component_props/derived_getter",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    state_object_never_written,
    "component_props/state_object_never_written",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    state_function_getter,
    "component_props/state_function_getter",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    literal_const_shorthand,
    "component_props/literal_const_shorthand",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    function_const_shorthand,
    "component_props/function_const_shorthand",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    snippet_shorthand_getter,
    "component_props/snippet_shorthand_getter",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    call_pure_global_inline,
    "component_props/call_pure_global_inline",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    call_local_fn_derived,
    "component_props/call_local_fn_derived",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    call_pure_callee_state_arg_derived,
    "component_props/call_pure_callee_state_arg_derived",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    call_pure_global_nested_inline,
    "component_props/call_pure_global_nested_inline",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    each_index_concat_unkeyed,
    "component_props/each_index_concat_unkeyed",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    each_index_concat_keyed_guard,
    "component_props/each_index_concat_keyed_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    each_item_concat_guard,
    "component_props/each_item_concat_guard",
    [prod, dev, ssr, ssr_dev]
);
