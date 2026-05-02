mod pipeline;

pub(crate) use svelte_transform::sanitize_location;

pub use pipeline::{
    gen_script, transform_component_module_program, transform_component_module_script,
    transform_module_program,
};
