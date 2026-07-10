use super::*;

compiler_case!(
    static_boolean,
    "attribute_autofocus/static_boolean",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    static_boolean_nested,
    "attribute_autofocus/static_boolean_nested",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    dynamic_state,
    "attribute_autofocus/dynamic_state",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    literal_false,
    "attribute_autofocus/literal_false",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    static_string,
    "attribute_autofocus/static_string",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    muted_guard,
    "attribute_autofocus/muted_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    case_insensitive,
    "attribute_autofocus/case_insensitive",
    [prod, dev, ssr, ssr_dev]
);
