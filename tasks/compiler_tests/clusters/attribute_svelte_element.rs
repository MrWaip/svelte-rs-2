use super::*;

compiler_case!(
    dynamic_class_style_directives,
    "attribute/svelte_element/dynamic_class_style_directives",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    dynamic_class_directives_only,
    "attribute/svelte_element/dynamic_class_directives_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    dynamic_style_directives_only,
    "attribute/svelte_element/dynamic_style_directives_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    dynamic_spread_guard,
    "attribute/svelte_element/dynamic_spread_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    static_element_class_style_guard,
    "attribute/svelte_element/static_element_class_style_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    dynamic_class_reactive_guard,
    "attribute/svelte_element/dynamic_class_reactive_guard",
    [prod, dev, ssr, ssr_dev]
);
