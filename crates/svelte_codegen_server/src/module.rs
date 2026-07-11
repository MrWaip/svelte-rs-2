use oxc_allocator::Allocator;
use oxc_ast::ast::{Program, Statement};
use oxc_codegen::Codegen;
use svelte_ast_builder::Builder;
use svelte_sourcemap::JsOutput;

pub fn generate_module<'a>(alloc: &'a Allocator, program: Program<'a>) -> JsOutput {
    let b = Builder::new(alloc);

    let mut program_body: Vec<Statement<'a>> = Vec::new();
    program_body.push(b.import_all("$", "svelte/internal/server"));
    program_body.extend(program.body);

    let comments: Vec<oxc_ast::Comment> = program.comments.iter().copied().collect();
    let source = program.source_text;
    let program = b.program(program_body, comments, source, source.len() as u32);

    JsOutput {
        code: Codegen::default().build(&program).code,
        map: None,
    }
}
