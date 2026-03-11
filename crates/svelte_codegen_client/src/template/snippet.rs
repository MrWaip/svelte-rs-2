//! SnippetBlock codegen — `{#snippet name(params)}...{/snippet}`

use oxc_ast::ast::Statement;

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::context::Ctx;

use super::gen_fragment;

/// Generate a `const name = ($$anchor, param1 = $.noop, ...) => { ... }` statement.
pub(crate) fn gen_snippet_block<'a>(ctx: &mut Ctx<'a>, id: NodeId) -> Statement<'a> {
    let block = ctx.snippet_block(id);
    let name = block.name.clone();

    let param_names: Vec<String> = ctx
        .analysis
        .snippet_params
        .get(&id)
        .cloned()
        .unwrap_or_default();

    // Set snippet params so expression codegen wraps them as thunk calls
    ctx.snippet_param_names = param_names.clone();

    let body_stmts = gen_fragment(ctx, FragmentKey::SnippetBody(id));

    // Clear snippet params
    ctx.snippet_param_names.clear();

    let params = build_snippet_params(ctx, &param_names);
    let arrow = ctx.b.arrow(params, body_stmts);

    ctx.b.const_stmt(&name, oxc_ast::ast::Expression::ArrowFunctionExpression(ctx.b.alloc(arrow)))
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
