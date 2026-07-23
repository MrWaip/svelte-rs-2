use super::super::ContextSignal;
use super::super::ExpressionSemanticsStore;
use super::super::data::{
    Evaluation, ExpressionData, ExpressionSemantics, LegacyWrap, SyntheticPropsCarrier, Volatility,
};
use super::collector::{ExprFacts, collect};
use super::derive;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{BindingSemantics, BlockerData, JsAst, PropBindingKind, SnippetData};
use crate::utils::node_id_utils::{argument_node_id, expression_node_id};
use crate::value_evaluation::{ReadContext, ValueEvaluation, ValueEvaluator};
use oxc_ast::ast::{Argument, ChainElement, Expression, Statement};
use smallvec::SmallVec;
use svelte_ast::{
    Attribute, Component, ConcatPart, Element, FragmentId, Node, NodeId, StyleDirectiveValue,
    SvelteElement,
};
use svelte_component_semantics::{ComponentSemantics, OxcNodeId};

#[allow(clippy::too_many_arguments)]
pub(super) fn populate<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    scoping: &ComponentScoping,
    snippets: &SnippetData,
    value_evaluation: &ValueEvaluation,
    has_class_state_fields: bool,
    blockers: &BlockerData,
    runes_mode: svelte_ast::RunesMode,
    store: &mut ExpressionSemanticsStore,
    dev: bool,
) {
    let evaluator = ValueEvaluator::new(
        parsed,
        component,
        scoping,
        semantics,
        reactivity,
        snippets,
        ReadContext::Runtime,
        dev,
    );
    let mut declared_evaluator = evaluator.duplicate_with_context(ReadContext::Declaration);
    declared_evaluator.ingest_const_tag_bindings(component, parsed);
    let ctx = Ctx {
        parsed,
        semantics,
        reactivity,
        scoping,
        value_evaluation,
        has_class_state_fields,
        blockers,
        uses_legacy_coarse_wrap: matches!(runes_mode, svelte_ast::RunesMode::HardLegacy),
        evaluator,
        declared_evaluator,
    };
    let mut sink = Sink { store };
    visit_fragment(component, component.root, &ctx, &mut sink);
}

pub(super) struct Ctx<'c, 'a> {
    pub(super) parsed: &'c JsAst<'a>,
    pub(super) semantics: &'c ComponentSemantics<'a>,
    pub(super) reactivity: &'c ReactivitySemantics,
    pub(super) scoping: &'c ComponentScoping<'a>,
    pub(super) value_evaluation: &'c ValueEvaluation,
    pub(super) has_class_state_fields: bool,
    pub(super) blockers: &'c BlockerData,
    pub(super) uses_legacy_coarse_wrap: bool,
    pub(super) evaluator: ValueEvaluator<'c, 'a>,
    pub(super) declared_evaluator: ValueEvaluator<'c, 'a>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SiteContext {
    Text,
    ElementAttr,
    StyleDirective,
    ComponentAttr,
    ComponentName,
    Structural,
    Inert,
}

struct Sink<'s> {
    store: &'s mut ExpressionSemanticsStore,
}

impl Sink<'_> {
    fn set(&mut self, id: NodeId, value: ExpressionSemantics) {
        self.store.set(id, value);
    }

    fn set_by_oxc(&mut self, id: OxcNodeId, value: ExpressionSemantics) {
        self.store.set_by_oxc(id, value);
    }

    fn note_context(&mut self, signal: ContextSignal) {
        self.store.note_context(signal);
    }
}

