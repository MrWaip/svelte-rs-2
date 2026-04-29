use super::super::{
    BlockSemantics, RenderArgLowering, RenderAsyncKind, RenderCalleeShape, RenderTagBlockSemantics,
};
use super::walker::Ctx;
use crate::types::data::{BindingSemantics, PropBindingKind, PropBindingSemantics};
use oxc_ast::ast::{Argument, AwaitExpression, CallExpression, Expression, IdentifierReference};
use oxc_ast_visit::Visit;
use smallvec::SmallVec;
use svelte_ast::RenderTag;
use svelte_component_semantics::{ReferenceId, SymbolId};

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &RenderTag) {
    let Some(expr) = ctx.parsed.expr(tag.expression.id()) else {
        return;
    };

    let (is_chain, call) = match expr {
        Expression::ChainExpression(chain) => match &chain.expression {
            oxc_ast::ast::ChainElement::CallExpression(call) => (true, call.as_ref()),
            _ => return,
        },
        Expression::CallExpression(call) => (false, call.as_ref()),
        _ => return,
    };

    let callee_sym = callee_symbol(&call.callee, ctx);
    let callee_shape = classify_callee_shape(ctx, is_chain, callee_sym);
    let (args, async_kind) = classify_args_and_async(ctx, &call.arguments);

    ctx.store.set(
        tag.id,
        BlockSemantics::Render(RenderTagBlockSemantics {
            callee_shape,
            callee_sym,
            args,
            async_kind,
        }),
    );
}

fn classify_callee_shape(
    ctx: &Ctx<'_, '_>,
    is_chain: bool,
    callee_sym: Option<SymbolId>,
) -> RenderCalleeShape {
    let is_dynamic = callee_sym.is_none_or(|sym| is_reactive_symbol(ctx, sym));
    match (is_dynamic, is_chain) {
        (false, false) => RenderCalleeShape::Static,
        (false, true) => RenderCalleeShape::StaticChain,
        (true, false) => RenderCalleeShape::Dynamic,
        (true, true) => RenderCalleeShape::DynamicChain,
    }
}

fn callee_symbol(callee: &Expression<'_>, ctx: &Ctx<'_, '_>) -> Option<SymbolId> {
    let Expression::Identifier(ident) = callee else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    ctx.semantics.get_reference(ref_id).symbol_id()
}

fn is_reactive_symbol(ctx: &Ctx<'_, '_>, sym: SymbolId) -> bool {
    !matches!(
        ctx.reactivity.binding_semantics(sym),
        BindingSemantics::NonReactive | BindingSemantics::Unresolved,
    )
}

fn classify_args_and_async<'a>(
    ctx: &Ctx<'_, 'a>,
    arguments: &oxc_allocator::Vec<'a, Argument<'a>>,
) -> (SmallVec<[RenderArgLowering; 4]>, RenderAsyncKind) {
    let mut args: SmallVec<[RenderArgLowering; 4]> = SmallVec::new();
    let mut any_await = false;
    let mut blockers: SmallVec<[u32; 2]> = SmallVec::new();

    for arg in arguments {
        let Argument::SpreadElement(_) = arg else {
            let expr = arg.to_expression();

            if let Some(sym) = prop_source_arg(ctx, expr) {
                args.push(RenderArgLowering::PropSource { sym });
                continue;
            }

            let facts = ArgFacts::collect(expr);
            let arg_blockers_found = union_blockers(ctx, &facts.refs, &mut blockers);
            any_await |= facts.has_await;

            args.push(match (facts.has_await, facts.has_call) {
                (true, _) => RenderArgLowering::MemoAsync,
                (false, true) => RenderArgLowering::MemoSync,
                (false, false) => RenderArgLowering::Plain,
            });
            let _ = arg_blockers_found;
            continue;
        };

        args.push(RenderArgLowering::Plain);
    }
    blockers.sort_unstable();

    let async_kind = if !any_await && blockers.is_empty() {
        RenderAsyncKind::Sync
    } else {
        RenderAsyncKind::Async { blockers }
    };

    (args, async_kind)
}

fn prop_source_arg(ctx: &Ctx<'_, '_>, arg: &Expression<'_>) -> Option<SymbolId> {
    let Expression::Identifier(ident) = arg else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    let sym = ctx.semantics.get_reference(ref_id).symbol_id()?;
    if matches!(
        ctx.reactivity.binding_semantics(sym),
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Source { .. },
            ..
        }),
    ) {
        Some(sym)
    } else {
        None
    }
}

fn union_blockers(ctx: &Ctx<'_, '_>, refs: &[ReferenceId], out: &mut SmallVec<[u32; 2]>) -> bool {
    let before = out.len();
    for ref_id in refs {
        let Some(sym) = ctx.semantics.get_reference(*ref_id).symbol_id() else {
            continue;
        };
        if let Some(idx) = ctx.blockers.symbol_blocker(sym)
            && !out.contains(&idx)
        {
            out.push(idx);
        }
    }
    out.len() > before
}

