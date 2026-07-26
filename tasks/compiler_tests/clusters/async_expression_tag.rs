use super::*;

compiler_case!(
    fragment_root,
    "async_expression_tag/fragment_root",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    if_block_body,
    "async_expression_tag/if_block_body",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    each_block_body,
    "async_expression_tag/each_block_body",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    key_block_body,
    "async_expression_tag/key_block_body",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    snippet_body,
    "async_expression_tag/snippet_body",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_children,
    "async_expression_tag/component_children",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    svelte_head_body,
    "async_expression_tag/svelte_head_body",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    svelte_element_body,
    "async_expression_tag/svelte_element_body",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    svelte_boundary_body,
    "async_expression_tag/svelte_boundary_body",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    element_child_guard,
    "async_expression_tag/element_child_guard"
);
compiler_case!(
    nested_element_child_guard,
    "async_expression_tag/nested_element_child_guard"
);
compiler_case!(
    element_in_if_block_guard,
    "async_expression_tag/element_in_if_block_guard"
);
compiler_case!(
    title_element_guard,
    "async_expression_tag/title_element_guard"
);
compiler_case!(
    pickled_non_tail_guard,
    "async_expression_tag/pickled_non_tail_guard"
);
compiler_case!(
    attribute_value_guard,
    "async_expression_tag/attribute_value_guard"
);
compiler_case!(
    function_boundary_guard,
    "async_expression_tag/function_boundary_guard"
);
