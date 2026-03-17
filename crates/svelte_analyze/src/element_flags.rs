//! ElementFlagsVisitor — precompute element attribute flags in one walker pass.

use oxc_semantic::ScopeId;
use svelte_ast::{Attribute, Element};
use svelte_span::Span;

use crate::data::AnalysisData;
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
            Attribute::ShorthandOrSpread(s) if s.is_spread => {
                data.element_flags.has_spread.insert(el.id);
            }
            Attribute::ClassDirective(_) => {
                data.element_flags.has_class_directives.insert(el.id);
            }
            Attribute::StyleDirective(_) => {
                data.element_flags.has_style_directives.insert(el.id);
            }
            Attribute::ExpressionAttribute(ea) if ea.name == "class" => {
                data.element_flags.has_class_attribute.insert(el.id);
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
            _ => {}
        }
    }
}
