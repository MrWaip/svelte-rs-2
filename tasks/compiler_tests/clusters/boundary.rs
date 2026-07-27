use super::*;

compiler_case!(
    nullish_pending_with_snippet,
    "boundary/nullish_pending_with_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    nullish_pending_with_failed,
    "boundary/nullish_pending_with_failed",
    [prod, dev, ssr, ssr_dev]
);
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
    "boundary/const_in_failed_snippet"
);
compiler_case!(
    boundary_const_in_pending_snippet,
    "boundary/const_in_pending_snippet"
);
compiler_case!(
    boundary_const_in_named_snippet,
    "boundary/const_in_named_snippet"
);
compiler_case!(
    boundary_multiple_const_in_snippet,
    "boundary/multiple_const_in_snippet"
);
compiler_case!(
    boundary_const_in_failed_snippet_dev,
    "boundary/const_in_failed_snippet_dev"
);

compiler_case!(
    boundary_snippet_no_const_guard,
    "boundary/snippet_no_const_guard"
);
compiler_case!(
    boundary_const_no_snippet_guard,
    "boundary/const_no_snippet_guard"
);
compiler_case!(
    boundary_component_snippet_const_guard,
    "boundary/component_snippet_const_guard"
);

compiler_case!(
    boundary_pending_snippet_text_first,
    "boundary/pending_snippet_text_first"
);
compiler_case!(
    boundary_pending_snippet_expression_first,
    "boundary/pending_snippet_expression_first"
);
compiler_case!(
    boundary_pending_snippet_no_failed_text_first,
    "boundary/pending_snippet_no_failed_text_first"
);
compiler_case!(
    boundary_pending_snippet_element_first_guard,
    "boundary/pending_snippet_element_first_guard"
);
compiler_case!(
    boundary_failed_snippet_text_first_guard,
    "boundary/failed_snippet_text_first_guard"
);
compiler_case!(
    boundary_children_text_first_guard,
    "boundary/children_text_first_guard"
);

compiler_case!(
    boundary_snippet_order_regular_before_nested_failed,
    "boundary/snippet_order_regular_before_nested_failed"
);
compiler_case!(
    boundary_snippet_order_regular_after_nested_failed,
    "boundary/snippet_order_regular_after_nested_failed"
);
compiler_case!(
    boundary_snippet_order_multiple_regular,
    "boundary/snippet_order_multiple_regular"
);
compiler_case!(
    boundary_snippet_order_plain_boundary_guard,
    "boundary/snippet_order_plain_boundary_guard"
);
compiler_case!(
    boundary_snippet_order_pending_nested_guard,
    "boundary/snippet_order_pending_nested_guard"
);

compiler_case!(
    pending_snippet_attr,
    "boundary/pending_snippet_attr",
    [prod, dev, ssr, ssr_dev]
);
