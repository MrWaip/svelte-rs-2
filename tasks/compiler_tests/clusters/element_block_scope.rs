use super::*;

compiler_case!(const_in_element, "element_block_scope/const_in_element");
compiler_case!(
    let_derived_in_element,
    "element_block_scope/let_derived_in_element"
);
compiler_case!(
    shadowing_in_element,
    "element_block_scope/shadowing_in_element"
);

compiler_case!(
    snippet_in_element_guard,
    "element_block_scope/snippet_in_element_guard"
);
compiler_case!(
    dynamic_element_no_block_guard,
    "element_block_scope/dynamic_element_no_block_guard"
);
compiler_case!(
    static_element_no_block_guard,
    "element_block_scope/static_element_no_block_guard"
);
