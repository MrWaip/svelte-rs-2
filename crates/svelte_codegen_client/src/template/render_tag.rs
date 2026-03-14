//! RenderTag codegen — `{@render snippet(args)}`

use rustc_hash::{FxHashMap, FxHashSet};

use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, Statement};
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan as _, SourceType};

use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

/// Generate a render tag call: `snippet(anchor, () => arg1, () => arg2, ...)`
pub(crate) fn gen_render_tag<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor_expr: Expression<'a>,
    stmts: &mut Vec<Statement<'a>>,
) {
    let tag = ctx.render_tag(id);
    let source = ctx.component.source_text(tag.expression_span);

    let (callee_text, arg_texts) = extract_call_parts(source);

    let mut call_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor_expr)];

    let prop_sources = ctx.prop_sources.clone();
    let prop_non_sources = ctx.prop_non_sources.clone();

    for arg_src in &arg_texts {
        let thunk = build_thunked_arg(ctx, arg_src, &prop_sources, &prop_non_sources);
        call_args.push(Arg::Expr(thunk));
    }

    stmts.push(ctx.b.call_stmt(callee_text, call_args));
}

/// Parse the render tag expression with OXC and extract callee name + argument source texts.
fn extract_call_parts(source: &str) -> (&str, Vec<&str>) {
    let alloc = Allocator::default();
    let Ok(expr) = OxcParser::new(&alloc, source, SourceType::default()).parse_expression() else {
        debug_assert!(false, "codegen: failed to parse render tag expression: {source}");
        eprintln!("[svelte-rs] warning: failed to parse render tag expression: {source}");
        return (source.trim(), vec![]);
    };

    let Expression::CallExpression(call) = &expr else {
        return (source.trim(), vec![]);
    };

    let callee_span = call.callee.span();
    let callee_text = &source[callee_span.start as usize..callee_span.end as usize];

    let arg_texts: Vec<&str> = call
        .arguments
        .iter()
        .map(|arg| {
            let sp = arg.span();
            &source[sp.start as usize..sp.end as usize]
        })
        .collect();

    (callee_text, arg_texts)
}

/// Build `() => expr` where expr has rune transforms applied.
/// For bare prop source identifiers, returns the getter directly (no thunk).
fn build_thunked_arg<'a>(
    ctx: &mut Ctx<'a>,
    source: &str,
    prop_sources: &FxHashSet<String>,
    prop_non_sources: &FxHashMap<String, String>,
) -> Expression<'a> {
    let trimmed = source.trim();
    if prop_sources.contains(trimmed) {
        // Prop getter is already a function — pass directly without thunk
        return ctx.b.rid_expr(trimmed);
    }

    let alloc = ctx.b.ast.allocator;
    let arena_source: &'a str = alloc.alloc_str(source);

    let mutated = &ctx.analysis.mutated_runes;
    let rune_names = &ctx.analysis.rune_names;

    let each_vars = &ctx.each_vars;
    let expr = super::expression::parse_and_transform(
        alloc,
        arena_source,
        mutated,
        rune_names,
        prop_sources,
        prop_non_sources,
        &[],
        each_vars,
    );

    let params = ctx.b.no_params();
    ctx.b.arrow_expr(params, [ctx.b.expr_stmt(expr)])
}
