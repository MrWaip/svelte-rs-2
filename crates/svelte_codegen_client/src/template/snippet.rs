//! SnippetBlock codegen — `{#snippet name(params)}...{/snippet}`
//!
//! Thin orchestrator in Plan → builders → emit form (matches
//! `each_block.rs` / `await_block.rs`). `gen_snippet_block` reads the
//! resolved [`SnippetBlockSemantics`], builds a plain-data `SnippetPlan`
//! once, then dispatches to focused builders for each piece of output
//! (formal parameters, destructure declarations, body, final const
//! statement).

use oxc_allocator::CloneIn;
use oxc_ast::ast::{
    AssignmentPattern, BindingPattern, ChainElement, Expression, FormalParameter, FormalParameters,
    PropertyKey, Statement,
};
use oxc_span::SPAN;
use rustc_hash::FxHashSet;
use svelte_analyze::{is_simple_expression, FragmentKey, SnippetBlockSemantics, SnippetParam};
use svelte_ast::NodeId;

/// Codegen-local classification of a destructure leaf's default. Mirrors
/// what the reference compiler's `build_fallback` branches on: no default
/// → plain thunk; simple default → inline `$.fallback`; arbitrary default
/// → lazy-thunk `$.fallback(..., true)`. Computed once when a
/// `BindingPattern::AssignmentPattern` is entered, via
/// `is_simple_expression(&assign.right)` — exactly the same check the
/// old Block Semantics payload pre-computed.
#[derive(Clone, Copy)]
enum LeafDefault {
    None,
    Constant,
    Computed,
}

impl LeafDefault {
    fn classify(right: &Expression<'_>) -> Self {
        if is_simple_expression(right) {
            Self::Constant
        } else {
            Self::Computed
        }
    }
}

use crate::context::Ctx;
use svelte_ast_builder::Arg;

use super::gen_fragment;

/// Generate `const name = ($$anchor, ...) => { ... }` (or `wrap_snippet`
/// in dev mode) for one `{#snippet}` block.
///
/// `prepend_stmts` are injected before the fragment body (e.g.,
/// duplicated `@const` tags from boundary parents). The root dispatch
/// already matched `BlockSemantics::Snippet`; this function trusts
/// `sem` and does not re-classify.
pub(crate) fn gen_snippet_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    sem: &SnippetBlockSemantics,
    prepend_stmts: Vec<Statement<'a>>,
) -> Statement<'a> {
    let plan = SnippetPlan::build(ctx, block_id, sem);

    let mut binding_decls: Vec<Statement<'a>> = Vec::new();
    let formal_params = build_formal_params(ctx, &plan, sem, &mut binding_decls);
    let body_stmts = build_body_stmts(ctx, block_id, &plan, binding_decls, prepend_stmts);

    emit_snippet_const(ctx, &plan, formal_params, body_stmts)
}

// ---------------------------------------------------------------------------
// Plan — the pre-computed, plain-data snapshot consumed by the builders.
// ---------------------------------------------------------------------------

/// Everything the output builders need to know, resolved up front from
/// `sem` and the pre-parsed snippet arrow. Builders take `&SnippetPlan`
/// and never re-inspect `sem` or the raw AST for classification.
///
/// The parsed `FormalParameter` slice is carried on the plan rather
/// than re-extracted in each builder: destructured params need the AST
/// subtree to build member paths and clone user defaults (which is
/// emission payload, not re-classification).
struct SnippetPlan<'a> {
    /// Const name of the snippet (`symbol_name(sem.name)`).
    name: String,
    /// Enclosing component identifier — used only in dev for
    /// `$.wrap_snippet(<component>, fn)`.
    component_name: &'a str,
    /// Dev mode — toggles `$.wrap_snippet` and `$.validate_snippet_args`.
    dev: bool,
    /// Fragment key for the body — passed to `gen_fragment`.
    body_key: FragmentKey,
    /// Pre-parsed `const <name> = (<params>) => {}` statement, owned by
    /// this plan. Builders read the FormalParameter slice through
    /// [`parsed_params`](Self::parsed_params) — keeping the statement
    /// on the plan avoids juggling arena-scoped references.
    parsed_stmt: Statement<'a>,
}

