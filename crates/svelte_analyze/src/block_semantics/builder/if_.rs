use super::super::{
    BlockSemantics, IfAlternate, IfAsyncKind, IfBlockSemantics, IfBranch, IfConditionKind,
};
use super::common::expression_if_facts;
use super::walker::Ctx;
use smallvec::SmallVec;
use svelte_ast::{IfBlock, Node};

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, block: &IfBlock) {
    ctx.visit_fragment(block.consequent);
    if let Some(alt) = block.alternate {
        ctx.visit_fragment(alt);
    }

    let (root_has_await, root_memoize, root_blockers) = test_facts(ctx, block);

    let mut branches: SmallVec<[IfBranch; 2]> = SmallVec::new();
    branches.push(IfBranch {
        block_id: block.id,
        condition: classify_condition(true, root_has_await, root_memoize),
    });

    let mut absorbed: SmallVec<[svelte_ast::NodeId; 2]> = SmallVec::new();
    let mut cur = block;
    let final_alternate = loop {
        let Some(alt) = &cur.alternate else {
            break IfAlternate::None;
        };

        let nested = elseif_child(ctx, *alt);
        let Some(nested) = nested else {
            break IfAlternate::Fragment {
                last_branch_block_id: cur.id,
            };
        };

        let (n_has_await, n_memoize, n_blockers) = test_facts(ctx, nested);

        let can_flatten = !n_has_await && is_blocker_subset(&n_blockers, &root_blockers);
        if !can_flatten {
            break IfAlternate::Fragment {
                last_branch_block_id: cur.id,
            };
        }

        branches.push(IfBranch {
            block_id: nested.id,
            condition: classify_condition(false, n_has_await, n_memoize),
        });
        absorbed.push(nested.id);
        cur = nested;
    };

    let async_kind = if !root_has_await && root_blockers.is_empty() {
        IfAsyncKind::Sync
    } else {
        IfAsyncKind::Async {
            root_has_await,
            blockers: root_blockers,
        }
    };

    ctx.store.set(
        block.id,
        BlockSemantics::If(IfBlockSemantics {
            branches,
            final_alternate,
            is_elseif_root: block.elseif,
            async_kind,
        }),
    );

    for id in absorbed {
        ctx.store.set(id, BlockSemantics::NonSpecial);
    }
}

fn test_facts(ctx: &Ctx<'_, '_>, block: &IfBlock) -> (bool, bool, SmallVec<[u32; 2]>) {
    let Some(expr) = ctx.parsed.expr(block.test.id()) else {
        return (false, false, SmallVec::new());
    };
    expression_if_facts(expr, ctx.semantics, ctx.blockers)
}

fn elseif_child<'c>(
    ctx: &'c Ctx<'_, '_>,
    fragment_id: svelte_ast::FragmentId,
) -> Option<&'c IfBlock> {
    let fragment = ctx.component.store.fragment(fragment_id);
    if fragment.nodes.len() != 1 {
        return None;
    }
    let node = ctx.component.store.get(fragment.nodes[0]);
    let Node::IfBlock(ib) = node else {
        return None;
    };
    ib.elseif.then_some(ib)
}

fn is_blocker_subset(nested: &[u32], root: &[u32]) -> bool {
    nested.iter().all(|b| root.contains(b))
}

