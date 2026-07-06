use super::*;

compiler_case!(
    leading_in_fragment,
    "text_node_marker/leading_in_fragment",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    leading_in_each_body,
    "text_node_marker/leading_in_each_body",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    inside_element_guard,
    "text_node_marker/inside_element_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    after_static_sibling_guard,
    "text_node_marker/after_static_sibling_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    two_expressions_guard,
    "text_node_marker/two_expressions_guard",
    [prod, dev, ssr, ssr_dev]
);
