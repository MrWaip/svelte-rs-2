//! ElementFlagsVisitor — precompute element attribute flags in one walker pass.

use svelte_ast::{Attribute, ComponentNode};
use svelte_span::Span;

use crate::types::data::{ClassDirectiveInfo, ComponentBindMode, ComponentPropInfo, ComponentPropKind, EventHandlerMode};
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
}

impl<'src> TemplateVisitor for ElementFlagsVisitor<'src> {
    fn visit_attribute(&mut self, attr: &Attribute, ctx: &mut VisitContext<'_>) {
        let Some(el_id) = ctx.nearest_element() else { return };
        match attr {
            Attribute::StringAttribute(sa) if sa.name == "class" => {
                ctx.data.element_flags.static_class.insert(el_id, self.source_text(sa.value_span).to_string());
            }
            Attribute::StringAttribute(sa) if sa.name == "style" => {
                ctx.data.element_flags.static_style.insert(el_id, self.source_text(sa.value_span).to_string());
            }
            Attribute::SpreadAttribute(_) => {
                ctx.data.element_flags.has_spread.insert(el_id);
            }
            Attribute::ClassDirective(cd) => {
                ctx.data.element_flags.class_directive_info
                    .get_or_default(el_id)
                    .push(ClassDirectiveInfo {
                        id: cd.id,
                        name: cd.name.clone(),
                        has_expression: cd.expression_span.is_some(),
                    });
            }
            Attribute::StyleDirective(sd) => {
                ctx.data.element_flags.style_directives
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
                    let (name, capture) = if let Some(base) = crate::utils::strip_capture_event(raw) {
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
                    ctx.data.element_flags.event_handler_mode.insert(ea.id, mode);
                }
            }
            Attribute::BindDirective(bd) => {
                if ctx.element_name() == Some("input")
                    && matches!(bd.name.as_str(), "value" | "checked" | "group")
                {
                    ctx.data.element_flags.needs_input_defaults.insert(el_id);
                }
            }
            Attribute::UseDirective(_) => {
                ctx.data.element_flags.has_use_directive.insert(el_id);
            }
            _ => {}
        }
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_>) {
        let data = &mut *ctx.data;
        for attr in &cn.attributes {
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
                    ComponentPropKind::Shorthand { attr_id: a.id, name }
                }
                Attribute::SpreadAttribute(a) => ComponentPropKind::Spread { attr_id: a.id },
                Attribute::BindDirective(b) if b.name == "this" => {
                    ComponentPropKind::BindThis { bind_id: b.id }
                }
                Attribute::BindDirective(b) => {
                    let root = data.scoping.root_scope_id();
                    let mode = data.scoping.find_binding(root, &b.name)
                        .map(|sym| {
                            if data.scoping.is_prop_source(sym) {
                                ComponentBindMode::PropSource
                            } else if data.scoping.is_rune(sym) && data.scoping.is_mutated(sym) {
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
                    }
                }
                _ => continue,
            };
            let is_dynamic = data.element_flags.is_dynamic_attr(attr.id());
            data.element_flags.component_props
                .get_or_default(cn.id)
                .push(ComponentPropInfo { kind, is_dynamic });
        }
    }
}
