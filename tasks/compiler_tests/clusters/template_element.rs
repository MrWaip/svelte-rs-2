use super::*;

compiler_case!(
    static_content,
    "template_element/static",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_child,
    "template_element/element_child",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    html_tag,
    "template_element/html_tag",
    [prod, dev, ssr, ssr_dev]
);
