use super::*;

compiler_case!(
    instance_assign_nested_filename_guard,
    "dev_location/instance_assign_nested_filename_guard"
);
compiler_case!(
    instance_assign_backslash_filename_guard,
    "dev_location/instance_assign_backslash_filename_guard"
);
compiler_case!(
    instance_assign_root_dir_guard,
    "dev_location/instance_assign_root_dir_guard"
);
compiler_case!(
    script_module_assign_nested_filename,
    "dev_location/script_module_assign_nested_filename"
);

compiler_case!(
    instance_async_derived_filename,
    "dev_location/instance_async_derived_filename",
    ignore = "async derived promise hoisting not implemented"
);
compiler_case!(
    instance_async_derived_pattern_filename,
    "dev_location/instance_async_derived_pattern_filename",
    ignore = "async derived promise hoisting not implemented"
);

compiler_module_case!(
    module_assign_nested_filename,
    "dev_location/module_assign_nested_filename"
);
compiler_module_case!(
    module_assign_flat_filename,
    "dev_location/module_assign_flat_filename"
);
compiler_module_case!(
    module_assign_root_dir,
    "dev_location/module_assign_root_dir"
);
compiler_module_case!(
    module_assign_async_nested_filename,
    "dev_location/module_assign_async_nested_filename"
);
