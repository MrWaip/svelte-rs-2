use super::*;

compiler_case!(
    overload_instance_fn,
    "typescript/overload/instance_fn",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    overload_module_export_fn,
    "typescript/overload/module_export_fn",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    overload_normal_fn_guard,
    "typescript/overload/normal_fn_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    overload_declare_fn_guard,
    "typescript/overload/declare_fn_guard",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    template_expr_handler_plain_guard,
    "typescript/template_expr/handler_plain_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_expr_handler_param_type,
    "typescript/template_expr/handler_param_type",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_expr_handler_as_cast,
    "typescript/template_expr/handler_as_cast",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_expr_handler_non_null,
    "typescript/template_expr/handler_non_null",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_expr_mustache_as_cast,
    "typescript/template_expr/mustache_as_cast",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    snippet_param_plain_guard,
    "typescript/snippet_param/param_plain_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    snippet_param_param_type,
    "typescript/snippet_param/param_type",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    snippet_param_param_optional,
    "typescript/snippet_param/param_optional",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    snippet_param_param_typed_default,
    "typescript/snippet_param/param_typed_default",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    snippet_param_param_destructure_type,
    "typescript/snippet_param/param_destructure_type",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    snippet_param_param_optional_default,
    "typescript/snippet_param/param_optional_default",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    lang_mode_instance_ts_guard,
    "typescript/lang_mode/instance_ts_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    lang_mode_module_ts_instance_plain,
    "typescript/lang_mode/module_ts_instance_plain",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    lang_mode_instance_ts_module_plain,
    "typescript/lang_mode/instance_ts_module_plain",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    module_export_export_type_only,
    "typescript/module_export/export_type_only",
    [prod, dev, ssr, ssr_dev]
);
