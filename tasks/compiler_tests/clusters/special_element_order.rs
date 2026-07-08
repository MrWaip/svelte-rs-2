use super::*;

compiler_case!(
    head_before_text,
    "special_element_order/head_before_text",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    window_before_text,
    "special_element_order/window_before_text",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    body_before_text,
    "special_element_order/body_before_text",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    head_before_element_guard,
    "special_element_order/head_before_element_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    head_before_multi_guard,
    "special_element_order/head_before_multi_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    head_only_guard,
    "special_element_order/head_only_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    text_only_guard,
    "special_element_order/text_only_guard",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    debug_before_element,
    "special_element_order/debug_before_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_bare_before_element,
    "special_element_order/debug_bare_before_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_before_text,
    "special_element_order/debug_before_text",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_before_multi,
    "special_element_order/debug_before_multi",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_after_element,
    "special_element_order/debug_after_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_before_element_legacy,
    "special_element_order/debug_before_element_legacy",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_in_if_before_element,
    "special_element_order/debug_in_if_before_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_in_each_before_element,
    "special_element_order/debug_in_each_before_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_in_snippet_before_element,
    "special_element_order/debug_in_snippet_before_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_before_component_guard,
    "special_element_order/debug_before_component_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    debug_only_guard,
    "special_element_order/debug_only_guard",
    [prod, dev, ssr, ssr_dev]
);