fn visit_fragment(
    component: &Component,
    fragment_id: FragmentId,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    for &id in component.fragment_nodes(fragment_id) {
        let node = component.store.get(id);
        match node {
            Node::ExpressionTag(tag) => {
                store_single(tag.id, tag.expression.id(), ctx, sink, SiteContext::Text);
            }
            Node::HtmlTag(tag) => {
                store_single(tag.id, tag.expression.id(), ctx, sink, SiteContext::Text);
            }
            Node::ConstTag(tag) => {
                store_const_tag(tag.id, tag.decl.id(), ctx, sink);
            }
            Node::DeclarationTag(tag) => {
                store_declaration_tag(tag.id, tag.declaration.id(), ctx, sink);
            }
            Node::Element(el) => visit_element(component, el, ctx, sink),
            Node::SvelteElement(el) => {
                if let Some(expr_ref) = el.this_expr() {
                    store_single(el.id, expr_ref.id(), ctx, sink, SiteContext::Inert);
                }
                visit_svelte_element(component, el, ctx, sink);
            }
            Node::SvelteWindow(el) => {
                visit_attributes(&el.attributes, ctx, sink, SiteContext::ElementAttr)
            }
            Node::SvelteDocument(el) => {
                visit_attributes(&el.attributes, ctx, sink, SiteContext::ElementAttr)
            }
            Node::SvelteBody(el) => {
                visit_attributes(&el.attributes, ctx, sink, SiteContext::ElementAttr)
            }
            Node::IfBlock(b) => {
                store_single(b.id, b.test.id(), ctx, sink, SiteContext::Text);
                visit_fragment(component, b.consequent, ctx, sink);
                if let Some(alt) = b.alternate {
                    visit_fragment(component, alt, ctx, sink);
                }
            }
            Node::EachBlock(b) => {
                store_single(b.id, b.expression.id(), ctx, sink, SiteContext::Text);
                if let (Some(key_id), Some(key)) = (b.key_id, b.key.as_ref()) {
                    store_single(key_id, key.id(), ctx, sink, SiteContext::Text);
                }
                visit_fragment(component, b.body, ctx, sink);
                if let Some(fb) = b.fallback {
                    visit_fragment(component, fb, ctx, sink);
                }
            }
            Node::RenderTag(t) => {
                store_render_tag(t.id, t.expression.id(), ctx, sink);
                store_render_args(t.expression.id(), ctx, sink);
            }
            Node::ComponentNode(cn) => {
                store_component_name(cn.id, cn.name.id(), ctx, sink);
                visit_attributes(&cn.attributes, ctx, sink, SiteContext::ComponentAttr);
                visit_fragment(component, cn.fragment, ctx, sink);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    visit_fragment(component, fid, ctx, sink);
                }
            }
            Node::SvelteComponentLegacy(cn) => {
                if let Some(this_expr) = cn.this_expr() {
                    store_svelte_component_this(cn.id, this_expr.id(), ctx, sink);
                }
                visit_attributes(&cn.attributes, ctx, sink, SiteContext::ElementAttr);
                visit_fragment(component, cn.fragment, ctx, sink);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    visit_fragment(component, fid, ctx, sink);
                }
            }
            Node::SvelteSelf(cn) => {
                visit_attributes(&cn.attributes, ctx, sink, SiteContext::ComponentAttr);
                visit_fragment(component, cn.fragment, ctx, sink);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    visit_fragment(component, fid, ctx, sink);
                }
            }
            Node::SlotElementLegacy(el) => {
                visit_attributes(&el.attributes, ctx, sink, SiteContext::ElementAttr);
                visit_fragment(component, el.fragment, ctx, sink);
            }
            Node::SnippetBlock(b) => visit_fragment(component, b.body, ctx, sink),
            Node::AwaitBlock(b) => {
                store_single(b.id, b.expression.id(), ctx, sink, SiteContext::Structural);
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
                store_single(b.id, b.expression.id(), ctx, sink, SiteContext::Text);
                visit_fragment(component, b.fragment, ctx, sink);
            }
            Node::SvelteFragmentLegacy(el) => visit_fragment(component, el.fragment, ctx, sink),
            Node::SvelteHead(el) => visit_fragment(component, el.fragment, ctx, sink),
            Node::SvelteBoundary(el) => {
                visit_attributes(&el.attributes, ctx, sink, SiteContext::ComponentAttr);
                visit_fragment(component, el.fragment, ctx, sink);
            }
            _ => {}
        }
    }
}

