//! Template-mode expression rewrites, keyed purely on
//! `ReferenceSemantics` / `reference_id`.
//!
//! Runs from `ComponentTransformer::enter_expression` when
//! `mode == Template`. Newly generated nodes (e.g. the `name` identifier
//! inside `$.get(name)`) have `reference_id = None`, so all branches here
//! are keyed by `id.reference_id.get()` and short-circuit when absent —
//! that's what prevents Traverse from infinitely recursing into the
//! rewritten sub-tree.
//!
//! Important: no legacy `find_binding(scope, name)` fallback. The VisitMut
//! implementation carried one; in the Traverse flow it would re-match the
//! newly-synthesized identifier by name and drive the transformer into
//! infinite recursion, and empirically reference_id is populated on every
//! template identifier the analyzer knows about. Any missed rewrite is an
//! upstream analyze bug — fix it there.

use oxc_ast::ast::Expression;

use svelte_analyze::{PropDeclarationKind, PropDeclarationSemantics};

use crate::rune_refs;

use super::model::ComponentTransformer;

pub(crate) fn rewrite_template_enter<'a>(
    t: &mut ComponentTransformer<'_, 'a>,
    it: &mut Expression<'a>,
    is_lhs: bool,
) {
    let analysis = t.analysis.expect("Template mode requires analysis");
    let alloc = t.b.ast.allocator;

    // Rest prop member access — shared with Script via
    // `rewrites::rewrite_rest_prop_member`. Returns on match so Traverse
    // does not redescend into the rewritten sub-tree.
    if super::rewrites::rewrite_rest_prop_member(analysis, alloc, it, is_lhs) {
        return;
    }


    // Identifier reads — shared with Script via `rewrites::rewrite_identifier_read`.
    // Newly-synthesized identifiers inside `$.get(name)` etc. have
    // `reference_id = None`, so the helper short-circuits and Traverse
    // doesn't recurse into freshly-written sub-trees.
    if matches!(it, Expression::Identifier(_)) {
        if super::rewrites::rewrite_identifier_read(analysis, alloc, &t.transform_data, it) {
            return;
        }

        // Legacy v1 fallback for cases where v2 `ReferenceSemantics` doesn't
        // yet cover the read (e.g. snippet parameter defaults). Uses the
        // `reference_id`-keyed symbol (NOT name-based `find_binding`) so that
        // synthesized identifiers — which have no reference_id — are skipped.
        let Expression::Identifier(id) = it else {
            unreachable!()
        };
        let name = id.name.as_str();
        let Some(sym_id) = analysis.symbol_for_identifier_reference(id) else {
            return;
        };
        if matches!(
            analysis.declaration_semantics(analysis.scoping.symbol_declaration(sym_id)),
            DeclarationSemantics::Prop(PropDeclarationSemantics {
                kind: PropDeclarationKind::NonSource,
                ..
            })
        ) {
            if let Some(prop_name) = analysis.binding_origin_key(sym_id) {
                *it = rune_refs::make_props_access(alloc, prop_name);
            }
            return;
        }

        // v2 emission dispatch by declaration kind + contextual side-tables.
        // Covers the cases that `rewrite_identifier_read` (v2 reference-semantics
        // path) doesn't reach — currently snippet default expressions and some
        // synthesized identifiers.
        use svelte_analyze::{
            ConstDeclarationSemantics, ContextualDeclarationSemantics, DeclarationSemantics,
        };
        let decl = analysis.declaration_semantics(analysis.scoping.symbol_declaration(sym_id));
        match decl {
            DeclarationSemantics::State(state) => {
                if state.var_declared {
                    *it = rune_refs::make_rune_safe_get(alloc, name);
                } else if analysis.scoping.is_mutated(sym_id) {
                    *it = rune_refs::make_rune_get(alloc, name);
                }
            }
            DeclarationSemantics::Derived(_) => {
                *it = rune_refs::make_rune_get(alloc, name);
            }
            DeclarationSemantics::Store(_) => {
                *it = rune_refs::make_rune_get(alloc, name);
            }
            DeclarationSemantics::Const(ConstDeclarationSemantics::ConstTag { destructured, .. }) => {
                if destructured {
                    *it = rune_refs::make_rune_safe_get(alloc, name);
                } else {
                    *it = rune_refs::make_rune_get(alloc, name);
                }
            }
            DeclarationSemantics::Contextual(kind) => {
                use svelte_analyze::{EachIndexStrategy, EachItemStrategy, SnippetParamStrategy};
                match kind {
                    ContextualDeclarationSemantics::EachItem(EachItemStrategy::Accessor)
                    | ContextualDeclarationSemantics::SnippetParam(SnippetParamStrategy::Accessor) => {
                        *it = rune_refs::make_thunk_call(alloc, name);
                    }
                    ContextualDeclarationSemantics::EachItem(EachItemStrategy::Direct)
                    | ContextualDeclarationSemantics::EachIndex(EachIndexStrategy::Direct) => {}
                    ContextualDeclarationSemantics::EachItem(EachItemStrategy::Signal)
                    | ContextualDeclarationSemantics::EachIndex(EachIndexStrategy::Signal)
                    | ContextualDeclarationSemantics::AwaitValue
                    | ContextualDeclarationSemantics::AwaitError
                    | ContextualDeclarationSemantics::SnippetParam(SnippetParamStrategy::Signal)
                    | ContextualDeclarationSemantics::LetDirective => {
                        *it = rune_refs::make_rune_get(alloc, name);
                    }
                }
            }
            _ => {}
        }
        return;
    }

    // UpdateExpression: rune/store identifier update + deep store mutate.
    // Both delegate to shared helpers in `rewrites.rs` — script's
    // `transform_update` calls the same helpers from its own enter path.
    if matches!(it, Expression::UpdateExpression(_)) {
        if super::rewrites::rewrite_signal_or_store_identifier_update(analysis, alloc, it) {
            return;
        }
        // Deep store update: `$store.count++` — must run on enter while the
        // original `$store` root identifier is still present (child descent
        // would otherwise rewrite it to `$store()` and we'd lose the store
        // symbol link).
        super::rewrites::rewrite_deep_store_member_update(analysis, alloc, it);
        return;
    }

    // Deep store member assignment: `$store.field = val`.
    // Same pre-walk reason as the update variant above.
    if matches!(it, Expression::AssignmentExpression(_)) {
        super::rewrites::rewrite_deep_store_member_assignment(analysis, alloc, it);
    }

    // CallExpression special cases — handled in exit (see `rewrite_template_exit`).
}

