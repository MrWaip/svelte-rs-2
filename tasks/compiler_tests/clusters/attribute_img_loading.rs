use super::*;

compiler_case!(lazy_child, "attribute/img_loading/lazy_child");
compiler_case!(
    static_parent_skips,
    "attribute/img_loading/static_parent_skips",
    [prod, dev, ssr, ssr_dev]
);
