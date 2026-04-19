//! `{#await}` population for Block Semantics.
//!
//! Free function invoked by the cluster-wide walker in [`super::walker`]:
//! given the shared `Ctx`, consume one `AwaitBlock` — record its
//! `BlockSemantics::Await(...)` payload — then recurse into its child
//! fragments through the same walker so nested blocks of every migrated
//! kind get visited inside a single template walk.

use super::super::{
    AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
    BlockSemantics,
};
use super::common::{
    binding_ident_of, binding_pattern_node_id, declarator_from_stmt, expression_async_facts,
};
use super::walker::Ctx;
use oxc_ast::ast::BindingPattern;
use smallvec::SmallVec;
use svelte_ast::{AwaitBlock, FragmentKey};
use svelte_component_semantics::{walk_bindings, SymbolId};

/// Populate `BlockSemantics::Await` for this block and recurse into its
/// pending / then / catch fragments.
pub(super) fn populate(ctx: &mut Ctx<'_, '_>, block: &AwaitBlock) {
    let pending = if block.pending.is_some() {
        AwaitBranch::Present {
            binding: AwaitBinding::None,
        }
    } else {
        AwaitBranch::Absent
    };
    let then = resolve_branch(
        ctx,
        block.then.is_some(),
        block.value_span,
        FragmentKey::AwaitThen(block.id),
    );
    let catch = resolve_branch(
        ctx,
        block.catch.is_some(),
        block.error_span,
        FragmentKey::AwaitCatch(block.id),
    );

    let (expression_has_await, blockers) = match ctx
        .parsed
        .expr_handle(block.expression_span.start)
        .and_then(|h| ctx.parsed.expr(h))
    {
        Some(expr) => expression_async_facts(expr, ctx.semantics, ctx.blockers),
        None => (false, SmallVec::new()),
    };
    let wrapper = if blockers.is_empty() {
        AwaitWrapper::None
    } else {
        AwaitWrapper::AsyncWrap { blockers }
    };

    // Recurse first so nested blocks (of any migrated kind) populate inside
    // the same template walk.
    if let Some(f) = &block.pending {
        ctx.visit_fragment(FragmentKey::AwaitPending(block.id), &f.nodes);
    }
    if let Some(f) = &block.then {
        ctx.visit_fragment(FragmentKey::AwaitThen(block.id), &f.nodes);
    }
    if let Some(f) = &block.catch {
        ctx.visit_fragment(FragmentKey::AwaitCatch(block.id), &f.nodes);
    }

    ctx.store.set(
        block.id,
        BlockSemantics::Await(AwaitBlockSemantics {
            pending,
            then,
            catch,
            expression_has_await,
            wrapper,
        }),
    );
}

/// Resolve a `{:then}` / `{:catch}` branch. `has_fragment` captures
/// branch presence; the optional binding pattern is pulled from the
/// pre-parsed `let <pattern>` statement at `binding_span`.
fn resolve_branch(
    ctx: &Ctx<'_, '_>,
    has_fragment: bool,
    binding_span: Option<svelte_span::Span>,
    scope_key: FragmentKey,
) -> AwaitBranch {
    if !has_fragment {
        return AwaitBranch::Absent;
    }
    let binding = match binding_span {
        None => AwaitBinding::None,
        Some(span) => ctx
            .parsed
            .stmt_handle(span.start)
            .and_then(|h| ctx.parsed.stmt(h))
            .and_then(declarator_from_stmt)
            .map(|decl| binding_from_pattern(ctx, &decl.id, scope_key))
            .unwrap_or(AwaitBinding::None),
    };
    AwaitBranch::Present { binding }
}

