use super::*;

compiler_case!(
    use_bare,
    "legacy/quoted_directive/use_bare",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    use_unquoted,
    "legacy/quoted_directive/use_unquoted",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    use_quoted_double,
    "legacy/quoted_directive/use_quoted_double",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    use_quoted_single,
    "legacy/quoted_directive/use_quoted_single",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    bind_quoted,
    "legacy/quoted_directive/bind_quoted",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    class_quoted,
    "legacy/quoted_directive/class_quoted",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    transition_quoted,
    "legacy/quoted_directive/transition_quoted",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    on_quoted,
    "legacy/quoted_directive/on_quoted",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    animate_quoted,
    "legacy/quoted_directive/animate_quoted",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    let_quoted,
    "legacy/quoted_directive/let_quoted",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    style_quoted_expr,
    "legacy/quoted_directive/style_quoted_expr",
    [prod, dev, ssr, ssr_dev]
);
