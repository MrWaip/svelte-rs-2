//! EachBlock code generation.
//!
//! `gen_each_block` is a thin orchestrator: it reads pre-computed
//! [`EachBlockSemantics`] and dispatches to focused builders for each
//! piece of the `$.each(...)` call (collection, key, fragment, fallback).

use oxc_allocator::CloneIn;
use oxc_ast::ast::{BindingPattern, Expression, Statement};

use svelte_analyze::{
    EachAsyncKind, EachBlockSemantics, EachCollectionKind, EachFlags, EachFlavor, EachIndexKind,
    EachItemKind, FragmentKey,
};
use svelte_ast::{EachBlock, NodeId};

use crate::context::Ctx;
use svelte_ast_builder::Arg;

use super::async_plan::wrap_async_block;
use super::expression::get_node_expr;
use super::gen_fragment;

/// The only runtime flag decided at the codegen call site (element-anchor
/// vs comment-anchor). All other bits ride on `sem.each_flags`.
const EACH_IS_CONTROLLED: u32 = 4;

const SYNTHETIC_ITEM_NAME: &str = "$$item";

/// Generate statements for an `{#each ...}` block.
///
/// `sem` is already resolved: the root consumer match happened before
/// this call (see SEMANTIC_LAYER_ARCHITECTURE.md — Root Consumer Migration).
pub(crate) fn gen_each_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    sem: &EachBlockSemantics,
    anchor: Expression<'a>,
    is_controlled: bool,
    body: &mut Vec<Statement<'a>>,
) {
    let block = ctx.query.each_block(block_id);
    let plan = EachPlan::build(ctx, block, sem, is_controlled);

    let context_pattern = take_context_pattern(ctx, block);
    let collection_fn = build_collection_fn(ctx, block, &plan);
    let key_fn = build_key_fn(ctx, block, &plan, context_pattern.as_ref());
    let frag_fn = build_fragment_fn(ctx, block_id, &plan, context_pattern);

    emit_each_call(
        ctx,
        block_id,
        block,
        &plan,
        anchor,
        collection_fn,
        key_fn,
        frag_fn,
        body,
    );
}

// ---------------------------------------------------------------------------
// Plan — the pre-computed, plain-data snapshot consumed by the builders.
// ---------------------------------------------------------------------------

/// Everything the output builders need to know, resolved up front from
/// `sem` and the source spans. Keeping it in one struct avoids threading
/// the same 8 values through every helper.
struct EachPlan {
    /// `$.each(...)` flags ORed with the call-site `IS_CONTROLLED` bit.
    flags: u32,
    /// Name to use for the item arrow parameter.
    item_param_name: String,
    /// User-declared `, <index>` name, if any.
    user_index_name: Option<String>,
    /// Name of the index param emitted on the fragment arrow, if we need one.
    render_index_name: Option<String>,
    /// Name of the synthetic `$$array` param emitted when shadowing forces it.
    collection_id_name: Option<String>,
    /// The key expression references the index binding.
    key_uses_index: bool,
    /// Collection read must go through `$.async` / `$.get($$collection)`.
    needs_async: bool,
    /// `await` literally present in the collection expression.
    has_await: bool,
    /// Blocker indices the async wrapper must wait on.
    blockers: Vec<u32>,
    /// Collection root is a prop-source getter — skip the thunk wrap.
    is_prop_source: bool,
    /// `{#each ... }{:else ...}` present.
    has_fallback: bool,
    /// ITEM_REACTIVE pre-computed by Block Semantics — controls how
    /// destructuring reads `$$item`.
    item_reactive: bool,
}

