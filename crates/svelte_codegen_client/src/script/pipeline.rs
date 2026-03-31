use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Program, Statement};
use oxc_ast::Comment;
use oxc_parser::Parser as OxcParser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::{GetSpan, SourceType};
use oxc_traverse::traverse_mut;
use rustc_hash::{FxHashMap, FxHashSet};
use svelte_analyze::{ComponentScoping, PropsAnalysis, RuneKind};
use svelte_ast::ScriptLanguage;

use crate::builder::Builder;
use crate::context::Ctx;

use super::{PropsGenInfo, ScriptTransformer};

pub struct ScriptOutput<'a> {
    pub imports: Vec<Statement<'a>>,
    pub body: Vec<Statement<'a>>,
    pub has_tracing: bool,
    pub comments: Vec<Comment>,
    pub source_text: &'a str,
    pub program_span_end: u32,
}

pub fn gen_script<'a>(ctx: &mut Ctx<'a>, dev: bool) -> ScriptOutput<'a> {
    if ctx.query.component.script.is_none() {
        return ScriptOutput {
            imports: vec![],
            body: vec![],
            has_tracing: false,
            comments: vec![],
            source_text: "",
            program_span_end: 0,
        };
    };

    let allocator = ctx.b.ast.allocator;
    let component_source = &ctx.query.component.source;
    let script_content_start = ctx.query.component.script.as_ref().unwrap().content_span.start;
    let filename = ctx.state.filename;

    let program = ctx.state.parsed.program.take();
    if let Some(program) = program {
        let component_scoping = ctx.query.scoping();
        let props = ctx.query.props();
        let b = Builder::new(allocator);
        let prop_defaults: Vec<Option<Expression<'a>>> = props
            .map(|pa| {
                pa.props
                    .iter()
                    .map(|p| p.default_text.as_deref().map(|text| b.parse_expression(text)))
                    .collect()
            })
            .unwrap_or_default();
        return transform_program(
            allocator,
            program,
            component_scoping,
            props,
            prop_defaults,
            Some(ctx.script_rune_call_kinds()),
            dev,
            component_source,
            script_content_start,
            filename,
        );
    }

    let component_scoping = ctx.query.scoping();
    let props = ctx.query.props();
    let script = ctx.query.component.script.as_ref().unwrap();
    let is_ts = script.language == ScriptLanguage::TypeScript;
    let script_text = ctx.query.component.source_text(script.content_span);
    transform_script_text(
        allocator,
        script_text,
        is_ts,
        component_scoping,
        props,
        true,
        dev,
        component_source,
        script_content_start,
        filename,
        None,
    )
}

pub fn transform_module_script<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    component_scoping: &ComponentScoping,
) -> ScriptOutput<'a> {
    transform_script_text(
        allocator,
        source,
        is_ts,
        component_scoping,
        None,
        false,
        false,
        source,
        0,
        "(unknown)",
        None,
    )
}

fn transform_script_text<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    is_ts: bool,
    component_scoping: &ComponentScoping,
    props: Option<&PropsAnalysis>,
    strip_exports: bool,
    dev: bool,
    component_source: &str,
    script_content_start: u32,
    filename: &str,
    script_rune_call_kinds: Option<&FxHashMap<u32, RuneKind>>,
) -> ScriptOutput<'a> {
    let src_type = if is_ts {
        SourceType::default().with_typescript(true).with_module(true)
    } else {
        SourceType::mjs()
    };
    let result = OxcParser::new(allocator, source, src_type).parse();

    let b = Builder::new(allocator);
    let mut program = result.program;
    let sem = SemanticBuilder::new().build(&program);
    let scoping = sem.semantic.into_scoping();

    let props_gen = props.map(PropsGenInfo::from_analysis);

    let prop_default_exprs: Vec<Option<Expression<'a>>> = match &props_gen {
        Some(pg) => pg
            .props
            .iter()
            .map(|prop| prop.default_text.as_deref().map(|text| b.parse_expression(text)))
            .collect(),
        None => Vec::new(),
    };

    let mut transformer = ScriptTransformer {
        b: &b,
        component_scoping,
        scoping,
        props_gen,
        derived_pending: FxHashSet::default(),
        strip_exports,
        dev,
        is_ts,
        function_info_stack: Vec::new(),
        has_tracing: false,
        component_source,
        script_content_start,
        filename,
        next_arrow_name: None,
        ident_counter: 0,
        class_state_stack: Vec::new(),
        prop_default_exprs,
        script_rune_call_kinds,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_scoping, ());

    if !transformer.derived_pending.is_empty() {
        super::traverse::wrap_derived_thunks(&b, &mut program, &transformer.derived_pending);
    }

    let has_tracing = transformer.has_tracing;

    if is_ts {
        reattach_orphaned_comments(&mut program);
    }

    let comments: Vec<Comment> = program.comments.iter().copied().collect();
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

    ScriptOutput {
        imports,
        body,
        has_tracing,
        comments,
        source_text,
        program_span_end,
    }
}

fn transform_program<'a>(
    allocator: &'a Allocator,
    mut program: Program<'a>,
    component_scoping: &ComponentScoping,
    props: Option<&PropsAnalysis>,
    prop_default_exprs: Vec<Option<Expression<'a>>>,
    script_rune_call_kinds: Option<&FxHashMap<u32, RuneKind>>,
    dev: bool,
    component_source: &str,
    script_content_start: u32,
    filename: &str,
) -> ScriptOutput<'a> {
    let b = Builder::new(allocator);
    let is_ts = program.source_type.is_typescript();
    let sem = SemanticBuilder::new().build(&program);
    let scoping = sem.semantic.into_scoping();
    let props_gen = props.map(PropsGenInfo::from_analysis);

    let mut transformer = ScriptTransformer {
        b: &b,
        component_scoping,
        scoping,
        props_gen,
        derived_pending: FxHashSet::default(),
        strip_exports: true,
        dev,
        is_ts,
        function_info_stack: Vec::new(),
        has_tracing: false,
        component_source,
        script_content_start,
        filename,
        next_arrow_name: None,
        ident_counter: 0,
        class_state_stack: Vec::new(),
        prop_default_exprs,
        script_rune_call_kinds,
    };

    let empty_scoping = Scoping::default();
    traverse_mut(&mut transformer, allocator, &mut program, empty_scoping, ());

    if !transformer.derived_pending.is_empty() {
        super::traverse::wrap_derived_thunks(&b, &mut program, &transformer.derived_pending);
    }

    let has_tracing = transformer.has_tracing;

    if is_ts {
        reattach_orphaned_comments(&mut program);
    }

    let comments: Vec<Comment> = program.comments.iter().copied().collect();
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

    ScriptOutput {
        imports,
        body,
        has_tracing,
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
