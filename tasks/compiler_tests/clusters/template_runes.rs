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
    guard_effect_tracking_in_script,
    "template_runes/guard_effect_tracking_in_script"
);
compiler_case!(
    guard_state_call_in_script,
    "template_runes/guard_state_call_in_script"
);

compiler_case!(
    state_call_in_handler,
    "template_runes/state_call_in_handler",
    ignore = "$state в template-выражении — отдельная проработка"
);
