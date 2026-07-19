use super::*;

compiler_case!(
    root_compound_class_before,
    "css_root/root_compound_class_before"
);
compiler_case!(
    root_compound_class_after,
    "css_root/root_compound_class_after"
);
compiler_case!(root_compound_unknown, "css_root/root_compound_unknown");
compiler_case!(root_nested_element, "css_root/root_nested_element");
compiler_case!(root_nested_has, "css_root/root_nested_has");
compiler_case!(root_nested_amp_has, "css_root/root_nested_amp_has");
compiler_case!(root_nested_unused, "css_root/root_nested_unused");

compiler_case!(root_bare_guard, "css_root/root_bare_guard");
compiler_case!(root_descendant_guard, "css_root/root_descendant_guard");
compiler_case!(root_has_used_guard, "css_root/root_has_used_guard");
compiler_case!(root_not_guard, "css_root/root_not_guard");
compiler_case!(
    root_descendant_unused_guard,
    "css_root/root_descendant_unused_guard"
);
