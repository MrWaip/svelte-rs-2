use super::*;

compiler_case!(store_sub, "render_tag_arg/store_sub");
compiler_case!(state_guard, "render_tag_arg/state_guard");
compiler_case!(call_guard, "render_tag_arg/call_guard");
compiler_case!(
    render_only_if_body_fragment_offset,
    "render_tag_arg/render_only_if_body_fragment_offset",
    [prod, dev, ssr, ssr_dev]
);
