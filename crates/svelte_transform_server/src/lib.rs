mod async_;
mod class_field;
mod derived;
mod destructure;
mod effect;
mod model;
mod props;
mod runtime_rune;
mod state;
mod store;
mod store_destructure;

use oxc_allocator::Allocator;
use oxc_ast::ast::{Program, Statement};
use oxc_ast_visit::VisitMut;
use svelte_analyze::{AnalysisData, IdentGen};
use svelte_ast_builder::Builder;

use model::ServerTransform;

fn is_dollar_labeled_statement(stmt: &Statement<'_>) -> bool {
    matches!(stmt, Statement::LabeledStatement(labeled) if labeled.label.name == "$")
}

pub fn transform_component<'a>(
    ctx: &mut svelte_types::CompileContext<'a, '_>,
    options: &svelte_types::TransformOptions,
) {
    let b = Builder::new(ctx.alloc);
    let mut transform = ServerTransform {
        b: &b,
        analysis: ctx.analysis,
        ident_gen: ctx.ident_gen,
        fn_depth: 0,
        dev: options.dev,
        strip_exports: false,
        enclosing_stmt_start: Vec::new(),
        parent_is_expr_statement: false,
    };
    if let Some(module_program) = ctx.js_arena.module_program.as_mut() {
        transform.strip_exports = false;
        transform.visit_program(module_program);
        if !ctx.analysis.reactivity.uses_runes() {
            module_program
                .body
                .retain(|stmt| !is_dollar_labeled_statement(stmt));
        }
    }
    transform.strip_exports = true;
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
    for statement in ctx.js_arena.iter_stmts_mut() {
        transform.visit_statement(statement);
    }
    for expression in ctx.js_arena.iter_exprs_mut() {
        transform.visit_expression(expression);
    }
}

pub fn transform_module<'a>(
    alloc: &'a Allocator,
    program: &mut Program<'a>,
    analysis: &AnalysisData<'a>,
    ident_gen: &mut IdentGen,
    options: &svelte_types::TransformOptions,
) {
    let b = Builder::new(alloc);
    let mut transform = ServerTransform {
        b: &b,
        analysis,
        ident_gen,
        fn_depth: 0,
        dev: options.dev,
        strip_exports: false,
        enclosing_stmt_start: Vec::new(),
        parent_is_expr_statement: false,
    };
    transform.visit_program(program);
}
