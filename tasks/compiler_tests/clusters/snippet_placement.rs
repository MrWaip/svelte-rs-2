use super::*;

compiler_case!(
    component_css_props_snippet_order,
    "snippet_placement/component_css_props_snippet_order",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_css_props_snippet_prop_memo,
    "snippet_placement/component_css_props_snippet_prop_memo",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_css_props_snippet_dynamic,
    "snippet_placement/component_css_props_snippet_dynamic",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_render_sibling,
    "snippet_placement/element_render_sibling"
);
compiler_case!(element_each_render, "snippet_placement/element_each_render");
compiler_case!(block_const_order, "snippet_placement/block_const_order");
compiler_case!(element_bind_this, "snippet_placement/element_bind_this");
compiler_case!(
    guard_toplevel_hoistable,
    "snippet_placement/guard_toplevel_hoistable"
);
compiler_case!(
    guard_toplevel_instance,
    "snippet_placement/guard_toplevel_instance"
);
compiler_case!(
    component_binding_hoists_snippet,
    "snippet_placement/component_binding_hoists_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_binding_hoists_multiple_snippets,
    "snippet_placement/component_binding_hoists_multiple_snippets",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_binding_no_snippet_guard,
    "snippet_placement/component_binding_no_snippet_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    snippet_no_component_binding_guard,
    "snippet_placement/snippet_no_component_binding_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_binding_hoistable_snippet_guard,
    "snippet_placement/component_binding_hoistable_snippet_guard",
    [prod, dev, ssr, ssr_dev]
);