fn visit_element(component: &Component, el: &Element, ctx: &Ctx<'_, '_>, sink: &mut Sink<'_>) {
    visit_attributes(&el.attributes, ctx, sink, SiteContext::ElementAttr);
    visit_fragment(component, el.fragment, ctx, sink);
}

fn visit_svelte_element(
    component: &Component,
    el: &SvelteElement,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    visit_attributes(&el.attributes, ctx, sink, SiteContext::ElementAttr);
    visit_fragment(component, el.fragment, ctx, sink);
}

fn visit_attributes(
    attrs: &[Attribute],
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
    context: SiteContext,
) {
    for attr in attrs {
        match attr {
            Attribute::ExpressionAttribute(a) => {
                store_single(a.id, a.expression.id(), ctx, sink, context);
            }
            Attribute::ConcatenationAttribute(a) => {
                for p in &a.parts {
                    if let ConcatPart::Dynamic { id, expr } = p {
                        store_single(*id, expr.id(), ctx, sink, context);
                    }
                }
                let parts = a.parts.iter().filter_map(|p| match p {
                    ConcatPart::Dynamic { expr, .. } => Some(expr.id()),
                    ConcatPart::Static(_) => None,
                });
                store_aggregate(a.id, parts, ctx, sink, context);
            }
            Attribute::SpreadAttribute(a) => {
                store_single(a.id, a.expression.id(), ctx, sink, context);
            }
            Attribute::ClassDirective(a) => {
                store_single(a.id, a.expression.id(), ctx, sink, context);
            }
            Attribute::StyleDirective(a) => match &a.value {
                StyleDirectiveValue::Concatenation(parts) => {
                    for p in parts {
                        if let ConcatPart::Dynamic { id, expr } = p {
                            store_single(*id, expr.id(), ctx, sink, SiteContext::StyleDirective);
                        }
                    }
                    let exprs = parts.iter().filter_map(|p| match p {
                        ConcatPart::Dynamic { expr, .. } => Some(expr.id()),
                        ConcatPart::Static(_) => None,
                    });
                    store_aggregate(a.id, exprs, ctx, sink, SiteContext::StyleDirective);
                }
                StyleDirectiveValue::Expression => {
                    store_single(
                        a.id,
                        a.expression.id(),
                        ctx,
                        sink,
                        SiteContext::StyleDirective,
                    );
                }
                StyleDirectiveValue::String(_) => {}
            },
            Attribute::BindDirective(a) => {
                store_single(a.id, a.expression.id(), ctx, sink, context);
            }
            Attribute::UseDirective(a) => {
                if let Some(expr) = &a.expression {
                    store_single(a.id, expr.id(), ctx, sink, context);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(expr) = &a.expression {
                    store_single(a.id, expr.id(), ctx, sink, context);
                }
            }
            Attribute::AnimateDirective(a) => {
                if let Some(expr) = &a.expression {
                    store_single(a.id, expr.id(), ctx, sink, context);
                }
            }
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(expr) = &a.expression {
                    store_single(a.id, expr.id(), ctx, sink, context);
                }
            }
            Attribute::AttachTag(a) => {
                store_single(a.id, a.expression.id(), ctx, sink, context);
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
    context: SiteContext,
) {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        sink.set(site_id, ExpressionSemantics::Expression(empty_data()));
        return;
    };
    let (data, facts) = compute(expr, ctx, context);
    update_aggregates(sink, &facts, ctx);
    let value = ExpressionSemantics::Expression(data);
    sink.set_by_oxc(expression_node_id(expr), value.clone());
    sink.set(site_id, value);
}

fn store_component_name(
    site_id: NodeId,
    expr_id: OxcNodeId,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        return;
    };
    let (data, _facts) = compute(expr, ctx, SiteContext::ComponentName);
    sink.set(site_id, ExpressionSemantics::Expression(data));
}

fn store_svelte_component_this(
    site_id: NodeId,
    this_expr_id: OxcNodeId,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    let Some(expr) = ctx.parsed.expr(this_expr_id) else {
        return;
    };
    let (data, _facts) = compute(expr, ctx, SiteContext::Structural);
    sink.set(site_id, ExpressionSemantics::Expression(data));
}

fn store_render_tag(site_id: NodeId, expr_id: OxcNodeId, ctx: &Ctx<'_, '_>, sink: &mut Sink<'_>) {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        sink.set(site_id, ExpressionSemantics::Expression(empty_data()));
        return;
    };
    let (data, facts) = compute(expr, ctx, SiteContext::Text);
    for &sym in facts.member_or_call_roots.iter() {
        if ctx.reactivity.is_rest_prop(sym) {
            sink.note_context(ContextSignal::REST_PROP_MEMBER);
        }
    }
    for &sym in facts.member_roots.iter() {
        if !is_safe_member_root(ctx.reactivity, sym) {
            sink.note_context(ContextSignal::IMPORT_OR_PROP_MEMBER);
        }
    }
    if facts.has_legacy_props_member_root {
        sink.note_context(ContextSignal::REST_PROP_MEMBER);
    }
    if facts.has_store_member_mutation {
        sink.note_context(ContextSignal::STORE_MUTATION);
    }
    let value = ExpressionSemantics::Expression(data);
    sink.set_by_oxc(expression_node_id(expr), value.clone());
    sink.set(site_id, value);
}

