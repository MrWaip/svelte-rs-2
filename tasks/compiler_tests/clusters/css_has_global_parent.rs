use super::*;

compiler_case!(bare_has, "css_has_global_parent/bare_has");
compiler_case!(amp_has, "css_has_global_parent/amp_has");
compiler_case!(both_has, "css_has_global_parent/both_has");

compiler_case!(
    compound_has_guard,
    "css_has_global_parent/compound_has_guard"
);
compiler_case!(unused_has_guard, "css_has_global_parent/unused_has_guard");
compiler_case!(root_has_guard, "css_has_global_parent/root_has_guard");
compiler_case!(plain_has_guard, "css_has_global_parent/plain_has_guard");
