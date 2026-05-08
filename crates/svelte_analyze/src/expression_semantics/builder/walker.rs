use super::super::ExpressionSemanticsStore;
use super::super::data::{ExprKind, ExpressionData, ExpressionSemantics, LegacyWrap, Memoization};
use super::collector::collect;
use super::derive;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::ComponentScoping;
use crate::types::data::{BlockerData, ExpressionInfo, JsAst, PickledAwaitOffsets};
use crate::types::node_table::NodeTable;
use smallvec::SmallVec;
use svelte_ast::{Component, Element, FragmentId, Node, NodeId, SvelteElement};
use svelte_component_semantics::{ComponentSemantics, OxcNodeId};

pub(super) fn populate<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    scoping: &ComponentScoping,
    expressions: &NodeTable<ExpressionInfo>,
    has_class_state_fields: bool,
    blockers: &BlockerData,
    pickled: &PickledAwaitOffsets,
    store: &mut ExpressionSemanticsStore,
) {
    let ctx = Ctx {
        parsed,
        semantics,
        reactivity,
        scoping,
        expressions,
        has_class_state_fields,
        blockers,
        pickled,
        runes: reactivity.uses_runes(),
    };
    visit_fragment(component, component.root, &ctx, store);
}

pub(super) struct Ctx<'c, 'a> {
    pub(super) parsed: &'c JsAst<'a>,
    pub(super) semantics: &'c ComponentSemantics<'a>,
    pub(super) reactivity: &'c ReactivitySemantics,
    pub(super) scoping: &'c ComponentScoping<'a>,
    pub(super) expressions: &'c NodeTable<ExpressionInfo>,
    pub(super) has_class_state_fields: bool,
    pub(super) blockers: &'c BlockerData,
    pub(super) pickled: &'c PickledAwaitOffsets,
    pub(super) runes: bool,
}

fn visit_fragment(
    component: &Component,
    fragment_id: FragmentId,
    ctx: &Ctx<'_, '_>,
    store: &mut ExpressionSemanticsStore,
) {
    let len = component.fragment_nodes(fragment_id).len();
    for i in 0..len {
        let id = component.fragment_nodes(fragment_id)[i];
        let node = component.store.get(id);
        match node {
            Node::ExpressionTag(tag) => {
                let data = build_expr_data(tag.id, tag.expression.id(), ctx);
                store.set(tag.id, ExpressionSemantics::Expression(data));
            }
            Node::Element(el) => visit_element(component, el, ctx, store),
            Node::SvelteElement(el) => visit_svelte_element(component, el, ctx, store),
            Node::IfBlock(b) => {
                visit_fragment(component, b.consequent, ctx, store);
                if let Some(alt) = b.alternate {
                    visit_fragment(component, alt, ctx, store);
                }
            }
            Node::EachBlock(b) => {
                visit_fragment(component, b.body, ctx, store);
                if let Some(fb) = b.fallback {
                    visit_fragment(component, fb, ctx, store);
                }
            }
            Node::ComponentNode(cn) => {
                visit_fragment(component, cn.fragment, ctx, store);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    visit_fragment(component, fid, ctx, store);
                }
            }
            Node::SlotElementLegacy(el) => visit_fragment(component, el.fragment, ctx, store),
            Node::SnippetBlock(b) => visit_fragment(component, b.body, ctx, store),
            Node::AwaitBlock(b) => {
                if let Some(f) = b.pending {
                    visit_fragment(component, f, ctx, store);
                }
                if let Some(f) = b.then {
                    visit_fragment(component, f, ctx, store);
                }
                if let Some(f) = b.catch {
                    visit_fragment(component, f, ctx, store);
                }
            }
            Node::KeyBlock(b) => visit_fragment(component, b.fragment, ctx, store),
            Node::SvelteFragmentLegacy(el) => visit_fragment(component, el.fragment, ctx, store),
            Node::SvelteHead(el) => visit_fragment(component, el.fragment, ctx, store),
            Node::SvelteBoundary(el) => visit_fragment(component, el.fragment, ctx, store),
            _ => {}
        }
    }
}

fn visit_element(
    component: &Component,
    el: &Element,
    ctx: &Ctx<'_, '_>,
    store: &mut ExpressionSemanticsStore,
) {
    visit_fragment(component, el.fragment, ctx, store);
}

fn visit_svelte_element(
    component: &Component,
    el: &SvelteElement,
    ctx: &Ctx<'_, '_>,
    store: &mut ExpressionSemanticsStore,
) {
    visit_fragment(component, el.fragment, ctx, store);
}

fn build_expr_data(tag_id: NodeId, expr_id: OxcNodeId, ctx: &Ctx<'_, '_>) -> ExpressionData {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        return ExpressionData {
            kind: ExprKind::Static,
            blockers: SmallVec::new(),
            legacy_wrap: LegacyWrap::None,
            memoization: Memoization::None,
            references: SmallVec::new(),
        };
    };
    let facts = collect(expr, ctx.semantics, ctx.reactivity, ctx.pickled);

    if matches!(expr, oxc_ast::ast::Expression::Identifier(_))
        && facts.references.len() == 1
        && let Some(folded) = ctx.scoping.known_value_by_sym(facts.references[0])
    {
        return ExpressionData {
            kind: ExprKind::Folded(folded.into()),
            blockers: SmallVec::new(),
            legacy_wrap: LegacyWrap::None,
            memoization: Memoization::None,
            references: SmallVec::new(),
        };
    }

    let info = ctx.expressions.get(tag_id);
    let is_dynamic = info.is_some_and(|info| {
        crate::passes::dynamism::classify_template_info_pub(
            info,
            ctx.scoping,
            ctx.reactivity,
            ctx.has_class_state_fields,
        )
    });
    let blockers = derive::blockers(&facts, ctx.blockers);
    let kind = derive::kind(&facts, !blockers.is_empty(), is_dynamic);
    ExpressionData {
        kind,
        blockers,
        legacy_wrap: derive::legacy_wrap(ctx.runes, &facts),
        memoization: derive::memoization(&facts),
        references: facts.references,
    }
}