impl<'a> SnippetPlan<'a> {
    fn build(ctx: &mut Ctx<'a>, block_id: NodeId, sem: &SnippetBlockSemantics) -> Self {
        let name = ctx.query.view.symbol_name(sem.name).to_string();

        let parsed_stmt = ctx
            .query
            .view
            .snippet_stmt_handle(block_id)
            .and_then(|h| ctx.state.parsed.take_stmt(h));
        let parsed_stmt = parsed_stmt.unwrap_or_else(|| {
            panic!(
                "snippet block {block_id:?} has no pre-parsed statement — parser invariant broken",
            )
        });

        Self {
            name,
            component_name: ctx.state.name,
            dev: ctx.state.dev,
            body_key: FragmentKey::SnippetBody(block_id),
            parsed_stmt,
        }
    }

    /// FormalParameter slice of the pre-parsed arrow, when present.
    fn parsed_params(&self) -> Option<&[FormalParameter<'a>]> {
        let Statement::VariableDeclaration(decl) = &self.parsed_stmt else {
            return None;
        };
        let declarator = decl.declarations.first()?;
        let Some(Expression::ArrowFunctionExpression(arrow)) = &declarator.init else {
            return None;
        };
        Some(arrow.params.items.as_slice())
    }
}

// ---------------------------------------------------------------------------
// Builders.
// ---------------------------------------------------------------------------

/// Assemble the snippet's `FormalParameters` — `$$anchor` followed by
/// one entry per user parameter (identifier → `= $.noop` when no
/// default; destructure → positional `$$argN`). Destructure bindings
/// that need pre-declarations accumulate into `decls` for
/// [`build_body_stmts`] to splice in front of the body.
fn build_formal_params<'a>(
    ctx: &mut Ctx<'a>,
    plan: &SnippetPlan<'a>,
    sem: &SnippetBlockSemantics,
    decls: &mut Vec<Statement<'a>>,
) -> FormalParameters<'a> {
    use oxc_ast::ast;

    let mut params: Vec<FormalParameter<'a>> = Vec::new();
    params.push(formal_param_ident(
        ctx, "$$anchor", /*with_noop_default*/ false,
    ));

    let Some(items) = plan.parsed_params() else {
        return ctx.b.ast.formal_parameters(
            SPAN,
            ast::FormalParameterKind::ArrowFormalParameters,
            ctx.b.ast.vec_from_iter(params),
            oxc_ast::NONE,
        );
    };

    // `sem.params` and the parsed arrow's items are populated in the
    // same source order by the block_semantics builder — zip them to
    // pair each positional slot with its semantic classification.
    for (i, (item, payload_param)) in items.iter().zip(sem.params.iter()).enumerate() {
        match payload_param {
            SnippetParam::Identifier { sym } => {
                // Reference compiler always emits `= $.noop` for
                // identifier params — any user-specified default is
                // dropped at the declaration site.
                let name = ctx.query.view.symbol_name(*sym).to_string();
                params.push(formal_param_ident_with_noop(ctx, &name));
            }
            SnippetParam::Pattern { pattern_id: _ } => {
                let arg_name = format!("$$arg{i}");
                params.push(formal_param_ident(
                    ctx, &arg_name, /*with_noop_default*/ false,
                ));
                emit_destructure_bindings(ctx, &item.pattern, &arg_name, decls);
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

/// Emit the destructure-leaf declarations for a single `$$argN`-routed
/// parameter. The structural recursion follows the OXC `BindingPattern`
/// subtree; default classification is inlined at each
/// `AssignmentPattern` via `is_simple_expression` (see [`LeafDefault`])
/// — there is no separate Block Semantics cursor payload to match
/// against the AST.
fn emit_destructure_bindings<'a>(
    ctx: &mut Ctx<'a>,
    pattern: &BindingPattern<'a>,
    arg_name: &str,
    decls: &mut Vec<Statement<'a>>,
) {
    let access = ctx
        .b
        .maybe_call_expr(ctx.b.rid_expr(arg_name), std::iter::empty::<Arg<'_, '_>>());
    let mut inserts = Vec::new();
    let mut paths = Vec::new();
    collect_binding_pattern(
        ctx,
        pattern,
        access,
        LeafDefault::None,
        &mut inserts,
        &mut paths,
    );

    let array_insert_names: FxHashSet<String> =
        inserts.iter().map(|insert| insert.name.clone()).collect();

    for insert in inserts {
        let derived = ctx
            .b
            .call_expr("$.derived", [Arg::Expr(ctx.b.thunk(insert.value))]);
        decls.push(ctx.b.var_stmt(&insert.name, derived));
    }

    for mut path in paths {
        rewrite_array_reads(ctx, &mut path.expression, &array_insert_names);
        emit_leaf_binding(ctx, &path.name, path.expression, path.default, decls);
    }
}

/// Build the snippet function body: optional dev prologue
/// (`$.validate_snippet_args`), prepend statements (boundary @const
/// duplicates), destructure declarations accumulated by
/// [`build_formal_params`], then the user-authored body fragment.
fn build_body_stmts<'a>(
    ctx: &mut Ctx<'a>,
    _block_id: NodeId,
    plan: &SnippetPlan<'a>,
    binding_decls: Vec<Statement<'a>>,
    prepend_stmts: Vec<Statement<'a>>,
) -> Vec<Statement<'a>> {
    let fragment_stmts = gen_fragment(ctx, plan.body_key);

    let mut all = prepend_stmts;
    if plan.dev {
        let args_id = ctx.b.rid_expr("arguments");
        all.push(
            ctx.b
                .call_stmt("$.validate_snippet_args", [Arg::Spread(args_id)]),
        );
    }
    all.extend(binding_decls);
    all.extend(fragment_stmts);
    all
}

