use super::*;

compiler_case!(
    effect_tracking_in_text,
    "template_runes/effect_tracking_in_text"
);
compiler_case!(effect_in_each_iife, "template_runes/effect_in_each_iife");
compiler_case!(
    effect_pre_in_each_iife,
    "template_runes/effect_pre_in_each_iife"
);
compiler_case!(
    effect_root_in_text_iife,
    "template_runes/effect_root_in_text_iife"
);

compiler_case!(
    state_snapshot_in_text,
    "template_runes/state_snapshot_in_text"
);
compiler_case!(
    effect_tracking_in_script_guard,
    "template_runes/effect_tracking_in_script_guard"
);
compiler_case!(
    state_call_in_script_guard,
    "template_runes/state_call_in_script_guard",
    [prod, dev, ssr, ssr_dev]
);

compiler_case!(
    state_call_in_handler,
    "template_runes/state_call_in_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    derived_call_in_handler,
    "template_runes/derived_call_in_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    state_primitive_in_handler,
    "template_runes/state_primitive_in_handler",
    [prod, dev, ssr, ssr_dev]
);
compiler_case!(
    state_call_in_nested_block_in_handler,
    "template_runes/state_call_in_nested_block_in_handler",
    [prod, dev, ssr, ssr_dev]
);
