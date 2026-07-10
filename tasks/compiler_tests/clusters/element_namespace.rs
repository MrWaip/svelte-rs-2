use super::*;

compiler_case!(
    green_known_svg,
    "element_namespace/green_known_svg",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    green_a_in_svg_direct,
    "element_namespace/green_a_in_svg_direct",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    green_a_in_svg_snippet,
    "element_namespace/green_a_in_svg_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    green_mathml,
    "element_namespace/green_mathml",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    green_multi_svg_plain,
    "element_namespace/green_multi_svg_plain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    red_a_downward_promote,
    "element_namespace/red_a_downward_promote",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    red_multi_svg_with_component,
    "element_namespace/red_multi_svg_with_component",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    red_circle_under_dynamic_svelte_element,
    "element_namespace/red_circle_under_dynamic_svelte_element",
    [prod, dev, ssr, ssr_dev]
);
