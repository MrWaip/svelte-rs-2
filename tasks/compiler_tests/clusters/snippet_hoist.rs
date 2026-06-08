use super::*;

compiler_case!(pure_no_export, "snippet_hoist/pure_no_export");
compiler_case!(export_simple, "snippet_hoist/export_simple");
compiler_case!(export_cross_snippet, "snippet_hoist/export_cross_snippet");
compiler_case!(export_with_instance, "snippet_hoist/export_with_instance");
compiler_case!(
    non_hoistable_instance_ref,
    "snippet_hoist/non_hoistable_instance_ref"
);
