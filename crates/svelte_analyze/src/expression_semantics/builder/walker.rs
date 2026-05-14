use super::super::ExpressionSemanticsStore;
use super::super::ContextSignal;
use super::super::data::{
    Evaluation, ExprKind, ExpressionData, ExpressionSemantics, LegacyWrap,
};
use super::super::evaluator::{self, EvalCtx};
use rustc_hash::{FxHashMap, FxHashSet};
use super::collector::{ExprFacts, collect};
use super::derive;
use crate::reactivity_semantics::data::ReactivitySemantics;
use crate::scope::{ComponentScoping, SymbolId};
use crate::types::data::{BindingSemantics, BlockerData, JsAst, PropBindingKind, PropBindingSemantics, SnippetData};
use oxc_ast::ast::{BindingPattern, Declaration, Expression, Function, Statement, VariableDeclaration};
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
    snippets: &SnippetData,
    has_class_state_fields: bool,
    blockers: &BlockerData,
    runes_mode: svelte_ast::RunesMode,
    store: &mut ExpressionSemanticsStore,
    dev: bool,
) {
    let (bindings_init, function_decls) = collect_bindings_init(parsed);
    let ctx = Ctx {
        parsed,
        semantics,
        reactivity,
        scoping,
        snippets,
        has_class_state_fields,
        blockers,
        uses_legacy_coarse_wrap: matches!(runes_mode, svelte_ast::RunesMode::HardLegacy),
        bindings_init,
        function_decls,
        dev,
    };
    let mut sink = Sink { store };
    visit_fragment(component, component.root, &ctx, &mut sink);
}

