use super::super::{
    BlockSemantics, RenderArgKind, RenderAsyncKind, RenderCallKind, RenderTagBlockSemantics,
};
use super::walker::Ctx;
use crate::ReferenceSemantics;
use crate::expression_semantics::{ExpressionSemantics, Volatility};
use crate::types::data::{BindingSemantics, PropBindingKind};
use crate::utils::node_id_utils::{argument_node_id, expression_node_id};
use oxc_ast::ast::{Argument, ChainElement, Expression};
use smallvec::SmallVec;
use svelte_ast::RenderTag;
use svelte_component_semantics::{ReferenceId, SymbolId};

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &RenderTag) {
    let Some(expr) = ctx.parsed.expr(tag.expression.id()) else {
        ctx.store.set(
            tag.id,
            BlockSemantics::Render(RenderTagBlockSemantics {
                call_kind: RenderCallKind::Plain,
                callee_sym: None,
                callee_volatility: Volatility::Static,
                args: SmallVec::new(),
                async_kind: RenderAsyncKind::Sync,
            }),
        );
        return;
    };

    let (call_kind, call_opt) = match expr.get_inner_expression() {
        Expression::ChainExpression(chain) => match &chain.expression {
            ChainElement::CallExpression(call) => {
                (RenderCallKind::OptionalChain, Some(call.as_ref()))
            }
            _ => (RenderCallKind::Plain, None),
        },
        Expression::CallExpression(call) => (RenderCallKind::Plain, Some(call.as_ref())),
        _ => (RenderCallKind::Plain, None),
    };

    let async_kind = derive_async_kind(ctx, tag);
    let (callee_sym, callee_volatility, args) = match call_opt {
        Some(call) => {
            let callee_sym = callee_symbol(&call.callee, ctx);
            let callee_volatility = callee_volatility(ctx, callee_sym);
            let args: SmallVec<[RenderArgKind; 4]> = call
                .arguments
                .iter()
                .map(|arg| derive_arg_kind(ctx, arg))
                .collect();
            (callee_sym, callee_volatility, args)
        }
        None => (None, Volatility::Static, SmallVec::new()),
    };

    ctx.store.set(
        tag.id,
        BlockSemantics::Render(RenderTagBlockSemantics {
            call_kind,
            callee_sym,
            callee_volatility,
            args,
            async_kind,
        }),
    );
}

fn callee_volatility(ctx: &Ctx<'_, '_>, callee_sym: Option<SymbolId>) -> Volatility {
    let Some(sym) = callee_sym else {
        return Volatility::Reactive;
    };
    match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyPropsObject
        | BindingSemantics::Unresolved => Volatility::Static,
        BindingSemantics::Prop(_)
        | BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Contextual(_)
        | BindingSemantics::LegacyApiExport => Volatility::Reactive,
    }
}

fn derive_async_kind(ctx: &Ctx<'_, '_>, tag: &RenderTag) -> RenderAsyncKind {
    match ctx.expressions.get(tag.id) {
        ExpressionSemantics::Expression(d) => match d.volatility {
            Volatility::Asynchronous => RenderAsyncKind::Awaited {
                blockers: d.blockers.clone(),
            },
            Volatility::Static | Volatility::Reactive | Volatility::Heavy => {
                if d.blockers.is_empty() {
                    RenderAsyncKind::Sync
                } else {
                    RenderAsyncKind::Deferred {
                        blockers: d.blockers.clone(),
                    }
                }
            }
        },
        ExpressionSemantics::NonSpecial => RenderAsyncKind::Sync,
    }
}

fn callee_symbol(callee: &Expression<'_>, ctx: &Ctx<'_, '_>) -> Option<SymbolId> {
    let Expression::Identifier(ident) = callee.get_inner_expression() else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    if let Some(store_sym) = store_reference_symbol(ctx, ref_id) {
        return Some(store_sym);
    }
    ctx.semantics.get_reference(ref_id).symbol_id()
}

fn store_reference_symbol(ctx: &Ctx<'_, '_>, ref_id: ReferenceId) -> Option<SymbolId> {
    match ctx.reactivity.reference_semantics(ref_id) {
        ReferenceSemantics::StoreRead { symbol }
        | ReferenceSemantics::StoreWrite { symbol }
        | ReferenceSemantics::StoreUpdate { symbol } => Some(symbol),
        ReferenceSemantics::ImportSubscribedRead { store_symbol } => Some(store_symbol),
        _ => None,
    }
}

