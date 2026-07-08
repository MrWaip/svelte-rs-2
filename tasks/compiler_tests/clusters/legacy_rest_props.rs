use super::*;

compiler_case!(
    rest_props_export_let_plain,
    "legacy/rest_props/export_let_plain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rest_props_export_let_renamed,
    "legacy/rest_props/export_let_renamed",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rest_props_export_const_plain,
    "legacy/rest_props/export_const_plain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rest_props_export_const_renamed,
    "legacy/rest_props/export_const_renamed",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    rest_props_export_function_renamed,
    "legacy/rest_props/export_function_renamed",
    [prod, dev, ssr, ssr_dev]
);