fn collect_bindings_init<'c, 'a>(
    parsed: &'c JsAst<'a>,
) -> (
    FxHashMap<SymbolId, &'c Expression<'a>>,
    FxHashSet<SymbolId>,
) {
    let mut map: FxHashMap<SymbolId, &'c Expression<'a>> = FxHashMap::default();
    let mut fn_decls: FxHashSet<SymbolId> = FxHashSet::default();

    fn ingest_var_decl<'c, 'a>(
        vd: &'c VariableDeclaration<'a>,
        map: &mut FxHashMap<SymbolId, &'c Expression<'a>>,
    ) {
        for decl in &vd.declarations {
            let Some(init) = decl.init.as_ref() else {
                continue;
            };
            let BindingPattern::BindingIdentifier(id) = &decl.id else {
                continue;
            };
            if let Some(sym) = id.symbol_id.get() {
                map.insert(sym, init);
            }
        }
    }

    fn ingest_fn_decl<'c, 'a>(
        fd: &'c Function<'a>,
        fn_decls: &mut FxHashSet<SymbolId>,
    ) {
        if let Some(id) = &fd.id
            && let Some(sym) = id.symbol_id.get()
        {
            fn_decls.insert(sym);
        }
    }

    let programs = [parsed.program.as_ref(), parsed.module_program.as_ref()];
    for prog in programs.into_iter().flatten() {
        for stmt in &prog.body {
            match stmt {
                Statement::VariableDeclaration(vd) => ingest_var_decl(vd, &mut map),
                Statement::FunctionDeclaration(fd) => ingest_fn_decl(fd, &mut fn_decls),
                Statement::ExportNamedDeclaration(en) => match &en.declaration {
                    Some(Declaration::VariableDeclaration(vd)) => ingest_var_decl(vd, &mut map),
                    Some(Declaration::FunctionDeclaration(fd)) => {
                        ingest_fn_decl(fd, &mut fn_decls)
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    (map, fn_decls)
}

pub(super) struct Ctx<'c, 'a> {
    pub(super) parsed: &'c JsAst<'a>,
    pub(super) semantics: &'c ComponentSemantics<'a>,
    pub(super) reactivity: &'c ReactivitySemantics,
    pub(super) scoping: &'c ComponentScoping<'a>,
    pub(super) snippets: &'c SnippetData,
    pub(super) has_class_state_fields: bool,
    pub(super) blockers: &'c BlockerData,
    pub(super) uses_legacy_coarse_wrap: bool,
    pub(super) bindings_init: FxHashMap<SymbolId, &'c Expression<'a>>,
    pub(super) function_decls: FxHashSet<SymbolId>,
    pub(super) dev: bool,
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
            Node::SvelteSelf(cn) => {
                visit_attributes(&cn.attributes, ctx, sink);
                visit_fragment(component, cn.fragment, ctx, sink);
                let slot_frags: Vec<_> = cn.legacy_slots.iter().map(|s| s.fragment).collect();
                for fid in slot_frags {
                    visit_fragment(component, fid, ctx, sink);
                }
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
    for &sym in facts.member_or_call_roots.iter() {
        if ctx.scoping.is_rest_prop(sym) {
            sink.store.note_context(ContextSignal::REST_PROP_MEMBER);
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
    let Some(Statement::VariableDeclaration(decl)) = ctx.parsed.stmt(stmt_id) else {
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
        kind: ExprKind::Computed { reactive: false },
        evaluation: Evaluation::unknown(),
        blockers: SmallVec::new(),
        legacy_wrap: LegacyWrap::None,
        references: SmallVec::new(),
    }
}

fn compute<'a>(
    expr: &Expression<'a>,
    ctx: &Ctx<'_, 'a>,
) -> (ExpressionData, ExprFacts) {
    let facts = collect(expr, ctx.semantics, ctx.reactivity);

    let eval_ctx = EvalCtx {
        scoping: ctx.scoping,
        semantics: ctx.semantics,
        reactivity: ctx.reactivity,
        snippets: ctx.snippets,
        bindings_init: &ctx.bindings_init,
        function_decls: &ctx.function_decls,
        dev: ctx.dev,
    };
    let evaluation = evaluator::evaluate(expr, &eval_ctx);

    let is_dynamic = derive::is_dynamic_template(
        &facts,
        ctx.scoping,
        ctx.reactivity,
        ctx.has_class_state_fields,
    );
    let blockers = derive::blockers(&facts, ctx.blockers);
    let kind = derive::kind(&facts, !blockers.is_empty(), is_dynamic, &evaluation);
    let has_context_member_root = facts.member_or_call_roots.iter().any(|&sym| {
        matches!(
            ctx.reactivity.binding_semantics(sym),
            BindingSemantics::MaybeReactive
                | BindingSemantics::Prop(PropBindingSemantics {
                    kind: PropBindingKind::Source { .. } | PropBindingKind::NonSource,
                    ..
                })
                | BindingSemantics::LegacyBindableProp(_)
                | BindingSemantics::Contextual(_)
        )
    });
    let data = ExpressionData {
        kind,
        evaluation,
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

fn update_aggregates(
    store: &mut ExpressionSemanticsStore,
    facts: &ExprFacts,
    ctx: &Ctx<'_, '_>,
) {
    for &sym in facts.member_or_call_roots.iter() {
        if matches!(
            ctx.reactivity.binding_semantics(sym),
            BindingSemantics::MaybeReactive
                | BindingSemantics::Prop(PropBindingSemantics {
                    kind: PropBindingKind::Source { .. } | PropBindingKind::NonSource,
                    ..
                })
                | BindingSemantics::LegacyBindableProp(_)
        ) {
            store.note_context(ContextSignal::IMPORT_OR_PROP_MEMBER);
        }
        if ctx.scoping.is_rest_prop(sym) {
            store.note_context(ContextSignal::REST_PROP_MEMBER);
        }
    }
    if facts.has_runtime_root {
        store.note_context(ContextSignal::IMPORT_OR_PROP_MEMBER);
    }
    if facts.has_store_member_mutation {
        store.note_context(ContextSignal::STORE_MUTATION);
    }
}

fn max_kind(a: &ExprKind, b: &ExprKind) -> ExprKind {
    fn rank(k: &ExprKind) -> u8 {
        match k {
            ExprKind::KnownLiteral => 0,
            ExprKind::SimpleRead { reactive: false } => 1,
            ExprKind::Computed { reactive: false } => 2,
            ExprKind::SimpleRead { reactive: true } => 3,
            ExprKind::Computed { reactive: true } => 4,
            ExprKind::Call => 5,
            ExprKind::Async { .. } => 6,
        }
    }
    if rank(a) >= rank(b) {
        a.clone()
    } else {
        b.clone()
    }
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
