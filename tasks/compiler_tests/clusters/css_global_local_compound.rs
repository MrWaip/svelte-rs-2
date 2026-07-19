use super::*;

compiler_case!(descendant, "css_global_local_compound/descendant");
compiler_case!(nested_bare, "css_global_local_compound/nested_bare");
compiler_case!(nested_amp, "css_global_local_compound/nested_amp");
compiler_case!(descendant_deep, "css_global_local_compound/descendant_deep");
compiler_case!(multi_local, "css_global_local_compound/multi_local");
compiler_case!(
    nested_descendant,
    "css_global_local_compound/nested_descendant"
);

compiler_case!(
    global_args_compound_guard,
    "css_global_local_compound/global_args_compound_guard"
);
compiler_case!(
    plain_descendant_guard,
    "css_global_local_compound/plain_descendant_guard"
);
