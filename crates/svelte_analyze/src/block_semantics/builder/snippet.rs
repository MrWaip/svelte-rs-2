//! `{#snippet}` population for Block Semantics.
//!
//! Free function invoked by the cluster-wide walker in [`super::walker`]:
//! given the shared `Ctx`, consume one `SnippetBlock` — record its
//! `BlockSemantics::Snippet(...)` payload — then recurse into the body
//! fragment through the same walker so nested blocks of every migrated
//! kind are visited inside a single template walk.
//!
//! Scope boundary: this module owns **declaration-shape** facts only
//! (how the `const <name> = ($$anchor, ...) => {...}` is assembled).
//! Per-symbol **read-side** classification (`name()` vs `$.get(name)`)
//! lives in `reactivity_semantics::ContextualDeclarationSemantics::SnippetParam`.

use super::super::{
    BlockSemantics, SnippetBlockSemantics, SnippetDefaultKind, SnippetDestructureKind,
    SnippetParam, SnippetPatternBinding,
};
use super::common::{binding_pattern_node_id, declarator_from_stmt};
use super::walker::{Ctx, SnippetScope};
use crate::utils::is_simple_expression;
use oxc_ast::ast::{
    ArrowFunctionExpression, BindingPattern, Expression, FormalParameter, Statement,
    VariableDeclarator,
};
use smallvec::SmallVec;
use svelte_ast::{FragmentKey, SnippetBlock};
use svelte_component_semantics::SymbolId;

/// Populate `BlockSemantics::Snippet` for this block and recurse into
/// its body fragment.
pub(super) fn populate(ctx: &mut Ctx<'_, '_>, block: &SnippetBlock) {
    let stmt = ctx
        .parsed
        .stmt_handle(block.expression_span.start)
        .and_then(|h| ctx.parsed.stmt(h));

    let declarator = stmt.and_then(declarator_from_stmt);

    let name_sym = declarator.and_then(|d| match &d.id {
        BindingPattern::BindingIdentifier(ident) => ident.symbol_id.get(),
        _ => None,
    });

    let arrow = declarator.and_then(arrow_from_declarator);
    let params = arrow.map(|arrow| collect_params(arrow)).unwrap_or_default();

    // Top-level status is fixed by position in the walk: this snippet
    // sits at the component fragment root iff the walker hasn't
    // descended into any container yet. Capture the flag before
    // recursing into the body — the recursion will bump the counter.
    let top_level = ctx.non_root_depth == 0;

    // Recurse into body first so nested blocks are visited inside the
    // same template walk.
    ctx.visit_fragment(&block.body.nodes);

    let Some(name) = name_sym else {
        // Parser invariant: every snippet block has a named declarator.
        // If the pre-parsed statement is missing or malformed, skip
        // payload creation — the store keeps the default `NonSpecial`
        // and consumers will fall through to the legacy path.
        return;
    };

    // Seed `hoistable: false`; finalize in walker::populate flips it to
    // true for top-level snippets whose body has no instance-scope
    // references.
    ctx.store.set(
        block.id,
        BlockSemantics::Snippet(SnippetBlockSemantics {
            name,
            hoistable: false,
            params,
        }),
    );

    // Track snippet name symbols so finalize can exclude sibling-snippet
    // calls from the hoistable taint set.
    ctx.snippet_name_syms.insert(name);

    // Register this snippet's body scope so the post-walk hoistable pass
    // can trace references back to the owning snippet.
    if let Some(body_scope) = ctx
        .semantics
        .fragment_scope(&FragmentKey::SnippetBody(block.id))
    {
        ctx.snippet_scopes.push(SnippetScope {
            block_id: block.id,
            body_scope,
            top_level,
        });
    }
}

fn arrow_from_declarator<'a>(
    decl: &'a VariableDeclarator<'a>,
) -> Option<&'a ArrowFunctionExpression<'a>> {
    match decl.init.as_ref()? {
        Expression::ArrowFunctionExpression(arrow) => Some(arrow.as_ref()),
        _ => None,
    }
}

fn collect_params<'a>(arrow: &ArrowFunctionExpression<'a>) -> SmallVec<[SnippetParam; 4]> {
    let mut out: SmallVec<[SnippetParam; 4]> = SmallVec::new();
    for param in &arrow.params.items {
        if let Some(classified) = classify_param(param) {
            out.push(classified);
        }
    }
    out
}