/// Wrap the assembled arrow in `$.wrap_snippet(component_name, fn)` in
/// dev, otherwise emit a plain arrow. Returns the final `const name = ...`
/// statement.
fn emit_snippet_const<'a>(
    ctx: &mut Ctx<'a>,
    plan: &SnippetPlan<'a>,
    params: FormalParameters<'a>,
    body_stmts: Vec<Statement<'a>>,
) -> Statement<'a> {
    let snippet_expr = if plan.dev {
        let fn_expr = ctx.b.function_expr(params, body_stmts);
        let component = ctx.b.rid_expr(plan.component_name);
        ctx.b
            .call_expr("$.wrap_snippet", [Arg::Expr(component), Arg::Expr(fn_expr)])
    } else {
        let arrow = ctx.b.arrow(params, body_stmts);
        Expression::ArrowFunctionExpression(ctx.b.alloc(arrow))
    };
    ctx.b.const_stmt(&plan.name, snippet_expr)
}

// ---------------------------------------------------------------------------
// Small AST-builder helpers for FormalParameter construction.
// ---------------------------------------------------------------------------

fn formal_param_ident<'a>(
    ctx: &Ctx<'a>,
    name: &str,
    with_noop_default: bool,
) -> FormalParameter<'a> {
    let inner = ctx
        .b
        .ast
        .binding_pattern_binding_identifier(SPAN, ctx.b.ast.atom(name));
    let pattern = if with_noop_default {
        let default_expr = ctx.b.static_member_expr(ctx.b.rid_expr("$"), "noop");
        ctx.b
            .ast
            .binding_pattern_assignment_pattern(SPAN, inner, default_expr)
    } else {
        inner
    };
    ctx.b.ast.formal_parameter(
        SPAN,
        ctx.b.ast.vec(),
        pattern,
        oxc_ast::NONE,
        oxc_ast::NONE,
        false,
        None,
        false,
        false,
    )
}

fn formal_param_ident_with_noop<'a>(ctx: &Ctx<'a>, name: &str) -> FormalParameter<'a> {
    formal_param_ident(ctx, name, true)
}

// Snippet params need the same insert-before-path ordering as the reference
// compiler's extract_paths, but we keep the helper local until the other
// destructuring sites are ready to share one abstraction.
struct SnippetInsert<'a> {
    name: String,
    value: Expression<'a>,
}

struct SnippetPath<'a> {
    name: String,
    expression: Expression<'a>,
    /// Set when the leaf's `BindingPattern` was wrapped by an
    /// `AssignmentPattern`. `Constant` → inline `$.fallback` form;
    /// `Computed` → thunk-wrapped `$.fallback(..., true)`; `None` →
    /// skip `$.fallback` entirely.
    default: LeafDefault,
}

