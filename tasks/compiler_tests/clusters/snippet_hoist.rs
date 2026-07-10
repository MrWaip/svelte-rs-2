use super::*;

compiler_case!(
    pure_no_export,
    "snippet_hoist/pure_no_export",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    export_simple,
    "snippet_hoist/export_simple",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    export_cross_snippet,
    "snippet_hoist/export_cross_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    export_with_instance,
    "snippet_hoist/export_with_instance",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    non_hoistable_instance_ref,
    "snippet_hoist/non_hoistable_instance_ref",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    autosub_instance_store,
    "snippet_hoist/autosub_instance_store",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    autosub_imported_store,
    "snippet_hoist/autosub_imported_store",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    autosub_module_store,
    "snippet_hoist/autosub_module_store",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    autosub_imported_store_update,
    "snippet_hoist/autosub_imported_store_update",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    import_plain_value_hoistable,
    "snippet_hoist/import_plain_value_hoistable",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    autosub_legacy_state_shadow,
    "snippet_hoist/autosub_legacy_state_shadow",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    ref_non_hoistable_snippet,
    "snippet_hoist/ref_non_hoistable_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    chain_non_hoistable_snippet,
    "snippet_hoist/chain_non_hoistable_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    ref_hoistable_snippet,
    "snippet_hoist/ref_hoistable_snippet",
    [prod, dev, ssr, ssr_dev]
);
