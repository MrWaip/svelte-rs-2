use super::*;

compiler_case!(
    group_blocker_order_mixed,
    "async_fragment_declaration/group_blocker_order_mixed",
    ignore = "script blockers and declaration blockers are two lists, so a block emits them grouped rather than in reference order"
);
compiler_case!(
    group_each_mixed_blockers,
    "async_fragment_declaration/group_each_mixed_blockers",
    ignore = "script blockers and declaration blockers are two lists, so a block emits them grouped rather than in reference order"
);
compiler_case!(
    group_snippet_before_declaration,
    "async_fragment_declaration/group_snippet_before_declaration",
    [prod, dev, ssr_todo, ssr_dev_todo]
);
compiler_case!(
    group_after_hoisted_declaration,
    "async_fragment_declaration/group_after_hoisted_declaration",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    group_after_hoisted_snippet,
    "async_fragment_declaration/group_after_hoisted_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    group_boundary_snippet_stays_inside,
    "async_fragment_declaration/group_boundary_snippet_stays_inside",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    group_snippet_blockers,
    "async_fragment_declaration/group_snippet_blockers",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    group_block_const_blockers,
    "async_fragment_declaration/group_block_const_blockers",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_tag_await,
    "async_fragment_declaration/const_tag_await",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_tag_order,
    "async_fragment_declaration/const_tag_order",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_tag_destructured,
    "async_fragment_declaration/const_tag_destructured",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_tag_in_each,
    "async_fragment_declaration/const_tag_in_each",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_tag_in_snippet,
    "async_fragment_declaration/const_tag_in_snippet",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_tag_multi_declarator,
    "async_fragment_declaration/declaration_tag_multi_declarator",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_tag_no_initializer,
    "async_fragment_declaration/declaration_tag_no_initializer",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_tag_destructured,
    "async_fragment_declaration/declaration_tag_destructured",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_tag_derived_await,
    "async_fragment_declaration/declaration_tag_derived_await",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    declaration_tag_outer_fragment_blocker,
    "async_fragment_declaration/declaration_tag_outer_fragment_blocker",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    mixed_declaration_then_const,
    "async_fragment_declaration/mixed_declaration_then_const",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    mixed_const_then_declaration,
    "async_fragment_declaration/mixed_const_then_declaration",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    snippet_blocks_on_outer_fragment,
    "async_fragment_declaration/snippet_blocks_on_outer_fragment",
    [prod, dev, ssr_todo, ssr_dev_todo]
);

compiler_case!(
    const_tag_blocker_single_guard,
    "async_fragment_declaration/const_tag_blocker_single_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_tag_blocker_multiple_guard,
    "async_fragment_declaration/const_tag_blocker_multiple_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_tag_after_blocker_guard,
    "async_fragment_declaration/const_tag_after_blocker_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    const_tag_sync_only_guard,
    "async_fragment_declaration/const_tag_sync_only_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_tag_await_guard,
    "async_fragment_declaration/declaration_tag_await_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_tag_state_await_guard,
    "async_fragment_declaration/declaration_tag_state_await_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    declaration_tag_in_element_guard,
    "async_fragment_declaration/declaration_tag_in_element_guard",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    if_each_declaration_blockers,
    "async_fragment_declaration/if_each_declaration_blockers",
    [prod_todo, dev_todo, ssr, ssr_dev]
);
