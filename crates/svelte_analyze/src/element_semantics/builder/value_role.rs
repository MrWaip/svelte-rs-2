use svelte_ast::{Attribute, Element, NodeId};

use super::super::ElementValueRole;
use crate::types::data::AnalysisData;

pub(super) fn classify(data: &AnalysisData, el: &Element) -> ElementValueRole {
    let flags = &data.elements.flags;
    let rich = flags.is_customizable_select(el.id);
    match el.name.as_str() {
        "select" if select_is_controlled(el) => ElementValueRole::Select { rich },
        "option" => ElementValueRole::Option {
            value: flags.option_synthetic_value_expr(el.id),
            rich,
        },
        "textarea" => match textarea_value(data, el) {
            Some(value) => ElementValueRole::TextareaValue { value },
            None => ElementValueRole::Plain,
        },
        "select" | "optgroup" if rich => ElementValueRole::RichContainer,
        _ => ElementValueRole::Plain,
    }
}

fn textarea_value(data: &AnalysisData, el: &Element) -> Option<NodeId> {
    if !data.elements.flags.needs_textarea_value_lowering(el.id) {
        return None;
    }
    data.fragment_single_expression_child_by_id(el.fragment)
}

fn select_is_controlled(el: &Element) -> bool {
    el.attributes.iter().any(|attr| match attr {
        Attribute::SpreadAttribute(_) => true,
        Attribute::BindDirective(d) => d.name == "value",
        Attribute::ExpressionAttribute(a) => a.name == "value",
        Attribute::StringAttribute(a) => a.name == "value",
        Attribute::ConcatenationAttribute(a) => a.name == "value",
        Attribute::BooleanAttribute(a) => a.name == "value",
        _ => false,
    })
}