/// Classify one `FormalParameter` into a `SnippetParam`. The outer
/// wrapper `FormalParameter.pattern` may be an `AssignmentPattern` when
/// the parameter has a top-level default (`(name = 5)` or
/// `({ a } = fallback)`); in that case we peel it off once to expose
/// the inner identifier / destructure shape.
fn classify_param<'a>(param: &FormalParameter<'a>) -> Option<SnippetParam> {
    // `FormalParameter.initializer` (OXC's form for `(x = default)`) and
    // the `AssignmentPattern` wrapper (for `({ a } = fallback)`) are
    // both peeled here so the match below sees the inner shape. Only
    // destructured patterns carry their default through — identifier
    // params drop it at lowering time per the reference compiler.
    let pattern = match &param.pattern {
        BindingPattern::AssignmentPattern(assign) => &assign.left,
        other => other,
    };

    match pattern {
        BindingPattern::BindingIdentifier(ident) => {
            let sym = ident.symbol_id.get()?;
            Some(SnippetParam::Identifier { sym })
        }
        BindingPattern::ObjectPattern(_) => Some(SnippetParam::Pattern {
            kind: SnippetDestructureKind::Object,
            pattern_id: binding_pattern_node_id(pattern),
            bindings: collect_pattern_bindings(pattern),
        }),
        BindingPattern::ArrayPattern(_) => Some(SnippetParam::Pattern {
            kind: SnippetDestructureKind::Array,
            pattern_id: binding_pattern_node_id(pattern),
            bindings: collect_pattern_bindings(pattern),
        }),
        // Nested AssignmentPattern inside AssignmentPattern isn't legal
        // in JS grammar — treat as no-op.
        BindingPattern::AssignmentPattern(_) => None,
    }
}

fn collect_pattern_bindings<'a>(
    pattern: &BindingPattern<'a>,
) -> SmallVec<[SnippetPatternBinding; 4]> {
    let mut out: SmallVec<[SnippetPatternBinding; 4]> = SmallVec::new();
    walk_pattern(pattern, false, SnippetDefaultKind::None, &mut out);
    out
}

fn walk_pattern<'a>(
    pattern: &BindingPattern<'a>,
    is_rest: bool,
    inherited_default: SnippetDefaultKind,
    out: &mut SmallVec<[SnippetPatternBinding; 4]>,
) {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => {
            if let Some(sym) = ident.symbol_id.get() {
                out.push(SnippetPatternBinding {
                    sym,
                    default: inherited_default,
                    is_rest,
                });
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            let d = default_kind_of(&assign.right);
            walk_pattern(&assign.left, is_rest, d, out);
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                walk_pattern(&prop.value, false, SnippetDefaultKind::None, out);
            }
            if let Some(rest) = &obj.rest {
                collect_rest_target(&rest.argument, /* is_rest */ true, out);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for el in arr.elements.iter().flatten() {
                walk_pattern(el, false, SnippetDefaultKind::None, out);
            }
            if let Some(rest) = &arr.rest {
                // Array rest isn't flagged as `is_rest` in the payload:
                // consumer emits it via array-slice, not via
                // exclude_from_object. Only the object-rest case needs
                // the flag. Keep the leaf symbol so name order stays
                // correct for downstream consumers.
                collect_rest_target(&rest.argument, /* is_rest */ false, out);
            }
        }
    }
}

fn collect_rest_target<'a>(
    target: &BindingPattern<'a>,
    is_rest: bool,
    out: &mut SmallVec<[SnippetPatternBinding; 4]>,
) {
    match target {
        BindingPattern::BindingIdentifier(ident) => {
            if let Some(sym) = ident.symbol_id.get() {
                out.push(SnippetPatternBinding {
                    sym,
                    default: SnippetDefaultKind::None,
                    is_rest,
                });
            }
        }
        // `{ ...{ a } }` / `[ ...[a] ]` — destructured rest. Recurse
        // without rest flag; leaves become regular bindings.
        _ => walk_pattern(target, false, SnippetDefaultKind::None, out),
    }
}

fn default_kind_of(expr: &Expression<'_>) -> SnippetDefaultKind {
    if is_simple_expression(expr) {
        SnippetDefaultKind::Constant
    } else {
        SnippetDefaultKind::Computed
    }
}