fn binding_from_pattern<'a>(
    ctx: &Ctx<'_, 'a>,
    pattern: &BindingPattern<'a>,
    scope_key: FragmentKey,
) -> AwaitBinding {
    if let Some(ident) = binding_ident_of(pattern) {
        return ctx
            .semantics
            .fragment_scope(&scope_key)
            .and_then(|scope| ctx.semantics.find_binding(scope, ident.name.as_str()))
            .map(AwaitBinding::Identifier)
            .unwrap_or(AwaitBinding::None);
    }
    let kind = match pattern {
        BindingPattern::ObjectPattern(_) => AwaitDestructureKind::Object,
        BindingPattern::ArrayPattern(_) => AwaitDestructureKind::Array,
        _ => return AwaitBinding::None,
    };
    let mut leaves: SmallVec<[SymbolId; 4]> = SmallVec::new();
    walk_bindings(pattern, |v| leaves.push(v.symbol));
    AwaitBinding::Pattern {
        kind,
        leaves,
        pattern_id: binding_pattern_node_id(pattern),
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_source;
    use crate::{
        AwaitBinding, AwaitBlockSemantics, AwaitBranch, AwaitDestructureKind, AwaitWrapper,
        BlockSemantics,
    };
    use svelte_ast::{AwaitBlock, Component, Node};

    fn first_await_block(component: &Component) -> &AwaitBlock {
        fn walk<'a>(
            component: &'a Component,
            nodes: &[svelte_ast::NodeId],
        ) -> Option<&'a AwaitBlock> {
            for &id in nodes {
                let node = component.store.get(id);
                if let Node::AwaitBlock(b) = node {
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
                    _ => continue,
                };
                if let Some(r) = walk(component, children) {
                    return Some(r);
                }
            }
            None
        }
        walk(component, &component.fragment.nodes).expect("no await block")
    }

    fn assert_await<F: FnOnce(&AwaitBlockSemantics)>(source: &str, check: F) {
        let (component, data) = analyze_source(source);
        let block = first_await_block(&component);
        let sem: &BlockSemantics = data.block_semantics(block.id);
        match sem {
            BlockSemantics::Await(s) => check(s),
            other => panic!("expected Await, got {other:?}"),
        }
    }

    #[test]
    fn await_plain_pending_only() {
        assert_await(
            r#"<script>let p = fetch('/x');</script>{#await p}loading{/await}"#,
            |sem| {
                assert!(matches!(
                    sem.pending,
                    AwaitBranch::Present {
                        binding: AwaitBinding::None
                    }
                ));
                assert!(matches!(sem.then, AwaitBranch::Absent));
                assert!(matches!(sem.catch, AwaitBranch::Absent));
                assert!(!sem.expression_has_await);
                assert!(matches!(sem.wrapper, AwaitWrapper::None));
            },
        );
    }

    #[test]
    fn await_then_ident() {
        assert_await(
            r#"<script>let p = fetch('/x');</script>{#await p then v}{v}{/await}"#,
            |sem| {
                assert!(matches!(sem.pending, AwaitBranch::Absent));
                match &sem.then {
                    AwaitBranch::Present {
                        binding: AwaitBinding::Identifier(_),
                    } => {}
                    other => panic!("expected then Identifier, got {other:?}"),
                }
                assert!(matches!(sem.catch, AwaitBranch::Absent));
            },
        );
    }

    #[test]
    fn await_catch_ident() {
        assert_await(
            r#"<script>let p = fetch('/x');</script>{#await p}...{:catch e}{e}{/await}"#,
            |sem| match &sem.catch {
                AwaitBranch::Present {
                    binding: AwaitBinding::Identifier(_),
                } => {}
                other => panic!("expected catch Identifier, got {other:?}"),
            },
        );
    }

    #[test]
    fn await_then_destructured_object() {
        assert_await(
            r#"<script>let p = fetch('/x');</script>{#await p then { a, b }}{a}{b}{/await}"#,
            |sem| match &sem.then {
                AwaitBranch::Present {
                    binding: AwaitBinding::Pattern { kind, leaves, .. },
                } => {
                    assert_eq!(*kind, AwaitDestructureKind::Object);
                    assert_eq!(leaves.len(), 2);
                }
                other => panic!("expected then destructured, got {other:?}"),
            },
        );
    }

    #[test]
    fn await_catch_destructured_array() {
        assert_await(
            r#"<script>let p = fetch('/x');</script>{#await p}...{:catch [msg]}{msg}{/await}"#,
            |sem| match &sem.catch {
                AwaitBranch::Present {
                    binding: AwaitBinding::Pattern { kind, leaves, .. },
                } => {
                    assert_eq!(*kind, AwaitDestructureKind::Array);
                    assert_eq!(leaves.len(), 1);
                }
                other => panic!("expected catch destructured array, got {other:?}"),
            },
        );
    }

    #[test]
    fn await_no_blockers_has_no_wrapper() {
        assert_await(
            r#"<script>let p = fetch('/x');</script>{#await p}...{/await}"#,
            |sem| {
                assert!(matches!(sem.wrapper, AwaitWrapper::None));
                assert!(!sem.expression_has_await);
            },
        );
    }
}
