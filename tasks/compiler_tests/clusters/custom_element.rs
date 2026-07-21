use super::*;

compiler_case!(
    dynamic_property_preserves_case,
    "custom_element/dynamic_property_preserves_case"
);
compiler_case!(
    static_property_not_inlined,
    "custom_element/static_property_not_inlined"
);
compiler_case!(value_property, "custom_element/value_property");
compiler_case!(
    is_attribute_marks_custom,
    "custom_element/is_attribute_marks_custom"
);

compiler_case!(
    guard_plain_element_lowercased,
    "custom_element/guard_plain_element_lowercased"
);
compiler_case!(
    guard_class_uses_set_class,
    "custom_element/guard_class_uses_set_class"
);
compiler_case!(
    guard_static_class_set_class,
    "custom_element/guard_static_class_set_class"
);
compiler_case!(
    guard_static_style_set_style,
    "custom_element/guard_static_style_set_style"
);
compiler_case!(child_value_property, "custom_element/child_value_property");
compiler_case!(child_scoped_class, "custom_element/child_scoped_class");
compiler_case!(
    is_attribute_static_under_spread,
    "custom_element/is_attribute_static_under_spread"
);
compiler_case!(
    legacy_rest_props_excludes_host,
    "custom_element/legacy_rest_props_excludes_host"
);
compiler_case!(shadow_mode_closed, "custom_element/shadow_mode_closed");

compiler_case!(shadow_none_guard, "custom_element/shadow_none_guard");
compiler_case!(
    shadow_open_string_guard,
    "custom_element/shadow_open_string_guard"
);
compiler_case!(
    prop_type_explicit_guard,
    "custom_element/prop_type_explicit_guard"
);
compiler_case!(
    prop_type_numeric_init_guard,
    "custom_element/prop_type_numeric_init_guard"
);
compiler_case!(prop_attribute_guard, "custom_element/prop_attribute_guard");
compiler_case!(
    prop_extra_export_empty_guard,
    "custom_element/prop_extra_export_empty_guard"
);
compiler_case!(extend_option_guard, "custom_element/extend_option_guard");
compiler_case!(no_tag_bare_guard, "custom_element/no_tag_bare_guard");
compiler_case!(
    shadow_none_extend_guard,
    "custom_element/shadow_none_extend_guard"
);

compiler_case!(shadow_init_options, "custom_element/shadow_init_options");
compiler_case!(
    prop_type_boolean_inference,
    "custom_element/prop_type_boolean_inference"
);