/// Exit-time CallExpression rewrites ($state.eager/snapshot, $effect.pending).
/// Done in exit so Traverse has already visited child expressions (rune refs
/// inside the call arguments got rewritten on descent), and the freshly-
/// synthesized arrow function we wrap the argument in isn't revisited by
/// the outer Traverse (which would panic on its missing scope_id).
/// Post-walk: AssignmentExpression with identifier LHS (signal/store writes)
/// + shared CallExpression rewrites ($state.eager, $state.snapshot, $effect.pending).
///
/// Call rewrites run in exit (not enter) for the same reason they did before:
/// the freshly-synthesized `() => arg` arrow for `$state.eager` carries no
/// `scope_id`, which would panic the outer Traverse on the next descent.
pub(crate) fn rewrite_template_exit<'a>(t: &mut ComponentTransformer<'_, 'a>, it: &mut Expression<'a>) {
    // Template is never dev-mode, so snapshot-uncloneable flag is always false.
    super::rewrites::rewrite_shared_call(t.b.ast.allocator, it, false);

    let analysis = t.analysis.expect("Template mode requires analysis");
    let alloc = t.b.ast.allocator;

    // AwaitExpression rewrite (moved from codegen's AwaitExprFinalizer).
    // Pickled awaits and dev await-reactivity-loss tracking wrap the await
    // in an IIFE; the wrapping is a template-phase AST transformation, not
    // a codegen concern.
    if let Expression::AwaitExpression(await_expr) = it {
        let ignored = t
            .template_owner_node
            .is_some_and(|id| analysis.output.ignore_data.is_ignored(id, "await_reactivity_loss"));
        let is_pickled = analysis.is_pickled_await(await_expr.span.start);

        let ast = oxc_ast::AstBuilder::new(alloc);
        let arg = std::mem::replace(
            &mut await_expr.argument,
            ast.expression_identifier(oxc_span::SPAN, ast.atom("")),
        );

        if is_pickled {
            // (await $.save(arg))()
            let save_call = rune_refs::make_dollar_call(alloc, "save", arg);
            await_expr.argument = save_call;
            let Expression::AwaitExpression(_) = &*it else { unreachable!() };
            let awaited = std::mem::replace(it, ast.expression_identifier(oxc_span::SPAN, ast.atom("")));
            *it = ast.expression_call(
                oxc_span::SPAN,
                awaited,
                oxc_ast::NONE,
                ast.vec(),
                false,
            );
            return;
        } else if t.dev && !ignored {
            // (await $.track_reactivity_loss(arg))()
            let track_call = rune_refs::make_dollar_call(alloc, "track_reactivity_loss", arg);
            await_expr.argument = track_call;
            let Expression::AwaitExpression(_) = &*it else { unreachable!() };
            let awaited = std::mem::replace(it, ast.expression_identifier(oxc_span::SPAN, ast.atom("")));
            *it = ast.expression_call(
                oxc_span::SPAN,
                awaited,
                oxc_ast::NONE,
                ast.vec(),
                false,
            );
            return;
        } else {
            // Restore the original argument (no wrapping).
            await_expr.argument = arg;
            return;
        }
    }

    // Signal/store identifier assignment — shared with Script (same helper).
    let suppress_proxy = t.in_bind_setter_traverse;
    super::rewrites::rewrite_signal_or_store_identifier_assignment(
        analysis,
        alloc,
        it,
        suppress_proxy,
    );
}
