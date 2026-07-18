use super::*;

compiler_case!(global_is, "css_global_pseudo_args/global_is");
compiler_case!(global_where, "css_global_pseudo_args/global_where");
compiler_case!(global_has, "css_global_pseudo_args/global_has");
compiler_case!(global_not, "css_global_pseudo_args/global_not");
compiler_case!(
    global_is_multiple,
    "css_global_pseudo_args/global_is_multiple"
);

compiler_case!(
    local_is_scoped_guard,
    "css_global_pseudo_args/local_is_scoped_guard"
);
compiler_case!(
    global_args_is_guard,
    "css_global_pseudo_args/global_args_is_guard"
);
