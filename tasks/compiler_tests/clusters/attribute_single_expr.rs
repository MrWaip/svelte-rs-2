use super::*;

compiler_case!(elem_reactive, "attribute/single_expr/elem_reactive");
compiler_case!(elem_call, "attribute/single_expr/elem_call");
compiler_case!(prop_reactive, "attribute/single_expr/prop_reactive");
compiler_case!(prop_call, "attribute/single_expr/prop_call");
compiler_case!(class_reactive, "attribute/single_expr/class_reactive");

compiler_case!(elem_quoted_const, "attribute/single_expr/elem_quoted_const");
compiler_case!(
    value_quoted_const,
    "attribute/single_expr/value_quoted_const"
);

compiler_case!(
    elem_unquoted_guard,
    "attribute/single_expr/elem_unquoted_guard"
);
compiler_case!(
    elem_unquoted_const_guard,
    "attribute/single_expr/elem_unquoted_const_guard"
);
compiler_case!(
    elem_concat_text_guard,
    "attribute/single_expr/elem_concat_text_guard"
);
compiler_case!(
    elem_two_expr_guard,
    "attribute/single_expr/elem_two_expr_guard"
);
compiler_case!(elem_static_guard, "attribute/single_expr/elem_static_guard");
compiler_case!(
    elem_boolean_guard,
    "attribute/single_expr/elem_boolean_guard"
);
compiler_case!(
    prop_unquoted_guard,
    "attribute/single_expr/prop_unquoted_guard"
);
compiler_case!(prop_concat_guard, "attribute/single_expr/prop_concat_guard");
compiler_case!(
    elem_concat_const_guard,
    "attribute/single_expr/elem_concat_const_guard"
);
compiler_case!(
    elem_two_const_guard,
    "attribute/single_expr/elem_two_const_guard"
);
