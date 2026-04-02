//! SnippetBlock codegen — `{#snippet name(params)}...{/snippet}`

use oxc_allocator::CloneIn;
use oxc_ast::ast::{
    BindingPattern, Expression, FormalParameters, PropertyKey, Statement,
};
use oxc_span::SPAN;

use svelte_analyze::FragmentKey;
use svelte_ast::NodeId;

use crate::builder::Arg;
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

    // Take the parsed snippet statement to walk formal parameters.
    // Pattern mirrors each_block.rs and const_tag.rs: take_stmt for owned access.
    let parsed_stmt = ctx
        .snippet_stmt_handle(id)
        .and_then(|h| ctx.state.parsed.take_stmt(h));

    // Build formal parameters and any inline declarations from destructured params.
    // The stmt handle must always be present — the parser sets it for every snippet block.
    let stmt = parsed_stmt.unwrap_or_else(|| {
        panic!("snippet block {:?} has no pre-parsed statement — parser invariant broken", id)
    });
    let mut declarations: Vec<Statement<'a>> = Vec::new();
    let params = build_snippet_params_from_parsed(ctx, &stmt, &mut declarations);

    let body_stmts = gen_fragment(ctx, FragmentKey::SnippetBody(id));

    let mut all_stmts = prepend_stmts;
    if ctx.state.dev {
        // $.validate_snippet_args(...arguments)
        let args_id = ctx.b.rid_expr("arguments");
        let validate_stmt = ctx.b.call_stmt("$.validate_snippet_args", [
            crate::builder::Arg::Spread(args_id),
        ]);
        all_stmts.push(validate_stmt);
    }
    // Destructuring declarations come before body statements.
    all_stmts.extend(declarations);
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

/// Build `FormalParameters` and emit declarations for each param.
///
/// For `BindingIdentifier` params: adds `name = $.noop` to formal params.
/// For `ObjectPattern`/`ArrayPattern` params: adds `$$argN` to formal params,
/// emits `let`/`var` declarations for each destructured leaf.
///
/// Does NOT hold a persistent borrow of `ctx.b` — accesses it inline so that
/// `ctx.gen_ident()` (which needs `&mut ctx`) can be called within the same scope.
fn build_snippet_params_from_parsed<'stmt, 'a: 'stmt>(
    ctx: &mut Ctx<'a>,
    stmt: &'stmt Statement<'a>,
    decls: &mut Vec<Statement<'a>>,
) -> FormalParameters<'a> {
    use oxc_ast::ast;

    let mut params: Vec<ast::FormalParameter<'a>> = Vec::new();

    // $$anchor (no default)
    let anchor_pattern = ctx.b.ast.binding_pattern_binding_identifier(SPAN, ctx.b.ast.atom("$$anchor"));
    params.push(ctx.b.ast.formal_parameter(
        SPAN, ctx.b.ast.vec(), anchor_pattern,
        oxc_ast::NONE, oxc_ast::NONE, false, None, false, false,
    ));

    // Extract the arrow's formal parameter items from `const name = (...) => {}`.
    let Some(items) = extract_arrow_param_items(stmt) else {
        return ctx.b.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::ArrowFormalParameters,
            ctx.b.ast.vec_from_iter(params),
            oxc_ast::NONE,
        );
    };

    // Process all params from the parsed source (no $$anchor there — we add it above).
    // `i` is used as the $$argN index for destructured params.
    for (i, item) in items.iter().enumerate() {
        match &item.pattern {
            BindingPattern::BindingIdentifier(id) => {
                // Simple identifier param: `name = $.noop`
                let name = ctx.b.ast.atom(id.name.as_str());
                let default_expr = ctx.b.static_member_expr(ctx.b.rid_expr("$"), "noop");
                let inner = ctx.b.ast.binding_pattern_binding_identifier(SPAN, name);
                let pattern = ctx.b.ast.binding_pattern_assignment_pattern(SPAN, inner, default_expr);
                params.push(ctx.b.ast.formal_parameter(
                    SPAN, ctx.b.ast.vec(), pattern,
                    oxc_ast::NONE, oxc_ast::NONE, false, None, false, false,
                ));
            }
            BindingPattern::ObjectPattern(obj) => {
                // Object-destructured param: `$$argN` plain formal param + inline let declarations.
                let arg_name = format!("$$arg{i}");
                let plain = ctx.b.ast.binding_pattern_binding_identifier(SPAN, ctx.b.ast.atom(&arg_name));
                params.push(ctx.b.ast.formal_parameter(
                    SPAN, ctx.b.ast.vec(), plain,
                    oxc_ast::NONE, oxc_ast::NONE, false, None, false, false,
                ));
                let alloc = ctx.b.ast.allocator;
                emit_object_destructuring(&ctx.b, alloc, &arg_name, obj, decls);
            }
            BindingPattern::ArrayPattern(arr) => {
                // Array-destructured param: `$$argN` plain formal param + inline let/var declarations.
                let arg_name = format!("$$arg{i}");
                // Generate array intermediary name before borrowing ctx.b.
                let array_name = ctx.gen_ident("$$array");
                let plain = ctx.b.ast.binding_pattern_binding_identifier(SPAN, ctx.b.ast.atom(&arg_name));
                params.push(ctx.b.ast.formal_parameter(
                    SPAN, ctx.b.ast.vec(), plain,
                    oxc_ast::NONE, oxc_ast::NONE, false, None, false, false,
                ));
                let alloc = ctx.b.ast.allocator;
                emit_array_destructuring(&ctx.b, alloc, &arg_name, arr, &array_name, decls);
            }
            // Other pattern kinds at the top level (rare) — pass through as-is.
            _ => {
                let alloc = ctx.b.ast.allocator;
                let pattern = item.pattern.clone_in(alloc);
                params.push(ctx.b.ast.formal_parameter(
                    SPAN, ctx.b.ast.vec(), pattern,
                    oxc_ast::NONE, oxc_ast::NONE, false, None, false, false,
                ));
            }
        }
    }

    ctx.b.ast.formal_parameters(
        SPAN,
        ast::FormalParameterKind::ArrowFormalParameters,
        ctx.b.ast.vec_from_iter(params),
        oxc_ast::NONE,
    )
}