fn collect_binding_pattern<'a>(
    ctx: &mut Ctx<'a>,
    pattern: &BindingPattern<'a>,
    expression: Expression<'a>,
    inherited_default: LeafDefault,
    inserts: &mut Vec<SnippetInsert<'a>>,
    paths: &mut Vec<SnippetPath<'a>>,
) {
    match pattern {
        BindingPattern::BindingIdentifier(id) => {
            paths.push(SnippetPath {
                name: id.name.as_str().to_string(),
                expression,
                default: inherited_default,
            });
        }
        BindingPattern::ObjectPattern(obj) => {
            collect_object_pattern(ctx, obj, expression, inherited_default, inserts, paths);
        }
        BindingPattern::ArrayPattern(arr) => {
            collect_array_pattern(ctx, arr, expression, inherited_default, inserts, paths);
        }
        BindingPattern::AssignmentPattern(assign) => {
            let leaf_default = LeafDefault::classify(&assign.right);
            let fallback = build_fallback_expr(ctx, &expression, assign, leaf_default);
            collect_binding_pattern(ctx, &assign.left, fallback, leaf_default, inserts, paths);
        }
    }
}

fn emit_leaf_binding<'a>(
    ctx: &mut Ctx<'a>,
    name: &str,
    access: Expression<'a>,
    default: LeafDefault,
    decls: &mut Vec<Statement<'a>>,
) {
    let init = if matches!(default, LeafDefault::None) {
        ctx.b.thunk(access)
    } else {
        // Both Constant and Computed defaults trigger the
        // `$.derived_safe_equal` wrap — the thunk ensures the default
        // is evaluated once, independently of whether `$.fallback`
        // inlined it or wrapped it in its own thunk.
        let thunk = ctx.b.thunk(access);
        ctx.b.call_expr("$.derived_safe_equal", [Arg::Expr(thunk)])
    };
    decls.push(ctx.b.let_init_stmt(name, init));
}

fn collect_object_pattern<'a>(
    ctx: &mut Ctx<'a>,
    obj: &oxc_ast::ast::ObjectPattern<'a>,
    expression: Expression<'a>,
    inherited_default: LeafDefault,
    inserts: &mut Vec<SnippetInsert<'a>>,
    paths: &mut Vec<SnippetPath<'a>>,
) {
    for prop in &obj.properties {
        let property_access = build_object_property_access(ctx, &expression, prop);
        collect_binding_pattern(
            ctx,
            &prop.value,
            property_access,
            inherited_default,
            inserts,
            paths,
        );
    }

    if let Some(rest) = &obj.rest {
        let rest_expr = build_object_rest_expr(ctx, &expression, obj);
        collect_binding_pattern(
            ctx,
            &rest.argument,
            rest_expr,
            inherited_default,
            inserts,
            paths,
        );
    }
}

fn collect_array_pattern<'a>(
    ctx: &mut Ctx<'a>,
    arr: &oxc_ast::ast::ArrayPattern<'a>,
    expression: Expression<'a>,
    inherited_default: LeafDefault,
    inserts: &mut Vec<SnippetInsert<'a>>,
    paths: &mut Vec<SnippetPath<'a>>,
) {
    let array_name = ctx.gen_ident("$$array");
    let to_array = if arr.rest.is_some() {
        ctx.b.call_expr("$.to_array", [Arg::Expr(expression)])
    } else {
        ctx.b.call_expr(
            "$.to_array",
            [Arg::Expr(expression), Arg::Num(arr.elements.len() as f64)],
        )
    };
    inserts.push(SnippetInsert {
        name: array_name.clone(),
        value: to_array,
    });

    for (index, element) in arr.elements.iter().enumerate() {
        let Some(pattern) = element else { continue };
        let element_access = build_computed_member_access(
            ctx,
            &ctx.b.rid_expr(&array_name),
            ctx.b.num_expr(index as f64),
        );
        collect_binding_pattern(
            ctx,
            pattern,
            element_access,
            inherited_default,
            inserts,
            paths,
        );
    }

    if let Some(rest) = &arr.rest {
        let rest_access = ctx.b.call_expr_callee(
            build_static_member_access(ctx, &ctx.b.rid_expr(&array_name), "slice"),
            [Arg::Num(arr.elements.len() as f64)],
        );
        collect_binding_pattern(
            ctx,
            &rest.argument,
            rest_access,
            inherited_default,
            inserts,
            paths,
        );
    }
}

