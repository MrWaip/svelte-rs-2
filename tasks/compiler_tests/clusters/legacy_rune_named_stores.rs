use super::*;

compiler_case!(
    rune_named_stores_derived_initializer_text,
    "legacy/rune_named_stores/derived_initializer_text",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rune_named_stores_state_initializer_text,
    "legacy/rune_named_stores/state_initializer_text",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rune_named_stores_class_state_field,
    "legacy/rune_named_stores/class_state_field",
    [prod, dev, ssr, ssr_dev]
);
