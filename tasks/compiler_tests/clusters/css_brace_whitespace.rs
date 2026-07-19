use super::*;

compiler_case!(top_level, "css_brace_whitespace/top_level");
compiler_case!(nested, "css_brace_whitespace/nested");
compiler_case!(at_rule, "css_brace_whitespace/at_rule");

compiler_case!(
    space_before_brace_guard,
    "css_brace_whitespace/space_before_brace_guard"
);
compiler_case!(
    tab_before_brace_guard,
    "css_brace_whitespace/tab_before_brace_guard"
);
