use super::*;

compiler_case!(
    cyrillic_before_element,
    "dev_element_locations/cyrillic_before_element"
);
compiler_case!(
    astral_before_element,
    "dev_element_locations/astral_before_element"
);
compiler_case!(
    nested_after_cyrillic,
    "dev_element_locations/nested_after_cyrillic"
);
compiler_case!(
    svelte_element_after_cyrillic,
    "dev_element_locations/svelte_element_after_cyrillic"
);
compiler_case!(ascii_line_guard, "dev_element_locations/ascii_line_guard");
compiler_case!(
    noscript_no_child_loc,
    "dev_element_locations/noscript_no_child_loc"
);
compiler_case!(
    component_css_props_child_loc,
    "dev_element_locations/component_css_props_child_loc"
);
