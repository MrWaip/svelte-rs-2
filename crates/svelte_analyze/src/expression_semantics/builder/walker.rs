use super::super::ExpressionSemanticsStore;
use super::super::ContextSignal;
use super::super::data::{ExprKind, ExpressionData, ExpressionSemantics, LegacyWrap, Memoization};
use super::collector::{ExprFacts, TopLevelShape, collect};
use super::derive;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::ComponentScoping;
use crate::types::data::{BlockerData, JsAst};
use smallvec::SmallVec;
use svelte_ast::{
    Attribute, Component, ConcatPart, Element, FragmentId, Node, NodeId, StyleDirectiveValue,
    SvelteElement,
};
use svelte_component_semantics::{ComponentSemantics, OxcNodeId};

pub(super) fn populate<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    scoping: &ComponentScoping,
    has_class_state_fields: bool,
    blockers: &BlockerData,
    store: &mut ExpressionSemanticsStore,
) {
    let ctx = Ctx {
        parsed,
        semantics,
        reactivity,
        scoping,
        has_class_state_fields,
        blockers,
        runes: reactivity.uses_runes(),
    };
    let mut sink = Sink { store };
    visit_fragment(component, component.root, &ctx, &mut sink);
}

pub(super) struct Ctx<'c, 'a> {
    pub(super) parsed: &'c JsAst<'a>,
    pub(super) semantics: &'c ComponentSemantics<'a>,
    pub(super) reactivity: &'c ReactivitySemantics,
    pub(super) scoping: &'c ComponentScoping<'a>,
    pub(super) has_class_state_fields: bool,
    pub(super) blockers: &'c BlockerData,
    pub(super) runes: bool,
}

struct Sink<'s> {
    store: &'s mut ExpressionSemanticsStore,
}

fn visit_fragment(
    component: &Component,
    fragment_id: FragmentId,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    let len = component.fragment_nodes(fragment_id).len();
    for i in 0..len {
        let id = component.fragment_nodes(fragment_id)[i];
        let node = component.store.get(id);
        match node {
            Node::ExpressionTag(tag) => {
                store_single(tag.id, tag.expression.id(), ctx, sink);
            }
            Node::HtmlTag(tag) => {
                store_single(tag.id, tag.expression.id(), ctx, sink);
            }
            Node::ConstTag(tag) => {
                store_const_tag(tag.id, tag.decl.id(), ctx, sink);
            }
            Node::Element(el) => visit_element(component, el, ctx, sink),
            Node::SvelteElement(el) => {
                if let Some(expr_ref) = el.this_expr() {
                    store_single(el.id, expr_ref.id(), ctx, sink);
                }
                visit_svelte_element(component, el, ctx, sink);
            }
            Node::SvelteWindow(el) => visit_attributes(&el.attributes, ctx, sink),
            Node::SvelteDocument(el) => visit_attributes(&el.attributes, ctx, sink),
            Node::SvelteBody(el) => visit_attributes(&el.attributes, ctx, sink),
            Node::IfBlock(b) => {
                store_single(b.id, b.test.id(), ctx, sink);
                visit_fragment(component, b.consequent, ctx, sink);
                if let Some(alt) = b.alternate {
                    visit_fragment(component, alt, ctx, sink);
                }
            }
            Node::EachBlock(b) => {
                visit_fragment(component, b.body, ctx, sink);
                if let Some(fb) = b.fallback {
                    visit_fragment(component, fb, ctx, sink);
                }
            }
            Node::RenderTag(t) => {
                store_render_tag(t.id, t.expression.id(), ctx, sink);
            }
            Node::ComponentNode(cn) => {
                visit_attributes(&cn.attributes, ctx, sink);
                visit_fragment(component, cn.fragment, ctx, sink);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    visit_fragment(component, fid, ctx, sink);
                }
            }
            Node::SvelteComponentLegacy(cn) => {
                visit_attributes(&cn.attributes, ctx, sink);
                visit_fragment(component, cn.fragment, ctx, sink);
            }
            Node::SlotElementLegacy(el) => {
                visit_attributes(&el.attributes, ctx, sink);
                visit_fragment(component, el.fragment, ctx, sink);
            }
            Node::SnippetBlock(b) => visit_fragment(component, b.body, ctx, sink),
            Node::AwaitBlock(b) => {
                store_single(b.id, b.expression.id(), ctx, sink);
                if let Some(f) = b.pending {
                    visit_fragment(component, f, ctx, sink);
                }
                if let Some(f) = b.then {
                    visit_fragment(component, f, ctx, sink);
                }
                if let Some(f) = b.catch {
                    visit_fragment(component, f, ctx, sink);
                }
            }
            Node::KeyBlock(b) => {
                store_single(b.id, b.expression.id(), ctx, sink);
                visit_fragment(component, b.fragment, ctx, sink);
            }
            Node::SvelteFragmentLegacy(el) => visit_fragment(component, el.fragment, ctx, sink),
            Node::SvelteHead(el) => visit_fragment(component, el.fragment, ctx, sink),
            Node::SvelteBoundary(el) => {
                visit_attributes(&el.attributes, ctx, sink);
                visit_fragment(component, el.fragment, ctx, sink);
            }
            _ => {}
        }
    }
}

