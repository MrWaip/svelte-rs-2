use svelte_ast::{
    AnimateDirective, Attribute, BindDirective, BooleanAttribute, ClassDirective, ConcatPart,
    ConcatenationAttribute, ExprRef, ExpressionAttribute, LetDirectiveLegacy, OnDirectiveLegacy,
    SpreadAttribute, StmtRef, StringAttribute, StyleDirective, StyleDirectiveValue,
    TransitionDirective, UseDirective,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::{GetSpan, Span};

use crate::Parser;
use crate::html::decode_attribute_value;
use crate::scanner::token;

impl<'a> Parser<'a> {
    pub(crate) fn convert_attributes(
        &mut self,
        token_attrs: &[token::Attribute],
        is_component: bool,
    ) -> Vec<Attribute> {
        let mut attributes = Vec::new();

        for attr in token_attrs {
            let attr_id = self.reserve_id();
            let attr_span = attr.span();
            match attr {
                token::Attribute::HTMLAttribute(html_attr) => {
                    let name = html_attr.name_span.source_text(self.source);

                    if !is_component
                        && matches!(name.as_bytes().first(), Some(&b) if b.is_ascii_digit() || b == b'-')
                    {
                        self.diagnostics.push(Diagnostic::error(
                            DiagnosticKind::AttributeInvalidName {
                                name: name.to_string(),
                            },
                            html_attr.name_span,
                        ));
                    }

                    let result = match &html_attr.value {
                        token::AttributeValue::String(span) => {
                            let decoded = decode_attribute_value(span.source_text(self.source));
                            Attribute::StringAttribute(StringAttribute {
                                id: attr_id,
                                span: attr_span,
                                name: name.to_string(),
                                value_span: *span,
                                decoded,
                            })
                        }
                        token::AttributeValue::ExpressionTag(expr_tag) => {
                            let name = name.to_string();
                            let event_name = name.strip_prefix("on").map(|s| s.to_string());
                            Attribute::ExpressionAttribute(ExpressionAttribute {
                                id: attr_id,
                                span: attr_span,
                                name,
                                expression: ExprRef::new(expr_tag.expression_span),
                                shorthand: false,
                                event_name,
                            })
                        }
                        token::AttributeValue::Concatenation(concat) => {
                            let parts = self.convert_concat_parts(&concat.parts);
                            Attribute::ConcatenationAttribute(ConcatenationAttribute {
                                id: attr_id,
                                span: attr_span,
                                name: name.to_string(),
                                quoted: matches!(
                                    self.source.as_bytes().get(concat.span.start as usize),
                                    Some(b'"') | Some(b'\'')
                                ),
                                parts,
                            })
                        }
                        token::AttributeValue::Empty => {
                            Attribute::BooleanAttribute(BooleanAttribute {
                                id: attr_id,
                                span: attr_span,
                                name: name.to_string(),
                            })
                        }
                    };
                    attributes.push(result);
                }
                token::Attribute::ExpressionTag(expr_tag) => {
                    if expr_tag
                        .expression_span
                        .source_text(self.source)
                        .starts_with("...")
                    {
                        let span = svelte_span::Span::new(
                            expr_tag.expression_span.start + 3,
                            expr_tag.expression_span.end,
                        );
                        attributes.push(Attribute::SpreadAttribute(SpreadAttribute {
                            id: attr_id,
                            span: attr_span,
                            expression: ExprRef::new(span),
                        }));
                    } else {
                        let name = expr_tag
                            .expression_span
                            .source_text(self.source)
                            .to_string();
                        let event_name = name.strip_prefix("on").map(|s| s.to_string());
                        attributes.push(Attribute::ExpressionAttribute(ExpressionAttribute {
                            id: attr_id,
                            span: attr_span,
                            name,
                            expression: ExprRef::new(expr_tag.expression_span),
                            shorthand: true,
                            event_name,
                        }));
                    }
                }
                token::Attribute::ClassDirective(cd) => {
                    let cd_name = cd.name_span.source_text(self.source);
                    attributes.push(Attribute::ClassDirective(ClassDirective {
                        id: attr_id,
                        span: attr_span,
                        name: cd_name.to_string(),
                        expression: ExprRef::new(cd.expression_span),
                        shorthand: cd.shorthand,
                    }));
                }
                token::Attribute::StyleDirective(sd) => {
                    let sd_name = sd.name_span.source_text(self.source);
                    let (value, expression_span) = if sd.shorthand {
                        (StyleDirectiveValue::Expression, sd.name_span)
                    } else {
                        match &sd.value {
                            token::AttributeValue::ExpressionTag(et) => {
                                (StyleDirectiveValue::Expression, et.expression_span)
                            }
                            token::AttributeValue::String(span) => {
                                let raw = span.source_text(self.source);
                                let value =
                                    decode_attribute_value(raw).unwrap_or_else(|| raw.to_string());
                                (StyleDirectiveValue::String(value), *span)
                            }
                            token::AttributeValue::Concatenation(c) => {
                                if let [token::ConcatenationPart::Expression(et)] =
                                    c.parts.as_slice()
                                {
                                    (StyleDirectiveValue::Expression, et.expression_span)
                                } else {
                                    let span = c.span;
                                    (
                                        StyleDirectiveValue::Concatenation(
                                            self.convert_concat_parts(&c.parts),
                                        ),
                                        span,
                                    )
                                }
                            }
                            token::AttributeValue::Empty => {
                                debug_assert!(
                                    sd.shorthand,
                                    "Empty value on non-shorthand style directive"
                                );
                                (StyleDirectiveValue::Expression, sd.name_span)
                            }
                        }
                    };
                    attributes.push(Attribute::StyleDirective(StyleDirective {
                        id: attr_id,
                        span: attr_span,
                        name: sd_name.to_string(),
                        expression: ExprRef::new(expression_span),
                        shorthand: sd.shorthand,
                        value,
                        important: sd.important,
                        invalid_modifier: sd.invalid_modifier,
                    }));
                }
                token::Attribute::BindDirective(bd) => {
                    let bd_name = bd.name_span.source_text(self.source);
                    attributes.push(Attribute::BindDirective(BindDirective {
                        id: attr_id,
                        span: attr_span,
                        name: bd_name.to_string(),
                        expression: ExprRef::new(bd.expression_span),
                        shorthand: bd.shorthand,
                    }));
                }
                token::Attribute::LetDirectiveLegacy(ld) => {
                    let binding_span = if ld.has_expression {
                        ld.expression_span
                    } else {
                        ld.name_span
                    };
                    attributes.push(Attribute::LetDirectiveLegacy(LetDirectiveLegacy {
                        id: attr_id,
                        span: attr_span,
                        name: ld.name_span.source_text(self.source).to_string(),
                        name_span: ld.name_span,
                        binding: Some(StmtRef::new(binding_span)),
                    }));
                }
                token::Attribute::UseDirective(ud) => {
                    let expression_span = if ud.shorthand {
                        None
                    } else {
                        Some(ud.expression_span)
                    };
                    attributes.push(Attribute::UseDirective(UseDirective {
                        id: attr_id,
                        span: attr_span,
                        name_ref: ExprRef::new(ud.name_span),
                        expression: expression_span.map(ExprRef::new),
                    }));
                }

                token::Attribute::OnDirectiveLegacy(od) => {
                    let expression_span = if od.has_expression {
                        Some(od.expression_span)
                    } else {
                        None
                    };
                    attributes.push(Attribute::OnDirectiveLegacy(OnDirectiveLegacy {
                        id: attr_id,
                        span: attr_span,
                        name: od.name_span.source_text(self.source).to_string(),
                        name_span: od.name_span,
                        expression: expression_span.map(ExprRef::new),
                        modifiers: od
                            .modifiers
                            .iter()
                            .map(|m| m.source_text(self.source).to_string())
                            .collect(),
                    }));
                }
                token::Attribute::TransitionDirective(td) => {
                    let expression_span = if td.has_expression {
                        Some(td.expression_span)
                    } else {
                        None
                    };
                    let direction = td.direction;
                    attributes.push(Attribute::TransitionDirective(TransitionDirective {
                        id: attr_id,
                        span: attr_span,
                        name_ref: ExprRef::new(td.name_span),
                        expression: expression_span.map(ExprRef::new),
                        modifiers: td
                            .modifiers
                            .iter()
                            .map(|m| m.source_text(self.source).to_string())
                            .collect(),
                        direction,
                    }));
                }
                token::Attribute::AnimateDirective(ad) => {
                    let expression_span = if ad.has_expression {
                        Some(ad.expression_span)
                    } else {
                        None
                    };
                    attributes.push(Attribute::AnimateDirective(AnimateDirective {
                        id: attr_id,
                        span: attr_span,
                        name_ref: ExprRef::new(ad.name_span),
                        expression: expression_span.map(ExprRef::new),
                    }));
                }
                token::Attribute::AttachTag(at) => {
                    attributes.push(Attribute::AttachTag(svelte_ast::AttachTag {
                        id: attr_id,
                        span: attr_span,
                        expression: ExprRef::new(at.expression_span),
                    }));
                }
            }
        }

        attributes
    }

    pub(crate) fn classify_this_attribute(attributes: &[svelte_ast::Attribute]) -> (Span, bool) {
        for attr in attributes {
            match attr {
                svelte_ast::Attribute::ExpressionAttribute(a) if a.name == "this" => {
                    return (a.expression.span, false);
                }
                svelte_ast::Attribute::StringAttribute(a) if a.name == "this" => {
                    return (a.value_span, true);
                }
                _ => {}
            }
        }
        (Span::new(0, 0), false)
    }

    fn convert_concat_parts(&mut self, parts: &[token::ConcatenationPart]) -> Vec<ConcatPart> {
        parts
            .iter()
            .map(|part| match part {
                token::ConcatenationPart::String(span) => {
                    let raw = span.source_text(self.source);
                    let value = decode_attribute_value(raw).unwrap_or_else(|| raw.to_string());
                    ConcatPart::Static(value)
                }
                token::ConcatenationPart::Expression(et) => ConcatPart::Dynamic {
                    id: self.reserve_id(),
                    expr: ExprRef::new(et.expression_span),
                },
            })
            .collect()
    }
}
