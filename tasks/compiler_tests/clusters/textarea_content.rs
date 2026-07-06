use super::*;

compiler_case!(mixed, "textarea_content/mixed");
compiler_case!(single_expr, "textarea_content/single_expr");
compiler_case!(
    static_text_guard,
    "textarea_content/static_text_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(value_attr_guard, "textarea_content/value_attr_guard");
compiler_case!(bind_value_guard, "textarea_content/bind_value_guard");
