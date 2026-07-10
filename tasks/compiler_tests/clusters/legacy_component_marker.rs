use super::*;

compiler_case!(
    component_marker_bind_this,
    "legacy/component_marker/bind_this",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_marker_bind_prop,
    "legacy/component_marker/bind_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    component_marker_no_bind,
    "legacy/component_marker/no_bind",
    [prod, dev, ssr, ssr_dev]
);
