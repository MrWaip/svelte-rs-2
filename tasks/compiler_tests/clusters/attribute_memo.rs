use super::*;

compiler_case!(
    concat_call,
    "attribute/memo/concat_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    concat_call_with_directive,
    "attribute/memo/concat_call_with_directive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    concat_call_with_directive_and_style,
    "attribute/memo/concat_call_with_directive_and_style",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    concat_call_with_style,
    "attribute/memo/concat_call_with_style",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    concat_reactive_with_directive,
    "attribute/memo/concat_reactive_with_directive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expression_call,
    "attribute/memo/expression_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    literal_root_call,
    "attribute/memo/literal_root_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    expression_call_with_directive,
    "attribute/memo/expression_call_with_directive",
    [prod, dev, ssr, ssr_dev]
);
