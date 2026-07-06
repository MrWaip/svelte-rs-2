use super::*;

compiler_case!(
    self_close_slash,
    "attribute/unquoted_value/self_close_slash",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    bare_slash_guard,
    "attribute/unquoted_value/bare_slash_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    quoted_slash_guard,
    "attribute/unquoted_value/quoted_slash_guard",
    [prod, dev, ssr, ssr_dev]
);