/// Build `$.fallback(access, <default>)` — form chosen from the
/// [`LeafDefault`] classified by the caller. `Constant` inlines the
/// default; `Computed` wraps it in a thunk plus the `true` eager flag
/// (matches reference compiler `build_fallback` in
/// `reference/compiler/utils/ast.js`). `None` is unreachable — the
/// AssignmentPattern branch only enters this helper when a default is
/// present.
fn build_fallback_expr<'a>(
    ctx: &Ctx<'a>,
    access: &oxc_ast::ast::Expression<'a>,
    assign: &AssignmentPattern<'a>,
    default: LeafDefault,
) -> oxc_ast::ast::Expression<'a> {
    let default_val = assign.right.clone_in(ctx.b.ast.allocator);
    match default {
        LeafDefault::Constant => ctx.b.call_expr(
            "$.fallback",
            [Arg::Expr(ctx.b.clone_expr(access)), Arg::Expr(default_val)],
        ),
        LeafDefault::Computed => ctx.b.call_expr(
            "$.fallback",
            [
                Arg::Expr(ctx.b.clone_expr(access)),
                Arg::Expr(ctx.b.thunk(default_val)),
                Arg::Bool(true),
            ],
        ),
        LeafDefault::None => {
            unreachable!("build_fallback_expr called without a default")
        }
    }
}

fn build_object_property_access<'a>(
    ctx: &Ctx<'a>,
    access: &oxc_ast::ast::Expression<'a>,
    prop: &oxc_ast::ast::BindingProperty<'a>,
) -> oxc_ast::ast::Expression<'a> {
    if !prop.computed {
        if let PropertyKey::StaticIdentifier(id) = &prop.key {
            return build_static_member_access(ctx, access, id.name.as_str());
        }
    }

    build_computed_member_access(ctx, access, clone_property_key_expr(ctx, &prop.key))
}

fn build_object_rest_expr<'a>(
    ctx: &Ctx<'a>,
    access: &oxc_ast::ast::Expression<'a>,
    obj: &oxc_ast::ast::ObjectPattern<'a>,
) -> oxc_ast::ast::Expression<'a> {
    let excluded = ctx.b.array_expr(
        obj.properties
            .iter()
            .map(|prop| excluded_key_expr(ctx, prop)),
    );
    ctx.b.call_expr(
        "$.exclude_from_object",
        [Arg::Expr(ctx.b.clone_expr(access)), Arg::Expr(excluded)],
    )
}

fn excluded_key_expr<'a>(
    ctx: &Ctx<'a>,
    prop: &oxc_ast::ast::BindingProperty<'a>,
) -> oxc_ast::ast::Expression<'a> {
    match &prop.key {
        PropertyKey::StaticIdentifier(id) if !prop.computed => ctx.b.str_expr(id.name.as_str()),
        PropertyKey::StringLiteral(str_) => ctx.b.str_expr(str_.value.as_str()),
        PropertyKey::NumericLiteral(num) => ctx.b.str_expr(&num.value.to_string()),
        key if prop.computed => ctx
            .b
            .call_expr("String", [Arg::Expr(clone_property_key_expr(ctx, key))]),
        _ => ctx.b.call_expr(
            "String",
            [Arg::Expr(clone_property_key_expr(ctx, &prop.key))],
        ),
    }
}

