use super::*;

compiler_case!(is_trailing_unused, "css_is_has_prune/is_trailing_unused");
compiler_case!(is_leading_unused, "css_is_has_prune/is_leading_unused");
compiler_case!(is_middle_unused, "css_is_has_prune/is_middle_unused");
compiler_case!(is_multiple_unused, "css_is_has_prune/is_multiple_unused");
compiler_case!(has_trailing_unused, "css_is_has_prune/has_trailing_unused");
compiler_case!(has_leading_unused, "css_is_has_prune/has_leading_unused");
compiler_case!(
    where_trailing_unused,
    "css_is_has_prune/where_trailing_unused"
);
compiler_case!(is_nested_is_unused, "css_is_has_prune/is_nested_is_unused");

compiler_case!(
    not_unused_kept_guard,
    "css_is_has_prune/not_unused_kept_guard"
);
compiler_case!(
    not_multichild_unused_kept_guard,
    "css_is_has_prune/not_multichild_unused_kept_guard"
);
compiler_case!(is_all_used_guard, "css_is_has_prune/is_all_used_guard");
compiler_case!(has_all_used_guard, "css_is_has_prune/has_all_used_guard");
compiler_case!(is_all_unused_guard, "css_is_has_prune/is_all_unused_guard");
