use svelte_ast::{Attribute, ComponentNode, Element, is_mathml, is_svg, is_void};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::attribute_semantics::data::ComponentPropMemo;
use crate::expression_semantics::ExprKind;
use crate::types::data::{
    BindingSemantics, BindTargetSemantics, ClassDirectiveInfo, ComponentBindMode, ComponentCssProp,
    ComponentCssPropValue, ComponentPropInfo, ComponentPropKind, EventHandlerMode, EventModifier,
    ParentKind, PropBindingKind, PropBindingSemantics, RichContentParentKind,
};
use crate::utils::{is_delegatable_event, is_passive_event, is_simple_identifier, strip_capture_event};
use crate::walker::{TemplateVisitor, VisitContext};

pub(crate) struct ElementFlagsVisitor<'src> {
    source: &'src str,
}

impl<'src> ElementFlagsVisitor<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source }
    }

    fn source_text(&self, span: Span) -> &str {
        &self.source[span.start as usize..span.end as usize]
    }

    fn modifier_flags(modifiers: &[String]) -> EventModifier {
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

    fn marks_input_defaults(name: &str) -> bool {
        matches!(name, "value" | "checked")
    }

    fn skip_input_defaults_gate(ctx: &VisitContext<'_, '_>, el_id: svelte_ast::NodeId) -> bool {
        ctx.data.has_attribute(el_id, "defaultValue")
            || ctx.data.has_attribute(el_id, "defaultChecked")
    }
}

