use super::AttributeSemanticsStore;
use super::data::{
    AttributeSemantics, BoundaryPropSemantics, ComponentAttachEmit, ComponentAttachSemantics,
    ComponentBindKind, ComponentBindSemantics, ComponentBindTarget, ComponentPropConcatSemantics,
    ComponentPropExpressionSemantics, ComponentPropMemo, ComponentPropSemantics,
    ComponentSpreadEmit, ComponentSpreadSemantics, ConcatPartEmit, DefaultAttrKind,
    DocumentBindSemantics, ElementBindPropertyKind, ElementBindSemantics, EventEmit,
    EventSemantics, GroupBindValue, HandlerEmit, HtmlBindKind, HtmlConcatPart, HtmlConcatSemantics,
    SpecialValueKind, SpecialValueSemantics, StyleDirectivesSemantics,
    SvelteComponentThisSemantics, TemplateEffect, WindowBindSemantics,
};
use crate::expression_semantics::{
    Evaluation, ExpressionData, ExpressionSemantics, ExpressionSemanticsStore, LegacyWrap,
    Volatility,
};
use crate::reactivity_semantics::data::{
    BindingSemantics, ConstBindingSemantics, ContextualBindingSemantics, EachIndexStrategy,
    EachItemStrategy, PropBindingKind, PropBindingSemantics, ReactivitySemantics,
    ReferenceSemantics, StateKind,
};
use crate::scope::ComponentScoping;
use crate::scope::SymbolId;
use crate::types::data::{
    BlockerData, ContentEditableKind, DocumentBindKind, ElementFacts, ElementSizeKind,
    EventModifier, IgnoreData, ImageNaturalSizeKind, JsAst, MediaBindKind, NamespaceKind,
    ResizeObserverKind, SnippetData, WindowBindKind,
};
use crate::utils::events::{is_delegatable_event, is_passive_event, strip_capture_event};
use crate::utils::expression_calls_or_awaits;
use crate::value_evaluation::{
    ReadContext, ValueEvaluation, ValueEvaluator, symbol_read_is_static,
};
use oxc_ast::ast::{
    ArrayExpressionElement, ArrowFunctionExpression, AssignmentExpression, CallExpression,
    Expression, Function, ObjectPropertyKind, UpdateExpression,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::ScopeFlags;
use smallvec::SmallVec;
use svelte_ast::{
    AttachTag, Attribute, BindDirective, Component, ConcatPart, Element, ExpressionAttribute,
    FragmentId, Node, NodeId, OnDirectiveLegacy, StyleDirective, SvelteBody, SvelteBoundary,
    SvelteDocument, SvelteWindow,
};
use svelte_component_semantics::{ComponentSemantics, SymbolFlags};

#[derive(Default, Debug)]
pub struct BindingGroupTable {
    pub ids: rustc_hash::FxHashMap<NodeId, u32>,
    pub count: u32,
    keys: Vec<BindingGroupKey>,
}

#[derive(Debug, PartialEq, Eq)]
struct BindingGroupKey {
    keypath: String,
    references: SmallVec<[SymbolId; 2]>,
    parent_each_blocks: SmallVec<[NodeId; 4]>,
}

impl BindingGroupTable {
    fn assign(&mut self, attr_id: NodeId, key: BindingGroupKey) {
        if let Some(idx) = self.keys.iter().position(|k| *k == key) {
            self.ids.insert(attr_id, idx as u32);
            return;
        }
        let id = self.count;
        self.keys.push(key);
        self.count += 1;
        self.ids.insert(attr_id, id);
    }
}

pub fn build<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    scoping: &ComponentScoping<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    expressions: &ExpressionSemanticsStore,
    snippets: &SnippetData,
    value_evaluation: &ValueEvaluation,
    blockers: &BlockerData,
    ignore_data: &IgnoreData,
    element_facts: &ElementFacts,
    dev: bool,
    node_count: u32,
) -> (AttributeSemanticsStore, BindingGroupTable) {
    let attach_eval = ValueEvaluator::new(
        parsed,
        scoping,
        semantics,
        reactivity,
        snippets,
        ReadContext::Declaration,
        dev,
    );
    let ctx = Ctx {
        component,
        parsed,
        semantics,
        reactivity,
        expressions,
        snippets,
        value_evaluation,
        attach_eval,
        blockers,
        ignore_data,
        element_facts,
        dev,
    };
    let mut store = AttributeSemanticsStore::new(node_count);
    let mut state = WalkState::default();
    let mut groups = BindingGroupTable::default();
    walk_fragment(&ctx, &mut state, component.root, &mut store, &mut groups);
    finalize_element_group_ids(&mut store, &groups);
    (store, groups)
}

fn finalize_element_group_ids(store: &mut AttributeSemanticsStore, groups: &BindingGroupTable) {
    for (attr_id, group_id) in &groups.ids {
        if let AttributeSemantics::ElementBind(b) = store.get(*attr_id).clone() {
            let mut updated = b;
            updated.group_id = Some(*group_id);
            store.set(*attr_id, AttributeSemantics::ElementBind(updated));
        }
    }
}

fn derive_group_key(ctx: &Ctx<'_, '_>, state: &WalkState, d: &BindDirective) -> BindingGroupKey {
    let data = ctx.expression_data(d.id);
    let references = data.map(|d| d.references.clone()).unwrap_or_default();
    let parent_each_blocks = derive_parent_each_blocks(ctx, state, d);
    let keypath = derive_group_keypath(ctx, d);
    BindingGroupKey {
        keypath,
        references,
        parent_each_blocks,
    }
}

fn derive_group_keypath(ctx: &Ctx<'_, '_>, d: &BindDirective) -> String {
    let Some(expr) = ctx.parsed.expr(d.expression.id()) else {
        return String::new();
    };
    let mut tokens: Vec<String> = Vec::new();
    push_group_keypath(expr, &mut tokens);
    tokens.join(".")
}

fn push_group_keypath(expr: &Expression<'_>, out: &mut Vec<String>) {
    match expr.get_inner_expression() {
        Expression::Identifier(id) => out.push(id.name.to_string()),
        Expression::ThisExpression(_) => out.push("this".to_string()),
        Expression::StaticMemberExpression(m) => {
            push_group_keypath(&m.object, out);
            out.push(m.property.name.to_string());
        }
        Expression::ComputedMemberExpression(m) => {
            push_group_keypath(&m.object, out);
            match m.expression.get_inner_expression() {
                Expression::Identifier(id) => out.push(format!("[{}]", id.name)),
                Expression::StringLiteral(s) => out.push(format!("[\"{}\"]", s.value)),
                Expression::NumericLiteral(n) => out.push(format!("[{}]", n.value)),
                _ => out.push("[?]".to_string()),
            }
        }
        _ => {}
    }
}

struct Ctx<'a, 'p> {
    component: &'p Component,
    parsed: &'p JsAst<'a>,
    semantics: &'p ComponentSemantics<'a>,
    reactivity: &'p ReactivitySemantics,
    expressions: &'p ExpressionSemanticsStore,
    snippets: &'p SnippetData,
    value_evaluation: &'p ValueEvaluation,
    attach_eval: ValueEvaluator<'p, 'a>,
    blockers: &'p BlockerData,
    ignore_data: &'p IgnoreData,
    element_facts: &'p ElementFacts,
    dev: bool,
}

fn expression_is_plain_read(expr: &Expression<'_>) -> bool {
    matches!(
        expr.get_inner_expression(),
        Expression::Identifier(_)
            | Expression::StaticMemberExpression(_)
            | Expression::ComputedMemberExpression(_)
            | Expression::PrivateFieldExpression(_)
    )
}