fn clone_property_key_expr<'a>(
    ctx: &Ctx<'a>,
    key: &PropertyKey<'a>,
) -> oxc_ast::ast::Expression<'a> {
    match key {
        PropertyKey::StaticIdentifier(id) => ctx.b.str_expr(id.name.as_str()),
        PropertyKey::PrivateIdentifier(id) => ctx.b.str_expr(id.name.as_str()),
        PropertyKey::BooleanLiteral(it) => {
            Expression::BooleanLiteral(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::NullLiteral(it) => Expression::NullLiteral(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::NumericLiteral(it) => {
            Expression::NumericLiteral(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::BigIntLiteral(it) => {
            Expression::BigIntLiteral(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::RegExpLiteral(it) => {
            Expression::RegExpLiteral(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::StringLiteral(it) => {
            Expression::StringLiteral(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TemplateLiteral(it) => {
            Expression::TemplateLiteral(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::Identifier(it) => Expression::Identifier(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::MetaProperty(it) => Expression::MetaProperty(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::Super(it) => Expression::Super(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::ArrayExpression(it) => {
            Expression::ArrayExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ArrowFunctionExpression(it) => {
            Expression::ArrowFunctionExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::AssignmentExpression(it) => {
            Expression::AssignmentExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::AwaitExpression(it) => {
            Expression::AwaitExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::BinaryExpression(it) => {
            Expression::BinaryExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::CallExpression(it) => {
            Expression::CallExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ChainExpression(it) => {
            Expression::ChainExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ClassExpression(it) => {
            Expression::ClassExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ConditionalExpression(it) => {
            Expression::ConditionalExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::FunctionExpression(it) => {
            Expression::FunctionExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ImportExpression(it) => {
            Expression::ImportExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::LogicalExpression(it) => {
            Expression::LogicalExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::NewExpression(it) => {
            Expression::NewExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ObjectExpression(it) => {
            Expression::ObjectExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ParenthesizedExpression(it) => {
            Expression::ParenthesizedExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::SequenceExpression(it) => {
            Expression::SequenceExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TaggedTemplateExpression(it) => {
            Expression::TaggedTemplateExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ThisExpression(it) => {
            Expression::ThisExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::UnaryExpression(it) => {
            Expression::UnaryExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::UpdateExpression(it) => {
            Expression::UpdateExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::YieldExpression(it) => {
            Expression::YieldExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::PrivateInExpression(it) => {
            Expression::PrivateInExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::JSXElement(it) => Expression::JSXElement(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::JSXFragment(it) => Expression::JSXFragment(it.clone_in(ctx.b.ast.allocator)),
        PropertyKey::TSAsExpression(it) => {
            Expression::TSAsExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TSSatisfiesExpression(it) => {
            Expression::TSSatisfiesExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TSTypeAssertion(it) => {
            Expression::TSTypeAssertion(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TSNonNullExpression(it) => {
            Expression::TSNonNullExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::TSInstantiationExpression(it) => {
            Expression::TSInstantiationExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::ComputedMemberExpression(it) => {
            Expression::ComputedMemberExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::StaticMemberExpression(it) => {
            Expression::StaticMemberExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::PrivateFieldExpression(it) => {
            Expression::PrivateFieldExpression(it.clone_in(ctx.b.ast.allocator))
        }
        PropertyKey::V8IntrinsicExpression(it) => {
            Expression::V8IntrinsicExpression(it.clone_in(ctx.b.ast.allocator))
        }
    }
}

fn build_static_member_access<'a>(
    ctx: &Ctx<'a>,
    object: &Expression<'a>,
    prop: &str,
) -> Expression<'a> {
    if let Expression::ChainExpression(chain) = object {
        let property = ctx.b.ast.identifier_name(SPAN, ctx.b.ast.atom(prop));
        let member = ctx.b.ast.static_member_expression(
            SPAN,
            clone_chain_element_expr(ctx, &chain.expression),
            property,
            false,
        );
        return Expression::ChainExpression(ctx.b.alloc(ctx.b.ast.chain_expression(
            SPAN,
            ChainElement::StaticMemberExpression(ctx.b.alloc(member)),
        )));
    }

    ctx.b.static_member_expr(ctx.b.clone_expr(object), prop)
}

fn build_computed_member_access<'a>(
    ctx: &Ctx<'a>,
    object: &Expression<'a>,
    property: Expression<'a>,
) -> Expression<'a> {
    if let Expression::ChainExpression(chain) = object {
        let member = ctx.b.ast.computed_member_expression(
            SPAN,
            clone_chain_element_expr(ctx, &chain.expression),
            property,
            false,
        );
        return Expression::ChainExpression(ctx.b.alloc(ctx.b.ast.chain_expression(
            SPAN,
            ChainElement::ComputedMemberExpression(ctx.b.alloc(member)),
        )));
    }

    ctx.b
        .computed_member_expr(ctx.b.clone_expr(object), property)
}

fn clone_chain_element_expr<'a>(ctx: &Ctx<'a>, element: &ChainElement<'a>) -> Expression<'a> {
    match element {
        ChainElement::CallExpression(call) => {
            Expression::CallExpression(call.clone_in(ctx.b.ast.allocator))
        }
        ChainElement::StaticMemberExpression(member) => {
            Expression::StaticMemberExpression(member.clone_in(ctx.b.ast.allocator))
        }
        ChainElement::ComputedMemberExpression(member) => {
            Expression::ComputedMemberExpression(member.clone_in(ctx.b.ast.allocator))
        }
        ChainElement::PrivateFieldExpression(member) => {
            Expression::PrivateFieldExpression(member.clone_in(ctx.b.ast.allocator))
        }
        ChainElement::TSNonNullExpression(expr) => {
            Expression::TSNonNullExpression(expr.clone_in(ctx.b.ast.allocator))
        }
    }
}

/// Post-process a codegen-built path expression, wrapping any identifier
/// whose name is in `array_insert_names` with `$.get(name)`. Runs over AST
/// this module just built (member/call/chain shapes from the collectors
/// above); `$.fallback`-hoisted user defaults are handled by descending
/// into call arguments.
///
/// We do not descend into function bodies, classes, JSX, or TS nodes —
/// those don't appear in our built path expressions, and the synthetic
/// array names (`$$arrayN`) cannot occur inside user code.
fn rewrite_array_reads<'a>(
    ctx: &Ctx<'a>,
    expr: &mut Expression<'a>,
    array_insert_names: &FxHashSet<String>,
) {
    // Post-order so inner replacements happen first; order doesn't actually
    // matter because the rewrite only fires on bare Identifier nodes, but
    // matching the original VisitMut ordering keeps behavior identical.
    match expr {
        Expression::StaticMemberExpression(member) => {
            rewrite_array_reads(ctx, &mut member.object, array_insert_names);
        }
        Expression::ComputedMemberExpression(member) => {
            rewrite_array_reads(ctx, &mut member.object, array_insert_names);
            rewrite_array_reads(ctx, &mut member.expression, array_insert_names);
        }
        Expression::ChainExpression(chain) => match &mut chain.expression {
            ChainElement::StaticMemberExpression(member) => {
                rewrite_array_reads(ctx, &mut member.object, array_insert_names);
            }
            ChainElement::ComputedMemberExpression(member) => {
                rewrite_array_reads(ctx, &mut member.object, array_insert_names);
                rewrite_array_reads(ctx, &mut member.expression, array_insert_names);
            }
            ChainElement::CallExpression(call) => {
                rewrite_array_reads(ctx, &mut call.callee, array_insert_names);
                for arg in call.arguments.iter_mut() {
                    if let Some(arg_expr) = arg.as_expression_mut() {
                        rewrite_array_reads(ctx, arg_expr, array_insert_names);
                    }
                }
            }
            _ => {}
        },
        Expression::CallExpression(call) => {
            rewrite_array_reads(ctx, &mut call.callee, array_insert_names);
            for arg in call.arguments.iter_mut() {
                if let Some(arg_expr) = arg.as_expression_mut() {
                    rewrite_array_reads(ctx, arg_expr, array_insert_names);
                }
            }
        }
        Expression::ArrayExpression(arr) => {
            for element in arr.elements.iter_mut() {
                if let Some(el_expr) = element.as_expression_mut() {
                    rewrite_array_reads(ctx, el_expr, array_insert_names);
                }
            }
        }
        _ => {}
    }

    // Replace the identifier itself after descending (matches the post-order
    // behavior of the previous `walk_expression(...)` then `*expr = ...`).
    let Expression::Identifier(ident) = expr else {
        return;
    };
    if !array_insert_names.contains(ident.name.as_str()) {
        return;
    }
    *expr = ctx.b.call_expr("$.get", [Arg::Ident(ident.name.as_str())]);
}