impl<'src> TemplateVisitor for ElementFlagsVisitor<'src> {
    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {
        if el.self_closing && !is_void(&el.name) && !is_svg(&el.name) && !is_mathml(&el.name) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::ElementInvalidSelfClosingTag {
                    name: el.name.clone(),
                },
                el.span,
            ));
        }

        let has_value_attr = ctx.data.has_attribute(el.id, "value");
        let fragment_id = el.fragment;

        if el.name == "textarea" && ctx.data.fragment_has_expression_child_by_id(fragment_id) {
            if has_value_attr {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::TextareaInvalidContent,
                    el.span,
                ));
            } else {
                ctx.data
                    .elements
                    .flags
                    .needs_textarea_value_lowering
                    .insert(el.id);
            }
        }

        if el.name == "option"
            && !has_value_attr
            && let Some(child_id) = ctx.data.fragment_single_expression_child_by_id(fragment_id)
        {
            ctx.data
                .elements
                .flags
                .option_synthetic_value_expr
                .insert(el.id, child_id);
        }

        let rich_content_parent = match el.name.as_str() {
            "select" => Some(RichContentParentKind::Select),
            "optgroup" => Some(RichContentParentKind::Optgroup),
            "option" => Some(RichContentParentKind::Option),
            _ => None,
        };
        if rich_content_parent.is_some_and(|parent| {
            ctx.data
                .fragment_has_rich_content_by_id(fragment_id, parent)
        }) {
            ctx.data.elements.flags.customizable_select.insert(el.id);
        }
        if el.name == "selectedcontent" {
            ctx.data.elements.flags.is_selectedcontent.insert(el.id);
        }

        let has_group = el.attributes.iter().any(|attr| {
            matches!(attr, Attribute::BindDirective(d) if d.name == "group")
        });
        if has_group {
            ctx.data
                .template
                .bind_semantics
                .has_bind_group
                .insert(el.id);
            ctx.data.template.bind_semantics.any_bind_group = true;
        }

        let has_contenteditable =
            ctx.data
                .has_true_boolean_attribute(el.id, &el.attributes, "contenteditable", self.source);
        let has_content_bind = el.attributes.iter().any(|attr| {
            matches!(attr, Attribute::BindDirective(d) if matches!(
                d.name.as_str(),
                "innerHTML" | "innerText" | "textContent",
            ))
        });
        if has_contenteditable && has_content_bind {
            ctx.data.elements.flags.bound_contenteditable.insert(el.id);
        }

        if ctx.data.script.dev
            && ctx
                .data
                .output
                .ignore_data
                .is_ignored(el.id, "hydration_attribute_changed")
        {
            ctx.data
                .elements
                .flags
                .hydration_attribute_changed_ignored
                .insert(el.id);
        }
    }

    fn visit_svelte_element(
        &mut self,
        el: &svelte_ast::SvelteElement,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        if ctx.data.script.dev
            && ctx
                .data
                .output
                .ignore_data
                .is_ignored(el.id, "hydration_attribute_changed")
        {
            ctx.data
                .elements
                .flags
                .hydration_attribute_changed_ignored
                .insert(el.id);
        }

        let mut has_spread = false;
        let mut has_class_directive = false;
        let mut has_style_directive = false;
        let mut has_class_attr = false;
        let mut has_style_attr = false;
        for attr in &el.attributes {
            match attr {
                Attribute::SpreadAttribute(_) => has_spread = true,
                Attribute::ClassDirective(_) => has_class_directive = true,
                Attribute::StyleDirective(_) => has_style_directive = true,
                Attribute::StringAttribute(a) => {
                    if a.name == "class" {
                        has_class_attr = true;
                    } else if a.name == "style" {
                        has_style_attr = true;
                    }
                }
                Attribute::ExpressionAttribute(a) => {
                    if a.name == "class" {
                        has_class_attr = true;
                    } else if a.name == "style" {
                        has_style_attr = true;
                    }
                }
                Attribute::ConcatenationAttribute(a) => {
                    if a.name == "class" {
                        has_class_attr = true;
                    } else if a.name == "style" {
                        has_style_attr = true;
                    }
                }
                _ => {}
            }
        }
        if !has_spread && has_class_directive && !has_class_attr {
            ctx.data.elements.flags.needs_class_base.insert(el.id);
        }
        if !has_spread && has_style_directive && !has_style_attr {
            ctx.data.elements.flags.needs_style_base.insert(el.id);
        }
    }

    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_, '_>) {
        let Some(el_id) = ctx.data.nearest_element(attr.id()) else {
            return;
        };
        match attr {
            Attribute::StringAttribute(sa) if sa.name == "class" => {
                ctx.data
                    .elements
                    .flags
                    .static_class
                    .insert(el_id, self.source_text(sa.value_span).to_string());
            }
            Attribute::StringAttribute(sa) if sa.name == "style" => {
                ctx.data
                    .elements
                    .flags
                    .static_style
                    .insert(el_id, self.source_text(sa.value_span).to_string());
            }
            Attribute::ClassDirective(cd) => {
                ctx.data
                    .elements
                    .flags
                    .class_directive_info
                    .get_or_default(el_id)
                    .push(ClassDirectiveInfo {
                        id: cd.id,
                        name: cd.name.clone(),
                        has_expression: true,
                        expr_id: cd.expression.id(),
                    });
            }
            Attribute::StyleDirective(sd) => {
                ctx.data
                    .elements
                    .flags
                    .style_directives
                    .get_or_default(el_id)
                    .push(sd.clone());
            }
            Attribute::ExpressionAttribute(ea) => {
                if ea.name == "class" {
                    ctx.data.elements.flags.class_attr_id.insert(el_id, ea.id);
                }
                if ctx.element_name() == Some("input")
                    && Self::marks_input_defaults(&ea.name)
                    && !Self::skip_input_defaults_gate(ctx, el_id)
                {
                    ctx.data.elements.flags.needs_input_defaults.insert(el_id);
                }
                if let Some(raw) = ea.event_name.as_deref() {
                    let (name, capture) = if let Some(base) = strip_capture_event(raw)
                    {
                        (base, true)
                    } else {
                        (raw, false)
                    };
                    let passive = is_passive_event(name);
                    let mode = if !capture && is_delegatable_event(name) {
                        EventHandlerMode::Delegated { passive }
                    } else {
                        EventHandlerMode::Direct { capture, passive }
                    };
                    ctx.data
                        .elements
                        .flags
                        .event_handler_mode
                        .insert(ea.id, mode);
                }
            }
            Attribute::ConcatenationAttribute(attr) => {
                if attr.name == "class" {
                    ctx.data.elements.flags.class_attr_id.insert(el_id, attr.id);
                }
                if ctx.element_name() == Some("input")
                    && Self::marks_input_defaults(&attr.name)
                    && !Self::skip_input_defaults_gate(ctx, el_id)
                {
                    ctx.data.elements.flags.needs_input_defaults.insert(el_id);
                }
            }
            Attribute::BooleanAttribute(ba) => {
                if ctx.element_name() == Some("input")
                    && Self::marks_input_defaults(&ba.name)
                    && !Self::skip_input_defaults_gate(ctx, el_id)
                {
                    ctx.data.elements.flags.needs_input_defaults.insert(el_id);
                }
            }
            Attribute::BindDirective(bd) => {
                if ctx.element_name() == Some("input")
                    && matches!(bd.name.as_str(), "value" | "checked" | "group")
                    && !Self::skip_input_defaults_gate(ctx, el_id)
                {
                    ctx.data.elements.flags.needs_input_defaults.insert(el_id);
                }
            }
            Attribute::SpreadAttribute(_) => {
                if ctx.element_name() == Some("input")
                    && !Self::skip_input_defaults_gate(ctx, el_id)
                {
                    ctx.data.elements.flags.needs_input_defaults.insert(el_id);
                }
            }
            Attribute::UseDirective(_) => {
                ctx.data.elements.flags.has_use_directive.insert(el_id);
            }
            _ => {}
        }
    }

    fn visit_svelte_component_legacy(
        &mut self,
        cn: &svelte_ast::SvelteComponentLegacy,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        self.process_component_like(cn.id, &cn.attributes, ctx);
        self.mark_bind_group_if_present(cn.id, &cn.attributes, ctx);
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_, '_>) {
        self.process_component_like(cn.id, &cn.attributes, ctx);
        self.mark_bind_group_if_present(cn.id, &cn.attributes, ctx);
    }

    fn visit_svelte_self(
        &mut self,
        cn: &svelte_ast::SvelteSelf,
        ctx: &mut VisitContext<'_, '_>,
    ) {
        self.process_component_like(cn.id, &cn.attributes, ctx);
        self.mark_bind_group_if_present(cn.id, &cn.attributes, ctx);
    }
}

