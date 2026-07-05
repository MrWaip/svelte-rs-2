use super::*;

compiler_case!(
    literal_guard,
    "style_directive/literal_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    plain_const,
    "style_directive/plain_const",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_interp,
    "style_directive/const_interp",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    custom_prop_const,
    "style_directive/custom_prop_const",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_let_guard,
    "style_directive/reactive_let_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(prop_guard, "style_directive/prop_guard");
compiler_case!(
    mixed_guard,
    "style_directive/mixed_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(global_call_suffix, "style_directive/global_call_suffix");
compiler_case!(
    user_call_suffix_guard,
    "style_directive/user_call_suffix_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    identifier_suffix_guard,
    "style_directive/identifier_suffix_guard",
    [prod, dev, ssr, ssr_dev]
);
