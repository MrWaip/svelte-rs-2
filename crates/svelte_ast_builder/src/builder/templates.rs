use std::borrow::Cow;

use super::*;

fn escape_template_raw(value: &str) -> Cow<'_, str> {
    let needs_escape = value.as_bytes().windows(2).any(|w| w == b"${")
        || value.bytes().any(|b| b == b'`' || b == b'\\');
    if !needs_escape {
        return Cow::Borrowed(value);
    }
    let mut out: Vec<u8> = Vec::with_capacity(value.len() + 4);
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' {
            out.extend_from_slice(b"\\\\");
            i += 1;
        } else if b == b'`' {
            out.extend_from_slice(b"\\`");
            i += 1;
        } else if b == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
            out.extend_from_slice(b"\\${");
            i += 2;
        } else {
            out.push(b);
            i += 1;
        }
    }
    Cow::Owned(String::from_utf8(out).expect("ascii-only edits preserve utf-8"))
}

impl<'a> Builder<'a> {
    pub fn template_str_expr(&self, value: &str) -> Expression<'a> {
        Expression::TemplateLiteral(self.alloc(self.template_str(value)))
    }

    pub fn template_str(&self, value: &str) -> TemplateLiteral<'a> {
        let mut quasis = self.ast.vec();
        let escaped = escape_template_raw(value);
        quasis.push(self.ast.template_element(
            SPAN,
            TemplateElementValue {
                cooked: None,
                raw: self.ast.atom(&escaped),
            },
            true,
            false,
        ));
        self.ast.template_literal(SPAN, quasis, self.ast.vec())
    }

    pub fn template_parts(
        &self,
        parts: impl IntoIterator<Item = TemplatePart<'a>>,
    ) -> TemplateLiteral<'a> {
        let mut quasis = self.ast.vec();
        let mut expressions = self.ast.vec();
        let parts: Vec<_> = parts.into_iter().collect();
        let n = parts.len();

        if n == 0 {
            quasis.push(self.ast.template_element(
                SPAN,
                TemplateElementValue {
                    cooked: None,
                    raw: Atom::from(""),
                },
                true,
                false,
            ));
            return self.ast.template_literal(SPAN, quasis, expressions);
        }

        let mut pending = String::new();

        for (i, part) in parts.into_iter().enumerate() {
            let is_last = i == n - 1;
            match part {
                TemplatePart::Str(s) => {
                    pending.push_str(&s);
                    if is_last {
                        let escaped = escape_template_raw(&pending);
                        quasis.push(self.ast.template_element(
                            SPAN,
                            TemplateElementValue {
                                cooked: None,
                                raw: self.ast.atom(&escaped),
                            },
                            true,
                            false,
                        ));
                    }
                }
                TemplatePart::Expr(expr, defined) => {
                    let escaped = escape_template_raw(&pending);
                    quasis.push(self.ast.template_element(
                        SPAN,
                        TemplateElementValue {
                            cooked: None,
                            raw: self.ast.atom(&escaped),
                        },
                        false,
                        false,
                    ));
                    pending.clear();

                    let value = if defined {
                        expr
                    } else {
                        let empty_str = self.ast.atom("");
                        self.ast.expression_logical(
                            SPAN,
                            expr,
                            ast::LogicalOperator::Coalesce,
                            self.ast.expression_string_literal(SPAN, empty_str, None),
                        )
                    };
                    expressions.push(value);

                    if is_last {
                        quasis.push(self.ast.template_element(
                            SPAN,
                            TemplateElementValue {
                                cooked: None,
                                raw: Atom::from(""),
                            },
                            true,
                            false,
                        ));
                    }
                }
            }
        }

        self.ast.template_literal(SPAN, quasis, expressions)
    }

    pub fn template_parts_expr(
        &self,
        parts: impl IntoIterator<Item = TemplatePart<'a>>,
    ) -> Expression<'a> {
        Expression::TemplateLiteral(self.alloc(self.template_parts(parts)))
    }
}