fn reference_symbol_needs_wrap(ctx: &Ctx<'_, '_>, data: &ExpressionData, sym: SymbolId) -> bool {
    use crate::expression_semantics::{Evaluation, ValueClass};
    if ctx
        .semantics
        .symbol_flags(sym)
        .contains(SymbolFlags::Import)
    {
        return true;
    }
    match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::Const(ConstBindingSemantics::ConstTag {
            reactive,
            initial_is_function,
            ..
        }) => reactive && !initial_is_function,
        BindingSemantics::Contextual(ContextualBindingSemantics::EachIndex(
            EachIndexStrategy::Direct,
        )) => false,
        BindingSemantics::NonReactive => {
            if symbol_read_is_static(ctx.value_evaluation, ctx.semantics, sym) {
                return false;
            }
            if matches!(data.evaluation, Evaluation::Known(_)) {
                return false;
            }
            if ctx.snippets.snippet_by_symbol(sym).is_some() {
                return true;
            }
            !matches!(data.evaluation.class(), Some(ValueClass::Function))
        }
        BindingSemantics::OptimizedRune(_) => !matches!(data.evaluation, Evaluation::Known(_)),
        BindingSemantics::Prop(_)
        | BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Store(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Contextual(_)
        | BindingSemantics::MaybeReactive
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => true,
    }
}

fn references_need_wrap(ctx: &Ctx<'_, '_>, data: &ExpressionData) -> bool {
    match data.volatility {
        Volatility::Reactive if data.references.is_empty() => return true,
        Volatility::Static
        | Volatility::Reactive
        | Volatility::Heavy
        | Volatility::Asynchronous => {}
    }
    data.references
        .iter()
        .any(|&sym| reference_symbol_needs_wrap(ctx, data, sym))
}

fn handler_reads_through_contextual_getter(semantics: BindingSemantics) -> bool {
    match semantics {
        BindingSemantics::Contextual(contextual) => match contextual {
            ContextualBindingSemantics::LetDirective
            | ContextualBindingSemantics::SnippetParam(_)
            | ContextualBindingSemantics::AwaitValue
            | ContextualBindingSemantics::AwaitError => true,
            ContextualBindingSemantics::EachItem(strategy) => match strategy {
                EachItemStrategy::Accessor
                | EachItemStrategy::Signal
                | EachItemStrategy::IndexedLegacy => true,
                EachItemStrategy::Direct => false,
            },
            ContextualBindingSemantics::EachIndex(strategy) => match strategy {
                EachIndexStrategy::Signal => true,
                EachIndexStrategy::Direct => false,
            },
            ContextualBindingSemantics::LetDirectiveDirect
            | ContextualBindingSemantics::LetDirectiveCarrierMember { .. } => false,
        },
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
        | BindingSemantics::MaybeReactive
        | BindingSemantics::NonReactive
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::Unresolved => false,
    }
}

fn handler_reads_through_cell(semantics: BindingSemantics) -> bool {
    if let BindingSemantics::Contextual(_) = semantics {
        return handler_reads_through_contextual_getter(semantics);
    }
    semantics.is_reactive() || semantics.is_reactive_const_tag()
}

fn references_include_reactive_const_tag(ctx: &Ctx<'_, '_>, expr_id: NodeId) -> bool {
    let Some(data) = ctx.expression_data(expr_id) else {
        return false;
    };
    data.references.iter().any(|&sym| {
        ctx.reactivity
            .binding_semantics(sym)
            .is_reactive_const_tag()
    })
}

impl<'a, 'p> Ctx<'a, 'p> {
    fn expression_data(&self, id: NodeId) -> Option<&ExpressionData> {
        match self.expressions.get(id) {
            ExpressionSemantics::Expression(d) => Some(d),
            ExpressionSemantics::NonSpecial => None,
        }
    }
}

#[derive(Default)]
struct WalkState {
    each_stack: SmallVec<[NodeId; 4]>,
}

fn walk_fragment(
    ctx: &Ctx<'_, '_>,
    state: &mut WalkState,
    fragment: FragmentId,
    store: &mut AttributeSemanticsStore,
    groups: &mut BindingGroupTable,
) {
    for &id in &ctx.component.store.fragment(fragment).nodes {
        let node = ctx.component.store.get(id);
        match node {
            Node::SvelteWindow(w) => classify_window(ctx, w, store),
            Node::SvelteDocument(d) => classify_document(ctx, d, store),
            Node::SvelteBody(b) => classify_body(ctx, b, store),
            Node::Element(el) => {
                classify_element(ctx, state, el, store, groups);
                walk_fragment(ctx, state, el.fragment, store, groups);
            }
            Node::ComponentNode(cn) => {
                classify_component_attrs(
                    ctx,
                    state,
                    &cn.attributes,
                    store,
                    groups,
                    ComponentPropCarrier::Component,
                );
                walk_fragment(ctx, state, cn.fragment, store, groups);
                for slot in &cn.legacy_slots {
                    walk_fragment(ctx, state, slot.fragment, store, groups);
                }
            }
            Node::SvelteComponentLegacy(cn) => {
                classify_component_attrs(
                    ctx,
                    state,
                    &cn.attributes,
                    store,
                    groups,
                    ComponentPropCarrier::SvelteComponentLegacy,
                );
                walk_fragment(ctx, state, cn.fragment, store, groups);
                for slot in &cn.legacy_slots {
                    walk_fragment(ctx, state, slot.fragment, store, groups);
                }
            }
            Node::SvelteSelf(cn) => {
                classify_component_attrs(
                    ctx,
                    state,
                    &cn.attributes,
                    store,
                    groups,
                    ComponentPropCarrier::Component,
                );
                walk_fragment(ctx, state, cn.fragment, store, groups);
                for slot in &cn.legacy_slots {
                    walk_fragment(ctx, state, slot.fragment, store, groups);
                }
            }
            Node::SvelteElement(el) => {
                classify_svelte_element(ctx, state, el, store, groups);
                walk_fragment(ctx, state, el.fragment, store, groups);
            }
            Node::SvelteFragmentLegacy(sf) => {
                walk_fragment(ctx, state, sf.fragment, store, groups);
            }
            Node::SlotElementLegacy(el) => {
                classify_component_attrs(
                    ctx,
                    state,
                    &el.attributes,
                    store,
                    groups,
                    ComponentPropCarrier::SlotLegacy,
                );
                walk_fragment(ctx, state, el.fragment, store, groups);
            }
            Node::SvelteHead(head) => walk_fragment(ctx, state, head.fragment, store, groups),
            Node::SvelteBoundary(b) => {
                classify_boundary(ctx, b, store);
                walk_fragment(ctx, state, b.fragment, store, groups);
            }
            Node::IfBlock(b) => {
                walk_fragment(ctx, state, b.consequent, store, groups);
                if let Some(alt) = b.alternate {
                    walk_fragment(ctx, state, alt, store, groups);
                }
            }
            Node::EachBlock(b) => {
                state.each_stack.push(b.id);
                walk_fragment(ctx, state, b.body, store, groups);
                state.each_stack.pop();
                if let Some(fallback) = b.fallback {
                    walk_fragment(ctx, state, fallback, store, groups);
                }
            }
            Node::AwaitBlock(b) => {
                if let Some(p) = b.pending {
                    walk_fragment(ctx, state, p, store, groups);
                }
                if let Some(t) = b.then {
                    walk_fragment(ctx, state, t, store, groups);
                }
                if let Some(c) = b.catch {
                    walk_fragment(ctx, state, c, store, groups);
                }
            }
            Node::KeyBlock(b) => walk_fragment(ctx, state, b.fragment, store, groups),
            Node::SnippetBlock(b) => walk_fragment(ctx, state, b.body, store, groups),
            _ => {}
        }
    }
}

fn classify_svelte_element(
    ctx: &Ctx<'_, '_>,
    state: &WalkState,
    el: &svelte_ast::SvelteElement,
    store: &mut AttributeSemanticsStore,
    groups: &mut BindingGroupTable,
) {
    classify_element_attrs(ctx, state, el.id, &el.attributes, None, store, groups);
}

