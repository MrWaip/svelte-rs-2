use super::*;

compiler_case!(
    dynamic_property_preserves_case,
    "custom_element/dynamic_property_preserves_case",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    static_property_not_inlined,
    "custom_element/static_property_not_inlined",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    value_property,
    "custom_element/value_property",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    is_attribute_marks_custom,
    "custom_element/is_attribute_marks_custom",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    guard_plain_element_lowercased,
    "custom_element/guard_plain_element_lowercased",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_class_uses_set_class,
    "custom_element/guard_class_uses_set_class",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_static_class_set_class,
    "custom_element/guard_static_class_set_class",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_static_style_set_style,
    "custom_element/guard_static_style_set_style",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    child_value_property,
    "custom_element/child_value_property",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    child_scoped_class,
    "custom_element/child_scoped_class",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    is_attribute_static_under_spread,
    "custom_element/is_attribute_static_under_spread",
    [prod, dev, ssr, ssr_dev]
);
