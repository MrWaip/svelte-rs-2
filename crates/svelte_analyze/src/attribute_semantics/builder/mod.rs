use super::AttributeSemanticsStore;
use super::data::{
    AttributeSemantics, BoundaryPropEmit, BoundaryPropSemantics, ComponentAttachEmit,
    ComponentAttachSemantics, ComponentBindKind, ComponentBindSemantics, ComponentBindTarget,
    ComponentPropConcatSemantics, ComponentPropExpressionSemantics, ComponentPropMemo,
    ComponentPropSemantics, ComponentSpreadEmit, ComponentSpreadSemantics, DocumentBindSemantics,
    ElementBindPropertyKind, ElementBindSemantics, EventEmit, EventSemantics, HandlerEmit,
    HtmlBindKind, WindowBindSemantics,
};
use crate::reactivity_semantics::data::{
    BindingSemantics, ContextualBindingSemantics, ReactivitySemantics, ReferenceSemantics,
};
use crate::scope::SymbolId;
use crate::types::data::{
    BlockerData, ContentEditableKind, DocumentBindKind, ElementSizeKind, EventModifier,
    IgnoreData, ImageNaturalSizeKind, JsAst, MediaBindKind, ResizeObserverKind, WindowBindKind,
};
use crate::utils::events::{is_delegatable_event, is_passive_event, strip_capture_event};
use oxc_ast::ast::Expression;
use oxc_ast_visit::{Visit, walk};
use smallvec::SmallVec;
use svelte_ast::{
    Attribute, BindDirective, Component, Element, ExpressionAttribute, FragmentId,
    Node, NodeId, SvelteBoundary, SvelteDocument, SvelteWindow,
};
use svelte_component_semantics::{ComponentSemantics, SymbolFlags};

pub fn build<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    semantics: &ComponentSemantics<'a>,
    reactivity: &ReactivitySemantics,
    blockers: &BlockerData,
    ignore_data: &IgnoreData,
    dev: bool,
    node_count: u32,
) -> AttributeSemanticsStore {
    let ctx = Ctx {
        component,
        parsed,
        semantics,
        reactivity,
        blockers,
        ignore_data,
        dev,
    };
    let mut store = AttributeSemanticsStore::new(node_count);
    let mut state = WalkState::default();
    walk_fragment(&ctx, &mut state, component.root, &mut store);
    store
}

struct Ctx<'a, 'p> {
    component: &'p Component,
    parsed: &'p JsAst<'a>,
    semantics: &'p ComponentSemantics<'a>,
    reactivity: &'p ReactivitySemantics,
    blockers: &'p BlockerData,
    ignore_data: &'p IgnoreData,
    dev: bool,
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
) {
    for &id in &ctx.component.store.fragment(fragment).nodes {
        let node = ctx.component.store.get(id);
        match node {
            Node::SvelteWindow(w) => classify_window(ctx, w, store),
            Node::SvelteDocument(d) => classify_document(ctx, d, store),
            Node::Element(el) => {
                classify_element(ctx, state, el, store);
                walk_fragment(ctx, state, el.fragment, store);
            }
            Node::ComponentNode(cn) => {
                classify_component_attrs(ctx, state, &cn.attributes, store);
                walk_fragment(ctx, state, cn.fragment, store);
            }
            Node::SvelteComponentLegacy(cn) => {
                classify_component_attrs(ctx, state, &cn.attributes, store);
                walk_fragment(ctx, state, cn.fragment, store);
            }
            Node::SvelteElement(el) => {
                classify_svelte_element(ctx, state, el, store);
                walk_fragment(ctx, state, el.fragment, store);
            }
            Node::SlotElementLegacy(el) => {
                classify_component_attrs(ctx, state, &el.attributes, store);
                walk_fragment(ctx, state, el.fragment, store);
            }
            Node::SvelteHead(head) => walk_fragment(ctx, state, head.fragment, store),
            Node::SvelteBoundary(b) => {
                classify_boundary(ctx, b, store);
                walk_fragment(ctx, state, b.fragment, store);
            }
            Node::IfBlock(b) => {
                walk_fragment(ctx, state, b.consequent, store);
                if let Some(alt) = b.alternate {
                    walk_fragment(ctx, state, alt, store);
                }
            }
            Node::EachBlock(b) => {
                state.each_stack.push(b.id);
                walk_fragment(ctx, state, b.body, store);
                state.each_stack.pop();
                if let Some(fallback) = b.fallback {
                    walk_fragment(ctx, state, fallback, store);
                }
            }
            Node::AwaitBlock(b) => {
                if let Some(p) = b.pending {
                    walk_fragment(ctx, state, p, store);
                }
                if let Some(t) = b.then {
                    walk_fragment(ctx, state, t, store);
                }
                if let Some(c) = b.catch {
                    walk_fragment(ctx, state, c, store);
                }
            }
            Node::KeyBlock(b) => walk_fragment(ctx, state, b.fragment, store),
            Node::SnippetBlock(b) => walk_fragment(ctx, state, b.body, store),
            _ => {}
        }
    }
}

