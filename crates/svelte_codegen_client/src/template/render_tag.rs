//! RenderTag codegen — `{@render snippet(args)}`

use std::collections::{HashMap, HashSet};

use oxc_ast::ast::{Expression, Statement};

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

    let (callee_name, arg_sources) = extract_call_parts(source);

    let mut call_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor_expr)];

    // Build prop sets once for all arguments
    let (prop_sources, prop_non_sources) = super::expression::build_prop_sets(ctx);

    for arg_src in &arg_sources {
        let thunk = build_thunked_arg(ctx, arg_src, &prop_sources, &prop_non_sources);
        call_args.push(Arg::Expr(thunk));
    }

    stmts.push(ctx.b.call_stmt(callee_name, call_args));
}

fn extract_call_parts(source: &str) -> (&str, Vec<&str>) {
    let paren_pos = match source.find('(') {
        Some(pos) => pos,
        None => return (source.trim(), vec![]),
    };

    let callee = source[..paren_pos].trim();
    let inner = &source[paren_pos + 1..source.len() - 1];
    if inner.trim().is_empty() {
        return (callee, vec![]);
    }

    let args = split_args(inner);
    (callee, args)
}

fn split_args(s: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut depth = 0u32;
    let mut start = 0;

    for (i, ch) in s.char_indices() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                let arg = s[start..i].trim();
                if !arg.is_empty() {
                    args.push(arg);
                }
                start = i + 1;
            }
            _ => {}
        }
    }

    let last = s[start..].trim();
    if !last.is_empty() {
        args.push(last);
    }

    args
}

/// Build `() => expr` where expr has rune transforms applied.
fn build_thunked_arg<'a>(
    ctx: &mut Ctx<'a>,
    source: &str,
    prop_sources: &HashSet<String>,
    prop_non_sources: &HashMap<String, String>,
) -> Expression<'a> {
    let alloc = ctx.b.ast.allocator;
    let arena_source: &'a str = alloc.alloc_str(source);

    let mutated = &ctx.analysis.mutated_runes;
    let rune_names = &ctx.analysis.rune_names;

    let expr = super::expression::parse_and_transform(
        alloc,
        arena_source,
        mutated,
        rune_names,
        prop_sources,
        prop_non_sources,
        &[],
    );

    let params = ctx.b.no_params();
    ctx.b.arrow_expr(params, [ctx.b.expr_stmt(expr)])
}
