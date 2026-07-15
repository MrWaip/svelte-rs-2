use super::*;

compiler_case!(
    block_in_table_keeps_space,
    "whitespace_trim/block_in_table_keeps_space"
);
compiler_case!(
    table_direct_removes_space_guard,
    "whitespace_trim/table_direct_removes_space_guard"
);
compiler_case!(
    svg_block_removes_space_guard,
    "whitespace_trim/svg_block_removes_space_guard"
);

compiler_case!(
    server_static_class_style_base,
    "whitespace_trim/server_static_class_style_base",
    [prod, dev, ssr, ssr_dev]
);
