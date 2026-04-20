//! Shared rewrite routines used by both Template and Script modes
//! of `ComponentTransformer`. Each function takes only what it needs
//! (`&AnalysisData`, allocator, optional `TransformData` for
//! template-specific `ConstAliasRead`) — no `&ComponentTransformer`
//! dependency — so the two code paths can call the same logic without
//! cross-wiring their different surrounding state (template walker
//! context vs. script TS-strip / derived_pending bookkeeping).

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;

use svelte_analyze::{
    AnalysisData, CarrierMemberReadSemantics, PropReferenceSemantics, ReferenceSemantics,
    SignalReferenceKind, StateKind,
};

use crate::data::TransformData;
use crate::rune_refs;

/// Identifier-read rewrite keyed on `ReferenceSemantics`. Returns `true`
/// when the identifier was rewritten and no further processing is
/// required; `false` means the caller should fall through to mode-specific
/// handling (legacy read_semantics for template; unchanged for script).
///
/// Handles:
/// - `StoreRead` → `name()` (thunk call)
/// - `SignalRead {safe}` (State / StateRaw / Derived) → `$.get(name)` / `$.safe_get(name)`
/// - `PropRead Source` → `name()` (thunk call)
/// - `PropRead NonSource` → `$$props.<origin_key>` (static member)
/// - `ConstAliasRead` → `$.get(tmp).<name>` (member on the const-tag tmp)
///
/// Left-hand-side detection (assignment/update target) is the caller's
/// responsibility — this function only fires on read use-sites.
pub(crate) fn rewrite_identifier_read<'a>(
    analysis: &AnalysisData<'a>,
    alloc: &'a Allocator,
    transform_data: &TransformData,
    expr: &mut Expression<'a>,
) -> bool {
    let Expression::Identifier(id) = &*expr else {
        return false;
    };
    let Some(ref_id) = id.reference_id.get() else {
        return false;
    };
    let name = id.name.as_str();

    match analysis.reference_semantics(ref_id) {
        ReferenceSemantics::StoreRead { .. } => {
            *expr = rune_refs::make_thunk_call(alloc, name);
            true
        }
        ReferenceSemantics::SignalRead { safe: false, .. } => {
            *expr = rune_refs::make_rune_get(alloc, name);
            true
        }
        ReferenceSemantics::SignalRead { safe: true, .. } => {
            *expr = rune_refs::make_rune_safe_get(alloc, name);
            true
        }
        ReferenceSemantics::PropRead(PropReferenceSemantics::Source { .. }) => {
            *expr = rune_refs::make_thunk_call(alloc, name);
            true
        }
        ReferenceSemantics::PropRead(PropReferenceSemantics::NonSource { symbol }) => {
            let prop_name = analysis.binding_origin_key(symbol).unwrap_or_else(|| {
                panic!(
                    "NonSource prop read missing binding origin key for ref {:?}",
                    ref_id
                )
            });
            *expr = rune_refs::make_props_access(alloc, prop_name);
            true
        }
        ReferenceSemantics::ConstAliasRead { owner_node } => {
            if let Some(tmp) = transform_data.const_tag_tmp_names.get(&owner_node) {
                *expr = rune_refs::make_member_get(alloc, tmp, name);
            }
            true
        }
        ReferenceSemantics::CarrierMemberRead(CarrierMemberReadSemantics {
            carrier_symbol,
            ..
        }) => {
            let carrier_name = analysis.scoping.symbol_name(carrier_symbol);
            *expr = rune_refs::make_member_get(alloc, carrier_name, name);
            true
        }
        _ => false,
    }
}

#[allow(dead_code)]
fn _phantom_markers(_: SignalReferenceKind) {}