fn visit_element(
    component: &Component,
    el: &Element,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    visit_attributes(&el.attributes, ctx, sink);
    visit_fragment(component, el.fragment, ctx, sink);
}

fn visit_svelte_element(
    component: &Component,
    el: &SvelteElement,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    visit_attributes(&el.attributes, ctx, sink);
    visit_fragment(component, el.fragment, ctx, sink);
}

fn visit_attributes(
    attrs: &[Attribute],
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    for attr in attrs {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                store_single(a.id, a.expression.id(), ctx, sink);
            }
            Attribute::ConcatenationAttribute(a) => {
                for p in &a.parts {
                    if let ConcatPart::Dynamic { id, expr } = p {
                        store_single(*id, expr.id(), ctx, sink);
                    }
                }
                let parts = a.parts.iter().filter_map(|p| match p {
                    ConcatPart::Dynamic { expr, .. } => Some(expr.id()),
                    ConcatPart::Static(_) => None,
                });
                store_aggregate(a.id, parts, ctx, sink);
            }
            Attribute::SpreadAttribute(a) => {
                store_single(a.id, a.expression.id(), ctx, sink);
            }
            Attribute::ClassDirective(a) => {
                store_single(a.id, a.expression.id(), ctx, sink);
            }
            Attribute::StyleDirective(a) => match &a.value {
                StyleDirectiveValue::Concatenation(parts) => {
                    for p in parts {
                        if let ConcatPart::Dynamic { id, expr } = p {
                            store_single(*id, expr.id(), ctx, sink);
                        }
                    }
                    let exprs = parts.iter().filter_map(|p| match p {
                        ConcatPart::Dynamic { expr, .. } => Some(expr.id()),
                        ConcatPart::Static(_) => None,
                    });
                    store_aggregate(a.id, exprs, ctx, sink);
                }
                StyleDirectiveValue::Expression => {
                    store_single(a.id, a.expression.id(), ctx, sink);
                }
                StyleDirectiveValue::String(_) => {}
            },
            Attribute::BindDirective(a) => {
                store_single(a.id, a.expression.id(), ctx, sink);
            }
            Attribute::UseDirective(a) => {
                if let Some(expr) = &a.expression {
                    store_single(a.id, expr.id(), ctx, sink);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(expr) = &a.expression {
                    store_single(a.id, expr.id(), ctx, sink);
                }
            }
            Attribute::AnimateDirective(a) => {
                if let Some(expr) = &a.expression {
                    store_single(a.id, expr.id(), ctx, sink);
                }
            }
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(expr) = &a.expression {
                    store_single(a.id, expr.id(), ctx, sink);
                }
            }
            Attribute::AttachTag(a) => {
                store_single(a.id, a.expression.id(), ctx, sink);
            }
            Attribute::StringAttribute(_)
            | Attribute::BooleanAttribute(_)
            | Attribute::LetDirectiveLegacy(_) => {}
        }
    }
}

fn store_single(
    site_id: NodeId,
    expr_id: OxcNodeId,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        sink.store.set(site_id, ExpressionSemantics::Expression(empty_data()));
        return;
    };
    let (data, facts) = compute(expr, ctx);
    update_aggregates(sink.store, &facts, ctx);
    sink.store.set(site_id, ExpressionSemantics::Expression(data));
}

fn store_render_tag(
    site_id: NodeId,
    expr_id: OxcNodeId,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        sink.store.set(site_id, ExpressionSemantics::Expression(empty_data()));
        return;
    };
    let (data, facts) = compute(expr, ctx);
    let context_sensitive_shape = matches!(
        facts.top_level_shape,
        TopLevelShape::Member | TopLevelShape::Call
    );
    if context_sensitive_shape {
        for &sym in facts.references.iter() {
            if ctx.scoping.is_rest_prop(sym) {
                sink.store.note_context(ContextSignal::REST_PROP_MEMBER);
            }
        }
    }
    if facts.has_store_member_mutation {
        sink.store.note_context(ContextSignal::STORE_MUTATION);
    }
    sink.store.set(site_id, ExpressionSemantics::Expression(data));
}

fn store_const_tag(
    site_id: NodeId,
    stmt_id: OxcNodeId,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) = ctx.parsed.stmt(stmt_id) else {
        return;
    };
    let Some(d) = decl.declarations.first() else {
        return;
    };
    let Some(expr) = d.init.as_ref() else {
        return;
    };
    let (data, facts) = compute(expr, ctx);
    update_aggregates(sink.store, &facts, ctx);
    sink.store.set(site_id, ExpressionSemantics::Expression(data));
}