fn classify_svelte_element(
    ctx: &Ctx<'_, '_>,
    state: &WalkState,
    el: &svelte_ast::SvelteElement,
    store: &mut AttributeSemanticsStore,
) {
    classify_element_attrs(ctx, state, &el.attributes, None, store);
}

fn classify_element(
    ctx: &Ctx<'_, '_>,
    state: &WalkState,
    el: &Element,
    store: &mut AttributeSemanticsStore,
) {
    classify_element_attrs(ctx, state, &el.attributes, Some(el), store);
}

fn classify_element_attrs(
    ctx: &Ctx<'_, '_>,
    state: &WalkState,
    attrs: &[Attribute],
    el: Option<&Element>,
    store: &mut AttributeSemanticsStore,
) {
    for attr in attrs {
        match attr {
            Attribute::BindDirective(d) => {
                if let Some(property) = element_property(&d.name) {
                    let kind = bind_kind(ctx, d);
                    let blockers = derive_blockers(ctx, d);
                    let (parent_each_blocks, group_value_attr) =
                        if matches!(property, ElementBindPropertyKind::Group) {
                            (
                                derive_parent_each_blocks(ctx, state, d),
                                el.and_then(find_value_attr_id),
                            )
                        } else {
                            (SmallVec::new(), None)
                        };
                    store.set(
                        d.id,
                        AttributeSemantics::ElementBind(ElementBindSemantics {
                            property,
                            kind,
                            blockers,
                            parent_each_blocks,
                            group_value_attr,
                        }),
                    );
                }
            }
            Attribute::ExpressionAttribute(ea) => {
                if ea.event_name.is_some() {
                    classify_html_event(ctx, ea, store);
                }
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
            _ => {}
        }
    }
}

fn derive_blockers(ctx: &Ctx<'_, '_>, d: &BindDirective) -> SmallVec<[u32; 2]> {
    let mut result: SmallVec<[u32; 2]> = SmallVec::new();
    if !ctx.blockers.has_async() {
        return result;
    }
    let Some(expr) = ctx.parsed.expr(d.expression.id()) else {
        return result;
    };
    let mut probe = ExprShapeProbe::new(ctx);
    probe.visit_expression(expr);
    for sym in probe.references {
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
    let Some(expr) = ctx.parsed.expr(d.expression.id()) else {
        return result;
    };
    let mut probe = ExprShapeProbe::new(ctx);
    probe.visit_expression(expr);
    for &each_id in state.each_stack.iter().rev() {
        let referenced = probe
            .references
            .iter()
            .any(|sym| ctx.reactivity.contextual_owner(*sym) == Some(each_id));
        if referenced && !result.contains(&each_id) {
            result.push(each_id);
        }
    }
    result
}

fn find_value_attr_id(el: &Element) -> Option<NodeId> {
    el.attributes.iter().find_map(|attr| match attr {
        Attribute::ExpressionAttribute(a) if a.name == "value" => Some(a.id),
        _ => None,
    })
}

fn bind_kind(ctx: &Ctx<'_, '_>, d: &BindDirective) -> HtmlBindKind {
    use crate::reactivity_semantics::data::PropReferenceSemantics;
    let Some(expr) = ctx.parsed.expr(d.expression.id()) else {
        return HtmlBindKind::Plain;
    };
    let ident = match expr {
        Expression::Identifier(ident) => ident,
        _ => return HtmlBindKind::Plain,
    };
    let Some(ref_id) = ident.reference_id.get() else {
        return HtmlBindKind::Plain;
    };
    match ctx.reactivity.reference_semantics(ref_id) {
        ReferenceSemantics::StoreRead { symbol: store_symbol }
        | ReferenceSemantics::StoreWrite { symbol: store_symbol }
        | ReferenceSemantics::StoreUpdate { symbol: store_symbol } => {
            let base_symbol = match ctx.reactivity.binding_semantics(store_symbol) {
                BindingSemantics::Store(store) => store.base_symbol,
                _ => store_symbol,
            };
            HtmlBindKind::StoreSubscribed { base_symbol }
        }
        ReferenceSemantics::PropRead(PropReferenceSemantics::Source { bindable: true, .. })
        | ReferenceSemantics::PropMutation { bindable: true, .. } => {
            HtmlBindKind::BindableProp
        }
        ReferenceSemantics::SignalRead { .. }
        | ReferenceSemantics::SignalWrite { .. }
        | ReferenceSemantics::SignalUpdate { .. } => HtmlBindKind::Rune,
        _ => HtmlBindKind::Plain,
    }
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
    let direct = match expr {
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
            let is_import = symbol.is_some_and(|sym| {
                ctx.semantics
                    .symbol_flags(sym)
                    .contains(SymbolFlags::Import)
            });
            is_function || (!ctx.dev && !is_import)
        }
        _ => false,
    };
    if direct {
        return HandlerEmit::Direct;
    }
    let mut probe = HandlerShapeProbe::default();
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
struct HandlerShapeProbe {
    has_call: bool,
    has_side_effects: bool,
    fn_depth: u32,
}

impl<'a> Visit<'a> for HandlerShapeProbe {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_call = true;
            self.has_side_effects = true;
        }
        walk::walk_call_expression(self, expr);
    }

    fn visit_assignment_expression(&mut self, expr: &oxc_ast::ast::AssignmentExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_side_effects = true;
        }
        walk::walk_assignment_expression(self, expr);
    }

    fn visit_update_expression(&mut self, expr: &oxc_ast::ast::UpdateExpression<'a>) {
        if self.fn_depth == 0 {
            self.has_side_effects = true;
        }
        walk::walk_update_expression(self, expr);
    }

    fn visit_arrow_function_expression(
        &mut self,
        arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
        self.fn_depth += 1;
        walk::walk_arrow_function_expression(self, arrow);
        self.fn_depth -= 1;
    }

    fn visit_function(&mut self, func: &oxc_ast::ast::Function<'a>, flags: oxc_semantic::ScopeFlags) {
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

fn classify_component_attrs(
    ctx: &Ctx<'_, '_>,
    _state: &WalkState,
    attrs: &[Attribute],
    store: &mut AttributeSemanticsStore,
) {
    for attr in attrs {
        match attr {
            Attribute::BindDirective(d) => {
                let kind = derive_component_bind_kind(ctx, d);
                let each_context_vars = if d.name == "this" {
                    derive_each_context_vars(ctx, d)
                } else {
                    SmallVec::new()
                };
                store.set(
                    d.id,
                    AttributeSemantics::ComponentBind(ComponentBindSemantics {
                        kind,
                        each_context_vars,
                    }),
                );
            }
            Attribute::ExpressionAttribute(ea) => {
                let memo = derive_component_prop_memo_for_expression(ctx, ea);
                store.set(
                    ea.id,
                    AttributeSemantics::ComponentProp(ComponentPropSemantics::Expression(
                        ComponentPropExpressionSemantics {
                            memo,
                            shorthand: ea.shorthand,
                        },
                    )),
                );
            }
            Attribute::ConcatenationAttribute(ca) => {
                let memo = derive_component_concat_memo(ctx, ca);
                store.set(
                    ca.id,
                    AttributeSemantics::ComponentProp(ComponentPropSemantics::Concat(
                        ComponentPropConcatSemantics { memo },
                    )),
                );
            }
            Attribute::SpreadAttribute(sa) => {
                let emit = derive_component_spread_emit(ctx, sa.expression.id());
                store.set(
                    sa.id,
                    AttributeSemantics::ComponentSpread(ComponentSpreadSemantics { emit }),
                );
            }
            Attribute::AttachTag(at) => {
                let emit = derive_component_attach_emit(ctx, at.expression.id());
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

fn derive_component_concat_memo(
    ctx: &Ctx<'_, '_>,
    ca: &svelte_ast::ConcatenationAttribute,
) -> ComponentPropMemo {
    for part in &ca.parts {
        if let svelte_ast::ConcatPart::Dynamic { expr, .. } = part
            && let Some(e) = ctx.parsed.expr(expr.id())
        {
            let mut probe = ExprShapeProbe::new(ctx);
            probe.visit_expression(e);
            if probe.is_dynamic || probe.is_import {
                return ComponentPropMemo::Getter;
            }
        }
    }
    ComponentPropMemo::Inline
}

fn derive_component_attach_emit(
    ctx: &Ctx<'_, '_>,
    expr_id: svelte_component_semantics::OxcNodeId,
) -> ComponentAttachEmit {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        return ComponentAttachEmit::Inline;
    };
    let mut probe = ExprShapeProbe::new(ctx);
    probe.visit_expression(expr);
    if probe.is_dynamic || probe.is_import {
        ComponentAttachEmit::Wrapped
    } else {
        ComponentAttachEmit::Inline
    }
}

fn derive_component_spread_emit(
    ctx: &Ctx<'_, '_>,
    expr_id: svelte_component_semantics::OxcNodeId,
) -> ComponentSpreadEmit {
    let Some(expr) = ctx.parsed.expr(expr_id) else {
        return ComponentSpreadEmit::Inline;
    };
    let mut probe = ExprShapeProbe::new(ctx);
    probe.visit_expression(expr);
    if probe.is_dynamic || probe.is_import {
        ComponentSpreadEmit::Thunk
    } else {
        ComponentSpreadEmit::Inline
    }
}

fn derive_component_prop_memo_for_expression(
    ctx: &Ctx<'_, '_>,
    ea: &ExpressionAttribute,
) -> ComponentPropMemo {
    let Some(expr) = ctx.parsed.expr(ea.expression.id()) else {
        return ComponentPropMemo::Inline;
    };
    if matches!(
        expr,
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_)
    ) {
        return ComponentPropMemo::Inline;
    }
    let mut probe = ExprShapeProbe::new(ctx);
    probe.visit_expression(expr);
    let simple_shape = matches!(
        expr,
        Expression::Identifier(_) | Expression::StaticMemberExpression(_)
    );
    if probe.has_call || (!simple_shape && probe.is_dynamic) {
        ComponentPropMemo::Derived
    } else if probe.is_dynamic {
        ComponentPropMemo::Getter
    } else {
        ComponentPropMemo::Inline
    }
}

struct ExprShapeProbe<'c, 'a> {
    ctx: &'c Ctx<'a, 'c>,
    has_call: bool,
    is_dynamic: bool,
    is_import: bool,
    references: SmallVec<[SymbolId; 4]>,
}

impl<'c, 'a> ExprShapeProbe<'c, 'a> {
    fn new(ctx: &'c Ctx<'a, 'c>) -> Self {
        Self {
            ctx,
            has_call: false,
            is_dynamic: false,
            is_import: false,
            references: SmallVec::new(),
        }
    }
}

impl<'c, 'a> Visit<'a> for ExprShapeProbe<'c, 'a> {
    fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
        self.has_call = true;
        walk::walk_call_expression(self, expr);
    }

    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        if matches!(
            self.ctx.reactivity.reference_semantics(ref_id),
            ReferenceSemantics::SignalRead { .. }
                | ReferenceSemantics::SignalWrite { .. }
                | ReferenceSemantics::SignalUpdate { .. }
                | ReferenceSemantics::StoreRead { .. }
                | ReferenceSemantics::StoreWrite { .. }
                | ReferenceSemantics::StoreUpdate { .. }
                | ReferenceSemantics::PropRead(_)
                | ReferenceSemantics::ContextualRead(_)
                | ReferenceSemantics::CarrierMemberRead(_)
                | ReferenceSemantics::ConstAliasRead { .. }
                | ReferenceSemantics::LegacyStateRead { .. }
                | ReferenceSemantics::LegacyStateWrite
                | ReferenceSemantics::LegacyStateUpdate { .. }
                | ReferenceSemantics::LegacyStateSubscribedRead { .. }
                | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
                | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
                | ReferenceSemantics::LegacyReactiveImportRead
                | ReferenceSemantics::ImportSubscribedRead { .. }
        ) {
            self.is_dynamic = true;
        }
        if let Some(symbol) = self.ctx.semantics.get_reference(ref_id).symbol_id() {
            if !matches!(
                self.ctx.reactivity.binding_semantics(symbol),
                BindingSemantics::NonReactive | BindingSemantics::Const(_),
            ) {
                self.is_dynamic = true;
            }
            if self
                .ctx
                .semantics
                .symbol_flags(symbol)
                .contains(SymbolFlags::Import)
            {
                self.is_import = true;
            }
            if !self.references.contains(&symbol) {
                self.references.push(symbol);
            }
        }
    }
}