fn classify_element(
    ctx: &Ctx<'_, '_>,
    state: &WalkState,
    el: &Element,
    store: &mut AttributeSemanticsStore,
    groups: &mut BindingGroupTable,
) {
    classify_element_attrs(ctx, state, el.id, &el.attributes, Some(el), store, groups);
}

fn classify_element_attrs(
    ctx: &Ctx<'_, '_>,
    state: &WalkState,
    owner_id: NodeId,
    attrs: &[Attribute],
    el: Option<&Element>,
    store: &mut AttributeSemanticsStore,
    groups: &mut BindingGroupTable,
) {
    if attrs
        .iter()
        .any(|attr| matches!(attr, Attribute::StyleDirective(_)))
    {
        store.set(
            owner_id,
            AttributeSemantics::StyleDirectives(StyleDirectivesSemantics {
                volatility: style_set_volatility(ctx, attrs),
            }),
        );
    }

    for attr in attrs {
        match attr {
            Attribute::BindDirective(d) => {
                if let Some(property) = element_property(&d.name) {
                    let is_this = matches!(property, ElementBindPropertyKind::This);
                    let kind = if is_this {
                        bind_kind_member_aware(ctx, d)
                    } else {
                        bind_kind(ctx, d)
                    };
                    let blockers = derive_blockers(ctx, d);
                    let (parent_each_blocks, group_value) =
                        if matches!(property, ElementBindPropertyKind::Group) {
                            (
                                derive_parent_each_blocks(ctx, state, d),
                                el.and_then(find_group_bind_value),
                            )
                        } else if is_this {
                            (derive_parent_each_blocks(ctx, state, d), None)
                        } else {
                            (SmallVec::new(), None)
                        };
                    let each_context_vars = if is_this {
                        derive_each_context_vars(ctx, d)
                    } else {
                        SmallVec::new()
                    };
                    if matches!(property, ElementBindPropertyKind::Group) {
                        groups.assign(d.id, derive_group_key(ctx, state, d));
                    }
                    store.set(
                        d.id,
                        AttributeSemantics::ElementBind(ElementBindSemantics {
                            property,
                            kind,
                            blockers,
                            parent_each_blocks,
                            each_context_vars,
                            group_value,
                            group_id: None,
                        }),
                    );
                }
            }
            Attribute::ExpressionAttribute(ea) => {
                if ea.event_name.is_some() {
                    classify_html_event(ctx, ea, store);
                } else if ea.name == "autofocus" {
                    store.set(ea.id, AttributeSemantics::Autofocus);
                } else if let Some(kind) = special_value_kind_for(el, &ea.name) {
                    store.set(
                        ea.id,
                        AttributeSemantics::SpecialValueAttr(SpecialValueSemantics {
                            kind,
                            defined: special_value_defined(ctx, ea.id),
                            volatile: special_value_volatile(ctx, ea.id),
                            concat: None,
                        }),
                    );
                }
            }
            Attribute::ConcatenationAttribute(ca) => {
                let semantics = derive_html_concat_semantics(ctx, ca);
                if let Some(kind) = special_value_kind_for(el, &ca.name) {
                    store.set(
                        ca.id,
                        AttributeSemantics::SpecialValueAttr(SpecialValueSemantics {
                            kind,
                            defined: concat_special_value_defined(ctx, ca),
                            volatile: special_value_volatile(ctx, ca.id),
                            concat: Some(semantics),
                        }),
                    );
                } else {
                    store.set(ca.id, AttributeSemantics::HtmlConcat(semantics));
                }
            }
            Attribute::OnDirectiveLegacy(d) => {
                classify_html_on_directive_legacy(ctx, d, store);
            }
            Attribute::BooleanAttribute(a) if a.name == "autofocus" => {
                store.set(a.id, AttributeSemantics::Autofocus);
            }
            Attribute::StringAttribute(a) if a.name == "autofocus" => {
                store.set(a.id, AttributeSemantics::Autofocus);
            }
            Attribute::StringAttribute(a)
                if a.name == "is"
                    && matches!(
                        ctx.element_facts.namespace(owner_id),
                        Some(NamespaceKind::Html)
                    ) =>
            {
                store.set(a.id, AttributeSemantics::StaticAttr);
            }
            Attribute::BooleanAttribute(a)
                if matches!(a.name.as_str(), "muted" | "defaultValue" | "defaultChecked") =>
            {
                store.set(
                    a.id,
                    AttributeSemantics::CannotBeStatic(default_attr_kind(ctx, el, &a.name)),
                );
            }
            Attribute::StringAttribute(a)
                if matches!(a.name.as_str(), "muted" | "defaultValue" | "defaultChecked") =>
            {
                store.set(
                    a.id,
                    AttributeSemantics::CannotBeStatic(default_attr_kind(ctx, el, &a.name)),
                );
            }
            _ => {}
        }
    }
}

fn style_set_volatility(ctx: &Ctx<'_, '_>, attrs: &[Attribute]) -> Volatility {
    let mut volatility = Volatility::Static;
    for attr in attrs {
        let attr_volatility = match attr {
            Attribute::StyleDirective(d) => style_directive_volatility(ctx, d),
            Attribute::ExpressionAttribute(ea) if ea.name == "style" => {
                style_value_volatility(ctx, ea.id)
            }
            Attribute::ConcatenationAttribute(ca) if ca.name == "style" => {
                style_value_volatility(ctx, ca.id)
            }
            _ => Volatility::Static,
        };
        volatility = volatility.max(attr_volatility);
    }
    volatility
}

fn style_value_volatility(ctx: &Ctx<'_, '_>, attr_id: NodeId) -> Volatility {
    ctx.expression_data(attr_id)
        .map(|data| data.volatility)
        .unwrap_or(Volatility::Static)
}

fn style_directive_volatility(ctx: &Ctx<'_, '_>, directive: &StyleDirective) -> Volatility {
    let base = style_value_volatility(ctx, directive.id);
    if !directive.shorthand {
        return base;
    }
    let references_reactive_decl = ctx.expression_data(directive.id).is_some_and(|data| {
        data.references.iter().any(|&sym| {
            let binding = ctx.reactivity.binding_semantics(sym);
            binding.is_reactive() || binding.is_optimized_rune()
        })
    });
    if references_reactive_decl {
        base.max(Volatility::Reactive)
    } else {
        base
    }
}

fn classify_html_on_directive_legacy(
    ctx: &Ctx<'_, '_>,
    d: &OnDirectiveLegacy,
    store: &mut AttributeSemanticsStore,
) {
    let modifiers = parse_event_modifiers(&d.modifiers);
    let handler = match &d.expression {
        Some(expr_ref) => derive_handler_emit(ctx, expr_ref.id()),
        None => HandlerEmit::Direct,
    };
    store.set(
        d.id,
        AttributeSemantics::Event(EventSemantics {
            modifiers,
            emit: EventEmit::HtmlDirect {
                capture: modifiers.contains(EventModifier::CAPTURE),
                passive: if modifiers.contains(EventModifier::PASSIVE) {
                    Some(true)
                } else if modifiers.contains(EventModifier::NONPASSIVE) {
                    Some(false)
                } else {
                    None
                },
                handler,
            },
        }),
    );
}

fn derive_blockers(ctx: &Ctx<'_, '_>, d: &BindDirective) -> SmallVec<[u32; 2]> {
    let mut result: SmallVec<[u32; 2]> = SmallVec::new();
    if !ctx.blockers.has_async() {
        return result;
    }
    let Some(data) = ctx.expression_data(d.id) else {
        return result;
    };
    for &sym in &data.references {
        if let Some(idx) = ctx.blockers.symbol_blocker(sym)
            && !result.contains(&idx)
        {
            result.push(idx);
        }
    }
    result.sort_unstable();
    result
}

