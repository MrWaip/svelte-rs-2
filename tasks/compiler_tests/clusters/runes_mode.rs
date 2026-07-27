use super::*;

compiler_case!(
    template_await_forces_runes,
    "runes_mode/template_await_forces_runes",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_await_in_snippet_forces_runes,
    "runes_mode/template_await_in_snippet_forces_runes",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_await_attribute_forces_runes,
    "runes_mode/template_await_attribute_forces_runes",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_await_if_expression_forces_runes,
    "runes_mode/template_await_if_expression_forces_runes",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_await_const_tag_forces_runes,
    "runes_mode/template_await_const_tag_forces_runes",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_await_in_handler_stays_legacy_guard,
    "runes_mode/template_await_in_handler_stays_legacy_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    instance_await_forces_runes_guard,
    "runes_mode/instance_await_forces_runes_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    instance_await_in_function_stays_legacy_guard,
    "runes_mode/instance_await_in_function_stays_legacy_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    module_await_stays_legacy,
    "runes_mode/module_await_stays_legacy",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    instance_for_await_stays_legacy,
    "runes_mode/instance_for_await_stays_legacy",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    module_rune_reference_forces_runes_guard,
    "runes_mode/module_rune_reference_forces_runes_guard",
    [prod, dev, ssr, ssr_dev]
);
