use super::*;

compiler_case!(expr_unquoted, "svelte_element/this_tag/expr_unquoted");
compiler_case!(text_plain, "svelte_element/this_tag/text_plain");
compiler_case!(
    expr_quoted_double,
    "svelte_element/this_tag/expr_quoted_double"
);
compiler_case!(
    expr_quoted_single,
    "svelte_element/this_tag/expr_quoted_single"
);
compiler_case!(
    expr_quoted_literal,
    "svelte_element/this_tag/expr_quoted_literal"
);
compiler_case!(
    expr_quoted_template_literal,
    "svelte_element/this_tag/expr_quoted_template_literal"
);
compiler_case!(expr_quoted_call, "svelte_element/this_tag/expr_quoted_call");
compiler_case!(
    expr_quoted_with_content,
    "svelte_element/this_tag/expr_quoted_with_content"
);
compiler_case!(concat_text_expr, "svelte_element/this_tag/concat_text_expr");
compiler_case!(
    async_tag_no_children,
    "svelte_element/this_tag/async_tag_no_children",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    dev_tag_source_order_nested,
    "svelte_element_this_tag/dev_tag_source_order_nested",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_state_rune,
    "svelte_element/this_tag/expr_quoted_state_rune",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_props_rune,
    "svelte_element/this_tag/expr_quoted_props_rune",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_derived_rune,
    "svelte_element/this_tag/expr_quoted_derived_rune",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_state_member,
    "svelte_element/this_tag/expr_quoted_state_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_template_literal_state,
    "svelte_element/this_tag/expr_quoted_template_literal_state",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_each_item,
    "svelte_element/this_tag/expr_quoted_each_item",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_legacy_prop,
    "svelte_element/this_tag/expr_quoted_legacy_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_store,
    "svelte_element/this_tag/expr_quoted_store",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_legacy_reactive,
    "svelte_element/this_tag/expr_quoted_legacy_reactive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_await_prop,
    "svelte_element/this_tag/expr_quoted_await_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_unquoted_state_rune_guard,
    "svelte_element/this_tag/expr_unquoted_state_rune_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    concat_text_expr_state_guard,
    "svelte_element/this_tag/concat_text_expr_state_guard",
    [prod, dev, ssr, ssr_dev]
);