impl EachPlan {
    fn build(
        ctx: &mut Ctx<'_>,
        block: &EachBlock,
        sem: &EachBlockSemantics,
        is_controlled: bool,
    ) -> Self {
        let (has_await, blockers) = match &sem.async_kind {
            EachAsyncKind::Sync => (false, Vec::new()),
            EachAsyncKind::Async {
                has_await,
                blockers,
            } => (*has_await, blockers.to_vec()),
        };
        let needs_async = has_await || !blockers.is_empty();

        let (body_uses_index, key_uses_index) = match sem.index {
            EachIndexKind::Declared {
                used_in_body,
                used_in_key,
                ..
            } => (used_in_body, used_in_key),
            EachIndexKind::Absent => (false, false),
        };

        let user_index_name = block
            .index_span
            .map(|s| ctx.query.component.source_text(s).trim().to_string());

        let needs_group_index = matches!(sem.flavor, EachFlavor::BindGroup);
        // Reference: EachBlock.js 316–318. A shadowed body must be able
        // to reach the outer collection, so the runtime threads an extra
        // `$$array` param through the fragment arrow — and an index
        // param always precedes it, used or not.
        let needs_collection_id = sem.shadows_outer;
        let render_index_name = render_index_name(
            ctx,
            block.id,
            user_index_name.as_ref(),
            body_uses_index || needs_group_index || needs_collection_id,
            needs_group_index,
        );
        let collection_id_name = needs_collection_id.then(|| ctx.gen_ident("$$array"));

        let mut flags: u32 = sem.each_flags.bits() as u32;
        if is_controlled {
            flags |= EACH_IS_CONTROLLED;
        }

        Self {
            flags,
            item_param_name: item_param_name(ctx, block, &sem.item),
            user_index_name,
            render_index_name,
            collection_id_name,
            key_uses_index,
            needs_async,
            has_await,
            blockers,
            is_prop_source: matches!(sem.collection_kind, EachCollectionKind::PropSource),
            has_fallback: block.fallback.is_some(),
            item_reactive: sem.each_flags.contains(EachFlags::ITEM_REACTIVE),
        }
    }
}

/// Pick the name for the item arrow parameter. Destructured patterns and
/// no-binding forms use the synthetic `$$item`; a plain identifier reuses
/// its source text so the generated body reads naturally.
fn item_param_name(ctx: &Ctx<'_>, block: &EachBlock, item: &EachItemKind) -> String {
    match item {
        EachItemKind::Identifier(_) => block
            .context_span
            .map(|cs| ctx.query.component.source_text(cs).trim().to_string())
            .unwrap_or_else(|| SYNTHETIC_ITEM_NAME.to_string()),
        EachItemKind::Pattern(_) | EachItemKind::NoBinding => SYNTHETIC_ITEM_NAME.to_string(),
    }
}

/// Decide whether the fragment arrow declares an index param and pick its
/// name. A `bind:group` inside the body also registers the generated name
/// so downstream codegen can refer to it.
fn render_index_name(
    ctx: &mut Ctx<'_>,
    block_id: NodeId,
    user_name: Option<&String>,
    needed: bool,
    needs_group_index: bool,
) -> Option<String> {
    if !needed {
        return None;
    }
    if let Some(name) = user_name {
        return Some(name.clone());
    }
    let generated = ctx.gen_ident("$$index");
    if needs_group_index {
        ctx.state
            .group_index_names
            .insert(block_id, generated.clone());
    }
    Some(generated)
}

// ---------------------------------------------------------------------------
// Pre-parsed subtree extraction.
// ---------------------------------------------------------------------------

/// Take ownership of the `as <pattern>` binding pattern. Returned value
/// is consumed by the key-fn builder (clones it for the arrow param) and
/// by destructuring (moves it into declarations).
fn take_context_pattern<'a>(ctx: &mut Ctx<'a>, block: &EachBlock) -> Option<BindingPattern<'a>> {
    let cs = block.context_span?;
    let handle = ctx
        .state
        .parsed
        .stmt_handle(cs.start)
        .expect("each block with context must have pre-parsed context stmt handle");
    let stmt = ctx
        .state
        .parsed
        .take_stmt(handle)
        .expect("each block with context must have pre-parsed context stmt");
    let Statement::VariableDeclaration(mut var_decl) = stmt else {
        unreachable!("each context stmt must be VariableDeclaration");
    };
    Some(var_decl.declarations.remove(0).id)
}