fn store_aggregate(
    site_id: NodeId,
    expr_ids: impl IntoIterator<Item = OxcNodeId>,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    let mut acc = empty_data();
    let mut any = false;
    for expr_id in expr_ids {
        any = true;
        let Some(expr) = ctx.parsed.expr(expr_id) else {
            continue;
        };
        let (part, facts) = compute(expr, ctx);
        update_aggregates(sink.store, &facts, ctx);
        acc.kind = max_kind(&acc.kind, &part.kind);
        acc.memoization = max_memoization(acc.memoization, part.memoization);
        acc.legacy_wrap = combine_legacy_wrap(acc.legacy_wrap, part.legacy_wrap);
        for b in part.blockers {
            if !acc.blockers.contains(&b) {
                acc.blockers.push(b);
            }
        }
        for sym in part.references {
            if !acc.references.contains(&sym) {
                acc.references.push(sym);
            }
        }
    }
    if !any {
        return;
    }
    acc.blockers.sort_unstable();
    sink.store.set(site_id, ExpressionSemantics::Expression(acc));
}

fn empty_data() -> ExpressionData {
    ExpressionData {
        kind: ExprKind::Static,
        blockers: SmallVec::new(),
        legacy_wrap: LegacyWrap::None,
        memoization: Memoization::None,
        references: SmallVec::new(),
    }
}

fn compute<'a>(
    expr: &oxc_ast::ast::Expression<'a>,
    ctx: &Ctx<'_, 'a>,
) -> (ExpressionData, ExprFacts) {
    let facts = collect(expr, ctx.semantics, ctx.reactivity);

    if matches!(expr, oxc_ast::ast::Expression::Identifier(_))
        && facts.references.len() == 1
        && let Some(folded) = ctx.scoping.known_value_by_sym(facts.references[0])
    {
        let data = ExpressionData {
            kind: ExprKind::Folded(folded.into()),
            blockers: SmallVec::new(),
            legacy_wrap: LegacyWrap::None,
            memoization: Memoization::None,
            references: SmallVec::new(),
        };
        return (data, facts);
    }

    let is_dynamic = derive::is_dynamic_template(
        &facts,
        ctx.scoping,
        ctx.reactivity,
        ctx.has_class_state_fields,
    );
    let blockers = derive::blockers(&facts, ctx.blockers);
    let kind = derive::kind(&facts, !blockers.is_empty(), is_dynamic);
    let data = ExpressionData {
        kind,
        blockers,
        legacy_wrap: derive::legacy_wrap(ctx.runes, &facts),
        memoization: derive::memoization(&facts),
        references: facts.references.clone(),
    };
    (data, facts)
}

fn update_aggregates(
    store: &mut ExpressionSemanticsStore,
    facts: &ExprFacts,
    ctx: &Ctx<'_, '_>,
) {
    let context_sensitive_shape = matches!(
        facts.top_level_shape,
        TopLevelShape::Member | TopLevelShape::Call
    );
    if context_sensitive_shape {
        for &sym in facts.references.iter() {
            if ctx.scoping.is_import(sym) || is_prop_source_or_non_source(sym, ctx.reactivity) {
                store.note_context(ContextSignal::IMPORT_OR_PROP_MEMBER);
            }
            if ctx.scoping.is_rest_prop(sym) {
                store.note_context(ContextSignal::REST_PROP_MEMBER);
            }
        }
    }
    if facts.has_store_member_mutation {
        store.note_context(ContextSignal::STORE_MUTATION);
    }
}

fn is_prop_source_or_non_source(
    sym: crate::scope::SymbolId,
    reactivity: &ReactivitySemantics,
) -> bool {
    use crate::types::data::{BindingSemantics, PropBindingKind, PropBindingSemantics};
    matches!(
        reactivity.binding_semantics(sym),
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Source { .. } | PropBindingKind::NonSource,
            ..
        }),
    )
}

fn max_kind(a: &ExprKind, b: &ExprKind) -> ExprKind {
    fn rank(k: &ExprKind) -> u8 {
        match k {
            ExprKind::Folded(_) => 0,
            ExprKind::Static => 1,
            ExprKind::Dynamic => 2,
            ExprKind::Async { .. } => 3,
        }
    }
    if rank(a) >= rank(b) {
        a.clone()
    } else {
        b.clone()
    }
}

fn max_memoization(a: Memoization, b: Memoization) -> Memoization {
    fn rank(m: Memoization) -> u8 {
        match m {
            Memoization::None => 0,
            Memoization::SyncMemo => 1,
            Memoization::AsyncMemo => 2,
        }
    }
    if rank(a) >= rank(b) { a } else { b }
}

fn combine_legacy_wrap(a: LegacyWrap, b: LegacyWrap) -> LegacyWrap {
    let coarse = matches!(a, LegacyWrap::CoarseWrap | LegacyWrap::CoarseAndSanitized)
        || matches!(b, LegacyWrap::CoarseWrap | LegacyWrap::CoarseAndSanitized);
    let sanitized = matches!(a, LegacyWrap::SanitizedProps | LegacyWrap::CoarseAndSanitized)
        || matches!(b, LegacyWrap::SanitizedProps | LegacyWrap::CoarseAndSanitized);
    match (coarse, sanitized) {
        (false, false) => LegacyWrap::None,
        (true, false) => LegacyWrap::CoarseWrap,
        (false, true) => LegacyWrap::SanitizedProps,
        (true, true) => LegacyWrap::CoarseAndSanitized,
    }
}
