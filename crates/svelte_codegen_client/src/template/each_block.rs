//! EachBlock code generation.

use oxc_allocator::CloneIn;
use oxc_ast::ast::{BindingPattern, Expression, Statement};

use svelte_analyze::{
    EachBlockSemantics, EachFlags, EachIndexKind, EachItemKind, EachKeyKind, FragmentKey,
    PropReferenceSemantics, ReferenceSemantics,
};
use svelte_ast::NodeId;

use crate::context::Ctx;
use svelte_ast_builder::Arg;

use super::async_plan::wrap_async_block;
use super::expression::get_node_expr;
use super::gen_fragment;

// IS_CONTROLLED is the only flag decided at the codegen call site; the
// rest are pre-computed by Block Semantics on `sem.each_flags`.
const EACH_IS_CONTROLLED: u32 = 4;

const SYNTHETIC_ITEM_NAME: &str = "$$item";

/// Generate statements for an `{#each ...}` block.
///
/// `sem` is the already-resolved [`EachBlockSemantics`] — the root consumer
/// point performed the one semantic `match` before calling this function
/// (see SEMANTIC_LAYER_ARCHITECTURE.md / REACTIVITY_ARCHITECTURE.md
/// "Root Consumer Migration Rule").
pub(crate) fn gen_each_block<'a>(
    ctx: &mut Ctx<'a>,
    block_id: NodeId,
    sem: &EachBlockSemantics,
    anchor: Expression<'a>,
    is_controlled: bool,
    body: &mut Vec<Statement<'a>>,
) {
    let block = ctx.query.each_block(block_id);
    let body_key = FragmentKey::EachBody(block_id);
    let expr_span = block.expression_span;
    let span_start = block.span.start;
    let has_key = !matches!(sem.key, EachKeyKind::Unkeyed);
    let has_index = matches!(sem.index, EachIndexKind::Declared { .. });
    let has_fallback = block.fallback.is_some();

    // Extract the context binding pattern once when `as ...` is present.
    // Consumed by key function (clone for arrow param) and destructuring (ownership transfer).
    let context_pattern = block.context_span.map(|cs| {
        let stmt = ctx
            .state
            .parsed
            .take_stmt(
                ctx.state
                    .parsed
                    .stmt_handle(cs.start)
                    .expect("each block with context must have pre-parsed context stmt handle"),
            )
            .expect("each block with context must have pre-parsed context stmt");
        let Statement::VariableDeclaration(mut var_decl) = stmt else {
            unreachable!("each context stmt must be VariableDeclaration");
        };
        var_decl.declarations.remove(0).id
    });

    // Item binding name for the arrow parameter: destructured patterns use
    // a synthetic `$$item`; simple identifiers read their name from source
    // via `context_span`.
    let context_name: String = match &sem.item {
        EachItemKind::Pattern(_) => SYNTHETIC_ITEM_NAME.to_string(),
        EachItemKind::Identifier(_) => block
            .context_span
            .map(|cs| ctx.query.component.source_text(cs).trim().to_string())
            .unwrap_or_else(|| SYNTHETIC_ITEM_NAME.to_string()),
        EachItemKind::NoBinding => SYNTHETIC_ITEM_NAME.to_string(),
    };

    let key_is_item = matches!(sem.key, EachKeyKind::KeyedByItem);

    // Flag computation is owned by Block Semantics. Codegen only OR's in
    // EACH_IS_CONTROLLED, which is a call-site lowering decision.
    let mut flags: u32 = sem.each_flags.bits() as u32;
    if is_controlled {
        flags |= EACH_IS_CONTROLLED;
    }

    // Prop-source detection for the collection read: prop-source getters
    // lower as a direct call without the usual thunk wrap. This is a
    // lowering-shape question answered by reactivity at the root
    // identifier of the collection expression.
    let collection_root_semantics = node_root_reference_semantics(ctx, block_id);
    let is_prop_source = matches!(
        collection_root_semantics,
        ReferenceSemantics::PropRead(PropReferenceSemantics::Source { .. })
    );

    // Async lowering shape is pre-computed by Block Semantics.
    let (has_await, blockers): (bool, &[u32]) = match &sem.async_kind {
        svelte_analyze::EachAsyncKind::Sync => (false, &[]),
        svelte_analyze::EachAsyncKind::Async {
            has_await,
            blockers,
        } => (*has_await, blockers.as_slice()),
    };
    let needs_async = has_await || !blockers.is_empty();

    // User-declared index name: read from source via index_span.
    let user_index_name: Option<String> = block
        .index_span
        .map(|is| ctx.query.component.source_text(is).trim().to_string());

    let (body_uses_index, key_uses_index) = match sem.index {
        EachIndexKind::Declared {
            used_in_body,
            used_in_key,
            ..
        } => (used_in_body, used_in_key),
        EachIndexKind::Absent => (false, false),
    };

    // Reference: EachBlock.js lines 316–318. When the each body shadows an outer
    // binding, the reference compiler always emits an index param (even if unused)
    // followed by a unique `$$array` param so the body can reach the collection.
    let needs_collection_id = sem.shadows_outer;

    // `EachFlavor::BindGroup` is already scope-qualified by the Block
    // Semantics builder: it's set only when a `bind:group={...}` in the
    // body references a symbol this each introduces (item / index /
    // destructured leaf). Codegen just reads it.
    let needs_group_index = matches!(sem.flavor, svelte_analyze::EachFlavor::BindGroup);
    let render_index_name = if body_uses_index || needs_group_index || needs_collection_id {
        user_index_name.clone().or_else(|| {
            let name = ctx.gen_ident("$$index");
            if needs_group_index {
                ctx.state.group_index_names.insert(block_id, name.clone());
            }
            Some(name)
        })
    } else {
        None
    };
    let collection_id_name = needs_collection_id.then(|| ctx.gen_ident("$$array"));

    // Build async thunk from expression BEFORE it gets consumed by the normal path
    let async_collection_thunk = if has_await {
        let expr = get_node_expr(ctx, block_id);
        Some(ctx.b.async_thunk(expr))
    } else {
        None
    };

    // Collection-read lowering — decided from the same semantic answer
    // used above for reactive/immutable flags.
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
            if key_uses_index {
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

    // Destructuring declarations prepended to body. `item_reactive`
    // mirrors the pre-computed ITEM_REACTIVE flag from Block Semantics.
    let item_reactive = sem.each_flags.contains(EachFlags::ITEM_REACTIVE);
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

    let frag_fn = match (&render_index_name, &collection_id_name) {
        (Some(idx), Some(arr)) => ctx.b.arrow_block_expr(
            ctx.b.params(["$$anchor", &context_name, idx, arr]),
            frag_body,
        ),
        (Some(idx), None) => ctx
            .b
            .arrow_block_expr(ctx.b.params(["$$anchor", &context_name, idx]), frag_body),
        (None, _) => ctx
            .b
            .arrow_block_expr(ctx.b.params(["$$anchor", &context_name]), frag_body),
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

        body.push(wrap_async_block(
            ctx,
            has_await,
            blockers,
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

/// Reference-semantics of the root identifier of an `{#each <expr> as ...}`
/// collection expression. Walks through member accesses and parentheses
/// to the leaf identifier and asks reactivity for its classification.
/// Returns `NonReactive` for non-identifier roots (literals, calls, ...).
fn node_root_reference_semantics(ctx: &Ctx<'_>, block_id: NodeId) -> ReferenceSemantics {
    let handle = ctx.query.view.node_expr_handle(block_id);
    let Some(expr) = ctx.state.parsed.expr(handle) else {
        return ReferenceSemantics::NonReactive;
    };
    let mut current = expr;
    loop {
        match current {
            Expression::StaticMemberExpression(m) => current = &m.object,
            Expression::ComputedMemberExpression(m) => current = &m.object,
            Expression::ParenthesizedExpression(p) => current = &p.expression,
            Expression::Identifier(id) => {
                return match id.reference_id.get() {
                    Some(ref_id) => ctx.query.analysis.reference_semantics(ref_id),
                    None => ReferenceSemantics::NonReactive,
                };
            }
            _ => return ReferenceSemantics::NonReactive,
        }
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
