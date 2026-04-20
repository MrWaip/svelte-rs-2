//! `{#key}` population for Block Semantics.
//!
//! Free function invoked by the cluster-wide walker in [`super::walker`]:
//! given the shared `Ctx`, consume one `KeyBlock` — record its
//! `BlockSemantics::Key(...)` payload — then recurse into its body
//! fragment through the same walker so nested blocks of every migrated
//! kind get visited inside a single template walk.
//!
//! Shape is minimal: Key branches on a single async axis (the key
//! expression's `has_await` + blocker set). No flattening, no
//! per-branch data, no memoization (reference compiler never wraps the
//! key in `$.derived` — it is always a direct `() => expr` thunk).

use super::super::{BlockSemantics, KeyAsyncKind, KeyBlockSemantics};
use super::common::expression_async_facts;
use super::walker::Ctx;
use smallvec::SmallVec;
use svelte_ast::KeyBlock;

/// Populate `BlockSemantics::Key` for this block and recurse into its
/// body fragment.
pub(super) fn populate(ctx: &mut Ctx<'_, '_>, block: &KeyBlock) {
    // Walker owns recursion: descend into the body first so nested
    // blocks (of any migrated kind) populate inside the same walk.
    ctx.visit_fragment(&block.fragment.nodes);

    let (has_await, blockers) = match ctx
        .parsed
        .expr_handle(block.expression_span.start)
        .and_then(|h| ctx.parsed.expr(h))
    {
        Some(expr) => expression_async_facts(expr, ctx.semantics, ctx.blockers),
        None => (false, SmallVec::new()),
    };

    let async_kind = if !has_await && blockers.is_empty() {
        KeyAsyncKind::Sync
    } else {
        KeyAsyncKind::Async {
            has_await,
            blockers,
        }
    };

    ctx.store.set(
        block.id,
        BlockSemantics::Key(KeyBlockSemantics { async_kind }),
    );
}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_source;
    use crate::{BlockSemantics, KeyAsyncKind, KeyBlockSemantics};
    use svelte_ast::{Component, KeyBlock, Node};

    fn first_key_block(component: &Component) -> &KeyBlock {
        fn walk<'a>(
            component: &'a Component,
            nodes: &[svelte_ast::NodeId],
        ) -> Option<&'a KeyBlock> {
            for &id in nodes {
                let node = component.store.get(id);
                if let Node::KeyBlock(b) = node {
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
                    Node::SnippetBlock(b) => &b.body.nodes,
                    _ => continue,
                };
                if let Some(r) = walk(component, children) {
                    return Some(r);
                }
            }
            None
        }
        walk(component, &component.fragment.nodes).expect("no key block")
    }

    fn assert_key<F: FnOnce(&KeyBlockSemantics)>(source: &str, check: F) {
        let (component, data) = analyze_source(source);
        let block = first_key_block(&component);
        let sem: &BlockSemantics = data.block_semantics(block.id);
        match sem {
            BlockSemantics::Key(s) => check(s),
            other => panic!("expected Key, got {other:?}"),
        }
    }

    #[test]
    fn key_sync_identifier() {
        assert_key(
            r#"<script>let { x } = $props();</script>{#key x}<span></span>{/key}"#,
            |sem| {
                assert!(matches!(sem.async_kind, KeyAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn key_sync_member_expression() {
        assert_key(
            r#"<script>let { obj } = $props();</script>{#key obj.prop}<span></span>{/key}"#,
            |sem| {
                assert!(matches!(sem.async_kind, KeyAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn key_sync_computed_expression() {
        assert_key(
            r#"<script>let { count } = $props();</script>{#key count % 2}<span></span>{/key}"#,
            |sem| {
                assert!(matches!(sem.async_kind, KeyAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn key_async_awaited_expression() {
        assert_key(
            r#"<script>let p = Promise.resolve(1);</script>{#key await p}<span></span>{/key}"#,
            |sem| match &sem.async_kind {
                KeyAsyncKind::Async {
                    has_await,
                    blockers,
                } => {
                    assert!(*has_await);
                    assert!(blockers.is_empty());
                }
                other => panic!("expected Async, got {other:?}"),
            },
        );
    }
}
