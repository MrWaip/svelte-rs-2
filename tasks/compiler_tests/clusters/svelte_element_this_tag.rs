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
    dev_tag_source_order_nested,
    "svelte_element_this_tag/dev_tag_source_order_nested",
    [prod, dev, ssr, ssr_dev]
);
