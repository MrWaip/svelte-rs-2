use oxc_allocator::Allocator;
use oxc_ast::Comment;
use oxc_ast::ast::{Program, Statement};
use svelte_analyze::{AnalysisData, ComponentScoping, IdentGen};

use svelte_ast_builder::Builder;
use svelte_transform_client::{IgnoreQuery, RestExcludes, transform_script};

use crate::context::Ctx;

pub struct ScriptOutput<'a> {
    pub imports: Vec<Statement<'a>>,
    pub body: Vec<Statement<'a>>,
    pub has_tracing: bool,
    pub needs_ownership_validator: bool,
    pub comments: Vec<Comment>,
    pub source_text: &'a str,
    pub program_span_end: u32,
    pub rest_excludes: Vec<RestExcludes>,
}

fn empty_script_output<'a>() -> ScriptOutput<'a> {
    ScriptOutput {
        imports: vec![],
        body: vec![],
        has_tracing: false,
        needs_ownership_validator: false,
        comments: vec![],
        source_text: "",
        program_span_end: 0,
        rest_excludes: vec![],
    }
}

pub fn gen_script<'a>(ctx: &mut Ctx<'a>, dev: bool) -> ScriptOutput<'a> {
    if ctx.query.component.instance_script.is_none() {
        return empty_script_output();
    }

    let Some(program) = ctx.state.parsed.program.take() else {
        return empty_script_output();
    };

    let allocator = ctx.b.ast.allocator;
    let component_source = &ctx.query.component.source;
    let filename = ctx.state.filename;
    let ignore_query = IgnoreQuery::new(ctx.query.analysis);
    let line_index = ctx.state.line_index;
    let component_scoping = ctx.query.scoping();
    let runes = ctx.query.runes();
    let accessors = ctx.query.accessors();
    let immutable = ctx.query.immutable();
    let experimental_async = ctx.state.experimental_async;
    let analysis = ctx.query.analysis;
    let ident_gen: &mut IdentGen = ctx.state.ident_gen;

    run_transform(
        allocator,
        program,
        Some(analysis),
        component_scoping,
        ident_gen,
        true,
        dev,
        component_source,
        line_index,
        filename,
        runes,
        accessors,
        immutable,
        experimental_async,
        ignore_query,
        true,
    )
}

pub fn transform_module_program<'a, 'b>(
    allocator: &'a Allocator,
    program: Program<'a>,
    analysis: Option<&'b AnalysisData<'a>>,
    component_scoping: &'b ComponentScoping<'a>,
    ident_gen: &'b mut IdentGen,
    line_index: &'b svelte_span::LineIndex,
    dev: bool,
) -> ScriptOutput<'a> {
    run_transform(
        allocator,
        program,
        analysis,
        component_scoping,
        ident_gen,
        false,
        dev,
        "",
        line_index,
        "(unknown)",
        true,
        false,
        false,
        false,
        IgnoreQuery::empty(),
        false,
    )
}

pub fn transform_component_module_program<'a, 'b>(
    allocator: &'a Allocator,
    program: Program<'a>,
    analysis: Option<&'b AnalysisData<'a>>,
    component_scoping: &'b ComponentScoping<'a>,
    ident_gen: &'b mut IdentGen,
    line_index: &'b svelte_span::LineIndex,
    dev: bool,
) -> ScriptOutput<'a> {
    run_transform(
        allocator,
        program,
        analysis,
        component_scoping,
        ident_gen,
        false,
        dev,
        "",
        line_index,
        "(unknown)",
        false,
        false,
        false,
        false,
        IgnoreQuery::empty(),
        true,
    )
}

fn run_transform<'a, 'b>(
    allocator: &'a Allocator,
    mut program: Program<'a>,
    analysis: Option<&'b AnalysisData<'a>>,
    component_scoping: &'b ComponentScoping<'a>,
    ident_gen: &'b mut IdentGen,
    strip_exports: bool,
    dev: bool,
    component_source: &str,
    component_line_index: &svelte_span::LineIndex,
    filename: &str,
    runes: bool,
    accessors: bool,
    immutable: bool,
    experimental_async: bool,
    ignore_query: IgnoreQuery<'_, 'a>,
    partition_imports: bool,
) -> ScriptOutput<'a> {
    let b = Builder::new(allocator);

    let out = transform_script(
        allocator,
        &mut program,
        &b,
        analysis,
        component_scoping,
        ident_gen,
        strip_exports,
        dev,
        component_source,
        component_line_index,
        filename,
        runes,
        accessors,
        immutable,
        experimental_async,
        ignore_query,
    );

    let source_text = program.source_text;
    let program_span_end = program.span.end;

    let mut imports = vec![];
    let mut body = vec![];
    if partition_imports {
        for stmt in program.body {
            match &stmt {
                Statement::ImportDeclaration(_) => imports.push(stmt),
                _ => body.push(stmt),
            }
        }
    } else {
        body.extend(program.body);
    }

    let comments: Vec<Comment> = if imports.is_empty() && body.is_empty() {
        vec![]
    } else {
        program.comments.iter().copied().collect()
    };

    ScriptOutput {
        imports,
        body,
        has_tracing: out.has_tracing,
        needs_ownership_validator: out.needs_ownership_validator,
        comments,
        source_text,
        program_span_end,
        rest_excludes: out.rest_excludes,
    }
}
