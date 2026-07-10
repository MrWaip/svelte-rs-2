use super::*;

compiler_case!(
    reactive_import_wrap_import_unmutated,
    "legacy/reactive_import_wrap/import_unmutated",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_import_wrap_instance_reassign_read,
    "legacy/reactive_import_wrap/instance_reassign_read",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_import_wrap_instance_member_mutate,
    "legacy/reactive_import_wrap/instance_member_mutate",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_import_wrap_instance_store_sub,
    "legacy/reactive_import_wrap/instance_store_sub",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_import_wrap_module_import_mutated,
    "legacy/reactive_import_wrap/module_import_mutated",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_import_wrap_runes_import_mutated,
    "legacy/reactive_import_wrap/runes_import_mutated",
    [prod, dev, ssr, ssr_dev]
);