fn store_render_args(expr_id: OxcNodeId, ctx: &Ctx<'_, '_>, sink: &mut Sink<'_>) {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        return;
    };
    let call = match expr.get_inner_expression() {
        Expression::CallExpression(c) => Some(c.as_ref()),
        Expression::ChainExpression(chain) => match &chain.expression {
            ChainElement::CallExpression(c) => Some(c.as_ref()),
            _ => None,
        },
        _ => None,
    };
    let Some(call) = call else { return };
    for arg in &call.arguments {
        if matches!(arg, Argument::SpreadElement(_)) {
            continue;
        }
        let arg_expr = arg.to_expression();
        let (data, facts) = compute(arg_expr, ctx, SiteContext::Text);
        update_aggregates(sink, &facts, ctx);
        sink.set_by_oxc(argument_node_id(arg), ExpressionSemantics::Expression(data));
    }
}

fn store_const_tag(site_id: NodeId, stmt_id: OxcNodeId, ctx: &Ctx<'_, '_>, sink: &mut Sink<'_>) {
    let Some(Statement::VariableDeclaration(decl)) = ctx.parsed.stmt(stmt_id) else {
        return;
    };
    let Some(d) = decl.declarations.first() else {
        return;
    };
    let Some(expr) = d.init.as_ref() else {
        return;
    };
    let (data, facts) = compute(expr, ctx, SiteContext::Text);
    update_aggregates(sink, &facts, ctx);
    let value = ExpressionSemantics::Expression(data);
    sink.set_by_oxc(expression_node_id(expr), value.clone());
    sink.set(site_id, value);
}

fn store_declaration_tag(
    site_id: NodeId,
    stmt_id: OxcNodeId,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
) {
    let Some(Statement::VariableDeclaration(decl)) = ctx.parsed.stmt(stmt_id) else {
        return;
    };
    let mut site_value: Option<ExpressionSemantics> = None;
    for d in &decl.declarations {
        let Some(expr) = d.init.as_ref() else {
            continue;
        };
        let (data, facts) = compute(expr, ctx, SiteContext::Text);
        update_aggregates(sink, &facts, ctx);
        let value = ExpressionSemantics::Expression(data);
        sink.set_by_oxc(expression_node_id(expr), value.clone());
        if site_value.is_none() {
            site_value = Some(value);
        }
    }
    if let Some(value) = site_value {
        sink.set(site_id, value);
    }
}