/// Rewrite `id = rhs` / `id op= rhs` for signal (state / state.raw) and
/// store identifier targets. Returns `true` when the node was replaced.
///
/// Script calls this at the top of `transform_assignment` before the
/// prop / private / member-dev-validation branches. Template calls it
/// from its exit-side rewrite. The `$.set(name, value)` / `$.store_set(base, value)`
/// we synthesize carry fresh identifiers without `reference_id`, so the
/// outer Traverse short-circuits on re-entry.
pub(crate) fn rewrite_signal_or_store_identifier_assignment<'a>(
    analysis: &AnalysisData<'a>,
    alloc: &'a Allocator,
    node: &mut Expression<'a>,
    suppress_proxy: bool,
) -> bool {
    let Expression::AssignmentExpression(assign) = node else {
        return false;
    };
    let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
        return false;
    };
    let Some(ref_id) = id.reference_id.get() else {
        return false;
    };
    let name = id.name.as_str().to_string();
    let operator = assign.operator;
    let semantics = analysis.reference_semantics(ref_id);

    match semantics {
        ReferenceSemantics::StoreWrite { symbol } | ReferenceSemantics::StoreUpdate { symbol } => {
            let base_name = analysis.scoping.symbol_name(symbol).to_string();
            let right = std::mem::replace(&mut assign.right, rune_refs::make_rune_get(alloc, ""));
            // Compound: current store value thunked via the `$name` accessor.
            let left_read = rune_refs::make_thunk_call(alloc, &name);
            let value = rune_refs::build_compound_value(alloc, operator, left_read, right);
            *node = rune_refs::make_store_set(alloc, &base_name, value);
            true
        }
        ReferenceSemantics::SignalWrite { kind } => {
            let right = std::mem::replace(&mut assign.right, rune_refs::make_rune_get(alloc, ""));
            let left_read = rune_refs::make_rune_get(alloc, &name);
            let value = rune_refs::build_compound_value(alloc, operator, left_read, right);
            let needs_proxy =
                !suppress_proxy && kind == StateKind::State && rune_refs::should_proxy(&value);
            *node = rune_refs::make_rune_set(alloc, &name, value, needs_proxy);
            true
        }
        ReferenceSemantics::SignalUpdate { kind, safe } => {
            let right = std::mem::replace(&mut assign.right, rune_refs::make_rune_get(alloc, ""));
            let left_read = if safe {
                rune_refs::make_rune_safe_get(alloc, &name)
            } else {
                rune_refs::make_rune_get(alloc, &name)
            };
            let value = rune_refs::build_compound_value(alloc, operator, left_read, right);
            let needs_proxy =
                !suppress_proxy && kind == StateKind::State && rune_refs::should_proxy(&value);
            *node = rune_refs::make_rune_set(alloc, &name, value, needs_proxy);
            true
        }
        _ => false,
    }
}

/// Rewrite `++id` / `id++` / `--id` / `id--` for signal and store targets.
/// Mirrors `rewrite_signal_or_store_identifier_assignment` for UpdateExpression.
pub(crate) fn rewrite_signal_or_store_identifier_update<'a>(
    analysis: &AnalysisData<'a>,
    alloc: &'a Allocator,
    node: &mut Expression<'a>,
) -> bool {
    let Expression::UpdateExpression(upd) = node else {
        return false;
    };
    let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument else {
        return false;
    };
    let Some(ref_id) = id.reference_id.get() else {
        return false;
    };
    let is_increment = upd.operator == oxc_syntax::operator::UpdateOperator::Increment;
    let name = id.name.as_str().to_string();
    let is_prefix = upd.prefix;

    match analysis.reference_semantics(ref_id) {
        ReferenceSemantics::StoreUpdate { symbol } => {
            let base_name = analysis.scoping.symbol_name(symbol).to_string();
            *node = rune_refs::make_store_update(alloc, &base_name, &name, is_prefix, is_increment);
            true
        }
        ReferenceSemantics::SignalUpdate {
            kind: StateKind::State | StateKind::StateRaw,
            ..
        } => {
            *node = rune_refs::make_rune_update(alloc, &name, is_prefix, is_increment);
            true
        }
        _ => false,
    }
}

/// Rewrite deep store member assignment: `$store.foo.bar = val` →
/// `$.store_mutate(store, $.untrack($store).foo.bar = val, $.untrack($store))`.
/// Runs on enter so the original `$store` root identifier is still present
/// (not yet rewritten to `$.get($store)` by child traversal).
pub(crate) fn rewrite_deep_store_member_assignment<'a>(
    analysis: &AnalysisData<'a>,
    alloc: &'a Allocator,
    node: &mut Expression<'a>,
) -> bool {
    let Expression::AssignmentExpression(assign) = node else {
        return false;
    };
    let Some(member) = assign.left.as_member_expression() else {
        return false;
    };
    let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
        return false;
    };
    let Some(ref_id) = root.reference_id.get() else {
        return false;
    };
    let ReferenceSemantics::StoreRead { symbol } = analysis.reference_semantics(ref_id) else {
        return false;
    };
    let root_name = root.name.as_str().to_string();
    let base_name = analysis.scoping.symbol_name(symbol).to_string();
    rune_refs::replace_expr_root_in_assign_target(
        &mut assign.left,
        rune_refs::make_untrack(alloc, &root_name),
    );
    let placeholder = rune_refs::make_rune_get(alloc, "");
    let mutation = std::mem::replace(node, placeholder);
    let untracked = rune_refs::make_untrack(alloc, &root_name);
    *node = rune_refs::make_store_mutate(alloc, &base_name, mutation, untracked);
    true
}

