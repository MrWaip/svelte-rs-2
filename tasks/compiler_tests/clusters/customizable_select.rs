use super::*;

compiler_case!(rich_select_top, "customizable_select/rich_select_top");
compiler_case!(
    rich_option_in_plain_select,
    "customizable_select/rich_option_in_plain_select"
);
compiler_case!(
    rich_optgroup_in_select,
    "customizable_select/rich_optgroup_in_select"
);
compiler_case!(rich_nested_all, "customizable_select/rich_nested_all");
compiler_case!(
    option_dynamic_attr_order,
    "customizable_select/option_dynamic_attr_order"
);
compiler_case!(static_rich_option, "customizable_select/static_rich_option");
compiler_case!(
    plain_select_text_option,
    "customizable_select/plain_select_text_option"
);
compiler_case!(two_optgroups_rich, "customizable_select/two_optgroups_rich");

compiler_case!(single_render_tag, "customizable_select/single_render_tag");
compiler_case!(single_if_block, "customizable_select/single_if_block");
compiler_case!(single_html_tag, "customizable_select/single_html_tag");
compiler_case!(single_each_block, "customizable_select/single_each_block");
compiler_case!(single_component, "customizable_select/single_component");
compiler_case!(multi_first_block, "customizable_select/multi_first_block");
compiler_case!(
    optgroup_single_render,
    "customizable_select/optgroup_single_render"
);

compiler_case!(
    single_element_guard,
    "customizable_select/single_element_guard"
);
compiler_case!(
    multi_static_first_guard,
    "customizable_select/multi_static_first_guard"
);
compiler_case!(
    multi_distinct_guard,
    "customizable_select/multi_distinct_guard"
);

compiler_case!(multi_dup_render, "customizable_select/multi_dup_render");
compiler_case!(multi_dup_option, "customizable_select/multi_dup_option");
compiler_case!(
    multi_dup_cross_parent,
    "customizable_select/multi_dup_cross_parent"
);
compiler_case!(
    static_then_dynamic_ident,
    "customizable_select/static_then_dynamic_ident"
);
