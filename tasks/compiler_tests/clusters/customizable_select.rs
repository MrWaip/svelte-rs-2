use super::*;

compiler_case!(
    rich_select_top,
    "customizable_select/rich_select_top",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rich_option_in_plain_select,
    "customizable_select/rich_option_in_plain_select",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rich_optgroup_in_select,
    "customizable_select/rich_optgroup_in_select",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rich_nested_all,
    "customizable_select/rich_nested_all",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    option_dynamic_attr_order,
    "customizable_select/option_dynamic_attr_order",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    static_rich_option,
    "customizable_select/static_rich_option",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    plain_select_text_option,
    "customizable_select/plain_select_text_option",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    two_optgroups_rich,
    "customizable_select/two_optgroups_rich",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    single_render_tag,
    "customizable_select/single_render_tag",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    single_if_block,
    "customizable_select/single_if_block",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    single_html_tag,
    "customizable_select/single_html_tag",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    single_each_block,
    "customizable_select/single_each_block",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    single_component,
    "customizable_select/single_component",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    multi_first_block,
    "customizable_select/multi_first_block",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    optgroup_single_render,
    "customizable_select/optgroup_single_render",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    single_element_guard,
    "customizable_select/single_element_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    multi_static_first_guard,
    "customizable_select/multi_static_first_guard",
    [prod, dev, ssr, ssr_dev]
);
