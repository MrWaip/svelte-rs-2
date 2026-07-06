mod attribute;
mod element;
mod error;
mod escape;
mod fragment;
mod legacy_props;
mod legacy_reactive;
mod model;
mod program;
mod renderer;
mod text;

use svelte_sourcemap::JsOutput;

pub use error::CodegenError;

pub fn generate<'a>(
    compile_ctx: svelte_types::CompileContext<'a, 'a>,
    options: &svelte_types::CodegenOptions,
) -> Result<JsOutput, CodegenError> {
    model::ServerCodegen::new(compile_ctx, options).generate()
}
