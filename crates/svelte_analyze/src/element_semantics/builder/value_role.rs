use svelte_ast::{Attribute, Component, Element, Node, NodeId};

use super::super::{ElementValueRole, TextareaBody, TextareaSegment};
use crate::attribute_semantics::{AttributeSemantics, ElementBindPropertyKind};
use crate::types::data::{AnalysisData, ContentEditableKind};

pub(super) fn classify(
    component: &Component,
    data: &AnalysisData,
    el: &Element,
) -> ElementValueRole {
    if let Some((bind_id, kind)) = contenteditable_bind(data, el) {
        return ElementValueRole::ContentEditable { bind_id, kind };
    }
    let flags = &data.elements.flags;
    let rich = flags.is_customizable_select(el.id);
    match el.name.as_str() {
        "script" | "style" => ElementValueRole::RawText,
        "select" if select_is_controlled(el) => ElementValueRole::Select { rich },
        "option" => ElementValueRole::Option {
            value: flags.option_synthetic_value_expr(el.id),
            rich,
        },
        "textarea" => match textarea_value(component, data, el) {
            Some(body) => ElementValueRole::TextareaValue { body },
            None => ElementValueRole::Plain,
        },
        "select" | "optgroup" if rich => ElementValueRole::RichContainer,
        _ => ElementValueRole::Plain,
    }
}

fn contenteditable_bind(
    data: &AnalysisData,
    el: &Element,
) -> Option<(NodeId, ContentEditableKind)> {
    for attr in &el.attributes {
        let Attribute::BindDirective(_) = attr else {
            continue;
        };
        let AttributeSemantics::ElementBind(sem) = data.attributes.get(attr.id()) else {
            continue;
        };
        let ElementBindPropertyKind::ContentEditable(kind) = sem.property else {
            continue;
        };
        return Some((attr.id(), kind));
    }
    None
}

fn textarea_value(
    component: &Component,
    data: &AnalysisData,
    el: &Element,
) -> Option<TextareaBody> {
    if data.elements.flags.needs_textarea_value_lowering(el.id) {
        if let Some(child) = data.fragment_single_expression_child_by_id(el.fragment)
            && let Node::ExpressionTag(tag) = component.store.get(child)
        {
            return Some(TextareaBody::Single(tag.expression.id()));
        }
        return Some(TextareaBody::Segments(textarea_segments(component, el)));
    }
    for attr in &el.attributes {
        match attr {
            Attribute::BindDirective(d) if d.name == "value" => {
                return Some(TextareaBody::Single(d.expression.id()));
            }
            Attribute::ExpressionAttribute(a) if a.name == "value" => {
                return Some(TextareaBody::Single(a.expression.id()));
            }
            _ => {}
        }
    }
    None
}

fn textarea_segments(component: &Component, el: &Element) -> Vec<TextareaSegment> {
    let source = component.source.as_str();
    let mut parts: Vec<TextareaSegment> = Vec::new();
    for child_id in component.store.fragment_nodes(el.fragment) {
        match component.store.get(*child_id) {
            Node::Text(t) => {
                let text = t.value(source);
                if let Some(TextareaSegment::Text(prev)) = parts.last_mut() {
                    prev.push_str(text);
                } else {
                    parts.push(TextareaSegment::Text(text.to_string()));
                }
            }
            Node::ExpressionTag(ex) => parts.push(TextareaSegment::Expression {
                node_id: ex.id,
                oxc_id: ex.expression.id(),
            }),
            _ => {}
        }
    }
    if let Some(TextareaSegment::Text(first)) = parts.first_mut()
        && let Some(stripped) = first
            .strip_prefix("\r\n")
            .or_else(|| first.strip_prefix('\n'))
    {
        *first = stripped.to_string();
    }
    parts
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
