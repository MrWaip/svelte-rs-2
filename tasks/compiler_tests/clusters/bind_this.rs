use super::*;

compiler_case!(
    guard_runes_state,
    "bind_this/guard_runes_state",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_plain_local,
    "bind_this/guard_plain_local",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    prop_ident,
    "bind_this/prop_ident",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    each_member,
    "bind_this/each_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_prop_each_member_writeback,
    "bind_this/runes_prop_each_member_writeback",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    each_index_keyed,
    "bind_this/each_index_keyed",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    member_prop_computed,
    "bind_this/member_prop_computed",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(component_prop_ident, "bind_this/component_prop_ident");
compiler_case!(component_each_member, "bind_this/component_each_member");
compiler_case!(component_runes_state, "bind_this/component_runes_state");
compiler_case!(
    export_const_each_member,
    "bind_this/export_const_each_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    export_const_each_destructured,
    "bind_this/export_const_each_destructured",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    local_const_each_member,
    "bind_this/local_const_each_member",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    window_size_prop,
    "bind_this/window_size_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    window_scroll_prop,
    "bind_this/window_scroll_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    window_scroll_store,
    "bind_this/window_scroll_store",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    window_device_pixel_ratio_prop,
    "bind_this/window_device_pixel_ratio_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    document_fullscreen_prop,
    "bind_this/document_fullscreen_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_window_size_rune,
    "bind_this/guard_window_size_rune",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_window_size_legacy_reactive,
    "bind_this/guard_window_size_legacy_reactive",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_window_size_member,
    "bind_this/guard_window_size_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    legacy_each_collection_untrack,
    "bind_this/legacy_each_collection_untrack"
);

compiler_case!(
    seq_runes_state,
    "bind_this/seq_runes_state",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    seq_component_runes_state,
    "bind_this/seq_component_runes_state"
);
compiler_case!(
    seq_each_member,
    "bind_this/seq_each_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    seq_snippet_param_member,
    "bind_this/seq_snippet_param_member",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_value_seq_non_this,
    "bind_this/guard_value_seq_non_this",
    [prod, dev, ssr, ssr_dev]
);
