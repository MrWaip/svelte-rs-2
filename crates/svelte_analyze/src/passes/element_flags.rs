//! ElementFlagsVisitor — precompute element attribute flags in one walker pass.

use svelte_ast::{is_mathml, is_svg, is_void, Attribute, ComponentNode, Element, SVELTE_SELF};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::Span;

use crate::types::data::{
    BindTargetSemantics, ClassDirectiveInfo, ComponentBindMode, ComponentPropInfo,
    ComponentPropKind, EventHandlerMode, EventModifier, ParentKind, RichContentParentKind,
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
}

impl<'src> TemplateVisitor for ElementFlagsVisitor<'src> {
    fn visit_element(&mut self, el: &Element, ctx: &mut VisitContext<'_, '_>) {
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
        let fragment_id = el.fragment.id;

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

        // <option>: single ExpressionTag child, no explicit value attribute → synthetic __value
        if el.name == "option" && !has_value_attr {
            if let Some(child_id) = ctx.data.fragment_single_expression_child_by_id(fragment_id) {
                ctx.data
                    .elements
                    .flags
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
        if rich_content_parent.is_some_and(|parent| {
            ctx.data
                .fragment_has_rich_content_by_id(fragment_id, parent)
        }) {
            ctx.data.elements.flags.customizable_select.insert(el.id);
        }
        if el.name == "selectedcontent" {
            ctx.data.elements.flags.is_selectedcontent.insert(el.id);
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
                // Shorthand `class:name` now carries a synthesized `Identifier`
                // expression through `ParserResult`, so the codegen can always
                // reach a transformed value via `get_attr_expr`. `has_expression`
                // stays true uniformly — the shorthand-vs-explicit distinction
                // no longer affects expression availability.
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
                if ctx.element_name() == Some("input") && Self::marks_input_defaults(&ea.name) {
                    ctx.data.elements.flags.needs_input_defaults.insert(el_id);
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
                if ctx.element_name() == Some("input") && Self::marks_input_defaults(&attr.name) {
                    ctx.data.elements.flags.needs_input_defaults.insert(el_id);
                }
            }
            Attribute::BindDirective(bd) => {
                if ctx.element_name() == Some("input")
                    && ctx
                        .data
                        .bind_target_semantics(bd.id)
                        .is_some_and(|semantics| semantics.property().marks_input_defaults())
                {
                    ctx.data.elements.flags.needs_input_defaults.insert(el_id);
                }
            }
            Attribute::OnDirectiveLegacy(dir) => {
                ctx.data
                    .elements
                    .directive_modifiers
                    .record(dir.id, Self::modifier_flags(&dir.modifiers));
            }
            Attribute::TransitionDirective(dir) => {
                ctx.data
                    .elements
                    .directive_modifiers
                    .record(dir.id, Self::modifier_flags(&dir.modifiers));
            }
            Attribute::UseDirective(_) => {
                ctx.data.elements.flags.has_use_directive.insert(el_id);
            }
            _ => {}
        }
    }

    fn visit_component_node(&mut self, cn: &ComponentNode, ctx: &mut VisitContext<'_, '_>) {
        let data = &mut *ctx.data;
        let base_name = cn.name.split('.').next().unwrap_or(cn.name.as_str());
        if let Some(sym_id) = data.scoping.find_binding(ctx.scope, base_name) {
            data.elements
                .flags
                .component_binding_sym
                .insert(cn.id, sym_id);
        }
        if cn.name == SVELTE_SELF {
            data.elements.flags.is_svelte_self.insert(cn.id);
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
                let expr_id = match attr {
                    Attribute::ExpressionAttribute(a) => Some(a.expression.id()),
                    Attribute::ConcatenationAttribute(_) => None,
                    Attribute::StringAttribute(_) => None,
                    _ => None,
                };
                if let Some(expr_id) = expr_id {
                    data.elements
                        .flags
                        .component_css_props
                        .get_or_default(cn.id)
                        .push((name.to_string(), attr.id(), expr_id));
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

                    if bind_semantics.is_this() {
                        ComponentPropKind::BindThis {
                            bind_id: b.id,
                            expr_id: b.expression.id(),
                        }
                    } else {
                        // Store-sub detection: only possible with explicit expression
                        // (`bind:value={$count}`). Shorthand `bind:count` binds to `count`,
                        // never to `$count`, so it can't be a store sub.
                        let expr_text = if b.shorthand {
                            None
                        } else {
                            Some(self.source_text(b.expression.span).to_string())
                        };

                        // Store-sub detection on `bind:value={$count}`: resolve
                        // the textual expression's `$foo` root identifier to a
                        // root-scope binding whose declaration semantics is
                        // `Store(_)`. Uses the expression text because the bind
                        // attribute's `attr_expression` may resolve to a synthesized
                        // reference whose symbol isn't classified as a store in v2.
                        let is_store = expr_text.as_deref().is_some_and(|t| {
                            let trimmed = t.trim();
                            trimmed.starts_with('$')
                                && trimmed.len() > 1
                                && !trimmed.starts_with("$$")
                                && {
                                    let base = &trimmed[1..];
                                    let root = data.scoping.root_scope_id();
                                    data.scoping.find_binding(root, base).is_some_and(|sym| {
                                        matches!(
                                            data.declaration_semantics(
                                                data.scoping.symbol_declaration(sym),
                                            ),
                                            crate::types::data::DeclarationSemantics::Store(_),
                                        )
                                    })
                                }
                        });

                        if is_store {
                            ComponentPropKind::Bind {
                                name: b.name.clone(),
                                bind_id: b.id,
                                expr_id: b.expression.id(),
                                mode: ComponentBindMode::StoreSub,
                                expr_name: expr_text,
                            }
                        } else {
                            let root = data.scoping.root_scope_id();
                            let mode = data
                                .scoping
                                .find_binding(root, &b.name)
                                .map(|sym| {
                                    let decl = data
                                        .reactivity
                                        .declaration_semantics(data.scoping.symbol_declaration(sym));
                                    match decl {
                                        crate::types::data::DeclarationSemantics::Prop(
                                            crate::types::data::PropDeclarationSemantics {
                                                kind: crate::types::data::PropDeclarationKind::Source { .. },
                                                ..
                                            },
                                        ) => ComponentBindMode::PropSource,
                                        crate::types::data::DeclarationSemantics::State(_)
                                        | crate::types::data::DeclarationSemantics::Derived(_)
                                        | crate::types::data::DeclarationSemantics::OptimizedRune(_) => {
                                            ComponentBindMode::Rune
                                        }
                                        _ => ComponentBindMode::Plain,
                                    }
                                })
                                .unwrap_or(ComponentBindMode::Plain);
                            ComponentPropKind::Bind {
                                name: b.name.clone(),
                                bind_id: b.id,
                                expr_id: b.expression.id(),
                                mode,
                                expr_name: None,
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
                    data.elements.directive_modifiers.record(a.id, flags);
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
                .get_or_default(cn.id)
                .push(ComponentPropInfo { kind, is_dynamic });
        }
    }
}
