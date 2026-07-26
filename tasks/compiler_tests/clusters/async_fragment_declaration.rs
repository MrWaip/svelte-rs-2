use super::*;

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
    ignore =
        "a declaration in a nested fragment does not block on the outer fragment's group member"
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
