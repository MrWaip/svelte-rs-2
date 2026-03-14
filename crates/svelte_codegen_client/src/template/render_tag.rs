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

    // Wrap each argument in a thunk () => arg
    for arg in call.unbox().arguments.into_iter() {
        let arg_expr = arg.into_expression();

        // After transform, prop sources become `name()` (CallExpression with no args).
        // Detect this pattern and pass the getter identifier directly without a thunk.
        let prop_getter_name = match &arg_expr {
            Expression::CallExpression(inner_call) if inner_call.arguments.is_empty() => {
                if let Expression::Identifier(ident) = &inner_call.callee {
                    let name = ident.name.as_str();
                    if ctx.prop_sources.contains(name) {
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

        let thunk = ctx.b.arrow_expr(ctx.b.no_params(), [ctx.b.expr_stmt(arg_expr)]);
        call_args.push(Arg::Expr(thunk));
    }

    stmts.push(ctx.b.call_stmt(callee_text, call_args));
}
