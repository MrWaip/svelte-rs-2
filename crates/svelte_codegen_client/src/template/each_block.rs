//! EachBlock code generation.

use oxc_allocator::CloneIn;
use oxc_ast::ast::{BindingPattern, Expression, Statement};

use svelte_analyze::{ExprSite, FragmentKey};
use svelte_ast::NodeId;

use crate::builder::Arg;
use crate::context::Ctx;

use super::async_plan::AsyncEmissionPlan;
use super::expression::get_node_expr;
use super::gen_fragment;

// Each block flags (from Svelte constants)
const EACH_ITEM_REACTIVE: u32 = 1;
const EACH_INDEX_REACTIVE: u32 = 2;
const EACH_IS_CONTROLLED: u32 = 4;
const EACH_IS_ANIMATED: u32 = 8;
const EACH_ITEM_IMMUTABLE: u32 = 16;

/// Generate statements for an `{#each ...}` block.
pub(crate) fn gen_each_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    anchor: Expression<'a>,
    is_controlled: bool,
    body: &mut Vec<Statement<'a>>,
) {
    let block = ctx.query.each_block(block_id);
    let body_key = FragmentKey::EachBody(block_id);
    let expr_span = block.expression_span;
    let span_start = block.span.start;
    let has_key = block.key_span.is_some();
    let has_index = block.index_span.is_some();
    let has_fallback = block.fallback.is_some();

    // Extract the context binding pattern once when `as ...` is present.
    // Consumed by key function (clone for arrow param) and destructuring (ownership transfer).
    let context_pattern = block.context_span.map(|cs| {
        let stmt = ctx.state.parsed.take_stmt(
            ctx.each_context_stmt_handle(block_id)
                .or_else(|| ctx.state.parsed.stmt_handle(cs.start))
                .expect("each block with context must have pre-parsed context stmt handle"),
        )
            .expect("each block with context must have pre-parsed context stmt");
        let Statement::VariableDeclaration(mut var_decl) = stmt else {
            unreachable!("each context stmt must be VariableDeclaration");
        };
        var_decl.declarations.remove(0).id
    });

    let context_name = ctx.query.each_context_name(block_id).to_string();

    let key_is_item = ctx.each_key_is_item(block_id);

    // Compute flags
    let mut flags: u32 = 0;

    // EACH_INDEX_REACTIVE: keyed block with user-declared index
    if has_key && has_index {
        flags |= EACH_INDEX_REACTIVE;
    }

    // EACH_ITEM_REACTIVE: collection references external state
    // In runes mode: skip when key_is_item (item identity is the key)
    let expr_deps = ctx
        .expr_deps(ExprSite::Node(block_id))
        .unwrap_or_else(|| panic!("missing expression deps for each block {:?}", block_id));
    let async_plan = AsyncEmissionPlan::for_node(ctx, block_id);
    let expr_has_refs = !expr_deps.info.ref_symbols.is_empty();
    let uses_store = expr_deps.info.has_store_ref;
    let has_await = async_plan.has_await();
    let needs_async = async_plan.needs_async();
    if uses_store || (expr_has_refs && !key_is_item) {
        flags |= EACH_ITEM_REACTIVE;
    }

    // EACH_ITEM_IMMUTABLE: runes mode without store dependency
    if !uses_store {
        flags |= EACH_ITEM_IMMUTABLE;
    }

    if is_controlled {
        flags |= EACH_IS_CONTROLLED;
    }

    if has_key && ctx.each_has_animate(block_id) {
        flags |= EACH_IS_ANIMATED;
    }

    // User-declared index name (always available regardless of body usage)
    let user_index_name = ctx.each_index_name(block_id);

    // Render function index param: only when body uses it or group binding needs it
    let needs_group_index = ctx.contains_group_binding(block_id);
    let body_uses_index = ctx.each_body_uses_index(block_id);
    let render_index_name = if body_uses_index || needs_group_index {
        user_index_name.clone().or_else(|| {
            needs_group_index.then(|| {
                let name = ctx.gen_ident("$$index");
                ctx.state.group_index_names.insert(block_id, name.clone());
                name
            })
        })
    } else {
        None
    };

    // Build async thunk from expression BEFORE it gets consumed by the normal path
    let async_collection_thunk = if has_await {
        let expr = get_node_expr(ctx, block_id);
        Some(ctx.b.async_thunk(expr))
    } else {
        None
    };

    // Pre-computed by analysis — no string-based symbol re-resolution.
    let is_prop_source = ctx.is_prop_source_node(block_id);
    let collection_fn = if needs_async {
        // Inside $.async callback: () => $.get($$collection)
        ctx.b
            .thunk(ctx.b.call_expr("$.get", [Arg::Ident("$$collection")]))
    } else if is_prop_source {
        // Prop getter is already a function — pass directly without thunk
        let expr_source = ctx.query.component.source_text(expr_span).trim();
        ctx.b.rid_expr(expr_source)
    } else {
        let collection = get_node_expr(ctx, block_id);
        ctx.b.thunk(collection)
    };

    // Key function: keyed each uses (pattern[, index]) => key_expr, unkeyed uses $.index
    let key_fn = {
        let key_span = ctx.query.each_block(block_id).key_span;
        let key_expr = key_span
            .and_then(|ks| ctx.state.parsed.expr_handle(ks.start))
            .and_then(|handle| ctx.state.parsed.take_expr(handle));
        if let Some(key_expr) = key_expr {
            let key_body = ctx.b.expr_stmt(key_expr);
            let context_param = match &context_pattern {
                Some(pattern) => {
                    let cloned = pattern.clone_in(ctx.b.ast.allocator);
                    ctx.b.formal_parameter_from_pattern(cloned)
                }
                None => ctx.b.formal_parameter_from_str(&context_name),
            };
            if ctx.each_key_uses_index(block_id) {
                let idx_name = user_index_name
                    .as_ref()
                    .expect("key_uses_index implies index exists");
                let idx_param = ctx.b.formal_parameter_from_str(idx_name);
                ctx.b.arrow_expr(
                    ctx.b.formal_parameters([context_param, idx_param]),
                    [key_body],
                )
            } else {
                ctx.b
                    .arrow_expr(ctx.b.formal_parameters([context_param]), [key_body])
            }
        } else {
            ctx.b.rid_expr("$.index")
        }
    };

    // Destructuring declarations prepended to body
    let item_reactive = flags & EACH_ITEM_REACTIVE != 0;
    let destructured_decls = match context_pattern {
        Some(pattern @ (BindingPattern::ArrayPattern(_) | BindingPattern::ObjectPattern(_))) => {
            gen_destructuring_declarations(ctx, pattern, item_reactive)
        }
        _ => Vec::new(),
    };

    let mut frag_body = gen_fragment(ctx, body_key);

    // Prepend destructuring declarations
    if !destructured_decls.is_empty() {
        let mut combined = destructured_decls;
        combined.append(&mut frag_body);
        frag_body = combined;
    }

    let frag_fn = if let Some(ref idx) = render_index_name {
        ctx.b
            .arrow_block_expr(ctx.b.params(["$$anchor", &context_name, idx]), frag_body)
    } else {
        ctx.b
            .arrow_block_expr(ctx.b.params(["$$anchor", &context_name]), frag_body)
    };

    if needs_async {
        // Inside $.async callback: use "node" as anchor
        let mut each_args: Vec<Arg<'a, '_>> = vec![
            Arg::Ident("node"),
            Arg::Num(flags as f64),
            Arg::Expr(collection_fn),
            Arg::Expr(key_fn),
            Arg::Expr(frag_fn),
        ];

        if has_fallback {
            let fallback_key = FragmentKey::EachFallback(block_id);
            let fallback_body = gen_fragment(ctx, fallback_key);
            let fallback_fn = ctx
                .b
                .arrow_block_expr(ctx.b.params(["$$anchor"]), fallback_body);
            each_args.push(Arg::Expr(fallback_fn));
        }

        let each_call = ctx.b.call_expr("$.each", each_args);
        let each_stmt = super::add_svelte_meta(ctx, each_call, span_start, "each");

        body.push(async_plan.wrap_async_block(
            ctx,
            anchor,
            "node",
            "$$collection",
            async_collection_thunk,
            vec![each_stmt],
        ));
    } else {
        let mut each_args: Vec<Arg<'a, '_>> = vec![
            Arg::Expr(anchor),
            Arg::Num(flags as f64),
            Arg::Expr(collection_fn),
            Arg::Expr(key_fn),
            Arg::Expr(frag_fn),
        ];

        if has_fallback {
            let fallback_key = FragmentKey::EachFallback(block_id);
            let fallback_body = gen_fragment(ctx, fallback_key);
            let fallback_fn = ctx
                .b
                .arrow_block_expr(ctx.b.params(["$$anchor"]), fallback_body);
            each_args.push(Arg::Expr(fallback_fn));
        }

        let each_call = ctx.b.call_expr("$.each", each_args);
        body.push(super::add_svelte_meta(ctx, each_call, span_start, "each"));
    }
}

