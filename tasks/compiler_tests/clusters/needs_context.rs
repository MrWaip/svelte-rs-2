use super::*;

compiler_case!(
    template_new_expression,
    "needs_context/template_new_expression"
);
compiler_case!(template_unsafe_iife, "needs_context/template_unsafe_iife");
compiler_case!(
    template_unsafe_call_of_call,
    "needs_context/template_unsafe_call_of_call"
);
compiler_case!(
    legacy_custom_element_over_emit,
    "needs_context/legacy_custom_element_over_emit"
);

compiler_case!(guard_plain_reactive, "needs_context/guard_plain_reactive");
compiler_case!(
    guard_prop_member_call,
    "needs_context/guard_prop_member_call"
);
compiler_case!(
    guard_instance_new_expression,
    "needs_context/guard_instance_new_expression"
);