fn derive_parent_each_blocks(
    ctx: &Ctx<'_, '_>,
    state: &WalkState,
    d: &BindDirective,
) -> SmallVec<[NodeId; 4]> {
    let mut result: SmallVec<[NodeId; 4]> = SmallVec::new();
    if state.each_stack.is_empty() {
        return result;
    }
    let Some(data) = ctx.expression_data(d.id) else {
        return result;
    };
    let mut ids: SmallVec<[SymbolId; 8]> = data.references.iter().copied().collect();
    for &each_id in state.each_stack.iter().rev() {
        let owns_any = ids
            .iter()
            .any(|&sym| ctx.reactivity.contextual_owner(sym) == Some(each_id));
        if !owns_any {
            continue;
        }
        if !result.contains(&each_id) {
            result.push(each_id);
        }
        for sym in each_collection_symbols(ctx, each_id) {
            if !ids.contains(&sym) {
                ids.push(sym);
            }
        }
    }
    result
}

fn each_collection_symbols(ctx: &Ctx<'_, '_>, each_id: NodeId) -> SmallVec<[SymbolId; 4]> {
    let Some(data) = ctx.expression_data(each_id) else {
        return SmallVec::new();
    };
    data.references.iter().copied().collect()
}

fn default_attr_kind(ctx: &Ctx<'_, '_>, el: Option<&Element>, name: &str) -> DefaultAttrKind {
    let Some(el) = el else {
        return DefaultAttrKind::PlainProperty;
    };
    let reconcile = match name {
        "defaultValue" => {
            el.attributes
                .iter()
                .any(|a| matches!(a, Attribute::StringAttribute(s) if s.name == "value"))
                || (el.name == "textarea" && !ctx.component.fragment_nodes(el.fragment).is_empty())
        }
        "defaultChecked" => el
            .attributes
            .iter()
            .any(|a| matches!(a, Attribute::BooleanAttribute(b) if b.name == "checked")),
        _ => false,
    };
    if reconcile {
        DefaultAttrKind::ReconcileWithValue
    } else {
        DefaultAttrKind::PlainProperty
    }
}

fn special_value_kind_for(el: Option<&Element>, attr_name: &str) -> Option<SpecialValueKind> {
    if attr_name != "value" {
        return None;
    }
    let el = el?;
    match el.name.as_str() {
        "select" => Some(SpecialValueKind::Select),
        "option" => Some(SpecialValueKind::Option),
        "input" => {
            let mut has_group = false;
            let mut has_checked = false;
            for attr in &el.attributes {
                if let Attribute::BindDirective(d) = attr {
                    match d.name.as_str() {
                        "group" => has_group = true,
                        "checked" => has_checked = true,
                        _ => {}
                    }
                }
            }
            if has_group {
                Some(SpecialValueKind::InputBindGroup)
            } else if has_checked {
                Some(SpecialValueKind::InputBindChecked)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn find_group_bind_value(el: &Element) -> Option<GroupBindValue> {
    el.attributes.iter().find_map(|attr| match attr {
        Attribute::ExpressionAttribute(a) if a.name == "value" => Some(GroupBindValue {
            expression: a.expression.id(),
            data: a.id,
        }),
        Attribute::ConcatenationAttribute(a) if a.name == "value" => match a.parts.as_slice() {
            [ConcatPart::Dynamic { id, expr }] => Some(GroupBindValue {
                expression: expr.id(),
                data: *id,
            }),
            _ => None,
        },
        _ => None,
    })
}

fn bind_kind_member_aware(ctx: &Ctx<'_, '_>, d: &BindDirective) -> HtmlBindKind {
    let Some(expr) = ctx.parsed.expr(d.expression.id()) else {
        return HtmlBindKind::Plain;
    };
    let expr = expr.get_inner_expression();
    if matches!(expr, Expression::Identifier(_)) {
        return bind_kind(ctx, d);
    }
    let mut current = expr;
    let ident = loop {
        match current {
            Expression::StaticMemberExpression(m) => current = m.object.get_inner_expression(),
            Expression::ComputedMemberExpression(m) => current = m.object.get_inner_expression(),
            Expression::Identifier(id) => break id,
            _ => return HtmlBindKind::Plain,
        }
    };
    classify_identifier_kind(ctx, ident)
}

fn classify_identifier_kind(
    ctx: &Ctx<'_, '_>,
    ident: &oxc_ast::ast::IdentifierReference,
) -> HtmlBindKind {
    use crate::reactivity_semantics::data::PropReferenceSemantics;
    let Some(ref_id) = ident.reference_id.get() else {
        return HtmlBindKind::Plain;
    };
    match ctx.reactivity.reference_semantics(ref_id) {
        ReferenceSemantics::StoreRead {
            symbol: store_symbol,
        }
        | ReferenceSemantics::StoreWrite {
            symbol: store_symbol,
        }
        | ReferenceSemantics::StoreUpdate {
            symbol: store_symbol,
        } => {
            let base_symbol = match ctx.reactivity.binding_semantics(store_symbol) {
                BindingSemantics::Store(store) => store.base_symbol,
                _ => store_symbol,
            };
            HtmlBindKind::StoreSubscribed { base_symbol }
        }
        ReferenceSemantics::PropRead(PropReferenceSemantics::Source { bindable: true, .. })
        | ReferenceSemantics::PropMutation { bindable: true, .. } => HtmlBindKind::BindableProp,
        ReferenceSemantics::SignalRead { .. }
        | ReferenceSemantics::SignalWrite { .. }
        | ReferenceSemantics::SignalUpdate { .. }
        | ReferenceSemantics::DerivedUpdate => HtmlBindKind::Rune,
        ReferenceSemantics::LegacyStateRead { .. }
        | ReferenceSemantics::LegacyStateWrite
        | ReferenceSemantics::LegacyStateUpdate { .. }
        | ReferenceSemantics::LegacyStateMemberMutationRoot { .. } => HtmlBindKind::LegacyState,
        ReferenceSemantics::NonReactive
        | ReferenceSemantics::Proxy
        | ReferenceSemantics::DerivedWrite
        | ReferenceSemantics::PropRead(_)
        | ReferenceSemantics::PropMutation { .. }
        | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
        | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
        | ReferenceSemantics::ConstAliasRead { .. }
        | ReferenceSemantics::ContextualRead(_)
        | ReferenceSemantics::CarrierMemberRead(_)
        | ReferenceSemantics::RestPropMemberRewrite
        | ReferenceSemantics::LegacyPropsIdentifierRead
        | ReferenceSemantics::LegacyRestPropsIdentifierRead
        | ReferenceSemantics::LegacySlotsIdentifierRead
        | ReferenceSemantics::LegacyStateSubscribedRead { .. }
        | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
        | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
        | ReferenceSemantics::LegacyReactiveImportRead
        | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
        | ReferenceSemantics::ImportSubscribedRead { .. }
        | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
        | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
        | ReferenceSemantics::EachItemIndexedLegacy { .. }
        | ReferenceSemantics::IllegalWrite
        | ReferenceSemantics::Unresolved => HtmlBindKind::Plain,
    }
}

fn each_item_write_root_symbol(
    ctx: &Ctx<'_, '_>,
    ident: &oxc_ast::ast::IdentifierReference,
) -> Option<SymbolId> {
    let ref_id = ident.reference_id.get()?;
    let sym = ctx.semantics.get_reference(ref_id).symbol_id()?;
    match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::Contextual(ContextualBindingSemantics::EachItem(
            EachItemStrategy::Accessor | EachItemStrategy::Signal | EachItemStrategy::Direct,
        )) => Some(sym),
        _ => None,
    }
}

fn bind_kind(ctx: &Ctx<'_, '_>, d: &BindDirective) -> HtmlBindKind {
    let Some(expr) = ctx.parsed.expr(d.expression.id()) else {
        return HtmlBindKind::Plain;
    };
    let ident = match expr.get_inner_expression() {
        Expression::Identifier(ident) => ident,
        _ => return HtmlBindKind::Plain,
    };
    if !ctx.reactivity.uses_runes()
        && let Some(symbol) = each_item_write_root_symbol(ctx, ident)
    {
        return HtmlBindKind::EachItemWriteLegacy { symbol };
    }
    classify_identifier_kind(ctx, ident)
}

fn classify_html_event(
    ctx: &Ctx<'_, '_>,
    ea: &ExpressionAttribute,
    store: &mut AttributeSemanticsStore,
) {
    let raw = ea
        .event_name
        .as_deref()
        .expect("classify_html_event requires event_name");
    let (name, capture) = match strip_capture_event(raw) {
        Some(base) => (base, true),
        None => (raw, false),
    };
    let passive = is_passive_event(name);
    let handler = derive_handler_emit(ctx, ea.expression.id());
    let emit = if !capture && is_delegatable_event(name) {
        EventEmit::HtmlDelegated { handler }
    } else {
        EventEmit::HtmlDirect {
            capture,
            passive: if passive { Some(true) } else { None },
            handler,
        }
    };
    store.set(
        ea.id,
        AttributeSemantics::Event(EventSemantics {
            modifiers: EventModifier::empty(),
            emit,
        }),
    );
}

fn derive_handler_emit(
    ctx: &Ctx<'_, '_>,
    expr_id: svelte_component_semantics::OxcNodeId,
) -> HandlerEmit {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        return HandlerEmit::WrappedInert;
    };
    let direct = match expr.get_inner_expression() {
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => true,
        Expression::Identifier(ident) => {
            let symbol = ident
                .reference_id
                .get()
                .and_then(|ref_id| ctx.semantics.get_reference(ref_id).symbol_id());
            let is_function = symbol.is_some_and(|sym| {
                ctx.semantics
                    .symbol_flags(sym)
                    .contains(SymbolFlags::Function)
            });
            let is_store_subscription = ident.reference_id.get().is_some_and(|ref_id| {
                ctx.reactivity
                    .reference_semantics(ref_id)
                    .is_store_subscription()
            });
            let needs_wrap = is_store_subscription
                || symbol.is_some_and(|sym| {
                    handler_reads_through_cell(ctx.reactivity.binding_semantics(sym))
                });
            is_function || (!ctx.dev && !needs_wrap)
        }
        _ => false,
    };
    if direct {
        return HandlerEmit::Direct;
    }
    let mut probe = HandlerKindProbe::default();
    probe.visit_expression(expr);
    if probe.has_call {
        HandlerEmit::WrappedMemoized
    } else if probe.has_side_effects {
        HandlerEmit::WrappedSideEffects
    } else {
        HandlerEmit::WrappedInert
    }
}

#[derive(Default)]
struct HandlerKindProbe {
    has_call: bool,
    has_side_effects: bool,
    fn_depth: u32,
}

impl<'a> Visit<'a> for HandlerKindProbe {
    fn visit_call_expression(&mut self, expr: &CallExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_call = true;
            self.has_side_effects = true;
        }
        walk::walk_call_expression(self, expr);
    }

    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_side_effects = true;
        }
        walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &UpdateExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_side_effects = true;
        }
        walk::walk_update_expression(self, expr);
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        self.fn_depth += 1;
        walk::walk_arrow_function_expression(self, arrow);
        self.fn_depth -= 1;
    }

    fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
        self.fn_depth += 1;
        walk::walk_function(self, func, flags);
        self.fn_depth -= 1;
    }
}

