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
    let name = block.name(ctx.source).to_string();

    let param_names: Vec<String> = extract_snippet_param_names(ctx, block.expression_span.start);

    // Set snippet params so expression codegen wraps them as thunk calls.
    // Save and restore to handle nested snippets correctly.
    let saved_params = std::mem::replace(&mut ctx.snippet_param_names, param_names.clone());

    let body_stmts = gen_fragment(ctx, FragmentKey::SnippetBody(id));

    ctx.snippet_param_names = saved_params;

    let params = build_snippet_params(ctx, &param_names);

    let mut all_stmts = prepend_stmts;
    all_stmts.extend(body_stmts);
    let arrow = ctx.b.arrow(params, all_stmts);

    ctx.b.const_stmt(&name, oxc_ast::ast::Expression::ArrowFunctionExpression(ctx.b.alloc(arrow)))
}

/// Extract param names from parsed `const name = (a, b) => {}` arrow.
fn extract_snippet_param_names(ctx: &Ctx<'_>, offset: u32) -> Vec<String> {
    use oxc_ast::ast::{Expression, Statement};
    ctx.parsed.stmts.get(&offset)
        .and_then(|stmt| match stmt {
            Statement::VariableDeclaration(decl) => decl.declarations.first(),
            _ => None,
        })
        .and_then(|d| match d.init.as_ref()? {
            Expression::ArrowFunctionExpression(arrow) => Some(
                arrow.params.items.iter()
                    .filter_map(|p| p.pattern.get_binding_identifier())
                    .map(|id| id.name.to_string())
                    .collect(),
            ),
            _ => None,
        })
        .unwrap_or_default()
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
