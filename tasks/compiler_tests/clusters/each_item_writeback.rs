use super::*;

compiler_case!(
    legacy_store_readonly_guard,
    "each_item_writeback/legacy_store_readonly_guard"
);
compiler_case!(legacy_store_bind, "each_item_writeback/legacy_store_bind");
compiler_case!(
    legacy_store_assign,
    "each_item_writeback/legacy_store_assign"
);
compiler_case!(
    legacy_reactive_array_bind,
    "each_item_writeback/legacy_reactive_array_bind"
);
compiler_case!(
    legacy_store_destructure_bind,
    "each_item_writeback/legacy_store_destructure_bind"
);
compiler_case!(
    legacy_store_assign_noindex,
    "each_item_writeback/legacy_store_assign_noindex"
);
compiler_case!(
    legacy_store_bind_noindex,
    "each_item_writeback/legacy_store_bind_noindex"
);
compiler_case!(
    legacy_reactive_array_bind_noindex,
    "each_item_writeback/legacy_reactive_array_bind_noindex"
);
compiler_case!(
    legacy_store_destructure_bind_noindex,
    "each_item_writeback/legacy_store_destructure_bind_noindex"
);
