//! ElementFlagsVisitor — precompute element attribute flags in one walker pass.

use svelte_ast::{is_mathml, is_svg, is_void, Attribute, ComponentNode, Element};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::types::data::{
    ClassDirectiveInfo, ComponentBindMode, ComponentPropInfo, ComponentPropKind, EventHandlerMode,
    EventModifier, FragmentKey, RichContentParentKind,
};
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
        modifiers.iter().fold(EventModifier::empty(), |mut flags, modifier| {
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
}

impl<'src> TemplateVisitor for ElementFlagsVisitor<'src> {
    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_>) {
        // Warn for non-void, non-SVG, non-MathML elements written as self-closing.
        if el.self_closing && !is_void(&el.name) && !is_svg(&el.name) && !is_mathml(&el.name) {
            ctx.warnings_mut().push(Diagnostic::warning(
                DiagnosticKind::ElementInvalidSelfClosingTag {
                    name: el.name.clone(),
                },
                el.span,
            ));
        }

        let has_value_attr = ctx.data.has_attribute(el.id, "value");
        let fragment_key = FragmentKey::Element(el.id);

        // <textarea>: detect expression children
        if el.name == "textarea" && ctx.data.fragment_has_expression_child(&fragment_key) {
            if has_value_attr {
                ctx.warnings_mut().push(Diagnostic::error(
                    DiagnosticKind::TextareaInvalidContent,
                    el.span,
                ));
            } else {
                ctx.data
                    .element_flags
                    .needs_textarea_value_lowering
                    .insert(el.id);
            }
        }

        // <option>: single ExpressionTag child, no explicit value attribute → synthetic __value
        if el.name == "option" && !has_value_attr {
            if let Some(child_id) = ctx.data.fragment_single_expression_child(&fragment_key) {
                ctx.data
                    .element_flags
                    .option_synthetic_value_expr
                    .insert(el.id, child_id);
            }
        }

        // Customizable select: <select>, <optgroup>, <option> with rich DOM content.
        let rich_content_parent = match el.name.as_str() {
            "select" => Some(RichContentParentKind::Select),
            "optgroup" => Some(RichContentParentKind::Optgroup),
            "option" => Some(RichContentParentKind::Option),
            _ => None,
        };
        if rich_content_parent
            .is_some_and(|parent| ctx.data.fragment_has_rich_content(&fragment_key, parent))
        {
            ctx.data.element_flags.customizable_select.insert(el.id);
        }
        if el.name == "selectedcontent" {
            ctx.data.element_flags.is_selectedcontent.insert(el.id);
        }
    }

    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_>) {
        let Some(el_id) = ctx.data.nearest_element(attr.id()) else {
            return;
        };
        match attr {
            Attribute::StringAttribute(sa) if sa.name == "class" => {
                ctx.data
                    .element_flags
                    .static_class
                    .insert(el_id, self.source_text(sa.value_span).to_string());
            }
            Attribute::StringAttribute(sa) if sa.name == "style" => {
                ctx.data
                    .element_flags
                    .static_style
                    .insert(el_id, self.source_text(sa.value_span).to_string());
            }
            Attribute::ClassDirective(cd) => {
                ctx.data
                    .element_flags
                    .class_directive_info
                    .get_or_default(el_id)
                    .push(ClassDirectiveInfo {
                        id: cd.id,
                        name: cd.name.clone(),
                        has_expression: cd.expression_span.is_some(),
                    });
            }
            Attribute::StyleDirective(sd) => {
                ctx.data
                    .element_flags
                    .style_directives
                    .get_or_default(el_id)
                    .push(sd.clone());
            }
            Attribute::ExpressionAttribute(ea) => {
                if ea.name == "class" {
                    ctx.data.element_flags.class_attr_id.insert(el_id, ea.id);
                }
                if ea.name == "value" && ctx.element_name() == Some("input") {
                    ctx.data.element_flags.needs_input_defaults.insert(el_id);
                }
                if let Some(raw) = ea.event_name.as_deref() {
                    let (name, capture) = if let Some(base) = crate::utils::strip_capture_event(raw)
                    {
                        (base, true)
                    } else {
                        (raw, false)
                    };
                    let passive = crate::utils::is_passive_event(name);
                    let mode = if !capture && crate::utils::is_delegatable_event(name) {
                        EventHandlerMode::Delegated { passive }
                    } else {
                        EventHandlerMode::Direct { capture, passive }
                    };
                    ctx.data
                        .element_flags
                        .event_handler_mode
                        .insert(ea.id, mode);
                }
            }
            Attribute::ConcatenationAttribute(attr) => {
                if attr.name == "class" {
                    ctx.data.element_flags.class_attr_id.insert(el_id, attr.id);
                }
            }
            Attribute::BindDirective(bd) => {
                if ctx.element_name() == Some("input")
                    && matches!(bd.name.as_str(), "value" | "checked" | "group")
                {
                    ctx.data.element_flags.needs_input_defaults.insert(el_id);
                }
            }
            Attribute::OnDirectiveLegacy(dir) => {
                ctx.data
                    .directive_modifiers
                    .record(dir.id, Self::modifier_flags(&dir.modifiers));
            }
            Attribute::TransitionDirective(dir) => {
                ctx.data
                    .directive_modifiers
                    .record(dir.id, Self::modifier_flags(&dir.modifiers));
            }
            Attribute::UseDirective(_) => {
                ctx.data.element_flags.has_use_directive.insert(el_id);
            }
            _ => {}
        }
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_>) {
        let data = &mut *ctx.data;
        // Dotted component names are dynamic (e.g., registry.Widget → $.component(...))
        if cn.name.contains('.') {
            data.element_flags.is_dynamic_component.insert(cn.id);
        }
        for attr in &cn.attributes {
            // CSS custom properties (`--name`) on a component are routed to the
            // wrapper-element + `$.css_props(...)` lowering, not into the regular
            // component props loop.
            let css_prop_name: Option<&str> = match attr {
                Attribute::ExpressionAttribute(a) if a.name.starts_with("--") => Some(&a.name),
                Attribute::StringAttribute(a) if a.name.starts_with("--") => Some(&a.name),
                Attribute::ConcatenationAttribute(a) if a.name.starts_with("--") => Some(&a.name),
                _ => None,
            };
            if let Some(name) = css_prop_name {
                data.element_flags
                    .component_css_props
                    .get_or_default(cn.id)
                    .push((name.to_string(), attr.id()));
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
                        shorthand: a.shorthand,
                        needs_memo,
                    }
                }
                Attribute::ConcatenationAttribute(a) => ComponentPropKind::Concatenation {
                    name: a.name.clone(),
                    attr_id: a.id,
                    parts: a.parts.clone(),
                },
                Attribute::Shorthand(a) => {
                    let name = self.source_text(a.expression_span).trim().to_string();
                    ComponentPropKind::Shorthand {
                        attr_id: a.id,
                        name,
                    }
                }
                Attribute::SpreadAttribute(a) => ComponentPropKind::Spread { attr_id: a.id },
                Attribute::BindDirective(b) if b.name == "this" => {
                    ComponentPropKind::BindThis { bind_id: b.id }
                }
                Attribute::BindDirective(b) => {
                    // Store-sub detection: only possible with explicit expression
                    // (`bind:value={$count}`). Shorthand `bind:count` binds to `count`,
                    // never to `$count`, so it can't be a store sub.
                    let expr_text = if b.shorthand {
                        None
                    } else {
                        b.expression_span
                            .map(|span| self.source_text(span).to_string())
                    };

                    let is_store = expr_text
                        .as_deref()
                        .is_some_and(|t| data.scoping.is_store_ref(t));

                    if is_store {
                        ComponentPropKind::Bind {
                            name: b.name.clone(),
                            bind_id: b.id,
                            mode: ComponentBindMode::StoreSub,
                            expr_name: expr_text,
                        }
                    } else {
                        let root = data.scoping.root_scope_id();
                        let mode = data
                            .scoping
                            .find_binding(root, &b.name)
                            .map(|sym| {
                                if data.scoping.is_prop_source(sym) {
                                    ComponentBindMode::PropSource
                                } else if data.scoping.is_rune(sym) && data.scoping.is_mutated(sym)
                                {
                                    ComponentBindMode::Rune
                                } else {
                                    ComponentBindMode::Plain
                                }
                            })
                            .unwrap_or(ComponentBindMode::Plain);
                        ComponentPropKind::Bind {
                            name: b.name.clone(),
                            bind_id: b.id,
                            mode,
                            expr_name: None,
                        }
                    }
                }
                Attribute::AttachTag(a) => ComponentPropKind::Attach { attr_id: a.id },
                Attribute::OnDirectiveLegacy(a) => {
                    let flags = Self::modifier_flags(&a.modifiers);
                    data.directive_modifiers.record(a.id, flags);
                    ComponentPropKind::Event {
                        name: a.name.clone(),
                        attr_id: a.id,
                        has_expression: a.expression_span.is_some(),
                        has_once_modifier: flags.contains(EventModifier::ONCE),
                    }
                }
                _ => continue,
            };
            let is_dynamic = data.element_flags.is_dynamic_attr(attr.id());
            data.element_flags
                .component_props
                .get_or_default(cn.id)
                .push(ComponentPropInfo { kind, is_dynamic });
        }
    }
}
