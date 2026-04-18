use super::*;

impl<'a> Builder<'a> {
    pub fn template_str_expr(&self, value: &str) -> Expression<'a> {
        Expression::TemplateLiteral(self.alloc(self.template_str(value)))
    }

    pub fn template_str(&self, value: &str) -> TemplateLiteral<'a> {
        let mut quasis = self.ast.vec();
        quasis.push(self.ast.template_element(
            SPAN,
            TemplateElementValue {
                cooked: None,
                raw: self.ast.atom(value),
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
                        quasis.push(self.ast.template_element(
                            SPAN,
                            TemplateElementValue {
                                cooked: None,
                                raw: self.ast.atom(&pending),
                            },
                            true,
                            false,
                        ));
                    }
                }
                TemplatePart::Expr(expr, defined) => {
                    quasis.push(self.ast.template_element(
                        SPAN,
                        TemplateElementValue {
                            cooked: None,
                            raw: self.ast.atom(&pending),
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