fn derive_arg_kind(ctx: &Ctx<'_, '_>, argument: &Argument<'_>) -> RenderArgKind {
    if let Argument::SpreadElement(_) = argument {
        return RenderArgKind::InertThunk;
    }
    let expr = argument.to_expression();

    if let Some(sym) = passthrough_prop_binding(ctx, expr) {
        return RenderArgKind::PropPassthrough { sym };
    }

    let oxc_id = argument_node_id(argument);
    let data = match ctx.expressions.get_by_oxc(oxc_id) {
        ExpressionSemantics::Expression(d) => d,
        _ => return RenderArgKind::InertThunk,
    };

    match data.volatility {
        Volatility::Asynchronous => {
            let inner_node_id = if let Expression::AwaitExpression(aw) = expr.get_inner_expression()
            {
                Some(expression_node_id(&aw.argument))
            } else {
                None
            };
            RenderArgKind::AwaitMemo {
                inner_node_id,
                suspension: data.suspension,
            }
        }
        Volatility::Heavy => {
            if data.blockers.is_empty() {
                RenderArgKind::NeedsMemo
            } else {
                RenderArgKind::InertThunk
            }
        }
        Volatility::Static | Volatility::Reactive => RenderArgKind::InertThunk,
    }
}

fn passthrough_prop_binding(ctx: &Ctx<'_, '_>, arg: &Expression<'_>) -> Option<SymbolId> {
    let Expression::Identifier(ident) = arg.get_inner_expression() else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    let sym = ctx.semantics.get_reference(ref_id).symbol_id()?;
    let is_source_prop = match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::Prop(prop) => match &prop.kind {
            PropBindingKind::Source { .. } => true,
            PropBindingKind::Identifier | PropBindingKind::Rest | PropBindingKind::NonSource => {
                false
            }
        },
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Contextual(_)
        | BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyPropsObject
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => false,
    };
    if is_source_prop { Some(sym) } else { None }
}

#[cfg(test)]
mod tests {
    use crate::tests::analyze_source;
    use crate::{
        BlockSemantics, RenderArgKind, RenderAsyncKind, RenderCallKind, RenderTagBlockSemantics,
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
                    Node::SvelteSelf(cn) => Some(cn.fragment),
                    Node::IfBlock(b) => {
                        if let Some(r) = walk(component, component.fragment_nodes(b.consequent)) {
                            return Some(r);
                        }
                        if let Some(alt) = b.alternate
                            && let Some(r) = walk(component, component.fragment_nodes(alt))
                        {
                            return Some(r);
                        }
                        continue;
                    }
                    Node::EachBlock(b) => Some(b.body),
                    Node::SnippetBlock(b) => Some(b.body),
                    _ => continue,
                };
                if let Some(fid) = child_fragment
                    && let Some(r) = walk(component, component.fragment_nodes(fid))
                {
                    return Some(r);
                }
            }
            None
        }
        walk(component, component.fragment_nodes(component.root)).expect("no render tag")
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
                assert_eq!(sem.call_kind, RenderCallKind::Plain);
                assert!(sem.callee_sym.is_some());
                assert!(!sem.callee_volatility.is_volatile());
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
                assert_eq!(sem.call_kind, RenderCallKind::OptionalChain);
            },
        );
    }

    #[test]
    fn render_dynamic_prop() {
        assert_render(
            r#"<script>let { row } = $props();</script>{@render row()}"#,
            |sem| {
                assert_eq!(sem.call_kind, RenderCallKind::Plain);
                assert!(sem.callee_sym.is_some());
                assert!(sem.callee_volatility.is_volatile());
            },
        );
    }

    #[test]
    fn render_dynamic_chain() {
        assert_render(
            r#"<script>let { row } = $props();</script>{@render row?.()}"#,
            |sem| {
                assert_eq!(sem.call_kind, RenderCallKind::OptionalChain);
            },
        );
    }

    #[test]
    fn render_arg_prop_passthrough() {
        assert_render(
            r#"<script>let { value = $bindable() } = $props(); function row(_) {} value = 1;</script>{@render row(value)}"#,
            |sem| {
                assert_eq!(sem.args.len(), 1);
                assert!(
                    matches!(sem.args[0], RenderArgKind::PropPassthrough { .. }),
                    "expected PropPassthrough, got {:?}",
                    sem.args[0]
                );
            },
        );
    }

    #[test]
    fn render_arg_needs_memo() {
        assert_render(
            r#"<script>let { row } = $props(); function label(x) { return x; }</script>{@render row(label(1))}"#,
            |sem| {
                assert_eq!(sem.args.len(), 1);
                assert_eq!(sem.args[0], RenderArgKind::NeedsMemo);
            },
        );
    }

    #[test]
    fn render_arg_inert_thunk() {
        assert_render(
            r#"<script>let { row } = $props(); const x = 1;</script>{@render row(x)}"#,
            |sem| {
                assert_eq!(sem.args.len(), 1);
                assert_eq!(sem.args[0], RenderArgKind::InertThunk);
            },
        );
    }

    #[test]
    fn render_arg_logical_inert() {
        assert_render(
            r#"<script>let { row, a, b } = $props();</script>{@render row(a ?? b)}"#,
            |sem| {
                assert_eq!(sem.args.len(), 1);
                assert_eq!(sem.args[0], RenderArgKind::InertThunk);
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
