use super::super::{BlockSemantics, SnippetBlockSemantics, SnippetParam};
use super::common::{binding_pattern_node_id, declarator_from_stmt};
use super::walker::{Ctx, SnippetScope};
use oxc_ast::ast::{
    ArrowFunctionExpression, BindingPattern, Expression, FormalParameter, VariableDeclarator,
};
use smallvec::SmallVec;
use svelte_ast::SnippetBlock;

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, block: &SnippetBlock) {
    let stmt = ctx.parsed.stmt(block.decl.id());

    let declarator = stmt.and_then(declarator_from_stmt);

    let name_sym = declarator.and_then(|d| match &d.id {
        BindingPattern::BindingIdentifier(ident) => ident.symbol_id.get(),
        _ => None,
    });

    let arrow = declarator.and_then(arrow_from_declarator);
    let params = arrow.map(|arrow| collect_params(arrow)).unwrap_or_default();

    let top_level = ctx.non_root_depth == 0;

    ctx.visit_fragment(block.body);

    let Some(name) = name_sym else {
        return;
    };

    ctx.store.set(
        block.id,
        BlockSemantics::Snippet(SnippetBlockSemantics {
            name,
            hoistable: false,
            params,
        }),
    );

    ctx.snippet_name_syms.insert(name);

    if let Some(body_scope) = ctx.semantics.fragment_scope_by_id(block.body) {
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

fn classify_param<'a>(param: &FormalParameter<'a>) -> Option<SnippetParam> {
    let pattern = match &param.pattern {
        BindingPattern::AssignmentPattern(assign) => &assign.left,
        other => other,
    };

    match pattern {
        BindingPattern::BindingIdentifier(ident) => {
            let sym = ident.symbol_id.get()?;
            Some(SnippetParam::Identifier { sym })
        }
        BindingPattern::ObjectPattern(_) | BindingPattern::ArrayPattern(_) => {
            Some(SnippetParam::Pattern {
                pattern_id: binding_pattern_node_id(pattern),
            })
        }

        BindingPattern::AssignmentPattern(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_source;
    use crate::{BlockSemantics, SnippetBlockSemantics, SnippetParam};
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
        walk(component, &root_nodes).expect("no snippet block")
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
                SnippetParam::Pattern { .. } => {}
                other => panic!("expected Pattern, got {other:?}"),
            },
        );
    }

    #[test]
    fn snippet_object_destructure_with_default() {
        with_snippet(
            r#"{#snippet row({ a = 1, b })}<p>{a} {b}</p>{/snippet}"#,
            |sem| match &sem.params[0] {
                SnippetParam::Pattern { .. } => {}
                other => panic!("expected Pattern, got {other:?}"),
            },
        );
    }

    #[test]
    fn snippet_object_destructure_with_rest() {
        with_snippet(
            r#"{#snippet row({ a, ...rest })}<p>{a}</p>{/snippet}"#,
            |sem| match &sem.params[0] {
                SnippetParam::Pattern { .. } => {}
                other => panic!("expected Pattern, got {other:?}"),
            },
        );
    }

    #[test]
    fn snippet_array_destructure() {
        with_snippet(
            r#"{#snippet row([x, y])}<p>{x} {y}</p>{/snippet}"#,
            |sem| match &sem.params[0] {
                SnippetParam::Pattern { .. } => {}
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
                assert!(matches!(sem.params[1], SnippetParam::Pattern { .. }));
                assert!(matches!(sem.params[2], SnippetParam::Pattern { .. }));
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
