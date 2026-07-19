use super::*;

compiler_case!(const_in_each, "declaration_tag/const_in_each");
compiler_case!(let_reassigned, "declaration_tag/let_reassigned");
compiler_case!(state_rune, "declaration_tag/state_rune");
compiler_case!(state_derived_chain, "declaration_tag/state_derived_chain");
compiler_case!(multi_declarator, "declaration_tag/multi_declarator");
compiler_case!(destructure_object, "declaration_tag/destructure_object");
compiler_case!(destructure_array, "declaration_tag/destructure_array");
compiler_case!(let_uninitialized, "declaration_tag/let_uninitialized");
compiler_case!(top_level, "declaration_tag/top_level");
compiler_case!(async_await, "declaration_tag/async_await");

compiler_case!(root_state_derived, "declaration_tag/root_state_derived");
compiler_case!(in_if_block, "declaration_tag/in_if_block");
compiler_case!(element_block_scope, "declaration_tag/element_block_scope");
compiler_case!(shadowing, "declaration_tag/shadowing");
compiler_case!(prop_reactive, "declaration_tag/prop_reactive");
compiler_case!(state_raw, "declaration_tag/state_raw");
compiler_case!(derived_by, "declaration_tag/derived_by");
compiler_case!(state_destructure, "declaration_tag/state_destructure");
compiler_case!(derived_destructure, "declaration_tag/derived_destructure");
compiler_case!(destructure_nested, "declaration_tag/destructure_nested");
compiler_case!(destructure_rest, "declaration_tag/destructure_rest");
compiler_case!(destructure_default, "declaration_tag/destructure_default");
compiler_case!(in_snippet, "declaration_tag/in_snippet");

compiler_case!(atconst_guard, "declaration_tag/atconst_guard");
compiler_case!(
    identifier_prefix_guard,
    "declaration_tag/identifier_prefix_guard"
);
compiler_case!(
    identifier_type_keyword_guard,
    "declaration_tag/identifier_type_keyword_guard"
);
