use super::*;

compiler_case!(nested_same_name, "component_dynamic_name/nested_same_name");
compiler_case!(
    nested_same_name_triple,
    "component_dynamic_name/nested_same_name_triple"
);
compiler_case!(single_guard, "component_dynamic_name/single_guard");
compiler_case!(
    siblings_same_name_guard,
    "component_dynamic_name/siblings_same_name_guard"
);
compiler_case!(
    nested_distinct_name_guard,
    "component_dynamic_name/nested_distinct_name_guard"
);
compiler_case!(
    svelte_component_nested_guard,
    "component_dynamic_name/svelte_component_nested_guard"
);
compiler_case!(
    mixed_dynamic_svelte_component_guard,
    "component_dynamic_name/mixed_dynamic_svelte_component_guard"
);
