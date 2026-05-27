use super::*;

compiler_case!(declaration_alias, "legacy/let_directive/declaration/alias");
compiler_case!(declaration_array_hole, "legacy/let_directive/declaration/array_hole");
compiler_case!(declaration_array_of_objects, "legacy/let_directive/declaration/array_of_objects");
compiler_case!(declaration_array_rest, "legacy/let_directive/declaration/array_rest", ignore = "binding-pattern-routing slice 8 (let: directive, codegen) · tag binding-pattern-routing#8");
compiler_case!(declaration_array_rest_nested, "legacy/let_directive/declaration/array_rest_nested", ignore = "binding-pattern-routing slice 8 (let: directive, codegen) · tag binding-pattern-routing#8");
compiler_case!(declaration_computed_key, "legacy/let_directive/declaration/computed_key");
compiler_case!(declaration_default_intermediate_array, "legacy/let_directive/declaration/default_intermediate_array", ignore = "binding-pattern-routing slice 8 (let: directive, codegen) · tag binding-pattern-routing#8");
compiler_case!(declaration_default_intermediate_object, "legacy/let_directive/declaration/default_intermediate_object", ignore = "binding-pattern-routing slice 8 (let: directive, codegen) · tag binding-pattern-routing#8");
compiler_case!(declaration_default_leaf_array, "legacy/let_directive/declaration/default_leaf_array", ignore = "binding-pattern-routing slice 8 (let: directive, codegen) · tag binding-pattern-routing#8");
compiler_case!(declaration_flat_object, "legacy/let_directive/declaration/flat_object");
compiler_case!(declaration_nested_array, "legacy/let_directive/declaration/nested_array");
compiler_case!(declaration_nested_object, "legacy/let_directive/declaration/nested_object");
compiler_case!(declaration_object_in_array_in_object, "legacy/let_directive/declaration/object_in_array_in_object");
compiler_case!(declaration_object_of_arrays, "legacy/let_directive/declaration/object_of_arrays");
compiler_case!(declaration_object_rest, "legacy/let_directive/declaration/object_rest");
compiler_case!(declaration_single_array, "legacy/let_directive/declaration/single_array");
compiler_case!(declaration_string_key, "legacy/let_directive/declaration/string_key");
