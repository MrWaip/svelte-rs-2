use super::*;

compiler_case!(
    self_close_slash,
    "attribute/unquoted_value/self_close_slash"
);
compiler_case!(
    bare_slash_guard,
    "attribute/unquoted_value/bare_slash_guard"
);
compiler_case!(
    quoted_slash_guard,
    "attribute/unquoted_value/quoted_slash_guard"
);
compiler_case!(eq_slash_gt, "attribute/unquoted_value/eq_slash_gt");
compiler_case!(eq_ws_slash_gt, "attribute/unquoted_value/eq_ws_slash_gt");
compiler_case!(leading_slash, "attribute/unquoted_value/leading_slash");
compiler_case!(
    slash_only_value,
    "attribute/unquoted_value/slash_only_value"
);
compiler_case!(
    terminator_backtick_guard,
    "attribute/unquoted_value/terminator_backtick_guard"
);
compiler_case!(
    terminator_lt_guard,
    "attribute/unquoted_value/terminator_lt_guard"
);
compiler_case!(
    terminator_space_guard,
    "attribute/unquoted_value/terminator_space_guard"
);
compiler_case!(
    quoted_sequence_guard,
    "attribute/unquoted_value/quoted_sequence_guard"
);
compiler_case!(
    legacy_text_expr_sequence,
    "attribute/unquoted_value/legacy_text_expr_sequence"
);
compiler_case!(
    legacy_expr_text_sequence,
    "attribute/unquoted_value/legacy_expr_text_sequence"
);
compiler_case!(
    legacy_two_expr_sequence,
    "attribute/unquoted_value/legacy_two_expr_sequence"
);
compiler_case!(
    legacy_component_sequence,
    "attribute/unquoted_value/legacy_component_sequence"
);
