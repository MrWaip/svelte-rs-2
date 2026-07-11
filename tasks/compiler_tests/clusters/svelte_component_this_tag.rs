use super::*;

compiler_case!(
    css_props,
    "svelte_component/this_tag/css_props",
    [prod, dev_todo, ssr, ssr_dev]
);
compiler_case!(
    expr_unquoted_conditional,
    "svelte_component/this_tag/expr_unquoted_conditional"
);
compiler_case!(
    expr_unquoted_static,
    "svelte_component/this_tag/expr_unquoted_static"
);
compiler_case!(
    expr_quoted_single,
    "svelte_component/this_tag/expr_quoted_single"
);
compiler_case!(
    expr_quoted_conditional,
    "svelte_component/this_tag/expr_quoted_conditional"
);
compiler_case!(
    expr_quoted_events,
    "svelte_component/this_tag/expr_quoted_events"
);
compiler_case!(
    expr_quoted_bindings,
    "svelte_component/this_tag/expr_quoted_bindings"
);
compiler_case!(
    expr_quoted_children,
    "svelte_component/this_tag/expr_quoted_children"
);
