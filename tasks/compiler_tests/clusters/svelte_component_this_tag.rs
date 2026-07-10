use super::*;

compiler_case!(
    expr_unquoted_conditional,
    "svelte_component/this_tag/expr_unquoted_conditional",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_unquoted_static,
    "svelte_component/this_tag/expr_unquoted_static",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_single,
    "svelte_component/this_tag/expr_quoted_single",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_conditional,
    "svelte_component/this_tag/expr_quoted_conditional",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_events,
    "svelte_component/this_tag/expr_quoted_events",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_bindings,
    "svelte_component/this_tag/expr_quoted_bindings",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expr_quoted_children,
    "svelte_component/this_tag/expr_quoted_children",
    [prod, dev, ssr, ssr_dev]
);
