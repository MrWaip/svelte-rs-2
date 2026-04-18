use rustc_hash::FxHashSet;
use svelte_ast::{
    AnimateDirective, Attribute, BindDirective, BooleanAttribute, ClassDirective, ConcatPart,
    ConcatenationAttribute, ExpressionAttribute, LetDirectiveLegacy, OnDirectiveLegacy,
    SpreadAttribute, StringAttribute, StyleDirective, StyleDirectiveValue, TransitionDirection,
    TransitionDirective, UseDirective,
};
use svelte_diagnostics::{Diagnostic, DiagnosticKind};
use svelte_span::{GetSpan, Span};

use crate::scanner::token;
use crate::Parser;

/// Records `key` in `seen` and emits `attribute_duplicate` if already present.
/// When `exclude_this` is true, the `this` attribute is never added to `seen`
/// (matches the reference compiler behaviour for the `Attribute`/`BindDirective` key space).
fn track_duplicate<'s>(
    seen: &mut FxHashSet<(&'s str, &'s str)>,
    key: (&'s str, &'s str),
    name_span: Span,
    diagnostics: &mut Vec<Diagnostic>,
    exclude_this: bool,
) {
    if seen.contains(&key) {
        diagnostics.push(Diagnostic::error(
            DiagnosticKind::AttributeDuplicate,
            name_span,
        ));
    } else if !exclude_this || key.1 != "this" {
        seen.insert(key);
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn convert_attributes(
        &mut self,
        token_attrs: &[token::Attribute],
        is_component: bool,
    ) -> Vec<Attribute> {
        let mut attributes = Vec::new();
        // Tracks (type_key, name) pairs to detect duplicates.
        // HTMLAttribute and BindDirective share the "attr" key space (per reference compiler).
        let mut seen: FxHashSet<(&str, &str)> = FxHashSet::default();

        for attr in token_attrs {
            let attr_id = self.reserve_id();
            let attr_span = attr.span();
            match attr {
                token::Attribute::HTMLAttribute(html_attr) => {
                    // Extract name once; &'a str tied to the source lifetime, no allocation yet.
                    let name = html_attr.name_span.source_text(self.source);

                    // Our scanner accepts only alphanumeric, '-', and ':' in attribute names, so
                    // the only cases that pass the scanner but violate the reference compiler's
                    // `regex_illegal_attribute_character` are names starting with a digit or '-'.
                    // Check the first byte — O(1), no scan of the rest of the name needed.
                    // Reference compiler runs `regex_illegal_attribute_character` only in
                    // `visitors/shared/element.js` — components allow CSS custom-property
                    // names like `--color`, which the wrapper-element lowering picks up.
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

                    // on* handler attributes must carry an expression value, not a plain string.
                    // ExpressionTag is the only valid token value; String/Concatenation/Empty are
                    // not. Two-byte prefix check via starts_with on bytes — O(1).
                    if name.len() > 2
                        && name.as_bytes().starts_with(b"on")
                        && !matches!(html_attr.value, token::AttributeValue::ExpressionTag(_))
                    {
                        self.diagnostics.push(Diagnostic::error(
                            DiagnosticKind::AttributeInvalidEventHandler,
                            html_attr.name_span,
                        ));
                    }

                    // HTMLAttribute shares the "attr" key space with BindDirective.
                    track_duplicate(
                        &mut seen,
                        ("attr", name),
                        html_attr.name_span,
                        &mut self.diagnostics,
                        true,
                    );

                    let result = match &html_attr.value {
                        token::AttributeValue::String(span) => {
                            Attribute::StringAttribute(StringAttribute {
                                id: attr_id,
                                span: attr_span,
                                name: name.to_string(),
                                value_span: *span,
                            })
                        }
                        token::AttributeValue::ExpressionTag(expr_tag) => {
                            let name = name.to_string();
                            let event_name = name.strip_prefix("on").map(|s| s.to_string());
                            Attribute::ExpressionAttribute(ExpressionAttribute {
                                id: attr_id,
                                span: attr_span,
                                name,
                                expression_span: expr_tag.expression_span,
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
                        // Skip the "..." prefix so expression_span covers only the spread expression
                        attributes.push(Attribute::SpreadAttribute(SpreadAttribute {
                            id: attr_id,
                            span: attr_span,
                            expression_span: svelte_span::Span::new(
                                expr_tag.expression_span.start + 3,
                                expr_tag.expression_span.end,
                            ),
                        }));
                    } else {
                        // `{foo}` shorthand becomes a regular ExpressionAttribute
                        // with `shorthand: true`. The parser handles the "name
                        // equals expression text" invariant so downstream code
                        // has a single code path for all expression attributes.
                        let name = expr_tag
                            .expression_span
                            .source_text(self.source)
                            .to_string();
                        attributes.push(Attribute::ExpressionAttribute(ExpressionAttribute {
                            id: attr_id,
                            span: attr_span,
                            name,
                            expression_span: expr_tag.expression_span,
                            shorthand: true,
                            event_name: None,
                        }));
                    }
                }
                token::Attribute::ClassDirective(cd) => {
                    let cd_name = cd.name_span.source_text(self.source);
                    track_duplicate(
                        &mut seen,
                        ("class", cd_name),
                        cd.name_span,
                        &mut self.diagnostics,
                        false,
                    );
                    // Shorthand `class:name` scanner already sets expression_span
                    // to name_span so a synthesized identifier expression can
                    // be parsed from it later (walk_js.rs::parse_span).
                    attributes.push(Attribute::ClassDirective(ClassDirective {
                        id: attr_id,
                        span: attr_span,
                        name: cd_name.to_string(),
                        expression_span: cd.expression_span,
                        shorthand: cd.shorthand,
                    }));
                }
                token::Attribute::StyleDirective(sd) => {
                    let sd_name = sd.name_span.source_text(self.source);
                    track_duplicate(
                        &mut seen,
                        ("style", sd_name),
                        sd.name_span,
                        &mut self.diagnostics,
                        false,
                    );
                    // `expression_span` is always populated:
                    // - shorthand `style:name`           → span of `name`
                    // - `style:name={expr}`              → span of `expr`
                    // - `style:name="str"` / concat      → span of the value
                    //   region (unused by expression consumers; `value` drives
                    //   emission instead).
                    let (value, expression_span) = if sd.shorthand {
                        (StyleDirectiveValue::Expression, sd.name_span)
                    } else {
                        match &sd.value {
                            token::AttributeValue::ExpressionTag(et) => {
                                (StyleDirectiveValue::Expression, et.expression_span)
                            }
                            token::AttributeValue::String(span) => (
                                StyleDirectiveValue::String(
                                    span.source_text(self.source).to_string(),
                                ),
                                *span,
                            ),
                            token::AttributeValue::Concatenation(c) => {
                                let span = c.span;
                                (
                                    StyleDirectiveValue::Concatenation(
                                        self.convert_concat_parts(&c.parts),
                                    ),
                                    span,
                                )
                            }
                            token::AttributeValue::Empty => {
                                // Only expected on shorthand; hit the same
                                // fallback (synthesize identifier on `name_span`).
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
                        expression_span,
                        shorthand: sd.shorthand,
                        value,
                        important: sd.important,
                    }));
                }
                token::Attribute::BindDirective(bd) => {
                    // BindDirective shares the "attr" key space with HTMLAttribute.
                    let bd_name = bd.name_span.source_text(self.source);
                    track_duplicate(
                        &mut seen,
                        ("attr", bd_name),
                        bd.name_span,
                        &mut self.diagnostics,
                        true,
                    );
                    // Shorthand `bind:name` scanner already sets expression_span
                    // to name_span so walk_js.rs::parse_span synthesizes an
                    // `Expression::Identifier` for that range, giving bind a
                    // single expression path regardless of shorthand form.
                    attributes.push(Attribute::BindDirective(BindDirective {
                        id: attr_id,
                        span: attr_span,
                        name: bd_name.to_string(),
                        expression_span: bd.expression_span,
                        shorthand: bd.shorthand,
                    }));
                }
                token::Attribute::LetDirectiveLegacy(ld) => {
                    attributes.push(Attribute::LetDirectiveLegacy(LetDirectiveLegacy {
                        id: attr_id,
                        span: attr_span,
                        name: ld.name_span.source_text(self.source).to_string(),
                        name_span: ld.name_span,
                        expression_span: ld.has_expression.then_some(ld.expression_span),
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
                        name: ud.name_span,
                        expression_span,
                    }));
                }
                // LEGACY(svelte4): on:directive
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
                        expression_span,
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
                    let direction = match td.direction_prefix.as_str() {
                        "in" => TransitionDirection::In,
                        "out" => TransitionDirection::Out,
                        _ => TransitionDirection::Both,
                    };
                    attributes.push(Attribute::TransitionDirective(TransitionDirective {
                        id: attr_id,
                        span: attr_span,
                        name: td.name_span,
                        expression_span,
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
                        name: ad.name_span,
                        expression_span,
                    }));
                }
                token::Attribute::AttachTag(at) => {
                    attributes.push(Attribute::AttachTag(svelte_ast::AttachTag {
                        id: attr_id,
                        span: attr_span,
                        expression_span: at.expression_span,
                    }));
                }
            }
        }

        attributes
    }

    /// Extract the `this` attribute from an attribute list, returning its expression span.
    /// Removes the `this` attribute from the vec.
    /// Returns (tag_span, is_static) — is_static is true for `this="literal"`.
    pub(crate) fn extract_this_attribute(
        attributes: &mut Vec<svelte_ast::Attribute>,
    ) -> (Span, bool) {
        let pos = attributes.iter().position(|attr| match attr {
            svelte_ast::Attribute::ExpressionAttribute(a) => a.name == "this",
            svelte_ast::Attribute::StringAttribute(a) => a.name == "this",
            _ => false,
        });

        if let Some(idx) = pos {
            let attr = attributes.remove(idx);
            match attr {
                svelte_ast::Attribute::ExpressionAttribute(a) => (a.expression_span, false),
                svelte_ast::Attribute::StringAttribute(a) => (a.value_span, true),
                _ => unreachable!(),
            }
        } else {
            // Missing `this` attribute — use empty span as fallback
            (Span::new(0, 0), false)
        }
    }

    fn convert_concat_parts(&mut self, parts: &[token::ConcatenationPart]) -> Vec<ConcatPart> {
        parts
            .iter()
            .map(|part| match part {
                token::ConcatenationPart::String(span) => {
                    ConcatPart::Static(span.source_text(self.source).to_string())
                }
                token::ConcatenationPart::Expression(et) => ConcatPart::Dynamic {
                    id: self.reserve_id(),
                    span: et.expression_span,
                },
            })
            .collect()
    }
}