impl<'src> ElementFlagsVisitor<'src> {
    fn mark_bind_group_if_present(
        &self,
        node_id: svelte_ast::NodeId,
        attributes: &[Attribute],
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let has_group = attributes.iter().any(|attr| {
            matches!(attr, Attribute::BindDirective(d) if d.name == "group")
        });
        if has_group {
            ctx.data
                .template
                .bind_semantics
                .has_bind_group
                .insert(node_id);
            ctx.data.template.bind_semantics.any_bind_group = true;
        }
    }

    fn process_component_like(
        &self,
        cn_id: svelte_ast::NodeId,
        attributes: &[Attribute],
        ctx: &mut VisitContext<'_, '_>,
    ) {
        let data = &mut *ctx.data;
        for attr in attributes {
            let css_prop_name: Option<&str> = match attr {
                Attribute::ExpressionAttribute(a) if a.name.starts_with("--") => Some(&a.name),
                Attribute::StringAttribute(a) if a.name.starts_with("--") => Some(&a.name),
                Attribute::ConcatenationAttribute(a) if a.name.starts_with("--") => Some(&a.name),
                _ => None,
            };
            if let Some(name) = css_prop_name {
                let (value, memo) = match attr {
                    Attribute::ExpressionAttribute(a) => {
                        let memo = derive_css_prop_memo(data, a.id);
                        (
                            Some(ComponentCssPropValue::Expression(a.expression.id())),
                            memo,
                        )
                    }
                    Attribute::StringAttribute(a) => (
                        Some(ComponentCssPropValue::StaticString(a.value_span)),
                        ComponentPropMemo::Inline,
                    ),
                    Attribute::ConcatenationAttribute(_) => (
                        Some(ComponentCssPropValue::Concatenation),
                        ComponentPropMemo::Inline,
                    ),
                    _ => (None, ComponentPropMemo::Inline),
                };
                if let Some(value) = value {
                    data.elements
                        .flags
                        .component_css_props
                        .get_or_default(cn_id)
                        .push(ComponentCssProp {
                            name: name.to_string(),
                            attr_id: attr.id(),
                            value,
                            memo,
                        });
                }
                continue;
            }
            let kind = match attr {
                Attribute::StringAttribute(a) => ComponentPropKind::String {
                    name: a.name.clone(),
                    value_span: a.value_span,
                },
                Attribute::BooleanAttribute(a) => ComponentPropKind::Boolean {
                    name: a.name.clone(),
                },
                Attribute::ExpressionAttribute(a) => {
                    let needs_memo = data.component_attr_needs_memo(a.id);
                    ComponentPropKind::Expression {
                        name: a.name.clone(),
                        attr_id: a.id,
                        expr_id: a.expression.id(),
                        shorthand: a.shorthand,
                        needs_memo,
                    }
                }
                Attribute::ConcatenationAttribute(a) => ComponentPropKind::Concatenation {
                    name: a.name.clone(),
                    attr_id: a.id,
                    parts: a.parts.clone(),
                },
                Attribute::SpreadAttribute(a) => ComponentPropKind::Spread {
                    attr_id: a.id,
                    expr_id: a.expression.id(),
                },
                Attribute::BindDirective(b) => {
                    let Some(bind_semantics) = BindTargetSemantics::from_parent_kind_and_name(
                        ParentKind::ComponentNode,
                        b.name.as_str(),
                    ) else {
                        continue;
                    };

                    let bind_id = b.id;
                    if bind_semantics.is_this() {
                        ComponentPropKind::BindThis {
                            bind_id,
                            expr_id: b.expression.id(),
                        }
                    } else {
                        let expr_text = if b.shorthand {
                            None
                        } else {
                            Some(self.source_text(b.expression.span).to_string())
                        };

                        let is_store = expr_text.as_deref().is_some_and(|t| {
                            let trimmed = t.trim();
                            trimmed.starts_with('$')
                                && trimmed.len() > 1
                                && !trimmed.starts_with("$$")
                                && {
                                    let root = data.scoping.root_scope_id();
                                    data.scoping.find_binding(root, trimmed).is_some_and(|sym| {
                                        matches!(
                                            data.binding_semantics(sym),
                                            BindingSemantics::Store(_),
                                        )
                                    })
                                }
                        });

                        if is_store {
                            ComponentPropKind::Bind {
                                name: b.name.clone(),
                                bind_id,
                                expr_id: b.expression.id(),
                                mode: ComponentBindMode::StoreSub,
                                expr_name: expr_text,
                                requires_ownership_emit: false,
                            }
                        } else {
                            let source_lookup_name = match &expr_text {
                                Some(text) if is_simple_identifier(text.trim()) => {
                                    text.trim().to_string()
                                }
                                Some(_) => b.name.clone(),
                                None => b.name.clone(),
                            };
                            let root = data.scoping.root_scope_id();
                            let mode = data
                                .scoping
                                .find_binding(root, &source_lookup_name)
                                .map(|sym| {
                                    let decl = data
                                        .reactivity
                                        .binding_semantics(sym);
                                    match decl {
                                        BindingSemantics::Prop(
                                            PropBindingSemantics {
                                                kind: PropBindingKind::Source { .. },
                                                ..
                                            },
                                        )
                                        | BindingSemantics::LegacyBindableProp(_) => {
                                            ComponentBindMode::PropSource
                                        }
                                        BindingSemantics::State(_)
                                        | BindingSemantics::Derived(_)
                                        | BindingSemantics::OptimizedDerived(_)
                                        | BindingSemantics::OptimizedRune(_) => {
                                            ComponentBindMode::Rune
                                        }
                                        _ => ComponentBindMode::Plain,
                                    }
                                })
                                .unwrap_or(ComponentBindMode::Plain);
                            let requires_ownership_emit = data.script.dev
                                && matches!(mode, ComponentBindMode::PropSource)
                                && !data
                                    .output
                                    .ignore_data
                                    .is_ignored(bind_id, "ownership_invalid_binding");
                            if requires_ownership_emit {
                                data.output.needs_component_bind_ownership = true;
                            }
                            let source_ident = match &expr_text {
                                Some(text) if is_simple_identifier(text.trim()) => {
                                    Some(text.trim().to_string())
                                }
                                _ => None,
                            };
                            ComponentPropKind::Bind {
                                name: b.name.clone(),
                                bind_id,
                                expr_id: b.expression.id(),
                                mode,
                                expr_name: source_ident,
                                requires_ownership_emit,
                            }
                        }
                    }
                }
                Attribute::AttachTag(a) => ComponentPropKind::Attach {
                    attr_id: a.id,
                    expr_id: a.expression.id(),
                },
                Attribute::OnDirectiveLegacy(a) => {
                    let flags = Self::modifier_flags(&a.modifiers);
                    ComponentPropKind::Event {
                        name: a.name.clone(),
                        attr_id: a.id,
                        expr_id: a.expression.as_ref().map(|r| r.id()),
                        has_expression: a.expression.is_some(),
                        has_once_modifier: flags.contains(EventModifier::ONCE),
                    }
                }
                _ => continue,
            };
            let is_dynamic = data.dynamism.is_dynamic_attr(attr.id());
            data.elements
                .flags
                .component_props
                .get_or_default(cn_id)
                .push(ComponentPropInfo { kind, is_dynamic });
        }
    }
}

fn derive_css_prop_memo(
    data: &crate::types::data::AnalysisData,
    attr_id: svelte_ast::NodeId,
) -> ComponentPropMemo {
    match data.expression_data(attr_id) {
        Some(d) if matches!(d.kind, ExprKind::Call { .. } | ExprKind::Async { .. }) => {
            ComponentPropMemo::Derived
        }
        _ => ComponentPropMemo::Inline,
    }
}