// ---------------------------------------------------------------------------
// Collection read.
// ---------------------------------------------------------------------------

/// Build the collection argument passed to `$.each(...)`.
///
/// - Async: reference `$$collection` signal introduced by `$.async`.
/// - Prop source: the getter is already a function, pass it raw.
/// - Otherwise: wrap the expression in a thunk so `$.each` can re-read it.
fn build_collection_fn<'a>(
    ctx: &mut Ctx<'a>,
    block: &EachBlock,
    plan: &EachPlan,
) -> Expression<'a> {
    if plan.needs_async {
        return ctx
            .b
            .thunk(ctx.b.call_expr("$.get", [Arg::Ident("$$collection")]));
    }
    if plan.is_prop_source {
        let src = ctx
            .query
            .component
            .source_text(block.expression_span)
            .trim();
        return ctx.b.rid_expr(src);
    }
    let expr = get_node_expr(ctx, block.id);
    ctx.b.thunk(expr)
}

// ---------------------------------------------------------------------------
// Key function.
// ---------------------------------------------------------------------------

/// Build the key argument. Unkeyed blocks pass `$.index`; keyed blocks
/// emit `(pattern[, index]) => <key_expr>`.
fn build_key_fn<'a>(
    ctx: &mut Ctx<'a>,
    block: &EachBlock,
    plan: &EachPlan,
    context_pattern: Option<&BindingPattern<'a>>,
) -> Expression<'a> {
    let Some(key_expr) = take_key_expr(ctx, block) else {
        return ctx.b.rid_expr("$.index");
    };
    let key_body = ctx.b.expr_stmt(key_expr);
    let context_param = match context_pattern {
        Some(pattern) => ctx
            .b
            .formal_parameter_from_pattern(pattern.clone_in(ctx.b.ast.allocator)),
        None => ctx.b.formal_parameter_from_str(&plan.item_param_name),
    };
    if plan.key_uses_index {
        let idx_name = plan
            .user_index_name
            .as_ref()
            .expect("key_uses_index implies a declared index binding");
        let idx_param = ctx.b.formal_parameter_from_str(idx_name);
        ctx.b.arrow_expr(
            ctx.b.formal_parameters([context_param, idx_param]),
            [key_body],
        )
    } else {
        ctx.b
            .arrow_expr(ctx.b.formal_parameters([context_param]), [key_body])
    }
}

fn take_key_expr<'a>(ctx: &mut Ctx<'a>, block: &EachBlock) -> Option<Expression<'a>> {
    let ks = block.key_span?;
    let handle = ctx.state.parsed.expr_handle(ks.start)?;
    ctx.state.parsed.take_expr(handle)
}

// ---------------------------------------------------------------------------
// Fragment arrow.
// ---------------------------------------------------------------------------

/// Build the per-item fragment arrow passed to `$.each(...)`, including
/// destructuring declarations prepended to the body when `as <pattern>`
/// is destructured.
fn build_fragment_fn<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    plan: &EachPlan,
    context_pattern: Option<BindingPattern<'a>>,
) -> Expression<'a> {
    let mut frag_body = gen_fragment(ctx, FragmentKey::EachBody(block_id));

    if let Some(pattern @ (BindingPattern::ArrayPattern(_) | BindingPattern::ObjectPattern(_))) =
        context_pattern
    {
        let mut decls = gen_destructuring_declarations(ctx, pattern, plan.item_reactive);
        decls.append(&mut frag_body);
        frag_body = decls;
    }

    match (&plan.render_index_name, &plan.collection_id_name) {
        (Some(idx), Some(arr)) => ctx.b.arrow_block_expr(
            ctx.b.params(["$$anchor", &plan.item_param_name, idx, arr]),
            frag_body,
        ),
        (Some(idx), None) => ctx.b.arrow_block_expr(
            ctx.b.params(["$$anchor", &plan.item_param_name, idx]),
            frag_body,
        ),
        (None, _) => ctx
            .b
            .arrow_block_expr(ctx.b.params(["$$anchor", &plan.item_param_name]), frag_body),
    }
}

