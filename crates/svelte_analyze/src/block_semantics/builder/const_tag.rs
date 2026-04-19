//! `{@const}` population for Block Semantics.
//!
//! Free function invoked by the cluster-wide walker in [`super::walker`]:
//! given the shared `Ctx`, consume one `ConstTag` node and record its
//! `BlockSemantics::ConstTag(...)` payload. `{@const}` does not own a
//! fragment of its own, so no recursion is required after populate.
//!
//! Scope boundary: this module owns **declaration-shape** facts only —
//! which symbols the tag introduces, whether the pattern is destructured,
//! how the init expression interacts with async. Per-symbol read-side
//! classification (`$.get` vs `$.safe_get` for the bindings themselves)
//! lives in `reactivity_semantics::ConstDeclarationSemantics::ConstTag`.

use super::super::{BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics};
use super::common::{declarator_from_stmt, expression_async_facts};
use super::walker::Ctx;
use oxc_ast::ast::BindingPattern;
use smallvec::SmallVec;
use svelte_ast::ConstTag;
use svelte_component_semantics::walk_bindings;

/// Populate `BlockSemantics::ConstTag` for this tag.
pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &ConstTag) {
    let Some(stmt_handle) = ctx.parsed.stmt_handle(tag.expression_span.start) else {
        return;
    };
    let Some(stmt) = ctx.parsed.stmt(stmt_handle) else {
        return;
    };
    let Some(declarator) = declarator_from_stmt(stmt) else {
        return;
    };

    let is_destructured = !matches!(declarator.id, BindingPattern::BindingIdentifier(_));
    let mut bindings: SmallVec<[_; 2]> = SmallVec::new();
    walk_bindings(&declarator.id, |v| bindings.push(v.symbol));
    if bindings.is_empty() {
        return;
    }

    let Some(init) = declarator.init.as_ref() else {
        return;
    };
    let (has_await, blockers) = expression_async_facts(init, ctx.semantics, ctx.blockers);
    let async_kind = if !has_await && blockers.is_empty() {
        ConstTagAsyncKind::Sync
    } else {
        ConstTagAsyncKind::Async {
            has_await,
            blockers,
        }
    };

    ctx.store.set(
        tag.id,
        BlockSemantics::ConstTag(ConstTagBlockSemantics {
            bindings,
            is_destructured,
            stmt_handle,
            async_kind,
        }),
    );
}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_source;
    use crate::{BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics};
    use svelte_ast::{Component, ConstTag, Node};

    fn first_const_tag(component: &Component) -> &ConstTag {
        fn walk<'a>(
            component: &'a Component,
            nodes: &[svelte_ast::NodeId],
        ) -> Option<&'a ConstTag> {
            for &id in nodes {
                let node = component.store.get(id);
                if let Node::ConstTag(tag) = node {
                    return Some(tag);
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
        walk(component, &component.fragment.nodes).expect("no const tag")
    }

    fn with_const_tag<F: FnOnce(&ConstTagBlockSemantics)>(source: &str, check: F) {
        let (component, data) = analyze_source(source);
        let tag = first_const_tag(&component);
        match data.block_semantics(tag.id) {
            BlockSemantics::ConstTag(s) => check(s),
            other => panic!("expected ConstTag, got {other:?}"),
        }
    }

    // `{@const}` requires an allowed placement (inside `{#if}`, `{#each}`,
    // component node, `<svelte:element slot="...">`, etc.) — bare
    // fragment-root placement emits `ConstTagInvalidPlacement`. Tests
    // wrap tags in a trivial `{#if true}...{/if}` to satisfy the rule
    // without adding other structural noise.
    #[test]
    fn const_tag_simple_sync() {
        with_const_tag(r#"{#if true}{@const x = 1}<p>{x}</p>{/if}"#, |sem| {
            assert_eq!(sem.bindings.len(), 1);
            assert!(!sem.is_destructured);
            assert!(matches!(sem.async_kind, ConstTagAsyncKind::Sync));
        });
    }

    #[test]
    fn const_tag_object_destructure() {
        with_const_tag(
            r#"<script>let obj = { a: 1, b: 2 };</script>
{#if true}{@const { a, b } = obj}<p>{a} {b}</p>{/if}"#,
            |sem| {
                assert_eq!(sem.bindings.len(), 2);
                assert!(sem.is_destructured);
                assert!(matches!(sem.async_kind, ConstTagAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn const_tag_array_destructure() {
        with_const_tag(
            r#"<script>let arr = [1, 2];</script>
{#if true}{@const [x, y] = arr}<p>{x} {y}</p>{/if}"#,
            |sem| {
                assert_eq!(sem.bindings.len(), 2);
                assert!(sem.is_destructured);
            },
        );
    }

    #[test]
    fn const_tag_nested_destructure() {
        with_const_tag(
            r#"<script>let obj = { a: { b: 1 } };</script>
{#if true}{@const { a: { b } } = obj}<p>{b}</p>{/if}"#,
            |sem| {
                assert_eq!(sem.bindings.len(), 1);
                assert!(sem.is_destructured);
            },
        );
    }

    #[test]
    fn const_tag_rest_destructure() {
        with_const_tag(
            r#"<script>let obj = { a: 1, b: 2, c: 3 };</script>
{#if true}{@const { a, ...rest } = obj}<p>{a}</p>{/if}"#,
            |sem| {
                assert_eq!(sem.bindings.len(), 2);
                assert!(sem.is_destructured);
            },
        );
    }

    #[test]
    fn const_tag_multiple_in_fragment() {
        // First const tag should get a correct payload; the walker must
        // visit every `{@const}` in a fragment.
        with_const_tag(
            r#"{#if true}
{@const x = 1}
{@const y = 2}
<p>{x} {y}</p>
{/if}"#,
            |sem| {
                assert_eq!(sem.bindings.len(), 1);
                assert!(!sem.is_destructured);
            },
        );
    }

    #[test]
    fn const_tag_ref_to_state() {
        // Referencing a `$state` binding stays Sync — `async_kind` only
        // flips when `await` or script-level blockers are involved.
        with_const_tag(
            r#"<script>let count = $state(0);</script>
{#if true}{@const doubled = count * 2}<p>{doubled}</p>{/if}"#,
            |sem| {
                assert_eq!(sem.bindings.len(), 1);
                assert!(matches!(sem.async_kind, ConstTagAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn const_tag_stmt_handle_matches_analysis() {
        // Payload's `stmt_handle` should be the same handle that
        // `AnalysisData::const_tag_stmt_handle` returns — this is what
        // emit-time clone / take relies on.
        let (component, data) =
            analyze_source(r#"{#if true}{@const answer = 42}<p>{answer}</p>{/if}"#);
        let tag = first_const_tag(&component);
        let sem = match data.block_semantics(tag.id) {
            BlockSemantics::ConstTag(s) => s.clone(),
            other => panic!("expected ConstTag, got {other:?}"),
        };
        let expected = data.const_tag_stmt_handle(tag.id).expect("stmt handle");
        assert_eq!(sem.stmt_handle, expected);
    }
}
