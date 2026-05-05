use oxc_allocator::Allocator;
use oxc_ast::Comment;
use oxc_ast::ast::{Program, Statement};
use oxc_span::GetSpan;
use svelte_analyze::{AnalysisData, ComponentScoping};

use svelte_ast_builder::Builder;
use svelte_transform::{IgnoreQuery, transform_script};

use crate::context::Ctx;

pub struct ScriptOutput<'a> {
    pub imports: Vec<Statement<'a>>,
    pub body: Vec<Statement<'a>>,
    pub has_tracing: bool,
    pub needs_ownership_validator: bool,
    pub comments: Vec<Comment>,
    pub source_text: &'a str,
    pub program_span_end: u32,
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

    run_transform(
        allocator,
        program,
        Some(ctx.query.analysis),
        component_scoping,
        ctx.instance_script_node_id_offset(),
        true,
        dev,
        component_source,
        line_index,
        filename,
        ctx.query.runes(),
        ctx.query.accessors(),
        ctx.query.immutable(),
        ctx.state.experimental_async,
        ignore_query,
    )
}

pub fn transform_module_program<'a, 'b>(
    allocator: &'a Allocator,
    program: Program<'a>,
    analysis: Option<&'b AnalysisData<'a>>,
    component_scoping: &'b ComponentScoping<'a>,
    line_index: &'b svelte_span::LineIndex,
    dev: bool,
) -> ScriptOutput<'a> {
    run_transform(
        allocator,
        program,
        analysis,
        component_scoping,
        0,
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
    )
}

pub fn transform_component_module_program<'a, 'b>(
    allocator: &'a Allocator,
    program: Program<'a>,
    analysis: Option<&'b AnalysisData<'a>>,
    component_scoping: &'b ComponentScoping<'a>,
    line_index: &'b svelte_span::LineIndex,
) -> ScriptOutput<'a> {
    run_transform(
        allocator,
        program,
        analysis,
        component_scoping,
        0,
        false,
        false,
        "",
        line_index,
        "(unknown)",
        false,
        false,
        false,
        false,
        IgnoreQuery::empty(),
    )
}

fn run_transform<'a>(
    allocator: &'a Allocator,
    mut program: Program<'a>,
    analysis: Option<&'_ AnalysisData<'a>>,
    component_scoping: &ComponentScoping<'a>,
    script_node_id_offset: u32,
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
) -> ScriptOutput<'a> {
    let b = Builder::new(allocator);
    let is_ts = program.source_type.is_typescript();
    let script_rune_calls = analysis.map(|a| a.script_rune_calls());

    let out = transform_script(
        allocator,
        &mut program,
        &b,
        analysis,
        component_scoping,
        script_rune_calls,
        script_node_id_offset,
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

    if is_ts {
        reattach_orphaned_comments(&mut program);
    }

    let source_text = program.source_text;
    let program_span_end = program.span.end;

    let mut imports = vec![];
    let mut body = vec![];
    for stmt in program.body {
        match &stmt {
            Statement::ImportDeclaration(_) => imports.push(stmt),
            _ => body.push(stmt),
        }
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
    }
}

fn reattach_orphaned_comments(program: &mut Program<'_>) {
    let mut stmt_starts: Vec<u32> = program.body.iter().map(|s| s.span().start).collect();
    stmt_starts.sort_unstable();

    for comment in program.comments.iter_mut() {
        if stmt_starts.binary_search(&comment.attached_to).is_ok() {
            continue;
        }
        let pos = comment.span.end;
        let next = stmt_starts.iter().find(|&&s| s >= pos).copied();
        if let Some(next_start) = next {
            comment.attached_to = next_start;
        }
    }
}
