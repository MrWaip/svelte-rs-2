use super::*;

compiler_case!(
    template_new_expression,
    "needs_context/template_new_expression",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_unsafe_iife,
    "needs_context/template_unsafe_iife",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_unsafe_call_of_call,
    "needs_context/template_unsafe_call_of_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_custom_element_over_emit,
    "needs_context/legacy_custom_element_over_emit",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    guard_plain_reactive,
    "needs_context/guard_plain_reactive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_prop_member_call,
    "needs_context/guard_prop_member_call"
);
compiler_case!(
    guard_instance_new_expression,
    "needs_context/guard_instance_new_expression",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(render_prop_member, "needs_context/render_prop_member");
compiler_case!(
    render_prop_identifier_guard,
    "needs_context/render_prop_identifier_guard"
);
compiler_case!(
    class_state_no_instance,
    "needs_context/class_state_no_instance"
);
compiler_case!(
    class_state_new_instance_guard,
    "needs_context/class_state_new_instance_guard"
);
