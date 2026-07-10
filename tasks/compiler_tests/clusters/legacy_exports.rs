use super::*;

compiler_case!(
    exports_specifier_var_with_api_export,
    "legacy/exports/specifier_var_with_api_export",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_specifier_alias_with_api_export,
    "legacy/exports/specifier_alias_with_api_export",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_specifier_const,
    "legacy/exports/specifier_const",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_specifier_const_alias,
    "legacy/exports/specifier_const_alias",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_specifier_function,
    "legacy/exports/specifier_function",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_specifier_class,
    "legacy/exports/specifier_class",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_specifier_import,
    "legacy/exports/specifier_import",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_declaration_class,
    "legacy/exports/declaration_class",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_specifier_string_exported,
    "legacy/exports/specifier_string_exported",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    exports_specifier_class_read_in_template,
    "legacy/exports/specifier_class_read_in_template",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    runes_exports_specifier_string_exported,
    "runes/exports/specifier_string_exported",
    [prod, dev, ssr, ssr_dev]
);
