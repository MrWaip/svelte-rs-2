use super::*;

compiler_case!(title_in_if, "head_title/title_in_if");
compiler_case!(title_in_if_else, "head_title/title_in_if_else");
compiler_case!(title_in_each, "head_title/title_in_each");
compiler_case!(title_in_nested_if, "head_title/title_in_nested_if");
compiler_case!(title_dynamic_in_if, "head_title/title_dynamic_in_if");

compiler_case!(title_direct_guard, "head_title/title_direct_guard");
compiler_case!(
    title_outside_head_guard,
    "head_title/title_outside_head_guard"
);
compiler_case!(meta_in_if_head_guard, "head_title/meta_in_if_head_guard");
