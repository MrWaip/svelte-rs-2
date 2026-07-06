mod effect;
mod model;
mod props;
mod state;

use oxc_ast_visit::VisitMut;
use svelte_ast_builder::Builder;

use model::ServerTransform;

pub fn transform_component<'a>(
    ctx: &mut svelte_types::CompileContext<'a, '_>,
    _options: &svelte_types::TransformOptions,
) {
    let b = Builder::new(ctx.alloc);
    let mut transform = ServerTransform {
        b: &b,
        analysis: ctx.analysis,
    };
    if let Some(program) = ctx.js_arena.program.as_mut() {
        transform.visit_program(program);
    }
    for expression in ctx.js_arena.iter_exprs_mut() {
        transform.visit_expression(expression);
    }
}
