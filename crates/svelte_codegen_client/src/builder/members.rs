use super::*;

impl<'a> Builder<'a> {
    pub fn this_expr(&self) -> Expression<'a> {
        self.ast.expression_this(SPAN)
    }

    pub fn new_target_expr(&self) -> Expression<'a> {
        Expression::MetaProperty(self.alloc(self.ast.meta_property(
            SPAN,
            self.ast.identifier_name(SPAN, self.ast.atom("new")),
            self.ast.identifier_name(SPAN, self.ast.atom("target")),
        )))
    }

    pub fn computed_member(
        &self,
        object: Expression<'a>,
        property: Expression<'a>,
    ) -> ComputedMemberExpression<'a> {
        self.ast
            .computed_member_expression(SPAN, object, property, false)
    }

    pub fn static_member(&self, object: Expression<'a>, prop: &str) -> StaticMemberExpression<'a> {
        let property = self.ast.identifier_name(SPAN, self.ast.atom(prop));
        self.ast
            .static_member_expression(SPAN, object, property, false)
    }

    pub fn static_member_expr(&self, object: Expression<'a>, prop: &str) -> Expression<'a> {
        Expression::StaticMemberExpression(self.alloc(self.static_member(object, prop)))
    }

    pub fn computed_member_expr(
        &self,
        object: Expression<'a>,
        property: Expression<'a>,
    ) -> Expression<'a> {
        let cme = self
            .ast
            .computed_member_expression(SPAN, object, property, false);
        Expression::ComputedMemberExpression(self.alloc(cme))
    }

    pub fn array_expr(&self, items: impl IntoIterator<Item = Expression<'a>>) -> Expression<'a> {
        let elements = items.into_iter().map(ast::ArrayExpressionElement::from);
        Expression::ArrayExpression(
            self.alloc(
                self.ast
                    .array_expression(SPAN, self.ast.vec_from_iter(elements)),
            ),
        )
    }

    pub fn promises_array(&self, indices: &[u32]) -> Expression<'a> {
        let elements: Vec<Expression<'a>> = indices
            .iter()
            .map(|&idx| {
                self.computed_member_expr(self.rid_expr("$$promises"), self.num_expr(idx as f64))
            })
            .collect();
        self.array_expr(elements)
    }

    pub fn make_optional_chain(&self, mut expr: Expression<'a>) -> Expression<'a> {
        {
            let mut current = &mut expr;
            while let Some(member) = current.as_member_expression_mut() {
                match member {
                    ast::MemberExpression::StaticMemberExpression(sme) => sme.optional = true,
                    ast::MemberExpression::ComputedMemberExpression(cme) => cme.optional = true,
                    ast::MemberExpression::PrivateFieldExpression(pfe) => pfe.optional = true,
                }
                current = member.object_mut();
            }
        }

        match expr {
            Expression::StaticMemberExpression(sme) => Expression::ChainExpression(
                self.alloc(
                    self.ast
                        .chain_expression(SPAN, ChainElement::StaticMemberExpression(sme)),
                ),
            ),
            Expression::ComputedMemberExpression(cme) => Expression::ChainExpression(
                self.alloc(
                    self.ast
                        .chain_expression(SPAN, ChainElement::ComputedMemberExpression(cme)),
                ),
            ),
            other => other,
        }
    }
}
