use super::super::{BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics};
use super::common::declarator_from_stmt;
use super::walker::Ctx;
use crate::expression_semantics::{ExpressionSemantics, Volatility};
use oxc_ast::ast::Statement;
use svelte_ast::ConstTag;

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &ConstTag) {
    let Some(stmt) = ctx.parsed.stmt(tag.decl.id()) else {
        return;
    };

    let Statement::VariableDeclaration(decl) = stmt else {
        return;
    };
    let decl_node_id = decl.node_id();
    let Some(_) = declarator_from_stmt(stmt) else {
        return;
    };
    let async_kind = match ctx.expressions.get(tag.id) {
        ExpressionSemantics::Expression(d) => {
            let blockers = d.blockers.clone();
            match d.volatility {
                Volatility::Asynchronous => ConstTagAsyncKind::Awaited { blockers },
                Volatility::Static | Volatility::Reactive | Volatility::Heavy => {
                    if blockers.is_empty() {
                        ConstTagAsyncKind::Sync
                    } else {
                        ConstTagAsyncKind::Deferred { blockers }
                    }
                }
            }
        }
        ExpressionSemantics::NonSpecial => ConstTagAsyncKind::Sync,
    };

    ctx.store.set(
        tag.id,
        BlockSemantics::ConstTag(ConstTagBlockSemantics {
            decl_node_id,
            async_kind,
        }),
    );
}

#[cfg(test)]
mod tests {
    use crate::tests::{analyze_source, analyze_source_experimental_async};
    use crate::{
        AnalysisData, BlockSemantics, ConstTagAsyncKind, ConstTagBlockSemantics,
        DeclaratorSemantics, DerivedEmit,
    };
    use oxc_ast::{AstKind, ast::BindingPattern};
    use svelte_ast::{Component, ConstTag, Node};
    use svelte_component_semantics::walk_bindings;

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
        walk(component, &root_nodes).expect("no const tag")
    }

    fn with_const_tag<F: FnOnce(&ConstTagBlockSemantics, &AnalysisData<'_>)>(
        source: &str,
        check: F,
    ) {
        let (component, data) = analyze_source(source);
        let tag = first_const_tag(&component);
        match data.block_semantics(tag.id) {
            BlockSemantics::ConstTag(s) => check(s, &data),
            other => panic!("expected ConstTag, got {other:?}"),
        }
    }

    fn with_const_tag_async<F: FnOnce(&ConstTagBlockSemantics, &AnalysisData<'_>)>(
        source: &str,
        check: F,
    ) {
        let (component, data) = analyze_source_experimental_async(source);
        let tag = first_const_tag(&component);
        match data.block_semantics(tag.id) {
            BlockSemantics::ConstTag(s) => check(s, &data),
            other => panic!("expected ConstTag, got {other:?}"),
        }
    }

    fn pattern_facts(sem: &ConstTagBlockSemantics, data: &AnalysisData<'_>) -> (usize, bool) {
        let kind = data
            .scoping
            .js_kind(sem.decl_node_id)
            .expect("decl node resolvable via js_kind");
        let AstKind::VariableDeclaration(decl) = kind else {
            panic!("ConstTag decl must resolve to a VariableDeclaration");
        };
        let declarator = decl.declarations.first().expect("one declarator");
        let is_destructured = !matches!(declarator.id, BindingPattern::BindingIdentifier(_));
        let mut count = 0;
        walk_bindings(&declarator.id, |_| count += 1);
        (count, is_destructured)
    }

    #[test]
    fn const_tag_simple_sync() {
        with_const_tag(r#"{#if true}{@const x = 1}<p>{x}</p>{/if}"#, |sem, data| {
            assert_eq!(pattern_facts(sem, data), (1, false));
            assert!(matches!(sem.async_kind, ConstTagAsyncKind::Sync));
        });
    }

    #[test]
    fn const_tag_object_destructure() {
        with_const_tag(
            r#"<script>let obj = { a: 1, b: 2 };</script>
{#if true}{@const { a, b } = obj}<p>{a} {b}</p>{/if}"#,
            |sem, data| {
                assert_eq!(pattern_facts(sem, data), (2, true));
                assert!(matches!(sem.async_kind, ConstTagAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn const_tag_array_destructure() {
        with_const_tag(
            r#"<script>let arr = [1, 2];</script>
{#if true}{@const [x, y] = arr}<p>{x} {y}</p>{/if}"#,
            |sem, data| {
                assert_eq!(pattern_facts(sem, data), (2, true));
            },
        );
    }

    #[test]
    fn const_tag_nested_destructure() {
        with_const_tag(
            r#"<script>let obj = { a: { b: 1 } };</script>
{#if true}{@const { a: { b } } = obj}<p>{b}</p>{/if}"#,
            |sem, data| {
                assert_eq!(pattern_facts(sem, data), (1, true));
            },
        );
    }

    #[test]
    fn const_tag_rest_destructure() {
        with_const_tag(
            r#"<script>let obj = { a: 1, b: 2, c: 3 };</script>
{#if true}{@const { a, ...rest } = obj}<p>{a}</p>{/if}"#,
            |sem, data| {
                assert_eq!(pattern_facts(sem, data), (2, true));
            },
        );
    }

    #[test]
    fn const_tag_multiple_in_fragment() {
        with_const_tag(
            r#"{#if true}
{@const x = 1}
{@const y = 2}
<p>{x} {y}</p>
{/if}"#,
            |sem, data| {
                assert_eq!(pattern_facts(sem, data), (1, false));
            },
        );
    }

    #[test]
    fn const_tag_declarator_semantics_sync() {
        with_const_tag(r#"{#if true}{@const x = 1}<p>{x}</p>{/if}"#, |sem, data| {
            assert_eq!(
                data.declarator_semantics(sem.decl_node_id),
                DeclaratorSemantics::ConstTag {
                    emit: DerivedEmit::Sync
                }
            );
        });
    }

    #[test]
    fn const_tag_declarator_semantics_async() {
        with_const_tag_async(
            r#"{#if true}{@const x = await foo()}<p>{x}</p>{/if}"#,
            |sem, data| {
                assert_eq!(
                    data.declarator_semantics(sem.decl_node_id),
                    DeclaratorSemantics::ConstTag {
                        emit: DerivedEmit::Async
                    }
                );
            },
        );
    }

    #[test]
    fn const_tag_declarator_semantics_async_inside_fn_is_sync() {
        with_const_tag_async(
            r#"{#if true}{@const x = (async () => await foo())}<p>{x}</p>{/if}"#,
            |sem, data| {
                assert_eq!(
                    data.declarator_semantics(sem.decl_node_id),
                    DeclaratorSemantics::ConstTag {
                        emit: DerivedEmit::Sync
                    }
                );
            },
        );
    }

    #[test]
    fn const_tag_ref_to_state() {
        with_const_tag(
            r#"<script>let count = $state(0);</script>
{#if true}{@const doubled = count * 2}<p>{doubled}</p>{/if}"#,
            |sem, data| {
                assert_eq!(pattern_facts(sem, data), (1, false));
                assert!(matches!(sem.async_kind, ConstTagAsyncKind::Sync));
            },
        );
    }
}
