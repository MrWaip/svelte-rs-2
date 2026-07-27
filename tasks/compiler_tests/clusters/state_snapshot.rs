use super::*;

compiler_case!(
    template_ignored_uncloneable,
    "state_snapshot/template_ignored_uncloneable",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    template_not_ignored_guard,
    "state_snapshot/template_not_ignored_guard"
);