/// Extract the param items slice from `const name = (...) => {}`.
/// The returned slice borrows from `stmt` — lifetime `'s` ties both together.
fn extract_arrow_param_items<'s, 'a: 's>(
    stmt: &'s Statement<'a>,
) -> Option<&'s [oxc_ast::ast::FormalParameter<'a>]> {
    if let Statement::VariableDeclaration(decl) = stmt {
        if let Some(declarator) = decl.declarations.first() {
            if let Some(Expression::ArrowFunctionExpression(arrow)) = &declarator.init {
                return Some(arrow.params.items.as_slice());
            }
        }
    }
    None
}

/// Emit `let` declarations for each property in an `ObjectPattern` param.
///
/// - Regular prop → `let name = () => $$argN?.().key`
/// - Default prop → `let name = $.derived_safe_equal(() => $.fallback($$argN?.().key, default))`
/// - Rest element → `let rest = () => $.exclude_from_object($$argN?.(), ["k1", "k2"])`
fn emit_object_destructuring<'a>(
    b: &crate::builder::Builder<'a>,
    alloc: &'a oxc_allocator::Allocator,
    arg_name: &str,
    obj: &oxc_ast::ast::ObjectPattern<'a>,
    decls: &mut Vec<Statement<'a>>,
) {
    // Collect non-rest key names for the exclusion list used by the rest element.
    let mut excluded_keys: Vec<String> = Vec::new();
    for prop in &obj.properties {
        if let PropertyKey::StaticIdentifier(id) = &prop.key {
            excluded_keys.push(id.name.as_str().to_string());
        }
    }

    for prop in &obj.properties {
        let key_name = match &prop.key {
            PropertyKey::StaticIdentifier(id) => id.name.as_str().to_string(),
            // Computed or literal keys — not expected in snippet params, skip.
            _ => continue,
        };

        match &prop.value {
            BindingPattern::BindingIdentifier(id) => {
                let name = id.name.as_str().to_string();
                // `let name = () => $$argN?.().key`
                let access = b.optional_call_member(b.rid_expr(arg_name), &key_name);
                decls.push(b.let_init_stmt(&name, b.thunk(access)));
            }
            BindingPattern::AssignmentPattern(assign) => {
                if let BindingPattern::BindingIdentifier(id) = &assign.left {
                    let name = id.name.as_str().to_string();
                    // `let name = $.derived_safe_equal(() => $.fallback($$argN?.().key, default))`
                    let access = b.optional_call_member(b.rid_expr(arg_name), &key_name);
                    let default_val = assign.right.clone_in(alloc);
                    let fallback = b.call_expr("$.fallback", [Arg::Expr(access), Arg::Expr(default_val)]);
                    let derived = b.call_expr("$.derived_safe_equal", [Arg::Expr(b.thunk(fallback))]);
                    decls.push(b.let_init_stmt(&name, derived));
                }
            }
            // Nested patterns not in current test scope — skip.
            _ => {}
        }
    }

    // Rest element: `let rest = () => $.exclude_from_object($$argN?.(), ["k1", ...])`
    if let Some(rest) = &obj.rest {
        if let BindingPattern::BindingIdentifier(id) = &rest.argument {
            let rest_name = id.name.as_str().to_string();
            let call_result = b.maybe_call_expr(b.rid_expr(arg_name), std::iter::empty::<Arg<'_, '_>>());
            let keys_array = b.array_from_args(
                excluded_keys.iter().map(|k| Arg::StrRef(k.as_str())),
            );
            let exclude = b.call_expr("$.exclude_from_object", [
                Arg::Expr(call_result),
                Arg::Expr(keys_array),
            ]);
            decls.push(b.let_init_stmt(&rest_name, b.thunk(exclude)));
        }
    }

    let _ = alloc; // used via clone_in in AssignmentPattern arms above
}

