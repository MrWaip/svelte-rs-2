use super::*;

compiler_case!(
    open_prop,
    "bind_property/open_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    indeterminate_prop,
    "bind_property/indeterminate_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    duration_prop,
    "bind_property/duration_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    video_width_prop,
    "bind_property/video_width_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    natural_width_prop,
    "bind_property/natural_width_prop",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    guard_checked_prop,
    "bind_property/guard_checked_prop",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    guard_open_member,
    "bind_property/guard_open_member",
    [prod, dev, ssr, ssr_dev]
);
