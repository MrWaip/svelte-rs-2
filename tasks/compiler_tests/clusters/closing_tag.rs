use super::*;

compiler_case!(implicit_close, "closing_tag/implicit_close");

compiler_case!(
    unclosed_before_style,
    "closing_tag/style_extraction/unclosed_before_style"
);
compiler_case!(
    unclosed_before_script,
    "closing_tag/style_extraction/unclosed_before_script"
);
compiler_case!(
    double_unclosed_before_style,
    "closing_tag/style_extraction/double_unclosed_before_style"
);
compiler_case!(
    closed_before_style_guard,
    "closing_tag/style_extraction/closed_before_style_guard"
);
compiler_case!(
    style_only_guard,
    "closing_tag/style_extraction/style_only_guard"
);
compiler_case!(
    nested_style_guard,
    "closing_tag/style_extraction/nested_style_guard"
);
