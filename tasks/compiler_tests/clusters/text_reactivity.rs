use super::*;

compiler_case!(const_literal, "text_reactivity/const_literal");
compiler_case!(
    const_member_nonreactive,
    "text_reactivity/const_member_nonreactive"
);
compiler_case!(derived_literal, "text_reactivity/derived_literal");
compiler_case!(
    derived_nonreactive_dep,
    "text_reactivity/derived_nonreactive_dep"
);
compiler_case!(
    derived_destructured_nonreactive,
    "text_reactivity/derived_destructured_nonreactive"
);
compiler_case!(derived_state_dep, "text_reactivity/derived_state_dep");
compiler_case!(derived_reactive_dep, "text_reactivity/derived_reactive_dep");
compiler_case!(
    derived_chain_nonreactive,
    "text_reactivity/derived_chain_nonreactive"
);
compiler_case!(derived_forward_ref, "text_reactivity/derived_forward_ref");
compiler_case!(
    derived_nonreactive_dep_attr,
    "text_reactivity/derived_nonreactive_dep_attr"
);
compiler_case!(literal_root_call, "text_reactivity/literal_root_call");
compiler_case!(
    legacy_rest_props_call,
    "text_reactivity/legacy_rest_props_call"
);
compiler_case!(
    legacy_props_object_call,
    "text_reactivity/legacy_props_object_call"
);
compiler_case!(
    each_sequence_reactive_read,
    "text_reactivity/each_sequence_reactive_read"
);
compiler_case!(
    legacy_static_sequence,
    "text_reactivity/legacy_static_sequence"
);
compiler_case!(
    legacy_reactive_let_text,
    "text_reactivity/legacy_reactive_let_text",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(call_local_fn, "text_reactivity/call_local_fn");
compiler_case!(
    call_literal_root_runes,
    "text_reactivity/call_literal_root_runes"
);
compiler_case!(
    call_global_root_guard,
    "text_reactivity/call_global_root_guard"
);
compiler_case!(call_effect_tracking, "text_reactivity/call_effect_tracking");
compiler_case!(tagged_template_call, "text_reactivity/tagged_template_call");
compiler_case!(
    call_reactive_arg_guard,
    "text_reactivity/call_reactive_arg_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    array_local_not_defined,
    "text_reactivity/array_local_not_defined",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    string_local_defined_guard,
    "text_reactivity/string_local_defined_guard"
);
compiler_case!(
    logical_shortcircuit_call,
    "text_reactivity/logical_shortcircuit_call"
);
compiler_case!(select_value_call, "text_reactivity/select_value_call");
compiler_case!(call_literal_chain, "text_reactivity/call_literal_chain");
