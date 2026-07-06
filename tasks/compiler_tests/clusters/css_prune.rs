use super::*;

compiler_case!(
    has_only_child_in_each,
    "css_prune/has_only_child_in_each",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    has_absent_in_each_pruned_guard,
    "css_prune/has_absent_in_each_pruned_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    has_match_static_guard,
    "css_prune/has_match_static_guard",
    [prod, dev, ssr, ssr_dev]
);
