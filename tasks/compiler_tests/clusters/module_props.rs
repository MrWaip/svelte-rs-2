use super::*;

compiler_module_case!(
    param_reference,
    "module_props/param_reference",
    [prod, dev, ssr, ssr_dev]
);
compiler_module_case!(
    param_only,
    "module_props/param_only",
    [prod, dev, ssr, ssr_dev]
);
compiler_module_case!(
    free_reference,
    "module_props/free_reference",
    [prod, dev, ssr, ssr_dev]
);
compiler_module_case!(
    shorthand_property_value,
    "module_props/shorthand_property_value",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    write_reference,
    "module_props/write_reference",
    [prod, dev, ssr, ssr_dev]
);
compiler_module_case!(
    update_reference,
    "module_props/update_reference",
    [prod, dev, ssr, ssr_dev]
);

compiler_module_case!(
    member_property_guard,
    "module_props/member_property_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_module_case!(
    slots_guard,
    "module_props/slots_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_module_case!(
    object_key_guard,
    "module_props/object_key_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_module_case!(
    rest_props_guard,
    "module_props/rest_props_guard",
    [prod, dev, ssr, ssr_dev]
);
