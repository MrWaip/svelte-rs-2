//! ElementFlagsVisitor — precompute element attribute flags in one walker pass.

use oxc_semantic::ScopeId;
use svelte_ast::{Attribute, Element};

use crate::data::AnalysisData;
use crate::walker::TemplateVisitor;

pub(crate) struct ElementFlagsVisitor;

impl TemplateVisitor for ElementFlagsVisitor {
    fn visit_attribute(
        &mut self,
        attr: &Attribute,
        _idx: usize,
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
                data.element_flags.static_class.insert(el.id, sa.value_span);
            }
            Attribute::StringAttribute(sa) if sa.name == "style" => {
                data.element_flags.static_style.insert(el.id, sa.value_span);
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
