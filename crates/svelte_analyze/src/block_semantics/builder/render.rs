//! `{@render}` population for Block Semantics.
//!
//! Free function invoked by the cluster-wide walker in [`super::walker`]:
//! given the shared `Ctx`, consume one `RenderTag` node and record its
//! `BlockSemantics::Render(...)` payload. `{@render}` does not own a
//! fragment of its own, so no recursion is required after populate.
//!
//! Scope boundary: this module folds the four former surfaces
//! (`RenderTagPlan.callee_mode`, `render_tag_is_chain`,
//! per-arg `ExpressionInfo` + `prop_source`, async-wrapper plan) into
//! one composite answer. Per-reference reactive meaning — which
//! symbols read as `$.get` vs `$.safe_get` inside the args — stays
//! in `reactivity_semantics`; the consumer resolves those through
//! the transformer before codegen sees the args.

use super::super::{
    BlockSemantics, RenderArgLowering, RenderAsyncKind, RenderCalleeShape, RenderTagBlockSemantics,
};
use super::walker::Ctx;
use crate::types::data::{DeclarationSemantics, PropDeclarationKind, PropDeclarationSemantics};
use oxc_ast::ast::{Argument, AwaitExpression, CallExpression, Expression, IdentifierReference};
use oxc_ast_visit::Visit;
use smallvec::SmallVec;
use svelte_ast::RenderTag;
use svelte_component_semantics::{ReferenceId, SymbolId};

/// Populate `BlockSemantics::Render` for this tag.
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
    // A callee is "dynamic" iff its binding has any reactive declaration
    // semantics. Non-identifier callees (member expressions, calls, etc.)
    // are treated as dynamic — matching the reference compiler's
    // `binding?.kind !== 'normal'` where a missing binding falls into
    // the non-normal branch.
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
    let decl = ctx.semantics.symbol_declaration(sym);
    !matches!(
        ctx.reactivity.declaration_semantics(decl),
        DeclarationSemantics::NonReactive | DeclarationSemantics::Unresolved,
    )
}

/// Single-pass classification of every argument: one OXC sub-walk per
/// arg collects `has_call`, `has_await`, and all identifier references
/// at once. That satisfies the Traversal Budget rule (single-pass per
/// subtree) — downstream derivation (`RenderArgLowering`, cross-arg
/// blocker union, top-level `async_kind`) is pure book-keeping over
/// the collected facts.
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
            // Fast-path: PropSource identifier needs no sub-walk.
            if let Some(sym) = prop_source_arg(ctx, expr) {
                args.push(RenderArgLowering::PropSource { sym });
                continue;
            }

            // One walk over the arg subtree collects everything we need.
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
        // Analyze layer rejects `SpreadElement` in render args via a
        // diagnostic, but defensively classify non-expression arguments
        // as `Plain` so the builder never panics on malformed input.
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

/// A prop-source argument is a single `Identifier` whose binding is a
/// `$props()` source. Membership expressions, even when rooted at a
/// prop, do not qualify — they need a thunk so the getter isn't
/// shadowed by property access.
fn prop_source_arg(ctx: &Ctx<'_, '_>, arg: &Expression<'_>) -> Option<SymbolId> {
    let Expression::Identifier(ident) = arg else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    let sym = ctx.semantics.get_reference(ref_id).symbol_id()?;
    let decl = ctx.semantics.symbol_declaration(sym);
    if matches!(
        ctx.reactivity.declaration_semantics(decl),
        DeclarationSemantics::Prop(PropDeclarationSemantics {
            kind: PropDeclarationKind::Source { .. },
            ..
        }),
    ) {
        Some(sym)
    } else {
        None
    }
}

/// Resolve each reference to its symbol's script-level blocker index
/// (if any) and union into the running list. Returns `true` iff at
/// least one blocker was added — the caller uses this to decide
/// whether the argument has an async dependency beyond a literal
/// `await`.
fn union_blockers(ctx: &Ctx<'_, '_>, refs: &[ReferenceId], out: &mut SmallVec<[u32; 2]>) -> bool {
    let before = out.len();
    for ref_id in refs {
        let Some(sym) = ctx.semantics.get_reference(*ref_id).symbol_id() else {
            continue;
        };
        if let Some(idx) = ctx.blockers.symbol_blocker(sym) {
            if !out.contains(&idx) {
                out.push(idx);
            }
        }
    }
    out.len() > before
}

/// Every fact Block Semantics needs from one argument subtree, in a
/// single OXC sub-walk: call present, await present, and the list of
/// reference ids used downstream to compute blockers.
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
                let children: &[NodeId] = match node {
                    Node::Element(el) => &el.fragment.nodes,
                    Node::ComponentNode(cn) => &cn.fragment.nodes,
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
        walk(component, &component.fragment.nodes).expect("no render tag")
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
        // Prop becomes a Source kind when it's bindable (accessor-style
        // lowering). Non-bindable, non-mutated destructured props stay as
        // NonSource and fall into the Plain arm — matching legacy
        // `resolve_render_tag_meta::resolve_arg_prop_sources`.
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