fn element_property(name: &str) -> Option<ElementBindPropertyKind> {
    use ElementBindPropertyKind as E;
    Some(match name {
        "value" => E::Value,
        "checked" => E::Checked,
        "group" => E::Group,
        "files" => E::Files,
        "indeterminate" => E::Indeterminate,
        "open" => E::Open,
        "this" => E::This,
        "innerHTML" => E::ContentEditable(ContentEditableKind::InnerHtml),
        "innerText" => E::ContentEditable(ContentEditableKind::InnerText),
        "textContent" => E::ContentEditable(ContentEditableKind::TextContent),
        "clientWidth" => E::ElementSize(ElementSizeKind::ClientWidth),
        "clientHeight" => E::ElementSize(ElementSizeKind::ClientHeight),
        "offsetWidth" => E::ElementSize(ElementSizeKind::OffsetWidth),
        "offsetHeight" => E::ElementSize(ElementSizeKind::OffsetHeight),
        "contentRect" => E::ResizeObserver(ResizeObserverKind::ContentRect),
        "contentBoxSize" => E::ResizeObserver(ResizeObserverKind::ContentBoxSize),
        "borderBoxSize" => E::ResizeObserver(ResizeObserverKind::BorderBoxSize),
        "devicePixelContentBoxSize" => {
            E::ResizeObserver(ResizeObserverKind::DevicePixelContentBoxSize)
        }
        "currentTime" => E::Media(MediaBindKind::CurrentTime),
        "playbackRate" => E::Media(MediaBindKind::PlaybackRate),
        "paused" => E::Media(MediaBindKind::Paused),
        "volume" => E::Media(MediaBindKind::Volume),
        "muted" => E::Media(MediaBindKind::Muted),
        "buffered" => E::Media(MediaBindKind::Buffered),
        "seekable" => E::Media(MediaBindKind::Seekable),
        "seeking" => E::Media(MediaBindKind::Seeking),
        "ended" => E::Media(MediaBindKind::Ended),
        "readyState" => E::Media(MediaBindKind::ReadyState),
        "played" => E::Media(MediaBindKind::Played),
        "duration" => E::Media(MediaBindKind::Duration),
        "videoWidth" => E::Media(MediaBindKind::VideoWidth),
        "videoHeight" => E::Media(MediaBindKind::VideoHeight),
        "naturalWidth" => E::ImageNaturalSize(ImageNaturalSizeKind::NaturalWidth),
        "naturalHeight" => E::ImageNaturalSize(ImageNaturalSizeKind::NaturalHeight),
        "focused" => E::Focused,
        _ => return None,
    })
}

#[derive(Copy, Clone)]
enum ComponentPropCarrier {
    Component,
    SvelteComponentLegacy,
    SlotLegacy,
}

fn classify_component_attrs(
    ctx: &Ctx<'_, '_>,
    state: &WalkState,
    attrs: &[Attribute],
    store: &mut AttributeSemanticsStore,
    groups: &mut BindingGroupTable,
    carrier: ComponentPropCarrier,
) {
    for attr in attrs {
        match attr {
            Attribute::BindDirective(d) => {
                let kind = derive_component_bind_kind(ctx, d);
                let each_context_vars =
                    if d.name == "this" || matches!(kind, ComponentBindKind::Expression) {
                        derive_each_context_vars(ctx, d)
                    } else {
                        SmallVec::new()
                    };
                if d.name == "group" {
                    groups.assign(d.id, derive_group_key(ctx, state, d));
                }
                store.set(
                    d.id,
                    AttributeSemantics::ComponentBind(ComponentBindSemantics {
                        kind,
                        each_context_vars,
                    }),
                );
            }
            Attribute::ExpressionAttribute(ea) => {
                if matches!(carrier, ComponentPropCarrier::SvelteComponentLegacy)
                    && ea.name == "this"
                {
                    store.set(
                        ea.id,
                        AttributeSemantics::SvelteComponentThis(SvelteComponentThisSemantics {
                            expr_id: ea.expression.id(),
                        }),
                    );
                    continue;
                }
                let memo = derive_component_prop_memo_for_expression(ctx, ea, carrier);
                let shorthand = ea.shorthand && !references_include_reactive_const_tag(ctx, ea.id);
                store.set(
                    ea.id,
                    AttributeSemantics::ComponentProp(ComponentPropSemantics::Expression(
                        ComponentPropExpressionSemantics { memo, shorthand },
                    )),
                );
            }
            Attribute::ConcatenationAttribute(ca) => {
                if matches!(carrier, ComponentPropCarrier::SvelteComponentLegacy)
                    && ca.name == "this"
                    && let [ConcatPart::Dynamic { expr, .. }] = ca.parts.as_slice()
                {
                    store.set(
                        ca.id,
                        AttributeSemantics::SvelteComponentThis(SvelteComponentThisSemantics {
                            expr_id: expr.id(),
                        }),
                    );
                    continue;
                }
                let (memo, plan) = derive_component_concat_semantics(ctx, ca, carrier);
                store.set(
                    ca.id,
                    AttributeSemantics::ComponentProp(ComponentPropSemantics::Concat(
                        ComponentPropConcatSemantics { memo, plan },
                    )),
                );
            }
            Attribute::SpreadAttribute(sa) => {
                let emit = derive_component_spread_emit(ctx, sa.id);
                store.set(
                    sa.id,
                    AttributeSemantics::ComponentSpread(ComponentSpreadSemantics { emit }),
                );
            }
            Attribute::AttachTag(at) => {
                let emit = derive_component_attach_emit(ctx, at);
                store.set(
                    at.id,
                    AttributeSemantics::ComponentAttach(ComponentAttachSemantics { emit }),
                );
            }
            Attribute::OnDirectiveLegacy(d) => {
                let modifiers = parse_event_modifiers(&d.modifiers);
                let handler = match &d.expression {
                    Some(expr_ref) => derive_handler_emit(ctx, expr_ref.id()),
                    None => HandlerEmit::Direct,
                };
                store.set(
                    d.id,
                    AttributeSemantics::Event(EventSemantics {
                        modifiers,
                        emit: EventEmit::Component { handler },
                    }),
                );
            }
            _ => {}
        }
    }
}

