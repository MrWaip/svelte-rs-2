use super::*;

compiler_case!(root_html, "doctype/root_html", [prod, dev, ssr, ssr_dev]);
compiler_case!(bare, "doctype/bare", [prod, dev, ssr, ssr_dev]);
compiler_case!(
    uppercase_lowercased,
    "doctype/uppercase_lowercased",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    nested_in_element,
    "doctype/nested_in_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    in_if_block,
    "doctype/in_if_block",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expression_attribute,
    "doctype/expression_attribute",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    bang_named_element,
    "doctype/bang_named_element",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    full_document,
    "doctype/full_document",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    comment_guard,
    "doctype/comment_guard",
    [prod, dev, ssr, ssr_dev]
);
