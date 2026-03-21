//! ElementFlagsVisitor — precompute element attribute flags in one walker pass.

use oxc_semantic::ScopeId;
use svelte_ast::{Attribute, Element, SvelteElement};
use svelte_span::Span;

use crate::data::{AnalysisData, ClassDirectiveInfo};
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
                    .entry(el.id)
                    .or_default()
                    .push(ClassDirectiveInfo {
                        id: cd.id,
                        name: cd.name.clone(),
                        has_expression: cd.expression_span.is_some(),
                    });
            }
            Attribute::StyleDirective(_) => {
                data.element_flags.has_style_directives.insert(el.id);
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
            Attribute::UseDirective(_) => {
                data.element_flags.has_use_directive.insert(el.id);
            }
            _ => {}
        }
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
            if let Attribute::ClassDirective(cd) = attr {
                data.element_flags.class_directive_info
                    .entry(el.id)
                    .or_default()
                    .push(ClassDirectiveInfo {
                        id: cd.id,
                        name: cd.name.clone(),
                        has_expression: cd.expression_span.is_some(),
                    });
            }
        }
    }
}
