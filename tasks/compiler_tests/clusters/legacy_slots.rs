use super::*;

compiler_case!(fallback_empty, "legacy_slots/fallback_empty");
compiler_case!(fallback_comment_only, "legacy_slots/fallback_comment_only");
compiler_case!(
    fallback_whitespace_only,
    "legacy_slots/fallback_whitespace_only"
);
compiler_case!(fallback_content, "legacy_slots/fallback_content");
compiler_case!(
    parent_default_children_prop,
    "legacy_slots/parent_default_children_prop"
);
compiler_case!(
    parent_default_children_prop_dev,
    "legacy_slots/parent_default_children_prop_dev"
);
compiler_case!(parent_default_plain, "legacy_slots/parent_default_plain");
compiler_case!(parent_default_let, "legacy_slots/parent_default_let");
compiler_case!(parent_named_slot, "legacy_slots/parent_named_slot");
compiler_case!(
    parent_children_prop_only,
    "legacy_slots/parent_children_prop_only"
);

compiler_case!(
    component_slot_let_shorthand,
    "legacy_slots/component_slot_let_shorthand"
);
compiler_case!(
    component_slot_let_aliased,
    "legacy_slots/component_slot_let_aliased"
);
compiler_case!(
    component_slot_let_object_destructure,
    "legacy_slots/component_slot_let_object_destructure"
);
compiler_case!(
    component_slot_let_array_destructure,
    "legacy_slots/component_slot_let_array_destructure"
);
compiler_case!(
    element_slot_let_guard,
    "legacy_slots/element_slot_let_guard"
);
compiler_case!(
    svelte_fragment_slot_let_guard,
    "legacy_slots/svelte_fragment_slot_let_guard"
);
compiler_case!(
    parent_default_element_plain_guard,
    "legacy_slots/parent_default_element_plain_guard"
);
compiler_case!(
    parent_default_slot_attribute,
    "legacy_slots/parent_default_slot_attribute"
);
compiler_case!(
    parent_default_slot_attribute_let,
    "legacy_slots/parent_default_slot_attribute_let"
);
compiler_case!(
    parent_default_element_let,
    "legacy_slots/parent_default_element_let"
);

compiler_case!(
    named_slot_dynamic_component_let,
    "legacy_slots/named_slot_dynamic_component_let"
);
compiler_case!(
    named_slot_static_component_let_guard,
    "legacy_slots/named_slot_static_component_let_guard"
);
compiler_case!(
    svelte_self_slot_let_guard,
    "legacy_slots/svelte_self_slot_let_guard",
    [prod, dev, ssr, ssr_dev]
);
