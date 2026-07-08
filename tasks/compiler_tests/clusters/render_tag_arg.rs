use super::*;

compiler_case!(
    store_sub,
    "render_tag_arg/store_sub",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    state_guard,
    "render_tag_arg/state_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    call_guard,
    "render_tag_arg/call_guard",
    [prod, dev, ssr, ssr_dev]
);
