use super::*;

compiler_case!(self_reference, "element_reset/self_reference");
compiler_case!(static_children_guard, "element_reset/static_children_guard");
compiler_case!(dynamic_child_guard, "element_reset/dynamic_child_guard");
compiler_case!(directive_child_guard, "element_reset/directive_child_guard");
compiler_case!(
    dir_attribute_reassert,
    "element_reset/dir_attribute_reassert",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    dir_static_parent_no_reset_guard,
    "element_reset/dir_static_parent_no_reset_guard",
    [prod, dev, ssr, ssr_dev]
);
