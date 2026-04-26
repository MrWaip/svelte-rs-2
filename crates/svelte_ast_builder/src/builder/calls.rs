use super::*;

impl<'a> Builder<'a> {
    pub fn array_from_args<'short>(
        &self,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        let elements = args.into_iter().map(|a| {
            if let Arg::Spread(e) = a {
                return ast::ArrayExpressionElement::SpreadElement(
                    self.alloc(self.ast.spread_element(SPAN, e)),
                );
            }
            let expr = match a {
                Arg::Str(v) => self.str_expr(&v),
                Arg::StrRef(v) => self.str_expr(v),
                Arg::Num(v) => self.num_expr(v),
                Arg::Ident(v) => self.rid_expr(v),
                Arg::Expr(e) => e,
                Arg::Arrow(a) => Expression::ArrowFunctionExpression(self.alloc(a)),
                Arg::Bool(v) => self.bool_expr(v),
                Arg::Spread(_) => unreachable!(),
            };
            ast::ArrayExpressionElement::from(expr)
        });
        Expression::ArrayExpression(
            self.alloc(
                self.ast
                    .array_expression(SPAN, self.ast.vec_from_iter(elements)),
            ),
        )
    }

    pub fn call<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> CallExpression<'a> {
        let ident = self.ast.expression_identifier(SPAN, self.ast.atom(callee));
        let args = args.into_iter().map(|a| self.arg_to_argument(a));
        self.ast
            .call_expression(SPAN, ident, NONE, self.ast.vec_from_iter(args), false)
    }

    pub fn call_expr<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        Expression::CallExpression(self.alloc(self.call(callee, args)))
    }

    pub fn call_stmt<'short>(
        &self,
        callee: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Statement<'a> {
        let call = self.call(callee, args);
        self.expr_stmt(Expression::CallExpression(self.alloc(call)))
    }

    pub fn call_expr_callee<'short>(
        &self,
        callee: Expression<'a>,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        let args = args.into_iter().map(|a| self.arg_to_argument(a));
        Expression::CallExpression(self.alloc(self.ast.call_expression(
            SPAN,
            callee,
            NONE,
            self.ast.vec_from_iter(args),
            false,
        )))
    }

    pub fn maybe_call_expr<'short>(
        &self,
        callee: Expression<'a>,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        let args = args.into_iter().map(|a| self.arg_to_argument(a));
        let call = self
            .ast
            .call_expression(SPAN, callee, NONE, self.ast.vec_from_iter(args), true);
        Expression::ChainExpression(
            self.alloc(
                self.ast
                    .chain_expression(SPAN, ChainElement::CallExpression(self.alloc(call))),
            ),
        )
    }

    pub fn optional_member_call_expr<'short>(
        &self,
        object: Expression<'a>,
        member: &str,
        args: impl IntoIterator<Item = Arg<'a, 'short>>,
    ) -> Expression<'a> {
        let property = self.ast.identifier_name(SPAN, self.ast.atom(member));
        let callee_inner = self
            .ast
            .static_member_expression(SPAN, object, property, true);
        let callee = Expression::StaticMemberExpression(self.alloc(callee_inner));
        let args = args.into_iter().map(|a| self.arg_to_argument(a));
        let call =
            self.ast
                .call_expression(SPAN, callee, NONE, self.ast.vec_from_iter(args), false);
        Expression::ChainExpression(
            self.alloc(
                self.ast
                    .chain_expression(SPAN, ChainElement::CallExpression(self.alloc(call))),
            ),
        )
    }

    pub(super) fn arg_to_argument<'short>(&self, arg: Arg<'a, 'short>) -> Argument<'a> {
        match arg {
            Arg::Str(v) => Argument::StringLiteral(self.alloc(self.str_lit(&v))),
            Arg::StrRef(v) => Argument::StringLiteral(self.alloc(self.str_lit(v))),
            Arg::Num(v) => Argument::NumericLiteral(self.alloc(self.num(v))),
            Arg::Ident(v) => Argument::Identifier(self.alloc(self.rid(v))),
            Arg::Expr(e) => Argument::from(e),
            Arg::Arrow(a) => Argument::ArrowFunctionExpression(self.alloc(a)),
            Arg::Bool(v) => Argument::BooleanLiteral(self.alloc(self.ast.boolean_literal(SPAN, v))),
            Arg::Spread(e) => Argument::SpreadElement(self.alloc(self.ast.spread_element(SPAN, e))),
        }
    }
}
