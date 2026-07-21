use super::*;

compiler_case!(
    string_number_coercion,
    "value_evaluation/string_number_coercion"
);
compiler_case!(regex_literal, "value_evaluation/regex_literal");

compiler_case!(
    string_number_coercion_guard,
    "value_evaluation/string_number_coercion_guard"
);
compiler_case!(regex_literal_guard, "value_evaluation/regex_literal_guard");