struct ArgFacts {
    has_call: bool,
    has_await: bool,
    refs: SmallVec<[ReferenceId; 4]>,
}

impl ArgFacts {
    fn collect(expr: &Expression<'_>) -> Self {
        let mut collector = ArgFactsCollector {
            has_call: false,
            has_await: false,
            refs: SmallVec::new(),
        };
        collector.visit_expression(expr);
        Self {
            has_call: collector.has_call,
            has_await: collector.has_await,
            refs: collector.refs,
        }
    }
}

struct ArgFactsCollector {
    has_call: bool,
    has_await: bool,
    refs: SmallVec<[ReferenceId; 4]>,
}

impl<'a> Visit<'a> for ArgFactsCollector {
    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        self.has_call = true;
        oxc_ast_visit::walk::walk_call_expression(self, expr);
    }
    fn visit_await_expression(&mut self, expr: &AwaitExpression<'a>) {
        self.has_await = true;
        oxc_ast_visit::walk::walk_await_expression(self, expr);
    }
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.refs.push(ref_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_source;
    use crate::{
        BlockSemantics, RenderArgLowering, RenderAsyncKind, RenderCalleeShape,
        RenderTagBlockSemantics,
    };
    use svelte_ast::{Component, Node, NodeId, RenderTag};

    fn first_render_tag(component: &Component) -> &RenderTag {
        fn walk<'a>(component: &'a Component, nodes: &[NodeId]) -> Option<&'a RenderTag> {
            for &id in nodes {
                let node = component.store.get(id);
                if let Node::RenderTag(t) = node {
                    return Some(t);
                }
                let child_fragment = match node {
                    Node::Element(el) => Some(el.fragment),
                    Node::ComponentNode(cn) => Some(cn.fragment),
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
        walk(component, &root_nodes).expect("no render tag")
    }

    fn assert_render<F: FnOnce(&RenderTagBlockSemantics)>(source: &str, check: F) {
        let (component, data) = analyze_source(source);
        let tag = first_render_tag(&component);
        let sem: &BlockSemantics = data.block_semantics(tag.id);
        match sem {
            BlockSemantics::Render(s) => check(s),
            other => panic!("expected Render, got {other:?}"),
        }
    }

    #[test]
    fn render_static_snippet_no_args() {
        assert_render(
            r#"{#snippet row()}<span></span>{/snippet}{@render row()}"#,
            |sem| {
                assert_eq!(sem.callee_shape, RenderCalleeShape::Static);
                assert_eq!(sem.args.len(), 0);
                assert!(matches!(sem.async_kind, RenderAsyncKind::Sync));
            },
        );
    }

    #[test]
    fn render_static_snippet_chain() {
        assert_render(
            r#"{#snippet row()}<span></span>{/snippet}{@render row?.()}"#,
            |sem| {
                assert_eq!(sem.callee_shape, RenderCalleeShape::StaticChain);
            },
        );
    }

    #[test]
    fn render_dynamic_prop() {
        assert_render(
            r#"<script>let { row } = $props();</script>{@render row()}"#,
            |sem| {
                assert_eq!(sem.callee_shape, RenderCalleeShape::Dynamic);
            },
        );
    }

    #[test]
    fn render_dynamic_chain() {
        assert_render(
            r#"<script>let { row } = $props();</script>{@render row?.()}"#,
            |sem| {
                assert_eq!(sem.callee_shape, RenderCalleeShape::DynamicChain);
            },
        );
    }

    #[test]
    fn render_arg_prop_source() {
        assert_render(
            r#"<script>let { value = $bindable() } = $props(); function row(_) {}</script>{@render row(value)}"#,
            |sem| {
                assert_eq!(sem.args.len(), 1);
                assert!(
                    matches!(sem.args[0], RenderArgLowering::PropSource { .. }),
                    "expected PropSource, got {:?}",
                    sem.args[0]
                );
            },
        );
    }

    #[test]
    fn render_arg_has_call() {
        assert_render(
            r#"<script>let { row } = $props(); function label(x) { return x; }</script>{@render row(label(1))}"#,
            |sem| {
                assert_eq!(sem.args.len(), 1);
                assert_eq!(sem.args[0], RenderArgLowering::MemoSync);
            },
        );
    }

    #[test]
    fn render_arg_plain_identifier() {
        assert_render(
            r#"<script>let { row } = $props(); const x = 1;</script>{@render row(x)}"#,
            |sem| {
                assert_eq!(sem.args.len(), 1);
                assert_eq!(sem.args[0], RenderArgLowering::Plain);
            },
        );
    }

    #[test]
    fn render_sync_when_no_await_no_blockers() {
        assert_render(
            r#"<script>let { row } = $props(); const x = 1;</script>{@render row(x)}"#,
            |sem| {
                assert!(matches!(sem.async_kind, RenderAsyncKind::Sync));
            },
        );
    }
}
