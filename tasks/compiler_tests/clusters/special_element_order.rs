use super::*;

compiler_case!(head_before_text, "special_element_order/head_before_text");
compiler_case!(
    window_before_text,
    "special_element_order/window_before_text"
);
compiler_case!(body_before_text, "special_element_order/body_before_text");
compiler_case!(
    head_before_element_guard,
    "special_element_order/head_before_element_guard"
);
compiler_case!(
    head_before_multi_guard,
    "special_element_order/head_before_multi_guard"
);
compiler_case!(head_only_guard, "special_element_order/head_only_guard");
compiler_case!(text_only_guard, "special_element_order/text_only_guard");
