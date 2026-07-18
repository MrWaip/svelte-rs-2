use super::*;

compiler_case!(
    html_lowercased_void,
    "element_name_casing/html_lowercased_void",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    html_lowercased_nonvoid,
    "element_name_casing/html_lowercased_nonvoid",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    html_lowercased_in_foreignobject,
    "element_name_casing/html_lowercased_in_foreignobject",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    svg_preserves_case_guard,
    "element_name_casing/svg_preserves_case_guard",
    [prod, dev, ssr, ssr_dev]
);
