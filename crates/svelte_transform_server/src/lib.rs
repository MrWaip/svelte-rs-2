mod async_;
mod derived;
mod effect;
mod model;
mod props;
mod state;

use oxc_ast::ast::Statement;
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
        ident_gen: ctx.ident_gen,
        fn_depth: 0,
    };
    if let Some(program) = ctx.js_arena.program.as_mut() {
        transform.visit_program(program);

        if transform.analysis.blocker_data().has_async() {
            let mut imports: Vec<Statement<'a>> = Vec::new();
            let mut rest: Vec<Statement<'a>> = Vec::new();
            for stmt in program.body.drain(..) {
                if matches!(stmt, Statement::ImportDeclaration(_)) {
                    imports.push(stmt);
                } else {
                    rest.push(stmt);
                }
            }
            let split = transform.split_async_instance_body(rest);
            program.body.extend(imports);
            program.body.extend(split);
        }
    }
    for expression in ctx.js_arena.iter_exprs_mut() {
        transform.visit_expression(expression);
    }
}
