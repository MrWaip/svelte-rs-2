use super::*;

compiler_case!(
    global_middle_class,
    "css_scope_position_global/global_middle_class"
);
compiler_case!(
    global_middle_multi_local,
    "css_scope_position_global/global_middle_multi_local"
);
compiler_case!(
    global_middle_id,
    "css_scope_position_global/global_middle_id"
);
compiler_case!(
    global_multi_global,
    "css_scope_position_global/global_multi_global"
);

compiler_case!(
    global_trailing_guard,
    "css_scope_position_global/global_trailing_guard"
);
compiler_case!(
    global_after_type_guard,
    "css_scope_position_global/global_after_type_guard"
);
compiler_case!(
    global_leading_guard,
    "css_scope_position_global/global_leading_guard"
);
compiler_case!(
    plain_compound_guard,
    "css_scope_position_global/plain_compound_guard"
);