fn classify_condition(is_root: bool, has_await: bool, memoize: bool) -> IfConditionKind {
    if is_root && has_await {
        IfConditionKind::AsyncParam
    } else if memoize {
        IfConditionKind::Memo
    } else {
        IfConditionKind::Raw
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_source;
    use crate::{BlockSemantics, IfAlternate, IfAsyncKind, IfBlockSemantics, IfConditionKind};
    use svelte_ast::{Component, IfBlock, Node, NodeId};

    fn all_if_blocks(component: &Component) -> Vec<NodeId> {
        fn walk(component: &Component, nodes: &[NodeId], out: &mut Vec<NodeId>) {
            for &id in nodes {
                let node = component.store.get(id);
                if let Node::IfBlock(b) = node {
                    out.push(id);
                    let cons = component.fragment_nodes(b.consequent).to_vec();
                    walk(component, &cons, out);
                    if let Some(alt) = b.alternate {
                        let alt_nodes = component.fragment_nodes(alt).to_vec();
                        walk(component, &alt_nodes, out);
                    }
                    continue;
                }
                let child_fragment: Option<svelte_ast::FragmentId> = match node {
                    Node::Element(el) => Some(el.fragment),
                    Node::ComponentNode(cn) => Some(cn.fragment),
                    Node::EachBlock(b) => Some(b.body),
                    Node::AwaitBlock(b) => {
                        if let Some(f) = b.pending {
                            let nodes = component.fragment_nodes(f).to_vec();
                            walk(component, &nodes, out);
                        }
                        if let Some(f) = b.then {
                            let nodes = component.fragment_nodes(f).to_vec();
                            walk(component, &nodes, out);
                        }
                        if let Some(f) = b.catch {
                            let nodes = component.fragment_nodes(f).to_vec();
                            walk(component, &nodes, out);
                        }
                        continue;
                    }
                    Node::SnippetBlock(b) => Some(b.body),
                    Node::KeyBlock(b) => Some(b.fragment),
                    Node::SvelteElement(el) => Some(el.fragment),
                    Node::SvelteBoundary(el) => Some(el.fragment),
                    _ => None,
                };
                if let Some(fid) = child_fragment {
                    let nodes = component.fragment_nodes(fid).to_vec();
                    walk(component, &nodes, out);
                }
            }
        }
        let mut out = Vec::new();
        let root_nodes = component.fragment_nodes(component.root).to_vec();
        walk(component, &root_nodes, &mut out);
        out
    }

    fn first_if_block(component: &Component) -> &IfBlock {
        let id = all_if_blocks(component)
            .into_iter()
            .next()
            .expect("no if block found");
        match component.store.get(id) {
            Node::IfBlock(b) => b,
            _ => unreachable!(),
        }
    }

    fn assert_if<F: FnOnce(&IfBlockSemantics)>(source: &str, check: F) {
        let (component, data) = analyze_source(source);
        let block = first_if_block(&component);
        let sem: &BlockSemantics = data.block_semantics(block.id);
        match sem {
            BlockSemantics::If(s) => check(s),
            other => panic!("expected If, got {other:?}"),
        }
    }

    #[test]
    fn if_simple_raw_condition() {
        assert_if(
            r#"<script>let { x } = $props();</script>{#if x}<p></p>{/if}"#,
            |sem| {
                assert_eq!(sem.branches.len(), 1);
                assert_eq!(sem.branches[0].condition, IfConditionKind::Raw);
                assert!(matches!(sem.final_alternate, IfAlternate::None));
                assert!(!sem.is_elseif_root);
                assert!(matches!(sem.async_kind, IfAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn if_else_fragment() {
        assert_if(
            r#"<script>let { x } = $props();</script>{#if x}<span></span>{:else}<span></span>{/if}"#,
            |sem| {
                assert_eq!(sem.branches.len(), 1);
                match &sem.final_alternate {
                    IfAlternate::Fragment {
                        last_branch_block_id,
                    } => {
                        assert_eq!(*last_branch_block_id, sem.branches[0].block_id);
                    }
                    other => panic!("expected Fragment, got {other:?}"),
                }
            },
        );
    }

    #[test]
    fn if_single_elseif_flattens() {
        assert_if(
            r#"<script>let { x, y } = $props();</script>{#if x}<span></span>{:else if y}<span></span>{/if}"#,
            |sem| {
                assert_eq!(sem.branches.len(), 2);
                assert_eq!(sem.branches[0].condition, IfConditionKind::Raw);
                assert_eq!(sem.branches[1].condition, IfConditionKind::Raw);
                assert!(matches!(sem.final_alternate, IfAlternate::None));
            },
        );
    }

    #[test]
    fn if_flat_chain_with_else() {
        assert_if(
            r#"<script>let { x, y, z } = $props();</script>
{#if x}<span></span>{:else if y}<span></span>{:else if z}<span></span>{:else}<span></span>{/if}"#,
            |sem| {
                assert_eq!(sem.branches.len(), 3);
                match &sem.final_alternate {
                    IfAlternate::Fragment {
                        last_branch_block_id,
                    } => {
                        assert_eq!(*last_branch_block_id, sem.branches[2].block_id);
                    }
                    other => panic!("expected Fragment on third branch, got {other:?}"),
                }
            },
        );
    }

    #[test]
    fn if_condition_needs_memo_on_call() {
        assert_if(
            r#"<script>function foo() { return true; }</script>{#if foo()}<p></p>{/if}"#,
            |sem| {
                assert_eq!(sem.branches[0].condition, IfConditionKind::Memo);
                assert!(matches!(sem.async_kind, IfAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn if_root_await_uses_async_param() {
        assert_if(
            r#"<script>let p = Promise.resolve(true);</script>{#if await p}<p></p>{/if}"#,
            |sem| {
                assert_eq!(sem.branches[0].condition, IfConditionKind::AsyncParam);
                match &sem.async_kind {
                    IfAsyncKind::Async {
                        root_has_await,
                        blockers,
                    } => {
                        assert!(*root_has_await);
                        assert!(blockers.is_empty());
                    }
                    other => panic!("expected Async, got {other:?}"),
                }
            },
        );
    }

    #[test]
    fn if_elseif_breaks_flattening_on_own_await() {
        let source = r#"<script>
let { x } = $props();
let q = Promise.resolve(true);
</script>{#if x}<span></span>{:else if await q}<span></span>{/if}"#;
        let (component, data) = analyze_source(source);
        let ids = all_if_blocks(&component);
        assert_eq!(ids.len(), 2);
        let root = ids[0];
        let inner = ids[1];

        match data.block_semantics(root) {
            BlockSemantics::If(sem) => {
                assert_eq!(sem.branches.len(), 1);
                match &sem.final_alternate {
                    IfAlternate::Fragment {
                        last_branch_block_id,
                    } => assert_eq!(*last_branch_block_id, root),
                    other => panic!("expected Fragment, got {other:?}"),
                }
            }
            other => panic!("root expected If, got {other:?}"),
        }

        match data.block_semantics(inner) {
            BlockSemantics::If(sem) => {
                assert!(sem.is_elseif_root);
                assert_eq!(sem.branches[0].condition, IfConditionKind::AsyncParam);
                assert!(matches!(
                    sem.async_kind,
                    IfAsyncKind::Async {
                        root_has_await: true,
                        ..
                    }
                ));
            }
            other => panic!("inner expected If, got {other:?}"),
        }
    }

    #[test]
    fn if_nested_real_if_not_absorbed() {
        let source = r#"<script>let { x, y } = $props();</script>
{#if x}<span></span>{:else}{#if y}<span></span>{/if}{/if}"#;
        let (component, data) = analyze_source(source);
        let ids = all_if_blocks(&component);
        assert_eq!(ids.len(), 2);
        let root = ids[0];
        let inner = ids[1];

        match data.block_semantics(root) {
            BlockSemantics::If(sem) => {
                assert_eq!(sem.branches.len(), 1);
                assert!(matches!(sem.final_alternate, IfAlternate::Fragment { .. }));
            }
            other => panic!("root expected If, got {other:?}"),
        }
        match data.block_semantics(inner) {
            BlockSemantics::If(sem) => assert!(!sem.is_elseif_root),
            other => panic!("inner expected If, got {other:?}"),
        }
    }

    #[test]
    fn if_absorbed_elseif_is_nonspecial() {
        let source = r#"<script>let { x, y } = $props();</script>
{#if x}<span></span>{:else if y}<span></span>{/if}"#;
        let (component, data) = analyze_source(source);
        let ids = all_if_blocks(&component);
        assert_eq!(ids.len(), 2);
        let root = ids[0];
        let absorbed = ids[1];
        match data.block_semantics(root) {
            BlockSemantics::If(sem) => assert_eq!(sem.branches.len(), 2),
            other => panic!("root expected If, got {other:?}"),
        }
        assert!(matches!(
            data.block_semantics(absorbed),
            BlockSemantics::NonSpecial
        ));
    }
}
