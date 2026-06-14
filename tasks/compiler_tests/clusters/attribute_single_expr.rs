use super::*;

compiler_case!(elem_reactive, "attribute/single_expr/elem_reactive");
compiler_case!(elem_call, "attribute/single_expr/elem_call");
compiler_case!(prop_reactive, "attribute/single_expr/prop_reactive");
compiler_case!(prop_call, "attribute/single_expr/prop_call");
compiler_case!(class_reactive, "attribute/single_expr/class_reactive");

compiler_case!(g_elem_unquoted, "attribute/single_expr/g_elem_unquoted");
compiler_case!(
    g_elem_concat_text,
    "attribute/single_expr/g_elem_concat_text"
);
compiler_case!(g_elem_two_expr, "attribute/single_expr/g_elem_two_expr");
compiler_case!(g_elem_static, "attribute/single_expr/g_elem_static");
compiler_case!(g_elem_boolean, "attribute/single_expr/g_elem_boolean");
compiler_case!(g_prop_unquoted, "attribute/single_expr/g_prop_unquoted");
compiler_case!(g_prop_concat, "attribute/single_expr/g_prop_concat");
