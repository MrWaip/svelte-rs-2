mod effect;
mod model;
mod state;

use oxc_ast_visit::VisitMut;
use svelte_ast_builder::Builder;

use model::ServerTransform;

pub fn transform_component<'a>(
    ctx: &mut svelte_types::CompileContext<'a, '_>,
    _options: &svelte_types::TransformOptions,
) {
    let Some(program) = ctx.js_arena.program.as_mut() else {
        return;
    };
    let b = Builder::new(ctx.alloc);
    let mut transform = ServerTransform {
        b: &b,
        analysis: ctx.analysis,
    };
    transform.visit_program(program);
}
