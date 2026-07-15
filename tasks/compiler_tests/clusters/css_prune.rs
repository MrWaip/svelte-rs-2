use super::*;

compiler_case!(has_only_child_in_each, "css_prune/has_only_child_in_each");
compiler_case!(
    has_absent_in_each_pruned_guard,
    "css_prune/has_absent_in_each_pruned_guard"
);
compiler_case!(has_match_static_guard, "css_prune/has_match_static_guard");
compiler_case!(
    class_object_key_no_match_unscoped,
    "css_prune/class_object_key_no_match_unscoped"
);
compiler_case!(
    class_object_key_match_scoped_guard,
    "css_prune/class_object_key_match_scoped_guard"
);