fn parse_event_modifiers(modifiers: &[String]) -> EventModifier {
    modifiers
        .iter()
        .fold(EventModifier::empty(), |mut flags, modifier| {
            flags |= match modifier.as_str() {
                "once" => EventModifier::ONCE,
                "capture" => EventModifier::CAPTURE,
                "preventDefault" => EventModifier::PREVENT_DEFAULT,
                "stopPropagation" => EventModifier::STOP_PROPAGATION,
                "stopImmediatePropagation" => EventModifier::STOP_IMMEDIATE_PROPAGATION,
                "passive" => EventModifier::PASSIVE,
                "nonpassive" => EventModifier::NONPASSIVE,
                "trusted" => EventModifier::TRUSTED,
                "self" => EventModifier::SELF,
                "global" => EventModifier::GLOBAL,
                _ => EventModifier::empty(),
            };
            flags
        })
}

fn special_value_defined(ctx: &Ctx<'_, '_>, node_id: NodeId) -> bool {
    ctx.expression_data(node_id)
        .map(|data| data.evaluation.is_defined())
        .unwrap_or(false)
}

fn special_value_volatile(ctx: &Ctx<'_, '_>, node_id: NodeId) -> bool {
    ctx.expression_data(node_id)
        .map(|data| data.volatility.is_volatile())
        .unwrap_or(false)
}

fn concat_special_value_defined(
    ctx: &Ctx<'_, '_>,
    ca: &svelte_ast::ConcatenationAttribute,
) -> bool {
    let [svelte_ast::ConcatPart::Dynamic { id, .. }] = ca.parts.as_slice() else {
        return true;
    };
    special_value_defined(ctx, *id)
}

fn derive_html_concat_semantics(
    ctx: &Ctx<'_, '_>,
    ca: &svelte_ast::ConcatenationAttribute,
) -> HtmlConcatSemantics {
    use crate::expression_semantics::Evaluation;

    let mut parts: SmallVec<[HtmlConcatPart; 4]> = SmallVec::with_capacity(ca.parts.len());
    let mut sync_index: u8 = 0;
    let mut async_index: u8 = 0;
    let mut has_async = false;
    let mut has_dynamic = false;

    let single = matches!(
        ca.parts.as_slice(),
        [svelte_ast::ConcatPart::Dynamic { .. }]
    );

    for part in &ca.parts {
        match part {
            svelte_ast::ConcatPart::Static(s) => {
                parts.push(HtmlConcatPart::StaticText(s.as_str().into()));
            }
            svelte_ast::ConcatPart::Dynamic { id, .. } => {
                let Some(data) = ctx.expression_data(*id) else {
                    parts.push(HtmlConcatPart::Inline {
                        part_id: *id,
                        defined: false,
                        wrap: LegacyWrap::None,
                    });
                    has_dynamic = true;
                    continue;
                };
                if !single && let Evaluation::Known(_) = &data.evaluation {
                    let text = data.evaluation.known_str().unwrap_or_default();
                    parts.push(HtmlConcatPart::StaticText(text.into()));
                    continue;
                }
                let defined = matches!(data.evaluation, Evaluation::Defined { .. });
                let wrap = data.legacy_wrap;
                if !data.blockers.is_empty() {
                    let index = async_index;
                    async_index += 1;
                    has_async = true;
                    parts.push(HtmlConcatPart::AsyncMemoSlot {
                        index,
                        part_id: *id,
                        defined,
                        wrap,
                    });
                    continue;
                }
                match data.volatility {
                    Volatility::Asynchronous => {
                        let index = async_index;
                        async_index += 1;
                        has_async = true;
                        parts.push(HtmlConcatPart::AsyncMemoSlot {
                            index,
                            part_id: *id,
                            defined,
                            wrap,
                        });
                    }
                    Volatility::Heavy => {
                        let index = sync_index;
                        sync_index += 1;
                        has_dynamic = true;
                        parts.push(HtmlConcatPart::SyncMemoSlot {
                            index,
                            part_id: *id,
                            defined,
                            wrap,
                        });
                    }
                    Volatility::Reactive => {
                        has_dynamic = true;
                        parts.push(HtmlConcatPart::Inline {
                            part_id: *id,
                            defined,
                            wrap,
                        });
                    }
                    Volatility::Static => {
                        if references_need_wrap(ctx, data) {
                            has_dynamic = true;
                        }
                        parts.push(HtmlConcatPart::Inline {
                            part_id: *id,
                            defined,
                            wrap,
                        });
                    }
                }
            }
        }
    }

    let effect = if has_async {
        TemplateEffect::Async
    } else if has_dynamic {
        TemplateEffect::Sync
    } else {
        TemplateEffect::None
    };

    HtmlConcatSemantics { effect, parts }
}

fn derive_component_concat_semantics(
    ctx: &Ctx<'_, '_>,
    ca: &svelte_ast::ConcatenationAttribute,
    carrier: ComponentPropCarrier,
) -> (ComponentPropMemo, SmallVec<[ConcatPartEmit; 4]>) {
    if let [ConcatPart::Dynamic { id, expr }] = ca.parts.as_slice()
        && let Some(expr_raw) = ctx.parsed.expr(expr.id())
        && let Some(data) = ctx.expression_data(*id)
    {
        let memo = derive_component_prop_memo_core(ctx, expr_raw, data, carrier);
        let emit = match memo {
            ComponentPropMemo::Derived => ConcatPartEmit::HoistDerived,
            ComponentPropMemo::Getter | ComponentPropMemo::Inline => ConcatPartEmit::Inline,
        };
        return (memo, SmallVec::from_iter([emit]));
    }
    let forces_wrap = ca.parts.iter().any(|part| {
        let svelte_ast::ConcatPart::Dynamic { id, expr } = part else {
            return false;
        };
        let Some(data) = ctx.expression_data(*id) else {
            return false;
        };
        match data.volatility {
            Volatility::Heavy | Volatility::Asynchronous => true,
            Volatility::Static | Volatility::Reactive => {
                if !data.blockers.is_empty() {
                    return true;
                }
                let Some(e) = ctx.parsed.expr(expr.id()) else {
                    return false;
                };
                !matches!(data.evaluation, Evaluation::Known(_))
                    && !expression_calls_or_awaits(e)
                    && !expression_is_plain_read(e)
            }
        }
    });

    let plan: SmallVec<[ConcatPartEmit; 4]> = ca
        .parts
        .iter()
        .map(|part| {
            let svelte_ast::ConcatPart::Dynamic { id, .. } = part else {
                return ConcatPartEmit::Static;
            };
            let Some(data) = ctx.expression_data(*id) else {
                return ConcatPartEmit::Inline;
            };
            match data.evaluation {
                Evaluation::Known(_) => ConcatPartEmit::Static,
                Evaluation::Defined { .. } | Evaluation::MaybeNullish { .. } => {
                    match data.volatility {
                        Volatility::Heavy | Volatility::Asynchronous => {
                            ConcatPartEmit::HoistDerived
                        }
                        Volatility::Static | Volatility::Reactive => {
                            if !data.blockers.is_empty() || forces_wrap {
                                ConcatPartEmit::HoistDerived
                            } else {
                                ConcatPartEmit::Inline
                            }
                        }
                    }
                }
            }
        })
        .collect();

    let memo = component_prop_memo(&plan);
    (memo, plan)
}

