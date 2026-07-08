use super::*;

compiler_case!(
    fallback_empty,
    "legacy_slots/fallback_empty",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    fallback_comment_only,
    "legacy_slots/fallback_comment_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    fallback_whitespace_only,
    "legacy_slots/fallback_whitespace_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    fallback_content,
    "legacy_slots/fallback_content",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    parent_default_children_prop,
    "legacy_slots/parent_default_children_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    parent_default_children_prop_dev,
    "legacy_slots/parent_default_children_prop_dev",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    parent_default_plain,
    "legacy_slots/parent_default_plain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    parent_default_let,
    "legacy_slots/parent_default_let",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    parent_named_slot,
    "legacy_slots/parent_named_slot",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    parent_children_prop_only,
    "legacy_slots/parent_children_prop_only",
    [prod, dev, ssr, ssr_dev]
);
