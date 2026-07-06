use super::*;

compiler_case!(
    literal_root_call,
    "if_else/literal_root_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    elseif_literal_root_call,
    "if_else/elseif_literal_root_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    local_fn_call,
    "if_else/local_fn_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    global_root_call,
    "if_else/global_root_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    reactive_arg_call,
    "if_else/reactive_arg_call",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    array_root_call,
    "if_else/array_root_call",
    [prod, dev, ssr, ssr_dev]
);