fn component_prop_memo(plan: &[ConcatPartEmit]) -> ComponentPropMemo {
    let has_hoist = plan.iter().any(|emit| match emit {
        ConcatPartEmit::HoistDerived => true,
        ConcatPartEmit::Inline | ConcatPartEmit::Static => false,
    });
    if has_hoist {
        return ComponentPropMemo::Derived;
    }
    let has_inline_dynamic = plan.iter().any(|emit| match emit {
        ConcatPartEmit::Inline => true,
        ConcatPartEmit::HoistDerived | ConcatPartEmit::Static => false,
    });
    if has_inline_dynamic {
        ComponentPropMemo::Getter
    } else {
        ComponentPropMemo::Inline
    }
}

fn derive_component_attach_emit(ctx: &Ctx<'_, '_>, at: &AttachTag) -> ComponentAttachEmit {
    use crate::expression_semantics::ValueClass;
    let Some(data) = ctx.expression_data(at.id) else {
        return ComponentAttachEmit::Inline;
    };
    if !references_need_wrap(ctx, data) {
        return ComponentAttachEmit::Inline;
    }
    let provably_function = ctx.parsed.expr(at.expression.id()).is_some_and(|expr| {
        matches!(
            ctx.attach_eval.evaluate(expr).class(),
            Some(ValueClass::Function)
        )
    });
    if provably_function {
        ComponentAttachEmit::Wrapped
    } else {
        ComponentAttachEmit::WrappedFallback
    }
}

fn derive_component_spread_emit(ctx: &Ctx<'_, '_>, attr_id: NodeId) -> ComponentSpreadEmit {
    let Some(data) = ctx.expression_data(attr_id) else {
        return ComponentSpreadEmit::Inline;
    };
    match data.volatility {
        Volatility::Heavy | Volatility::Asynchronous => return ComponentSpreadEmit::MemoThunk,
        Volatility::Static | Volatility::Reactive => {}
    }
    if !data.blockers.is_empty() {
        return ComponentSpreadEmit::MemoThunk;
    }
    if references_need_wrap(ctx, data) {
        ComponentSpreadEmit::Thunk
    } else {
        ComponentSpreadEmit::Inline
    }
}

fn derive_component_prop_memo_for_expression(
    ctx: &Ctx<'_, '_>,
    ea: &ExpressionAttribute,
    carrier: ComponentPropCarrier,
) -> ComponentPropMemo {
    let Some(expr_raw) = ctx.parsed.expr(ea.expression.id()) else {
        return ComponentPropMemo::Inline;
    };
    let Some(data) = ctx.expression_data(ea.id) else {
        return ComponentPropMemo::Inline;
    };
    derive_component_prop_memo_core(ctx, expr_raw, data, carrier)
}

fn derive_component_prop_memo_core(
    ctx: &Ctx<'_, '_>,
    expr_raw: &Expression<'_>,
    data: &ExpressionData,
    carrier: ComponentPropCarrier,
) -> ComponentPropMemo {
    let expr = expr_raw.get_inner_expression();
    if matches!(
        expr,
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
    ) {
        return ComponentPropMemo::Inline;
    }
    if matches!(
        expr,
        Expression::ObjectExpression(_) | Expression::ArrayExpression(_)
    ) && object_array_literal_is_inline(ctx, expr)
    {
        return ComponentPropMemo::Inline;
    }
    let simple_shape = matches!(
        expr,
        Expression::Identifier(_)
            | Expression::StaticMemberExpression(_)
            | Expression::ComputedMemberExpression(_)
    );
    let needs_wrap = references_need_wrap(ctx, data);
    if matches!(
        data.volatility,
        Volatility::Heavy | Volatility::Asynchronous
    ) || !data.blockers.is_empty()
    {
        ComponentPropMemo::Derived
    } else if needs_wrap && simple_shape {
        ComponentPropMemo::Getter
    } else if needs_wrap {
        match carrier {
            ComponentPropCarrier::Component | ComponentPropCarrier::SvelteComponentLegacy => {
                ComponentPropMemo::Derived
            }
            ComponentPropCarrier::SlotLegacy => ComponentPropMemo::Getter,
        }
    } else {
        ComponentPropMemo::Inline
    }
}

fn object_array_literal_is_inline(ctx: &Ctx<'_, '_>, expr: &Expression<'_>) -> bool {
    match expr.get_inner_expression() {
        Expression::ObjectExpression(o) => o.properties.iter().all(|p| match p {
            ObjectPropertyKind::ObjectProperty(prop) => is_stable_literal_value(ctx, &prop.value),
            ObjectPropertyKind::SpreadProperty(_) => false,
        }),
        Expression::ArrayExpression(a) => a.elements.iter().all(|el| match el {
            ArrayExpressionElement::SpreadElement(_) => false,
            ArrayExpressionElement::Elision(_) => true,
            other => other
                .as_expression()
                .is_some_and(|e| is_stable_literal_value(ctx, e)),
        }),
        _ => false,
    }
}

fn is_stable_literal_value(ctx: &Ctx<'_, '_>, expr: &Expression<'_>) -> bool {
    let expr = expr.get_inner_expression();
    match expr {
        Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_)
        | Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_) => true,
        Expression::TemplateLiteral(t) => t.expressions.is_empty(),
        Expression::ObjectExpression(_) | Expression::ArrayExpression(_) => {
            object_array_literal_is_inline(ctx, expr)
        }
        Expression::Identifier(id) => {
            let Some(ref_id) = id.reference_id.get() else {
                return false;
            };
            let Some(sym) = ctx.semantics.get_reference(ref_id).symbol_id() else {
                return false;
            };
            if ctx
                .semantics
                .symbol_flags(sym)
                .contains(SymbolFlags::Import)
            {
                return false;
            }
            ctx.reactivity.binding_semantics(sym).is_non_reactive()
                && symbol_read_is_static(ctx.value_evaluation, ctx.semantics, sym)
        }
        _ => false,
    }
}

fn derive_each_context_vars(ctx: &Ctx<'_, '_>, d: &BindDirective) -> SmallVec<[SymbolId; 4]> {
    let mut result: SmallVec<[SymbolId; 4]> = SmallVec::new();
    let Some(data) = ctx.expression_data(d.id) else {
        return result;
    };
    for &sym in &data.references {
        let each_contextual = match ctx.reactivity.binding_semantics(sym) {
            BindingSemantics::Contextual(contextual) => match contextual {
                ContextualBindingSemantics::EachItem(_)
                | ContextualBindingSemantics::EachIndex(_) => true,
                ContextualBindingSemantics::AwaitValue
                | ContextualBindingSemantics::AwaitError
                | ContextualBindingSemantics::LetDirective
                | ContextualBindingSemantics::LetDirectiveDirect
                | ContextualBindingSemantics::LetDirectiveCarrierMember { .. }
                | ContextualBindingSemantics::SnippetParam(_) => false,
            },
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
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::Unresolved => false,
        };
        if each_contextual && !result.contains(&sym) {
            result.push(sym);
        }
    }
    result
}

