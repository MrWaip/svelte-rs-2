use super::*;

compiler_case!(
    const_in_each,
    "declaration_tag/const_in_each",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    let_reassigned,
    "declaration_tag/let_reassigned",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    state_rune,
    "declaration_tag/state_rune",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    state_derived_chain,
    "declaration_tag/state_derived_chain",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    multi_declarator,
    "declaration_tag/multi_declarator",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    destructure_object,
    "declaration_tag/destructure_object",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    destructure_array,
    "declaration_tag/destructure_array",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    let_uninitialized,
    "declaration_tag/let_uninitialized",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    top_level,
    "declaration_tag/top_level",
    ignore = "declaration-tag: unimplemented (svelte 5.56.4)"
);
compiler_case!(
    async_await,
    "declaration_tag/async_await",
    ignore = "declaration-tag: unimplemented async (svelte 5.56.4)"
);

compiler_case!(atconst_guard, "declaration_tag/atconst_guard");
compiler_case!(
    identifier_prefix_guard,
    "declaration_tag/identifier_prefix_guard"
);
compiler_case!(
    identifier_type_keyword_guard,
    "declaration_tag/identifier_type_keyword_guard"
);