/// Same as `rewrite_deep_store_member_assignment` but for UpdateExpression
/// (`$store.foo++`). Returns `true` when rewritten.
pub(crate) fn rewrite_deep_store_member_update<'a>(
    analysis: &AnalysisData<'a>,
    alloc: &'a Allocator,
    node: &mut Expression<'a>,
) -> bool {
    let Expression::UpdateExpression(upd) = node else {
        return false;
    };
    let Some(member) = upd.argument.as_member_expression() else {
        return false;
    };
    let Some(root) = rune_refs::find_expr_root_identifier(member.object()) else {
        return false;
    };
    let Some(ref_id) = root.reference_id.get() else {
        return false;
    };
    let ReferenceSemantics::StoreRead { symbol } = analysis.reference_semantics(ref_id) else {
        return false;
    };
    let root_name = root.name.as_str().to_string();
    let base_name = analysis.scoping.symbol_name(symbol).to_string();
    rune_refs::replace_expr_root_in_simple_target(
        &mut upd.argument,
        rune_refs::make_untrack(alloc, &root_name),
    );
    let placeholder = rune_refs::make_rune_get(alloc, "");
    let mutation = std::mem::replace(node, placeholder);
    let untracked = rune_refs::make_untrack(alloc, &root_name);
    *node = rune_refs::make_store_mutate(alloc, &base_name, mutation, untracked);
    true
}

/// Rewrite the three built-in call forms `$state.eager(...)`,
/// `$state.snapshot(...)`, and `$effect.pending(...)` that appear in both
/// template and script. The `dev_snapshot_uncloneable_ignored` flag
/// (script dev-mode only) controls whether to append the `true` marker
/// argument on `$state.snapshot`.
///
/// Returns `true` on match so the caller can skip further processing.
pub(crate) fn rewrite_shared_call<'a>(
    alloc: &'a Allocator,
    expr: &mut Expression<'a>,
    dev_snapshot_uncloneable_ignored: bool,
) -> bool {
    let Expression::CallExpression(call) = expr else {
        return false;
    };
    let Expression::StaticMemberExpression(member) = &call.callee else {
        return false;
    };
    let Expression::Identifier(obj) = &member.object else {
        return false;
    };
    match (obj.name.as_str(), member.property.name.as_str()) {
        ("$state", "eager") => {
            if let Expression::CallExpression(call) =
                std::mem::replace(expr, rune_refs::make_eager_pending(alloc))
            {
                let mut call = call.unbox();
                if !call.arguments.is_empty() {
                    let arg = call.arguments.remove(0).into_expression();
                    *expr = rune_refs::make_eager_thunk(alloc, arg);
                }
            }
            true
        }
        ("$state", "snapshot") => {
            let Expression::CallExpression(call) = expr else {
                unreachable!()
            };
            let ast = oxc_ast::AstBuilder::new(alloc);
            call.callee = rune_refs::make_dollar_member(&ast, "snapshot");
            if dev_snapshot_uncloneable_ignored {
                call.arguments.push(oxc_ast::ast::Argument::from(
                    ast.expression_boolean_literal(oxc_span::SPAN, true),
                ));
            }
            true
        }
        ("$effect", "pending") => {
            *expr = rune_refs::make_eager_pending(alloc);
            true
        }
        _ => false,
    }
}

/// Rest-prop member rewrite: `rest.xyz` → `$$props.xyz` where `rest` is
/// a `...rest` binding from `$props()` destructuring and `xyz` is not
/// a shadowed sibling. `is_lhs` tells the caller to skip rewrite when the
/// member is on the left-hand side of an assignment/update (caller computes
/// via `ctx.parent()` — kept out of this helper so it stays pure).
///
/// Returns `true` when the member's object was rewritten.
pub(crate) fn rewrite_rest_prop_member<'a>(
    analysis: &AnalysisData<'a>,
    alloc: &'a Allocator,
    expr: &mut Expression<'a>,
    is_lhs: bool,
) -> bool {
    if is_lhs {
        return false;
    }
    let Expression::StaticMemberExpression(member) = expr else {
        return false;
    };
    let Expression::Identifier(id) = &member.object else {
        return false;
    };
    let Some(ref_id) = id.reference_id.get() else {
        return false;
    };
    if !matches!(
        analysis.reference_semantics(ref_id),
        ReferenceSemantics::RestPropMemberRewrite
    ) {
        return false;
    }
    let ast = oxc_ast::AstBuilder::new(alloc);
    member.object = ast.expression_identifier(oxc_span::SPAN, ast.atom("$$props"));
    true
}