fn bind_root_identifier_symbol(ctx: &Ctx<'_, '_>, d: &BindDirective) -> Option<SymbolId> {
    let expr = ctx.parsed.expr(d.expression.id())?;
    let Expression::Identifier(ident) = expr.get_inner_expression() else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    ctx.semantics.get_reference(ref_id).symbol_id()
}

fn derive_component_bind_kind(ctx: &Ctx<'_, '_>, d: &BindDirective) -> ComponentBindKind {
    let symbol = bind_root_identifier_symbol(ctx, d);

    if d.name == "this" {
        let target = match symbol {
            Some(sym) => derive_component_bind_target(ctx, d, sym),
            None => ComponentBindTarget::Plain,
        };
        return ComponentBindKind::This { symbol, target };
    }

    if let ReferenceSemantics::StoreRead {
        symbol: store_symbol,
    }
    | ReferenceSemantics::StoreWrite {
        symbol: store_symbol,
    }
    | ReferenceSemantics::StoreUpdate {
        symbol: store_symbol,
    } = bind_root_reference_semantics(ctx, d).unwrap_or(ReferenceSemantics::Unresolved)
    {
        let base_symbol = match ctx.reactivity.binding_semantics(store_symbol) {
            BindingSemantics::Store(store) => store.base_symbol,
            _ => store_symbol,
        };
        return ComponentBindKind::StoreSubscribed { base_symbol };
    }

    let Some(sym) = symbol else {
        if let Some(Expression::SequenceExpression(seq)) = ctx
            .parsed
            .expr(d.expression.id())
            .map(|e| e.get_inner_expression())
            && seq.expressions.len() == 2
        {
            return ComponentBindKind::FunctionPair;
        }
        return ComponentBindKind::Expression;
    };

    if ctx
        .reactivity
        .binding_semantics(sym)
        .is_each_item_indexed_legacy()
    {
        return ComponentBindKind::Expression;
    }

    let target = derive_component_bind_target(ctx, d, sym);
    ComponentBindKind::Identifier {
        symbol: sym,
        target,
    }
}

fn derive_component_bind_target(
    ctx: &Ctx<'_, '_>,
    d: &BindDirective,
    sym: SymbolId,
) -> ComponentBindTarget {
    if !ctx.reactivity.uses_runes()
        && let Some(Expression::Identifier(ident)) = ctx
            .parsed
            .expr(d.expression.id())
            .map(|e| e.get_inner_expression())
        && let Some(symbol) = each_item_write_root_symbol(ctx, ident)
    {
        return ComponentBindTarget::EachItemWriteLegacy { symbol };
    }
    let base = match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Source { .. },
            ..
        })
        | BindingSemantics::LegacyBindableProp(_) => ComponentBindTarget::PropSource,
        BindingSemantics::State(state) => ComponentBindTarget::Rune {
            proxy: state.kind == StateKind::State,
        },
        BindingSemantics::OptimizedRune(opt) => ComponentBindTarget::Rune {
            proxy: opt.proxy_init,
        },
        BindingSemantics::Derived(_) | BindingSemantics::OptimizedDerived(_) => {
            ComponentBindTarget::RuneDerived
        }
        BindingSemantics::LegacyState(_) => {
            if ctx.reactivity.store_shadow_of_internal(sym).is_some() {
                ComponentBindTarget::LegacyStateSubscribed
            } else {
                ComponentBindTarget::LegacyState
            }
        }
        _ => ComponentBindTarget::Plain,
    };
    if matches!(base, ComponentBindTarget::PropSource)
        && ctx.dev
        && !ctx
            .ignore_data
            .is_ignored(d.id, "ownership_invalid_binding")
    {
        ComponentBindTarget::PropSourceOwned
    } else {
        base
    }
}

fn bind_root_reference_semantics(
    ctx: &Ctx<'_, '_>,
    d: &BindDirective,
) -> Option<ReferenceSemantics> {
    let expr = ctx.parsed.expr(d.expression.id())?;
    let Expression::Identifier(ident) = expr.get_inner_expression() else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    Some(ctx.reactivity.reference_semantics(ref_id))
}

fn classify_boundary(ctx: &Ctx<'_, '_>, b: &SvelteBoundary, store: &mut AttributeSemanticsStore) {
    for attr in &b.attributes {
        if let Attribute::ExpressionAttribute(ea) = attr {
            let volatility = derive_boundary_prop_volatility(ctx, ea);
            store.set(
                ea.id,
                AttributeSemantics::BoundaryProp(BoundaryPropSemantics { volatility }),
            );
        }
    }
}

fn derive_boundary_prop_volatility(ctx: &Ctx<'_, '_>, ea: &ExpressionAttribute) -> Volatility {
    ctx.expression_data(ea.id)
        .map_or(Volatility::Static, |data| data.volatility)
}

fn classify_window(ctx: &Ctx<'_, '_>, w: &SvelteWindow, store: &mut AttributeSemanticsStore) {
    for attr in &w.attributes {
        match attr {
            Attribute::BindDirective(d) => {
                if let Some(property) = window_property(&d.name) {
                    store.set(
                        d.id,
                        AttributeSemantics::WindowBind(WindowBindSemantics {
                            property,
                            kind: bind_kind(ctx, d),
                            blockers: derive_blockers(ctx, d),
                        }),
                    );
                }
            }
            Attribute::ExpressionAttribute(ea) if ea.event_name.is_some() => {
                classify_html_event(ctx, ea, store);
            }
            Attribute::OnDirectiveLegacy(d) => {
                classify_html_on_directive_legacy(ctx, d, store);
            }
            _ => {}
        }
    }
}

fn classify_document(ctx: &Ctx<'_, '_>, d: &SvelteDocument, store: &mut AttributeSemanticsStore) {
    for attr in &d.attributes {
        match attr {
            Attribute::BindDirective(dir) => {
                if let Some(property) = document_property(&dir.name) {
                    store.set(
                        dir.id,
                        AttributeSemantics::DocumentBind(DocumentBindSemantics {
                            property,
                            kind: bind_kind(ctx, dir),
                            blockers: derive_blockers(ctx, dir),
                        }),
                    );
                }
            }
            Attribute::ExpressionAttribute(ea) if ea.event_name.is_some() => {
                classify_html_event(ctx, ea, store);
            }
            Attribute::OnDirectiveLegacy(dir) => {
                classify_html_on_directive_legacy(ctx, dir, store);
            }
            _ => {}
        }
    }
}

fn classify_body(ctx: &Ctx<'_, '_>, b: &SvelteBody, store: &mut AttributeSemanticsStore) {
    for attr in &b.attributes {
        match attr {
            Attribute::ExpressionAttribute(ea) if ea.event_name.is_some() => {
                classify_html_event(ctx, ea, store);
            }
            Attribute::OnDirectiveLegacy(d) => {
                classify_html_on_directive_legacy(ctx, d, store);
            }
            _ => {}
        }
    }
}

fn document_property(name: &str) -> Option<DocumentBindKind> {
    match name {
        "activeElement" => Some(DocumentBindKind::ActiveElement),
        "fullscreenElement" => Some(DocumentBindKind::FullscreenElement),
        "pointerLockElement" => Some(DocumentBindKind::PointerLockElement),
        "visibilityState" => Some(DocumentBindKind::VisibilityState),
        _ => None,
    }
}

fn window_property(name: &str) -> Option<WindowBindKind> {
    match name {
        "scrollX" => Some(WindowBindKind::ScrollX),
        "scrollY" => Some(WindowBindKind::ScrollY),
        "innerWidth" => Some(WindowBindKind::InnerWidth),
        "innerHeight" => Some(WindowBindKind::InnerHeight),
        "outerWidth" => Some(WindowBindKind::OuterWidth),
        "outerHeight" => Some(WindowBindKind::OuterHeight),
        "online" => Some(WindowBindKind::Online),
        "devicePixelRatio" => Some(WindowBindKind::DevicePixelRatio),
        _ => None,
    }
}
