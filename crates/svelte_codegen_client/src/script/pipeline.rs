use oxc_allocator::Allocator;
use oxc_ast::Comment;
use oxc_ast::ast::{Program, Statement};
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan, SourceType};
use svelte_analyze::{AnalysisData, ComponentScoping, ScriptRuneCalls};
use svelte_ast::ScriptLanguage;

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

pub fn gen_script<'a>(ctx: &mut Ctx<'a>, dev: bool) -> ScriptOutput<'a> {
    if ctx.query.component.instance_script.is_none() {
        return ScriptOutput {
            imports: vec![],
            body: vec![],
            has_tracing: false,
            needs_ownership_validator: false,
            comments: vec![],
            source_text: "",
            program_span_end: 0,
        };
    };

    let allocator = ctx.b.ast.allocator;
    let component_source = &ctx.query.component.source;
    let script_content_start = ctx
        .query
        .component
        .instance_script
        .as_ref()
        .expect("early return above when instance_script is None")
        .content_span
        .start;
    let filename = ctx.state.filename;
    let ignore_query = IgnoreQuery::new(ctx.query.analysis);

    let line_index = ctx.state.line_index;
    let program = ctx.state.parsed.program.take();
    if let Some(program) = program {
        let component_scoping = ctx.query.scoping();
        return run_transform(
            allocator,
            program,
            Some(ctx.query.analysis),
            component_scoping,
            Some(ctx.script_rune_calls()),
            ctx.instance_script_node_id_offset(),
            true,
            dev,
            component_source,
            line_index,
            script_content_start,
            filename,
            ctx.query.runes(),
            ctx.query.accessors(),
            ctx.query.immutable(),
            ctx.state.experimental_async,
            ignore_query,
            false,
        );
    }

    let component_scoping = ctx.query.scoping();
    let script = ctx
        .query
        .component
        .instance_script
        .as_ref()
        .expect("early return above when instance_script is None");
    let is_ts = script.language == ScriptLanguage::TypeScript;
    let script_text = ctx.query.component.source_text(script.content_span);
    transform_script_text(
        allocator,
        script_text,
        is_ts,
        Some(ctx.query.analysis),
        component_scoping,
        true,
        dev,
        component_source,
        line_index,
        script_content_start,
        filename,
        ctx.query.runes(),
        ctx.query.accessors(),
        ctx.query.immutable(),
        None,
        0,
        ctx.state.experimental_async,
        ignore_query,
        true,
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
        None,
        0,
        false,
        dev,
        "",
        line_index,
        0,
        "(unknown)",
        true,
        false,
        false,
        false,
        IgnoreQuery::empty(),
        false,
    )
}

pub fn transform_component_module_script<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
) -> ScriptOutput<'a> {
    let empty_scoping = ComponentScoping::new_empty();
    let line_index = svelte_span::LineIndex::new(source);
    transform_script_text(
        allocator,
        source,
        is_ts,
        None,
        &empty_scoping,
        false,
        false,
        source,
        &line_index,
        0,
        "(unknown)",
        false,
        false,
        false,
        None,
        0,
        false,
        IgnoreQuery::empty(),
        true,
    )
}

pub fn transform_component_module_program<'a, 'b>(
    allocator: &'a Allocator,
    program: Program<'a>,
    analysis: Option<&'b AnalysisData<'a>>,
    component_scoping: &'b ComponentScoping<'a>,
    script_rune_calls: Option<&ScriptRuneCalls>,
    line_index: &'b svelte_span::LineIndex,
) -> ScriptOutput<'a> {
    run_transform(
        allocator,
        program,
        analysis,
        component_scoping,
        script_rune_calls,
        0,
        false,
        false,
        "",
        line_index,
        0,
        "(unknown)",
        false,
        false,
        false,
        false,
        IgnoreQuery::empty(),
        false,
    )
}

fn transform_script_text<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    analysis: Option<&'_ AnalysisData<'a>>,
    component_scoping: &ComponentScoping<'a>,
    strip_exports: bool,
    dev: bool,
    component_source: &str,
    component_line_index: &svelte_span::LineIndex,
    script_content_start: u32,
    filename: &str,
    runes: bool,
    accessors: bool,
    immutable: bool,
    script_rune_calls: Option<&ScriptRuneCalls>,
    script_node_id_offset: u32,
    experimental_async: bool,
    ignore_query: IgnoreQuery<'_, 'a>,
    prepare_semantic: bool,
) -> ScriptOutput<'a> {
    let src_type = if is_ts {
        SourceType::default()
            .with_typescript(true)
            .with_module(true)
    } else {
        SourceType::mjs()
    };
    let result = OxcParser::new(allocator, source, src_type).parse();
    let program = result.program;

    run_transform(
        allocator,
        program,
        analysis,
        component_scoping,
        script_rune_calls,
        script_node_id_offset,
        strip_exports,
        dev,
        component_source,
        component_line_index,
        script_content_start,
        filename,
        runes,
        accessors,
        immutable,
        experimental_async,
        ignore_query,
        prepare_semantic,
    )
}

fn run_transform<'a>(
    allocator: &'a Allocator,
    mut program: Program<'a>,
    analysis: Option<&'_ AnalysisData<'a>>,
    component_scoping: &ComponentScoping<'a>,
    script_rune_calls: Option<&ScriptRuneCalls>,
    script_node_id_offset: u32,
    strip_exports: bool,
    dev: bool,
    component_source: &str,
    component_line_index: &svelte_span::LineIndex,
    script_content_start: u32,
    filename: &str,
    runes: bool,
    accessors: bool,
    immutable: bool,
    experimental_async: bool,
    ignore_query: IgnoreQuery<'_, 'a>,
    prepare_semantic: bool,
) -> ScriptOutput<'a> {
    let b = Builder::new(allocator);
    let is_ts = program.source_type.is_typescript();

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
        script_content_start,
        filename,
        runes,
        accessors,
        immutable,
        experimental_async,
        ignore_query,
        prepare_semantic,
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
