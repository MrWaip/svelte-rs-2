use super::*;

compiler_case!(
    literal_folds_hash,
    "set_class_scope_hash/literal_folds_hash",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    empty_literal_folds_to_hash,
    "set_class_scope_hash/empty_literal_folds_to_hash",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    static_attr_folds_hash_guard,
    "set_class_scope_hash/static_attr_folds_hash_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    dynamic_keeps_separate_hash_guard,
    "set_class_scope_hash/dynamic_keeps_separate_hash_guard",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    template_literal_hash_separate_arg,
    "set_class_scope_hash/template_literal_hash_separate_arg",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    hash_class_before_style_dir,
    "set_class_scope_hash/hash_class_before_style_dir",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    hash_class_before_style_dir_after_attr,
    "set_class_scope_hash/hash_class_before_style_dir_after_attr",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    real_class_before_style_dir_guard,
    "set_class_scope_hash/real_class_before_style_dir_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    real_style_attr_before_hash_class_guard,
    "set_class_scope_hash/real_style_attr_before_hash_class_guard",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    class_dir_before_style_dir_guard,
    "set_class_scope_hash/class_dir_before_style_dir_guard",
    [prod, dev, ssr, ssr_dev]
);