fn store_aggregate(
    site_id: NodeId,
    expr_ids: impl IntoIterator<Item = OxcNodeId>,
    ctx: &Ctx<'_, '_>,
    sink: &mut Sink<'_>,
    context: SiteContext,
) {
    let mut acc = empty_data();
    let mut any = false;
    for expr_id in expr_ids {
        any = true;
        let Some(expr) = ctx.parsed.expr(expr_id) else {
            continue;
        };
        let (part, facts) = compute(expr, ctx, context);
        update_aggregates(sink, &facts, ctx);
        acc.volatility = acc.volatility.max(part.volatility);
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
    sink.set(site_id, ExpressionSemantics::Expression(acc));
}

fn empty_data() -> ExpressionData {
    ExpressionData {
        volatility: Volatility::Static,
        evaluation: Evaluation::unknown(),
        declared_evaluation: Evaluation::unknown(),
        blockers: SmallVec::new(),
        legacy_wrap: LegacyWrap::None,
        references: SmallVec::new(),
    }
}

fn is_detached_const_read(expr: &Expression<'_>, ctx: &Ctx<'_, '_>) -> bool {
    let Expression::Identifier(id) = expr else {
        return false;
    };
    let Some(ref_id) = id.reference_id.get() else {
        return false;
    };
    ctx.reactivity.is_detached_const_read(ref_id)
}

fn compute<'a>(
    expr: &Expression<'a>,
    ctx: &Ctx<'_, 'a>,
    context: SiteContext,
) -> (ExpressionData, ExprFacts) {
    let facts = collect(expr, ctx.semantics, ctx.reactivity);

    let evaluation = ctx.evaluator.evaluate(expr);

    let is_reactive = derive::is_reactive_template(&facts, ctx);
    let blockers = derive::blockers(&facts, ctx.blockers);
    let has_blockers = !blockers.is_empty();
    let reactive_gate = match context {
        SiteContext::Text => derive::volatile(
            &facts,
            has_blockers,
            is_reactive,
            &evaluation,
            ctx.reactivity,
        ),
        SiteContext::ElementAttr | SiteContext::StyleDirective => {
            derive::volatile_element_attr(is_reactive, &facts.references, ctx)
        }
        SiteContext::ComponentAttr => is_reactive,
        SiteContext::ComponentName => derive::volatile_component_name(
            expr,
            ctx.reactivity.uses_runes(),
            ctx.scoping,
            ctx.reactivity,
        ),
        SiteContext::Structural => true,
        SiteContext::Inert => false,
    };
    let has_context_member_root = facts
        .top_member_or_call_roots
        .iter()
        .any(|&sym| is_context_member_root(ctx.reactivity.binding_semantics(sym)));
    let volatility = derive::volatility(reactive_gate, &facts);
    let inline_style_emit = match context {
        SiteContext::StyleDirective => true,
        SiteContext::Text
        | SiteContext::ElementAttr
        | SiteContext::ComponentAttr
        | SiteContext::ComponentName
        | SiteContext::Structural
        | SiteContext::Inert => false,
    };
    let evaluation = match volatility {
        Volatility::Static | Volatility::Reactive => evaluation,
        Volatility::Heavy | Volatility::Asynchronous => {
            if inline_style_emit {
                match &evaluation {
                    Evaluation::Known(_) => Evaluation::unknown(),
                    Evaluation::Defined { .. } | Evaluation::MaybeNullish { .. } => evaluation,
                }
            } else {
                Evaluation::unknown()
            }
        }
    };
    let declared_evaluation = ctx.declared_evaluator.evaluate(expr);
    let evaluation = if is_detached_const_read(expr, ctx) {
        declared_evaluation.clone()
    } else {
        evaluation
    };
    let data = ExpressionData {
        volatility,
        evaluation,
        declared_evaluation,
        blockers,
        legacy_wrap: derive::legacy_wrap(
            ctx.uses_legacy_coarse_wrap,
            &facts,
            has_context_member_root,
        ),
        references: facts.references.clone(),
    };
    (data, facts)
}

