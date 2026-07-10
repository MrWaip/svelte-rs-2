use super::*;

compiler_case!(
    bare_selector_inside_svelte_fragment,
    "css_scope_svelte_fragment/bare_selector_inside_svelte_fragment",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    descendant_through_component_default_slot,
    "css_scope_svelte_fragment/descendant_through_component_default_slot",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    descendant_through_element_and_component,
    "css_scope_svelte_fragment/descendant_through_element_and_component",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    descendant_through_svelte_head,
    "css_scope_svelte_fragment/descendant_through_svelte_head",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    descendant_through_snippet,
    "css_scope_svelte_fragment/descendant_through_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    descendant_has_through_component_default_slot,
    "css_scope_svelte_fragment/descendant_has_through_component_default_slot",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    descendant_through_svelte_fragment,
    "css_scope_svelte_fragment/descendant_through_svelte_fragment",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    descendant_nested_in_svelte_fragment,
    "css_scope_svelte_fragment/descendant_nested_in_svelte_fragment",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    child_combinator_through_svelte_fragment,
    "css_scope_svelte_fragment/child_combinator_through_svelte_fragment",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    descendant_has_through_svelte_fragment,
    "css_scope_svelte_fragment/descendant_has_through_svelte_fragment",
    [prod, dev, ssr, ssr_dev]
);
