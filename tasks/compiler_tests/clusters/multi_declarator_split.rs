use super::*;

compiler_case!(
    export_props_multi,
    "multi_declarator_split/export_props_multi",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    export_prop_single,
    "multi_declarator_split/export_prop_single",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    bare_legacy_state_multi,
    "multi_declarator_split/bare_legacy_state_multi",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_state_multi,
    "multi_declarator_split/runes_state_multi",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    export_props_multi_destructure,
    "multi_declarator_split/export_props_multi_destructure",
    [prod, dev, ssr, ssr_dev]
);
