use super::*;

compiler_case!(
    boundary_reactive_handler_inline,
    "boundary/reactive_handler_inline"
);
compiler_case!(boundary_plain_handler_guard, "boundary/plain_handler_guard");
compiler_case!(
    boundary_state_handler_ident_guard,
    "boundary/state_handler_ident_guard"
);
compiler_case!(boundary_global_member_guard, "boundary/global_member_guard");

compiler_case!(
    boundary_const_in_failed_snippet,
    "boundary/const_in_failed_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    boundary_const_in_pending_snippet,
    "boundary/const_in_pending_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    boundary_const_in_named_snippet,
    "boundary/const_in_named_snippet"
);
compiler_case!(
    boundary_multiple_const_in_snippet,
    "boundary/multiple_const_in_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    boundary_const_in_failed_snippet_dev,
    "boundary/const_in_failed_snippet_dev",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    boundary_snippet_no_const_guard,
    "boundary/snippet_no_const_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    boundary_const_no_snippet_guard,
    "boundary/const_no_snippet_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    boundary_component_snippet_const_guard,
    "boundary/component_snippet_const_guard"
);
