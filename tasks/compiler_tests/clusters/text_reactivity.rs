use super::*;

compiler_case!(const_literal, "text_reactivity/const_literal");
compiler_case!(const_member_nonreactive, "text_reactivity/const_member_nonreactive");
compiler_case!(derived_literal, "text_reactivity/derived_literal");
compiler_case!(
    derived_nonreactive_dep,
    "text_reactivity/derived_nonreactive_dep",
    ignore = "blocked on is_symbol_dynamic/is_known architecture normalization: derived text dynamism must follow reference has_state (!evaluate.is_known), not d.reactive"
);
compiler_case!(
    derived_destructured_nonreactive,
    "text_reactivity/derived_destructured_nonreactive",
    ignore = "blocked on is_symbol_dynamic/is_known architecture normalization: derived text dynamism must follow reference has_state (!evaluate.is_known), not d.reactive"
);
compiler_case!(derived_state_dep, "text_reactivity/derived_state_dep");
