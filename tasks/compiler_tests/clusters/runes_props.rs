use super::*;

compiler_case!(props_id_declaration, "runes/props/id");
compiler_case!(slots_deconflict_ident, "runes/props/slots_deconflict_ident");
compiler_case!(slots_deconflict_rest, "runes/props/slots_deconflict_rest");
compiler_case!(
    slots_no_deconflict_guard,
    "runes/props/slots_no_deconflict_guard"
);
compiler_case!(assignment_array, "runes/props/assignment/array");
compiler_case!(assignment_array_rest, "runes/props/assignment/array_rest");
compiler_case!(
    assignment_computed_key,
    "runes/props/assignment/computed_key"
);
compiler_case!(assignment_default, "runes/props/assignment/default");
compiler_case!(
    assignment_nested_array,
    "runes/props/assignment/nested_array"
);
compiler_case!(assignment_object, "runes/props/assignment/object");
compiler_case!(assignment_object_rest, "runes/props/assignment/object_rest");
compiler_case!(declaration_alias, "runes/props/declaration/alias");
compiler_case!(declaration_bindable, "runes/props/declaration/bindable");
compiler_case!(
    declaration_bindable_alias,
    "runes/props/declaration/bindable_alias"
);
compiler_case!(
    declaration_bindable_default,
    "runes/props/declaration/bindable_default"
);
compiler_case!(
    bindable_proxy_default_lazy,
    "runes/props/bindable_proxy_default_lazy"
);
compiler_case!(
    declaration_default_alias,
    "runes/props/declaration/default_alias"
);
compiler_case!(
    declaration_default_leaf,
    "runes/props/declaration/default_leaf"
);
compiler_case!(
    declaration_flat_object,
    "runes/props/declaration/flat_object"
);
compiler_case!(
    declaration_mixed_rest_default,
    "runes/props/declaration/mixed_rest_default"
);
compiler_case!(declaration_rest, "runes/props/declaration/rest");
compiler_case!(declaration_single, "runes/props/declaration/single");
compiler_case!(declaration_string_key, "runes/props/declaration/string_key");
compiler_case!(
    optional_member_derived,
    "runes/props/optional_member_derived"
);
compiler_case!(
    optional_computed_member_guard,
    "runes/props/optional_computed_member_guard"
);
compiler_case!(
    static_member_derived_guard,
    "runes/props/static_member_derived_guard"
);
compiler_case!(
    fn_default_before_plain_prop,
    "runes/props/fn_default_before_plain_prop"
);
compiler_case!(
    arrow_default_before_bindable_prop,
    "runes/props/arrow_default_before_bindable_prop"
);