// ---------------------------------------------------------------------------
// Fallback arrow.
// ---------------------------------------------------------------------------

fn build_fallback_fn<'a>(ctx: &mut Ctx<'a>, block_id: NodeId) -> Expression<'a> {
    let fallback_body = gen_fragment(ctx, FragmentKey::EachFallback(block_id));
    ctx.b
        .arrow_block_expr(ctx.b.params(["$$anchor"]), fallback_body)
}

// ---------------------------------------------------------------------------
// Emit the `$.each(...)` call, wrapping it in `$.async` when required.
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn emit_each_call<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    block: &EachBlock,
    plan: &EachPlan,
    anchor: Expression<'a>,
    collection_fn: Expression<'a>,
    key_fn: Expression<'a>,
    frag_fn: Expression<'a>,
    body: &mut Vec<Statement<'a>>,
) {
    let span_start = block.span.start;
    let fallback_fn = plan.has_fallback.then(|| build_fallback_fn(ctx, block_id));

    if plan.needs_async {
        // Inside the `$.async` callback the anchor is the `node` param;
        // the caller-supplied `anchor` is consumed by `wrap_async_block`.
        let collection_expr = get_node_expr(ctx, block_id);
        let async_thunk = ctx.b.async_thunk(collection_expr);
        let each_call = build_each_invocation(
            ctx,
            Arg::Ident("node"),
            plan.flags,
            collection_fn,
            key_fn,
            frag_fn,
            fallback_fn,
        );
        let each_stmt = super::add_svelte_meta(ctx, each_call, span_start, "each");
        body.push(wrap_async_block(
            ctx,
            plan.has_await,
            &plan.blockers,
            anchor,
            "node",
            "$$collection",
            Some(async_thunk),
            vec![each_stmt],
        ));
    } else {
        let each_call = build_each_invocation(
            ctx,
            Arg::Expr(anchor),
            plan.flags,
            collection_fn,
            key_fn,
            frag_fn,
            fallback_fn,
        );
        body.push(super::add_svelte_meta(ctx, each_call, span_start, "each"));
    }
}

#[allow(clippy::too_many_arguments)]
fn build_each_invocation<'a, 'b>(
    ctx: &Ctx<'a>,
    anchor_arg: Arg<'a, 'b>,
    flags: u32,
    collection_fn: Expression<'a>,
    key_fn: Expression<'a>,
    frag_fn: Expression<'a>,
    fallback_fn: Option<Expression<'a>>,
) -> Expression<'a> {
    let mut args: Vec<Arg<'a, 'b>> = vec![
        anchor_arg,
        Arg::Num(flags as f64),
        Arg::Expr(collection_fn),
        Arg::Expr(key_fn),
        Arg::Expr(frag_fn),
    ];
    if let Some(fb) = fallback_fn {
        args.push(Arg::Expr(fb));
    }
    ctx.b.call_expr("$.each", args)
}

// ---------------------------------------------------------------------------
// Destructuring declarations for `{#each items as { a, b }}` / `[a, b]`.
// ---------------------------------------------------------------------------

/// Generate destructuring declarations prepended to the fragment body.
///
/// Object `{ a, b }` → `let a = () => $.get($$item).a;` (thunks)
/// Object `{ a = 5 }` → `let a = $.derived_safe_equal(() => $.fallback(..., 5));`
/// Array `[a, b]` → intermediate `$.derived(() => $.to_array($.get($$item), N))` +
///                  per-element thunks reading that array.
fn gen_destructuring_declarations<'a>(
    ctx: &mut Ctx<'a>,
    pattern: BindingPattern<'a>,
    item_reactive: bool,
) -> Vec<Statement<'a>> {
    match pattern {
        BindingPattern::ArrayPattern(arr) => gen_array_destructure(ctx, arr.unbox(), item_reactive),
        BindingPattern::ObjectPattern(obj) => {
            gen_object_destructure(ctx, obj.unbox(), item_reactive)
        }
        _ => unreachable!("each context pattern must be object or array"),
    }
}