/// Generate destructuring declarations for `{#each items as { name, value }}` patterns.
///
/// Object `{ a, b }` → `let a = () => $.get($$item).a;` (thunks)
/// Object `{ a = 5 }` → `let a = $.derived_safe_equal(() => $.fallback($.get($$item).a, 5));`
/// Array `[a, b]` → intermediate `$.derived(() => $.to_array($.get($$item), N))` + per-element thunks
fn gen_destructuring_declarations<'a>(
    ctx: &mut Ctx<'a>,
    pattern: BindingPattern<'a>,
    item_reactive: bool,
) -> Vec<Statement<'a>> {
    use oxc_ast::ast::{BindingPattern, PropertyKey};

    let mut decls = Vec::new();

    match pattern {
        BindingPattern::ArrayPattern(arr) => {
            let elements: Vec<_> = arr.unbox().elements.into_iter().flatten().collect();
            let count = elements.len();
            let array_name = ctx.gen_ident("$$array");

            let item_access = if item_reactive {
                ctx.b.call_expr("$.get", [Arg::Ident("$$item")])
            } else {
                ctx.b.rid_expr("$$item")
            };
            let to_array = ctx.b.call_expr(
                "$.to_array",
                [Arg::Expr(item_access), Arg::Num(count as f64)],
            );
            let derived = ctx
                .b
                .call_expr("$.derived", [Arg::Expr(ctx.b.thunk(to_array))]);
            decls.push(ctx.b.var_stmt(&array_name, derived));

            for (i, elem) in elements.into_iter().enumerate() {
                let name = match &elem {
                    BindingPattern::BindingIdentifier(id) => id.name.as_str().to_string(),
                    BindingPattern::AssignmentPattern(assign) => {
                        if let BindingPattern::BindingIdentifier(id) = &assign.left {
                            id.name.as_str().to_string()
                        } else {
                            continue;
                        }
                    }
                    _ => continue,
                };
                let get_array = ctx.b.call_expr("$.get", [Arg::Ident(&array_name)]);
                let access = ctx
                    .b
                    .computed_member_expr(get_array, ctx.b.num_expr(i as f64));
                let thunk = ctx.b.thunk(access);
                decls.push(ctx.b.let_init_stmt(&name, thunk));
            }
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in obj.unbox().properties {
                let key_name = match &prop.key {
                    PropertyKey::StaticIdentifier(id) => Some(id.name.as_str().to_string()),
                    _ => None,
                };
                match prop.value {
                    BindingPattern::BindingIdentifier(id) => {
                        let name = id.name.as_str().to_string();
                        let prop_key = key_name.as_deref().unwrap_or(&name);
                        let item_access = if item_reactive {
                            ctx.b.call_expr("$.get", [Arg::Ident("$$item")])
                        } else {
                            ctx.b.rid_expr("$$item")
                        };
                        let member = ctx.b.static_member_expr(item_access, prop_key);
                        decls.push(ctx.b.let_init_stmt(&name, ctx.b.thunk(member)));
                    }
                    BindingPattern::AssignmentPattern(assign) => {
                        let assign = assign.unbox();
                        let BindingPattern::BindingIdentifier(id) = assign.left else {
                            continue;
                        };
                        let name = id.name.as_str().to_string();
                        let prop_key = key_name.as_deref().unwrap_or(&name);
                        let item_access = if item_reactive {
                            ctx.b.call_expr("$.get", [Arg::Ident("$$item")])
                        } else {
                            ctx.b.rid_expr("$$item")
                        };
                        let member = ctx.b.static_member_expr(item_access, prop_key);
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
        }
        _ => unreachable!("each context pattern must be object or array"),
    }

    decls
}
