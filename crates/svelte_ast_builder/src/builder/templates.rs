use std::borrow::Cow;

use super::*;

fn escape_template_raw(value: &str) -> Cow<'_, str> {
    let bytes = value.as_bytes();
    let first = {
        let mut from = 0;
        loop {
            match memchr::memchr3(b'\\', b'`', b'$', &bytes[from..]) {
                None => return Cow::Borrowed(value),
                Some(rel) => {
                    let pos = from + rel;
                    if bytes[pos] != b'$' || bytes.get(pos + 1) == Some(&b'{') {
                        break pos;
                    }
                    from = pos + 1;
                }
            }
        }
    };

    let mut out = String::with_capacity(value.len() + 8);
    out.push_str(&value[..first]);
    let mut i = first;
    while i < bytes.len() {
        match memchr::memchr3(b'\\', b'`', b'$', &bytes[i..]) {
            None => {
                out.push_str(&value[i..]);
                break;
            }
            Some(rel) => {
                let pos = i + rel;
                out.push_str(&value[i..pos]);
                match bytes[pos] {
                    b'\\' => {
                        out.push_str("\\\\");
                        i = pos + 1;
                    }
                    b'`' => {
                        out.push_str("\\`");
                        i = pos + 1;
                    }
                    _ if bytes.get(pos + 1) == Some(&b'{') => {
                        out.push_str("\\${");
                        i = pos + 2;
                    }
                    _ => {
                        out.push('$');
                        i = pos + 1;
                    }
                }
            }
        }
    }
    Cow::Owned(out)
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
