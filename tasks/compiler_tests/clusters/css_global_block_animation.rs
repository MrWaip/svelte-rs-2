use super::*;

compiler_case!(lone_animation, "css_global_block_animation/lone_animation");
compiler_case!(
    lone_animation_name,
    "css_global_block_animation/lone_animation_name"
);
compiler_case!(
    nested_lone_animation,
    "css_global_block_animation/nested_lone_animation"
);
compiler_case!(lone_multiple, "css_global_block_animation/lone_multiple");

compiler_case!(
    prefixed_animation_guard,
    "css_global_block_animation/prefixed_animation_guard"
);
compiler_case!(
    keyframes_inside_global_guard,
    "css_global_block_animation/keyframes_inside_global_guard"
);
compiler_case!(
    local_animation_guard,
    "css_global_block_animation/local_animation_guard"
);
