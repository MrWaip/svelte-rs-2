use super::*;

compiler_case!(pure_no_export, "snippet_hoist/pure_no_export");
compiler_case!(export_simple, "snippet_hoist/export_simple");
compiler_case!(export_cross_snippet, "snippet_hoist/export_cross_snippet");
compiler_case!(export_with_instance, "snippet_hoist/export_with_instance");
compiler_case!(
    non_hoistable_instance_ref,
    "snippet_hoist/non_hoistable_instance_ref"
);
compiler_case!(
    autosub_instance_store,
    "snippet_hoist/autosub_instance_store"
);
compiler_case!(
    autosub_imported_store,
    "snippet_hoist/autosub_imported_store"
);
compiler_case!(autosub_module_store, "snippet_hoist/autosub_module_store");
compiler_case!(
    autosub_imported_store_update,
    "snippet_hoist/autosub_imported_store_update"
);
compiler_case!(
    import_plain_value_hoistable,
    "snippet_hoist/import_plain_value_hoistable"
);
compiler_case!(
    autosub_legacy_state_shadow,
    "snippet_hoist/autosub_legacy_state_shadow"
);
