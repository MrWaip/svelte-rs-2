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
    "text_reactivity/legacy_reactive_let_text"
);