fn gen_array_destructure<'a>(
    ctx: &mut Ctx<'a>,
    arr: oxc_ast::ast::ArrayPattern<'a>,
    item_reactive: bool,
) -> Vec<Statement<'a>> {
    let elements: Vec<_> = arr.elements.into_iter().flatten().collect();
    let count = elements.len();
    let array_name = ctx.gen_ident("$$array");

    let to_array = ctx.b.call_expr(
        "$.to_array",
        [
            Arg::Expr(item_read(ctx, item_reactive)),
            Arg::Num(count as f64),
        ],
    );
    let derived = ctx
        .b
        .call_expr("$.derived", [Arg::Expr(ctx.b.thunk(to_array))]);

    let mut decls = vec![ctx.b.var_stmt(&array_name, derived)];
    for (i, elem) in elements.into_iter().enumerate() {
        let Some(name) = array_element_name(&elem) else {
            continue;
        };
        let get_array = ctx.b.call_expr("$.get", [Arg::Ident(&array_name)]);
        let access = ctx
            .b
            .computed_member_expr(get_array, ctx.b.num_expr(i as f64));
        decls.push(ctx.b.let_init_stmt(&name, ctx.b.thunk(access)));
    }
    decls
}

fn array_element_name(pattern: &BindingPattern<'_>) -> Option<String> {
    match pattern {
        BindingPattern::BindingIdentifier(id) => Some(id.name.as_str().to_string()),
        BindingPattern::AssignmentPattern(assign) => match &assign.left {
            BindingPattern::BindingIdentifier(id) => Some(id.name.as_str().to_string()),
            _ => None,
        },
        _ => None,
    }
}

fn gen_object_destructure<'a>(
    ctx: &mut Ctx<'a>,
    obj: oxc_ast::ast::ObjectPattern<'a>,
    item_reactive: bool,
) -> Vec<Statement<'a>> {
    use oxc_ast::ast::PropertyKey;

    let mut decls = Vec::new();
    for prop in obj.properties {
        let key_name = match &prop.key {
            PropertyKey::StaticIdentifier(id) => Some(id.name.as_str().to_string()),
            _ => None,
        };
        match prop.value {
            BindingPattern::BindingIdentifier(id) => {
                let name = id.name.as_str().to_string();
                let prop_key = key_name.as_deref().unwrap_or(&name);
                let member = ctx
                    .b
                    .static_member_expr(item_read(ctx, item_reactive), prop_key);
                decls.push(ctx.b.let_init_stmt(&name, ctx.b.thunk(member)));
            }
            BindingPattern::AssignmentPattern(assign) => {
                let assign = assign.unbox();
                let BindingPattern::BindingIdentifier(id) = assign.left else {
                    continue;
                };
                let name = id.name.as_str().to_string();
                let prop_key = key_name.as_deref().unwrap_or(&name);
                let member = ctx
                    .b
                    .static_member_expr(item_read(ctx, item_reactive), prop_key);
                let fallback = ctx
                    .b
                    .call_expr("$.fallback", [Arg::Expr(member), Arg::Expr(assign.right)]);
                let derived = ctx
                    .b
                    .call_expr("$.derived_safe_equal", [Arg::Expr(ctx.b.thunk(fallback))]);
                decls.push(ctx.b.let_init_stmt(&name, derived));
            }
            _ => continue,
        }
    }
    decls
}

/// `$$item` read shape: reactive items go through `$.get`; immutable ones
/// are passed by value.
fn item_read<'a>(ctx: &Ctx<'a>, item_reactive: bool) -> Expression<'a> {
    if item_reactive {
        ctx.b.call_expr("$.get", [Arg::Ident("$$item")])
    } else {
        ctx.b.rid_expr("$$item")
    }
}
