use super::*;

compiler_case!(
    plain_function_guard,
    "attach_component/plain_function_guard"
);
compiler_case!(
    known_function_derived_guard,
    "attach_component/known_function_derived_guard"
);
compiler_case!(call_noop, "attach_component/call_noop");
compiler_case!(logical_noop, "attach_component/logical_noop");
compiler_case!(
    maybe_null_derived_noop,
    "attach_component/maybe_null_derived_noop"
);
