use super::*;

compiler_case!(
    element_render_sibling,
    "snippet_placement/element_render_sibling",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_each_render,
    "snippet_placement/element_each_render",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    block_const_order,
    "snippet_placement/block_const_order",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_bind_this,
    "snippet_placement/element_bind_this",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_toplevel_hoistable,
    "snippet_placement/guard_toplevel_hoistable",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_toplevel_instance,
    "snippet_placement/guard_toplevel_instance",
    [prod, dev, ssr, ssr_dev]
);