fn derive_each_context_vars(ctx: &Ctx<'_, '_>, d: &BindDirective) -> SmallVec<[SymbolId; 4]> {
    let mut result: SmallVec<[SymbolId; 4]> = SmallVec::new();
    let Some(expr) = ctx.parsed.expr(d.expression.id()) else {
        return result;
    };
    let mut probe = ExprShapeProbe::new(ctx);
    probe.visit_expression(expr);
    for sym in probe.references {
        if matches!(
            ctx.reactivity.binding_semantics(sym),
            BindingSemantics::Contextual(
                ContextualBindingSemantics::EachItem(_)
                    | ContextualBindingSemantics::EachIndex(_),
            )
        ) && !result.contains(&sym)
        {
            result.push(sym);
        }
    }
    result
}

fn bind_root_identifier_symbol(ctx: &Ctx<'_, '_>, d: &BindDirective) -> Option<SymbolId> {
    let expr = ctx.parsed.expr(d.expression.id())?;
    let Expression::Identifier(ident) = expr else {
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

    if let ReferenceSemantics::StoreRead { symbol: store_symbol }
    | ReferenceSemantics::StoreWrite { symbol: store_symbol }
    | ReferenceSemantics::StoreUpdate { symbol: store_symbol } =
        bind_root_reference_semantics(ctx, d).unwrap_or(ReferenceSemantics::Unresolved)
    {
        let base_symbol = match ctx.reactivity.binding_semantics(store_symbol) {
            BindingSemantics::Store(store) => store.base_symbol,
            _ => store_symbol,
        };
        return ComponentBindKind::StoreSubscribed { base_symbol };
    }

    let Some(sym) = symbol else {
        return ComponentBindKind::Expression;
    };

    let target = derive_component_bind_target(ctx, d, sym);
    ComponentBindKind::Identifier { symbol: sym, target }
}

fn derive_component_bind_target(
    ctx: &Ctx<'_, '_>,
    d: &BindDirective,
    sym: SymbolId,
) -> ComponentBindTarget {
    let base = match ctx.reactivity.binding_semantics(sym) {
        BindingSemantics::Prop(crate::reactivity_semantics::data::PropBindingSemantics {
            kind: crate::reactivity_semantics::data::PropBindingKind::Source { .. },
            ..
        })
        | BindingSemantics::LegacyBindableProp(_) => ComponentBindTarget::PropSource,
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedRune(_) => ComponentBindTarget::Rune,
        _ => ComponentBindTarget::Plain,
    };
    if matches!(base, ComponentBindTarget::PropSource)
        && ctx.dev
        && !ctx.ignore_data.is_ignored(d.id, "ownership_invalid_binding")
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
    let Expression::Identifier(ident) = expr else {
        return None;
    };
    let ref_id = ident.reference_id.get()?;
    Some(ctx.reactivity.reference_semantics(ref_id))
}

fn classify_boundary(ctx: &Ctx<'_, '_>, b: &SvelteBoundary, store: &mut AttributeSemanticsStore) {
    for attr in &b.attributes {
        if let Attribute::ExpressionAttribute(ea) = attr {
            let emit = derive_boundary_prop_emit(ctx, ea);
            store.set(
                ea.id,
                AttributeSemantics::BoundaryProp(BoundaryPropSemantics { emit }),
            );
        }
    }
}

fn derive_boundary_prop_emit(ctx: &Ctx<'_, '_>, ea: &ExpressionAttribute) -> BoundaryPropEmit {
    let Some(expr) = ctx.parsed.expr(ea.expression.id()) else {
        return BoundaryPropEmit::KeyValue;
    };
    let mut probe = ExprShapeProbe::new(ctx);
    probe.visit_expression(expr);
    if probe.is_dynamic || probe.is_import {
        BoundaryPropEmit::Getter
    } else {
        BoundaryPropEmit::KeyValue
    }
}

fn classify_window(ctx: &Ctx<'_, '_>, w: &SvelteWindow, store: &mut AttributeSemanticsStore) {
    for attr in &w.attributes {
        if let Attribute::BindDirective(d) = attr
            && let Some(property) = window_property(&d.name)
        {
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
}

fn classify_document(ctx: &Ctx<'_, '_>, d: &SvelteDocument, store: &mut AttributeSemanticsStore) {
    for attr in &d.attributes {
        if let Attribute::BindDirective(dir) = attr
            && let Some(property) = document_property(&dir.name)
        {
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
