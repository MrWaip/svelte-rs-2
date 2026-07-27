use super::*;

compiler_case!(
    component_prop,
    "async_construct_wrap/component_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_prop_standalone,
    "async_construct_wrap/component_prop_standalone",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_prop_multiple,
    "async_construct_wrap/component_prop_multiple",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_prop_sync_and_async,
    "async_construct_wrap/component_prop_sync_and_async",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_prop_blocker_only,
    "async_construct_wrap/component_prop_blocker_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_prop_blocker_and_await,
    "async_construct_wrap/component_prop_blocker_and_await",
    ignore =
        "blocker identity is the promise index, not the declaration, so one index cannot repeat"
);
compiler_case!(
    component_spread,
    "async_construct_wrap/component_spread",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_bind,
    "async_construct_wrap/component_bind",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_css_props,
    "async_construct_wrap/component_css_props",
    ignore = "custom css props memoize outside the component's async value numbering"
);
compiler_case!(
    component_dynamic,
    "async_construct_wrap/component_dynamic",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_self,
    "async_construct_wrap/component_self",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    slot_prop,
    "async_construct_wrap/slot_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    slot_prop_fallback,
    "async_construct_wrap/slot_prop_fallback",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_spread,
    "async_construct_wrap/element_spread",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_spread_sync_and_async,
    "async_construct_wrap/element_spread_sync_and_async",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_spread_blocker_only,
    "async_construct_wrap/element_spread_blocker_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_spread_class_directive,
    "async_construct_wrap/element_spread_class_directive",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    component_prop_sync_guard,
    "async_construct_wrap/component_prop_sync_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    slot_prop_sync_guard,
    "async_construct_wrap/slot_prop_sync_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    element_spread_sync_guard,
    "async_construct_wrap/element_spread_sync_guard",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    component_snippet_child,
    "async_construct_wrap/component_snippet_child",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_css_prop_and_prop,
    "async_construct_wrap/component_css_prop_and_prop",
    [prod_todo, dev_todo, ssr, ssr_dev]
);

compiler_case!(
    component_concat_prop,
    "async_construct_wrap/component_concat_prop",
    [prod_todo, dev_todo, ssr, ssr_dev]
);
compiler_case!(
    element_concat_and_class_directive,
    "async_construct_wrap/element_concat_and_class_directive",
    [prod_todo, dev_todo, ssr, ssr_dev]
);
compiler_case!(
    element_spread_style_directive_concat,
    "async_construct_wrap/element_spread_style_directive_concat",
    [prod_todo, dev_todo, ssr, ssr_dev]
);
compiler_case!(
    svelte_element_attr,
    "async_construct_wrap/svelte_element_attr",
    [prod_todo, dev_todo, ssr, ssr_dev]
);
compiler_case!(
    svelte_element_attr_in_snippet,
    "async_construct_wrap/svelte_element_attr_in_snippet",
    [prod_todo, dev_todo, ssr, ssr_dev]
);
compiler_case!(
    svelte_element_attr_in_slot,
    "async_construct_wrap/svelte_element_attr_in_slot",
    [prod_todo, dev_todo, ssr, ssr_dev]
);

compiler_case!(
    element_indirect_blocker_duplicate,
    "async_construct_wrap/element_indirect_blocker_duplicate",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    element_bind_this_blocker,
    "async_construct_wrap/element_bind_this_blocker",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    debug_tag_blocker,
    "async_construct_wrap/debug_tag_blocker",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    state_eager_derived_raw,
    "async_construct_wrap/state_eager_derived_raw",
    [prod_todo, dev_todo, ssr, ssr_dev]
);

compiler_case!(
    component_prop_blocked_binding,
    "async_construct_wrap/component_prop_blocked_binding",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    text_blocked_folded_value,
    "async_construct_wrap/text_blocked_folded_value",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    server_text_blocked_folded,
    "async_construct_wrap/server_text_blocked_folded",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    element_attr_blocked_binding,
    "async_construct_wrap/element_attr_blocked_binding",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    html_tag_async_anchor,
    "async_construct_wrap/html_tag_async_anchor",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    each_async_anchor,
    "async_construct_wrap/each_async_anchor",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    html_tag_deferred_value,
    "async_construct_wrap/html_tag_deferred_value",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    directive_async_value,
    "async_construct_wrap/directive_async_value",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    spread_attr_async_value,
    "async_construct_wrap/spread_attr_async_value",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    class_attr_clsx_hoist,
    "async_construct_wrap/class_attr_clsx_hoist",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    memo_slot_numbering,
    "async_construct_wrap/memo_slot_numbering",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    svelte_element_async_anchor,
    "async_construct_wrap/svelte_element_async_anchor",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    memo_slot_numbering_directives,
    "async_construct_wrap/memo_slot_numbering_directives",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    special_value_placeholder_name_guard,
    "async_construct_wrap/special_value_placeholder_name_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    special_value_bind_checked_await,
    "async_construct_wrap/special_value_bind_checked_await",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    special_value_option_attr_hoist_order,
    "async_construct_wrap/special_value_option_attr_hoist_order",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    special_value_option_spread,
    "async_construct_wrap/special_value_option_spread",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    special_value_input_static_init,
    "async_construct_wrap/special_value_input_static_init",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    special_value_concat_slot_order,
    "async_construct_wrap/special_value_concat_slot_order",
    ignore = "concatenated special values memoize eagerly, so their slots precede the children's"
);
compiler_case!(
    special_value_slot_after_children,
    "async_construct_wrap/special_value_slot_after_children",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    special_value_slot_option_synthetic,
    "async_construct_wrap/special_value_slot_option_synthetic",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    special_value_slot_explicit_option,
    "async_construct_wrap/special_value_slot_explicit_option",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_spread_plain,
    "async_construct_wrap/attr_blockers_spread_plain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_spread_class_directive,
    "async_construct_wrap/attr_blockers_spread_class_directive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_spread_style_directive,
    "async_construct_wrap/attr_blockers_spread_style_directive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_select_value,
    "async_construct_wrap/attr_blockers_select_value",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_option_value,
    "async_construct_wrap/attr_blockers_option_value",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_concat_folded_part,
    "async_construct_wrap/attr_blockers_concat_folded_part",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    attr_blockers_concat_inline_read,
    "async_construct_wrap/attr_blockers_concat_inline_read",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_default_value,
    "async_construct_wrap/attr_blockers_default_value",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_bind_omitted_ssr,
    "async_construct_wrap/attr_blockers_bind_omitted_ssr",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_bind_file_value,
    "async_construct_wrap/attr_blockers_bind_file_value",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_bind_select_value,
    "async_construct_wrap/attr_blockers_bind_select_value",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_text_folded_child,
    "async_construct_wrap/attr_blockers_text_folded_child",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_class_attr,
    "async_construct_wrap/attr_blockers_class_attr",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_class_directive,
    "async_construct_wrap/attr_blockers_class_directive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_style_directive,
    "async_construct_wrap/attr_blockers_style_directive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_style_shorthand,
    "async_construct_wrap/attr_blockers_style_shorthand",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    memo_slot_numbering_text,
    "async_construct_wrap/memo_slot_numbering_text",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    memo_slot_numbering_concat,
    "async_construct_wrap/memo_slot_numbering_concat",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    attr_blockers_textarea_content,
    "async_construct_wrap/attr_blockers_textarea_content",
    [prod, dev, ssr, ssr_dev]
);
