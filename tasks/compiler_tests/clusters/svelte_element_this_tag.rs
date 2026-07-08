use super::*;

compiler_case!(
    expr_unquoted,
    "svelte_element/this_tag/expr_unquoted",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    text_plain,
    "svelte_element/this_tag/text_plain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_double,
    "svelte_element/this_tag/expr_quoted_double",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_single,
    "svelte_element/this_tag/expr_quoted_single",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_literal,
    "svelte_element/this_tag/expr_quoted_literal",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_template_literal,
    "svelte_element/this_tag/expr_quoted_template_literal",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_call,
    "svelte_element/this_tag/expr_quoted_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_with_content,
    "svelte_element/this_tag/expr_quoted_with_content",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    concat_text_expr,
    "svelte_element/this_tag/concat_text_expr",
    [prod, dev, ssr, ssr_dev]
);
