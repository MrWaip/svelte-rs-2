//! SnippetBlock codegen — `{#snippet name(params)}...{/snippet}`

use oxc_ast::ast::Statement;

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::context::Ctx;

use super::gen_fragment;

/// Generate a `const name = ($$anchor, param1 = $.noop, ...) => { ... }` statement.
///
/// `prepend_stmts` are injected before the fragment body (e.g., duplicated @const tags
/// from boundary parents).
pub(crate) fn gen_snippet_block<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    prepend_stmts: Vec<Statement<'a>>,
) -> Statement<'a> {
    let block = ctx.snippet_block(id);
    let name = block.name(ctx.state.source).to_string();

    let param_names: Vec<String> = ctx.snippet_params(id).to_vec();

    // Set snippet params so expression codegen wraps them as thunk calls.
    // Save and restore to handle nested snippets correctly.
    let saved_params = std::mem::replace(&mut ctx.state.snippet_param_names, param_names.clone());

    let body_stmts = gen_fragment(ctx, FragmentKey::SnippetBody(id));

    ctx.state.snippet_param_names = saved_params;

    let params = build_snippet_params(ctx, &param_names);

    let mut all_stmts = prepend_stmts;
    if ctx.state.dev {
        // $.validate_snippet_args(...arguments)
        let args_id = ctx.b.rid_expr("arguments");
        let validate_stmt = ctx.b.call_stmt("$.validate_snippet_args", [
            crate::builder::Arg::Spread(args_id),
        ]);
        all_stmts.push(validate_stmt);
    }
    all_stmts.extend(body_stmts);

    let snippet_expr = if ctx.state.dev {
        let fn_expr = ctx.b.function_expr(params, all_stmts);
        let component_name = ctx.b.rid_expr(ctx.state.name);
        ctx.b.call_expr("$.wrap_snippet", [
            crate::builder::Arg::Expr(component_name),
            crate::builder::Arg::Expr(fn_expr),
        ])
    } else {
        let arrow = ctx.b.arrow(params, all_stmts);
        oxc_ast::ast::Expression::ArrowFunctionExpression(ctx.b.alloc(arrow))
    };

    ctx.b.const_stmt(&name, snippet_expr)
}

/// Build FormalParameters: `($$anchor, name = $.noop, ...)`
fn build_snippet_params<'a>(
    ctx: &Ctx<'a>,
    param_names: &[String],
) -> oxc_ast::ast::FormalParameters<'a> {
    use oxc_ast::ast;
    use oxc_span::SPAN;

    let b = &ctx.b;
    let mut params = Vec::new();

    // $$anchor (no default)
    let anchor_pattern = b.ast.binding_pattern_binding_identifier(SPAN, b.ast.atom("$$anchor"));
    params.push(b.ast.formal_parameter(
        SPAN, b.ast.vec(), anchor_pattern,
        oxc_ast::NONE, oxc_ast::NONE, false, None, false, false,
    ));

    // Each snippet param: name = $.noop
    for name in param_names {
        let default_expr = b.static_member_expr(b.rid_expr("$"), "noop");
        let inner_pattern = b.ast.binding_pattern_binding_identifier(SPAN, b.ast.atom(name.as_str()));
        let pattern = b.ast.binding_pattern_assignment_pattern(SPAN, inner_pattern, default_expr);
        params.push(b.ast.formal_parameter(
            SPAN, b.ast.vec(), pattern,
            oxc_ast::NONE, oxc_ast::NONE, false, None, false, false,
        ));
    }

    b.ast.formal_parameters(
        SPAN,
        ast::FormalParameterKind::ArrowFormalParameters,
        b.ast.vec_from_iter(params),
        oxc_ast::NONE,
    )
}
