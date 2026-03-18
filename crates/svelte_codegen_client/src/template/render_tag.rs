//! RenderTag codegen — `{@render snippet(args)}`

use oxc_ast::ast::{Expression, Statement};
use oxc_span::GetSpan;

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
    // Extract callee name from original source using the stored expression's callee span
    let tag = ctx.render_tag(id);
    let full_source = ctx.component.source_text(tag.expression_span);

    // Pre-computed per-argument has_call flags (from original source, before transform)
    let arg_has_call = ctx.analysis.render_tag_arg_has_call(id)
        .map(|v| v.to_vec())
        .unwrap_or_default();

    // Take ownership from ParsedExprs
    let expr = ctx.parsed.exprs.remove(&id)
        .expect("render tag expression should be pre-parsed");

    let Expression::CallExpression(call) = expr else {
        debug_assert!(false, "render tag expression should be a CallExpression");
        return;
    };

    // Get callee text from original source (spans are 0-based relative to expression source)
    let callee_span = call.callee.span();
    let callee_text = &full_source[callee_span.start as usize..callee_span.end as usize];

    let mut call_args: Vec<Arg<'a, '_>> = vec![Arg::Expr(anchor_expr)];
    let mut memo_counter: u32 = 0;
    let mut memo_stmts: Vec<Statement<'a>> = Vec::new();

    // Wrap each argument in a thunk () => arg
    for (i, arg) in call.unbox().arguments.into_iter().enumerate() {
        let arg_expr = arg.into_expression();

        // After transform, prop sources become `name()` (CallExpression with no args).
        // Detect this pattern and pass the getter identifier directly without a thunk.
        let prop_getter_name = match &arg_expr {
            Expression::CallExpression(inner_call) if inner_call.arguments.is_empty() => {
                if let Expression::Identifier(ident) = &inner_call.callee {
                    let name = ident.name.as_str();
                    let root = ctx.analysis.scoping.root_scope_id();
                    let is_prop_source = ctx.analysis.scoping.find_binding(root, name)
                        .is_some_and(|s| ctx.analysis.scoping.is_prop_source(s));
                    if is_prop_source {
                        Some(name.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(name) = &prop_getter_name {
            call_args.push(Arg::Expr(ctx.b.rid_expr(name)));
            continue;
        }

        let has_call = arg_has_call.get(i).copied().unwrap_or(false);

        if has_call {
            let memo_name = format!("${memo_counter}");
            memo_counter += 1;
            let thunk = ctx.b.thunk(arg_expr);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            memo_stmts.push(ctx.b.let_init_stmt(&memo_name, derived));
            let memo_ref = ctx.b.alloc_str(&memo_name);
            let get = ctx.b.call_expr("$.get", [Arg::Ident(memo_ref)]);
            let thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(get)]);
            call_args.push(Arg::Expr(thunk));
        } else {
            let thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(arg_expr)]);
            call_args.push(Arg::Expr(thunk));
        }
    }

    let call_stmt = ctx.b.call_stmt(callee_text, call_args);

    if memo_stmts.is_empty() {
        stmts.push(call_stmt);
    } else {
        memo_stmts.push(call_stmt);
        stmts.push(ctx.b.block_stmt(memo_stmts));
    }
}