fn update_aggregates(sink: &mut Sink<'_>, facts: &ExprFacts, ctx: &Ctx<'_, '_>) {
    for &sym in facts.member_or_call_roots.iter() {
        if !is_safe_member_root(ctx.reactivity, sym) {
            sink.note_context(ContextSignal::IMPORT_OR_PROP_MEMBER);
        }
        if ctx.reactivity.is_rest_prop(sym) {
            sink.note_context(ContextSignal::REST_PROP_MEMBER);
        }
    }
    if facts.has_legacy_props_member_root {
        sink.note_context(ContextSignal::REST_PROP_MEMBER);
    }
    if facts.has_runtime_root {
        sink.note_context(ContextSignal::IMPORT_OR_PROP_MEMBER);
    }
    if facts.has_unsafe_member_root {
        sink.note_context(ContextSignal::IMPORT_OR_PROP_MEMBER);
    }
    if facts.has_unsafe_callee_or_new {
        sink.note_context(ContextSignal::UNSAFE_CALLEE_OR_NEW);
    }
    if facts.has_store_member_mutation {
        sink.note_context(ContextSignal::STORE_MUTATION);
    }
}

fn is_context_member_root(semantics: BindingSemantics) -> bool {
    match semantics {
        BindingSemantics::MaybeReactive
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::Contextual(_) => true,
        BindingSemantics::Prop(prop) => match &prop.kind {
            PropBindingKind::Source { .. } | PropBindingKind::NonSource => true,
            PropBindingKind::Identifier | PropBindingKind::Rest => false,
        },
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => false,
    }
}

fn is_safe_member_root(reactivity: &ReactivitySemantics, sym: SymbolId) -> bool {
    match reactivity.binding_semantics(sym) {
        BindingSemantics::MaybeReactive | BindingSemantics::LegacyBindableProp(_) => false,
        BindingSemantics::Prop(prop) => match &prop.kind {
            PropBindingKind::Source { .. } | PropBindingKind::NonSource => false,
            PropBindingKind::Identifier | PropBindingKind::Rest => true,
        },
        BindingSemantics::Store(store) => is_safe_member_root(reactivity, store.base_symbol),
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Contextual(_)
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => true,
    }
}

fn combine_legacy_wrap(a: LegacyWrap, b: LegacyWrap) -> LegacyWrap {
    let coarse = is_coarse(a) || is_coarse(b);
    let carrier = combine_synthetic_carrier(carrier_of(a), carrier_of(b));
    match (coarse, carrier) {
        (false, None) => LegacyWrap::None,
        (true, None) => LegacyWrap::CoarseWrap,
        (false, Some(c)) => LegacyWrap::Synthetic(c),
        (true, Some(c)) => LegacyWrap::CoarseAndSynthetic(c),
    }
}

fn is_coarse(w: LegacyWrap) -> bool {
    matches!(
        w,
        LegacyWrap::CoarseWrap | LegacyWrap::CoarseAndSynthetic(_)
    )
}

fn carrier_of(w: LegacyWrap) -> Option<SyntheticPropsCarrier> {
    match w {
        LegacyWrap::Synthetic(c) | LegacyWrap::CoarseAndSynthetic(c) => Some(c),
        LegacyWrap::None | LegacyWrap::CoarseWrap => None,
    }
}

fn combine_synthetic_carrier(
    a: Option<SyntheticPropsCarrier>,
    b: Option<SyntheticPropsCarrier>,
) -> Option<SyntheticPropsCarrier> {
    let (reads_props, reads_rest) = carrier_bits(a);
    let (reads_props_b, reads_rest_b) = carrier_bits(b);
    derive::synthetic_props_carrier(reads_props || reads_props_b, reads_rest || reads_rest_b)
}

fn carrier_bits(c: Option<SyntheticPropsCarrier>) -> (bool, bool) {
    match c {
        None => (false, false),
        Some(SyntheticPropsCarrier::SanitizedProps) => (true, false),
        Some(SyntheticPropsCarrier::RestProps) => (false, true),
        Some(SyntheticPropsCarrier::Both) => (true, true),
    }
}
