mod attribute;
mod await_block;
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
mod module;
mod program;
mod render;
mod renderer;
mod slot;
mod snippet;
mod svelte_boundary;
mod svelte_element;
mod svelte_head;
mod text;

use svelte_sourcemap::JsOutput;

pub use error::CodegenError;
pub use module::generate_module;

pub fn generate<'a>(
    compile_ctx: svelte_types::CompileContext<'a, 'a>,
    options: &svelte_types::CodegenOptions,
    injected_css_text: Option<&str>,
) -> Result<JsOutput, CodegenError> {
    model::ServerCodegen::new(compile_ctx, options, injected_css_text).generate()
}
