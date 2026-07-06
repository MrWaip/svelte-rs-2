mod attribute;
mod component;
mod const_tag;
mod debug_tag;
mod each_block;
mod element;
mod error;
mod escape;
mod fragment;
mod html_tag;
mod if_block;
mod key_block;
mod legacy_props;
mod legacy_reactive;
mod model;
mod program;
mod render;
mod renderer;
mod snippet;
mod svelte_element;
mod svelte_head;
mod text;

use svelte_sourcemap::JsOutput;

pub use error::CodegenError;

pub fn generate<'a>(
    compile_ctx: svelte_types::CompileContext<'a, 'a>,
    options: &svelte_types::CodegenOptions,
) -> Result<JsOutput, CodegenError> {
    model::ServerCodegen::new(compile_ctx, options).generate()
}
