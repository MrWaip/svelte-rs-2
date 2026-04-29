use super::super::{BlockSemantics, KeyAsyncKind, KeyBlockSemantics};
use super::common::expression_async_facts;
use super::walker::Ctx;
use smallvec::SmallVec;
use svelte_ast::KeyBlock;

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, block: &KeyBlock) {
    ctx.visit_fragment(block.fragment);

    let (has_await, blockers) = match ctx.parsed.expr(block.expression.id()) {
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
                let child_fragment = match node {
                    Node::Element(el) => Some(el.fragment),
                    Node::IfBlock(b) => {
                        let cons = component.fragment_nodes(b.consequent).to_vec();
                        if let Some(r) = walk(component, &cons) {
                            return Some(r);
                        }
                        if let Some(alt) = b.alternate {
                            let alt_nodes = component.fragment_nodes(alt).to_vec();
                            if let Some(r) = walk(component, &alt_nodes) {
                                return Some(r);
                            }
                        }
                        continue;
                    }
                    Node::EachBlock(b) => Some(b.body),
                    Node::SnippetBlock(b) => Some(b.body),
                    _ => continue,
                };
                if let Some(fid) = child_fragment {
                    let nodes = component.fragment_nodes(fid).to_vec();
                    if let Some(r) = walk(component, &nodes) {
                        return Some(r);
                    }
                }
            }
            None
        }
        let root_nodes = component.fragment_nodes(component.root).to_vec();
        walk(component, &root_nodes).expect("no key block")
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
