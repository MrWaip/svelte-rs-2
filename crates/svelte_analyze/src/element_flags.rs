//! ElementFlagsVisitor — precompute element attribute flags in one walker pass.

use oxc_semantic::ScopeId;
use svelte_ast::{Attribute, ComponentNode, Element, SvelteElement};
use svelte_span::Span;

use crate::data::{AnalysisData, ClassDirectiveInfo, ComponentBindMode, ComponentPropInfo, ComponentPropKind, EventHandlerMode};
use crate::walker::TemplateVisitor;

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
    fn visit_attribute(
        &mut self,
        attr: &Attribute,
        el: &Element,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        match attr {
            Attribute::SpreadAttribute(_) => {
                data.element_flags.has_spread.insert(el.id);
            }
            Attribute::ClassDirective(cd) => {
                data.element_flags.class_directive_info
                    .get_or_default(el.id)
                    .push(ClassDirectiveInfo {
                        id: cd.id,
                        name: cd.name.clone(),
                        has_expression: cd.expression_span.is_some(),
                    });
            }
            Attribute::StyleDirective(sd) => {
                data.element_flags.style_directives
                    .get_or_default(el.id)
                    .push(sd.clone());
            }
            Attribute::ExpressionAttribute(ea) if ea.name == "class" => {
                data.element_flags.class_attr_id.insert(el.id, ea.id);
            }
            Attribute::StringAttribute(sa) if sa.name == "class" => {
                data.element_flags.static_class.insert(el.id, self.source_text(sa.value_span).to_string());
            }
            Attribute::StringAttribute(sa) if sa.name == "style" => {
                data.element_flags.static_style.insert(el.id, self.source_text(sa.value_span).to_string());
            }
            Attribute::BindDirective(bd) if el.name == "input" && matches!(bd.name.as_str(), "value" | "checked" | "group") => {
                data.element_flags.needs_input_defaults.insert(el.id);
            }
            Attribute::ExpressionAttribute(ea) if ea.name == "value" && el.name == "input" => {
                data.element_flags.needs_input_defaults.insert(el.id);
            }
            Attribute::ExpressionAttribute(ea) if ea.event_name.is_some() => {
                let raw = ea.event_name.as_deref().unwrap();
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
                data.element_flags.event_handler_mode.insert(ea.id, mode);
            }
            Attribute::UseDirective(_) => {
                data.element_flags.has_use_directive.insert(el.id);
            }
            _ => {}
        }
    }

    fn visit_component_attribute(
        &mut self,
        attr: &Attribute,
        cn: &ComponentNode,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
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
                // Non-bind:this: classify getter/setter pattern
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
            // Directives that don't become props
            _ => return,
        };
        let is_dynamic = data.element_flags.is_dynamic_attr(attr.id());
        data.element_flags.component_props
            .get_or_default(cn.id)
            .push(ComponentPropInfo { kind, is_dynamic });
    }

    /// SvelteElement attributes aren't dispatched through visit_attribute
    /// (which takes &Element), so collect class directives here.
    fn visit_svelte_element(
        &mut self,
        el: &SvelteElement,
        _scope: ScopeId,
        data: &mut AnalysisData,
    ) {
        for attr in &el.attributes {
            match attr {
                Attribute::ClassDirective(cd) => {
                    data.element_flags.class_directive_info
                        .get_or_default(el.id)
                        .push(ClassDirectiveInfo {
                            id: cd.id,
                            name: cd.name.clone(),
                            has_expression: cd.expression_span.is_some(),
                        });
                }
                Attribute::StyleDirective(sd) => {
                    data.element_flags.style_directives
                        .get_or_default(el.id)
                        .push(sd.clone());
                }
                _ => {}
            }
        }
    }
}
