use super::*;

compiler_case!(arrow_positions, "paren_strip/arrow_positions");
compiler_case!(required_guard, "paren_strip/required_guard");

compiler_case!(iife_grouped, "paren_strip/iife_grouped");
compiler_case!(iife_grouped_legacy, "paren_strip/iife_grouped_legacy");
compiler_case!(
    iife_grouped_class_guard,
    "paren_strip/iife_grouped_class_guard"
);
