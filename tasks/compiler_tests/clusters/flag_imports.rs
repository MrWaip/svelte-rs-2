use super::*;

compiler_case!(
    async_legacy_order,
    "flag_imports/async_legacy_order",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    async_tracing_order,
    "flag_imports/async_tracing_order",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    async_only_guard,
    "flag_imports/async_only_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_only_guard,
    "flag_imports/legacy_only_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    tracing_only_guard,
    "flag_imports/tracing_only_guard",
    [prod, dev, ssr, ssr_dev]
);