// Suppress unused warnings from Statement / SymbolId imports when the
// module's tests module isn't yet populated — referenced via type
// aliases above.
#[allow(dead_code)]
fn _ensure_imports_used(_: &Statement<'_>, _: SymbolId) {}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_source;
    use crate::{
        BlockSemantics, SnippetBlockSemantics, SnippetDefaultKind, SnippetDestructureKind,
        SnippetParam,
    };
    use svelte_ast::{Component, Node, SnippetBlock};

    fn first_snippet(component: &Component) -> &SnippetBlock {
        fn walk<'a>(
            component: &'a Component,
            nodes: &[svelte_ast::NodeId],
        ) -> Option<&'a SnippetBlock> {
            for &id in nodes {
                let node = component.store.get(id);
                if let Node::SnippetBlock(b) = node {
                    return Some(b);
                }
                let children: &[svelte_ast::NodeId] = match node {
                    Node::Element(el) => &el.fragment.nodes,
                    Node::IfBlock(b) => {
                        if let Some(r) = walk(component, &b.consequent.nodes) {
                            return Some(r);
                        }
                        if let Some(alt) = &b.alternate {
                            if let Some(r) = walk(component, &alt.nodes) {
                                return Some(r);
                            }
                        }
                        continue;
                    }
                    Node::EachBlock(b) => &b.body.nodes,
                    _ => continue,
                };
                if let Some(r) = walk(component, children) {
                    return Some(r);
                }
            }
            None
        }
        walk(component, &component.fragment.nodes).expect("no snippet block")
    }

    fn with_snippet<F: FnOnce(&SnippetBlockSemantics)>(source: &str, check: F) {
        let (component, data) = analyze_source(source);
        let block = first_snippet(&component);
        let sem: &BlockSemantics = data.block_semantics(block.id);
        match sem {
            BlockSemantics::Snippet(s) => check(s),
            other => panic!("expected Snippet, got {other:?}"),
        }
    }

    #[test]
    fn snippet_plain_ident() {
        with_snippet(r#"{#snippet row(item)}<p>{item()}</p>{/snippet}"#, |sem| {
            assert!(sem.hoistable);
            assert_eq!(sem.params.len(), 1);
            assert!(matches!(sem.params[0], SnippetParam::Identifier { .. }));
        });
    }

    #[test]
    fn snippet_object_destructure() {
        with_snippet(
            r#"{#snippet row({ a, b })}<p>{a} {b}</p>{/snippet}"#,
            |sem| match &sem.params[0] {
                SnippetParam::Pattern { kind, bindings, .. } => {
                    assert_eq!(*kind, SnippetDestructureKind::Object);
                    assert_eq!(bindings.len(), 2);
                    assert_eq!(bindings[0].default, SnippetDefaultKind::None);
                    assert!(!bindings[0].is_rest);
                }
                other => panic!("expected Pattern, got {other:?}"),
            },
        );
    }

    #[test]
    fn snippet_object_destructure_with_default() {
        with_snippet(
            r#"{#snippet row({ a = 1, b })}<p>{a} {b}</p>{/snippet}"#,
            |sem| match &sem.params[0] {
                SnippetParam::Pattern { bindings, .. } => {
                    assert_eq!(bindings[0].default, SnippetDefaultKind::Constant);
                    assert_eq!(bindings[1].default, SnippetDefaultKind::None);
                }
                other => panic!("expected Pattern, got {other:?}"),
            },
        );
    }

    #[test]
    fn snippet_object_destructure_with_rest() {
        with_snippet(
            r#"{#snippet row({ a, ...rest })}<p>{a}</p>{/snippet}"#,
            |sem| match &sem.params[0] {
                SnippetParam::Pattern { bindings, .. } => {
                    assert_eq!(bindings.len(), 2);
                    assert!(!bindings[0].is_rest);
                    assert!(bindings[1].is_rest);
                }
                other => panic!("expected Pattern, got {other:?}"),
            },
        );
    }

    #[test]
    fn snippet_array_destructure() {
        with_snippet(
            r#"{#snippet row([x, y])}<p>{x} {y}</p>{/snippet}"#,
            |sem| match &sem.params[0] {
                SnippetParam::Pattern { kind, bindings, .. } => {
                    assert_eq!(*kind, SnippetDestructureKind::Array);
                    assert_eq!(bindings.len(), 2);
                }
                other => panic!("expected Pattern, got {other:?}"),
            },
        );
    }

    #[test]
    fn snippet_mixed_params() {
        with_snippet(
            r#"{#snippet row(label, { id }, [value])}<p>{label()} {id} {value}</p>{/snippet}"#,
            |sem| {
                assert_eq!(sem.params.len(), 3);
                assert!(matches!(sem.params[0], SnippetParam::Identifier { .. }));
                assert!(matches!(
                    sem.params[1],
                    SnippetParam::Pattern {
                        kind: SnippetDestructureKind::Object,
                        ..
                    }
                ));
                assert!(matches!(
                    sem.params[2],
                    SnippetParam::Pattern {
                        kind: SnippetDestructureKind::Array,
                        ..
                    }
                ));
            },
        );
    }

    #[test]
    fn snippet_hoistable_top_level() {
        with_snippet(r#"{#snippet row(a)}<p>{a()}</p>{/snippet}"#, |sem| {
            assert!(sem.hoistable);
        });
    }

    #[test]
    fn snippet_non_hoistable_nested() {
        with_snippet(
            r#"{#if true}{#snippet row(a)}<p>{a()}</p>{/snippet}{/if}"#,
            |sem| {
                assert!(!sem.hoistable);
            },
        );
    }

    #[test]
    fn snippet_non_hoistable_script_ref() {
        with_snippet(
            r#"<script>let x = $state(10);</script>
{#snippet row(a = x)}<p>{a()}</p>{/snippet}"#,
            |sem| {
                assert!(!sem.hoistable);
            },
        );
    }
}
