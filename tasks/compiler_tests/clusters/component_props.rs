use super::*;

compiler_case!(
    state_literal_shorthand,
    "component_props/state_literal_shorthand"
);
compiler_case!(
    state_reassigned_getter,
    "component_props/state_reassigned_getter"
);
compiler_case!(const_call_getter, "component_props/const_call_getter");
compiler_case!(derived_getter, "component_props/derived_getter");
compiler_case!(
    state_object_never_written,
    "component_props/state_object_never_written"
);
compiler_case!(
    state_function_getter,
    "component_props/state_function_getter"
);
compiler_case!(
    literal_const_shorthand,
    "component_props/literal_const_shorthand"
);
compiler_case!(
    function_const_shorthand,
    "component_props/function_const_shorthand"
);
compiler_case!(
    snippet_shorthand_getter,
    "component_props/snippet_shorthand_getter"
);