/// Emit `var`/`let` declarations for each element in an `ArrayPattern` param.
///
/// - `var $$array = $.derived(() => $.to_array($$argN?.()[, N]))` (no N when rest present)
/// - Per element → `let name = () => $.get($$array)[j]`
/// - Rest        → `let name = () => $.get($$array).slice(j)`
fn emit_array_destructuring<'a>(
    b: &crate::builder::Builder<'a>,
    alloc: &'a oxc_allocator::Allocator,
    arg_name: &str,
    arr: &oxc_ast::ast::ArrayPattern<'a>,
    array_name: &str,
    decls: &mut Vec<Statement<'a>>,
) {
    let non_rest_count = arr.elements.len();
    let has_rest = arr.rest.is_some();

    // `var $$array = $.derived(() => $.to_array($$argN?.(). [, N]))`
    {
        let call_result = b.maybe_call_expr(b.rid_expr(arg_name), std::iter::empty::<Arg<'_, '_>>());
        let to_array = if has_rest {
            b.call_expr("$.to_array", [Arg::Expr(call_result)])
        } else {
            b.call_expr("$.to_array", [Arg::Expr(call_result), Arg::Num(non_rest_count as f64)])
        };
        let derived = b.call_expr("$.derived", [Arg::Expr(b.thunk(to_array))]);
        decls.push(b.var_stmt(array_name, derived));
    }

    // Per element: `let name = () => $.get($$array)[j]`
    for (j, elem) in arr.elements.iter().enumerate() {
        let elem = match elem {
            Some(e) => e,
            None => continue, // hole in array pattern
        };

        match elem {
            BindingPattern::BindingIdentifier(id) => {
                let name = id.name.as_str().to_string();
                let get_call = b.call_expr("$.get", [Arg::Ident(array_name)]);
                let index_access = b.computed_member_expr(get_call, b.num_expr(j as f64));
                decls.push(b.let_init_stmt(&name, b.thunk(index_access)));
            }
            BindingPattern::AssignmentPattern(assign) => {
                if let BindingPattern::BindingIdentifier(id) = &assign.left {
                    let name = id.name.as_str().to_string();
                    let get_call = b.call_expr("$.get", [Arg::Ident(array_name)]);
                    let index_access = b.computed_member_expr(get_call, b.num_expr(j as f64));
                    let default_val = assign.right.clone_in(alloc);
                    let fallback = b.call_expr("$.fallback", [Arg::Expr(index_access), Arg::Expr(default_val)]);
                    let derived = b.call_expr("$.derived_safe_equal", [Arg::Expr(b.thunk(fallback))]);
                    decls.push(b.let_init_stmt(&name, derived));
                }
            }
            _ => {}
        }
    }

    // Rest element: `let name = () => $.get($$array).slice(non_rest_count)`
    if let Some(rest) = &arr.rest {
        if let BindingPattern::BindingIdentifier(id) = &rest.argument {
            let rest_name = id.name.as_str().to_string();
            let get_call = b.call_expr("$.get", [Arg::Ident(array_name)]);
            let slice_callee = b.static_member_expr(get_call, "slice");
            let slice_call = b.call_expr_callee(slice_callee, [Arg::Num(non_rest_count as f64)]);
            decls.push(b.let_init_stmt(&rest_name, b.thunk(slice_call)));
        }
    }

    let _ = alloc; // used via clone_in in AssignmentPattern arms above
}

