use super::*;

impl<'a> Builder<'a> {
    pub fn directive_prop(&self, name: &str, expr: Expression<'a>, same_name: bool) -> ObjProp<'a> {
        let name_alloc = self.alloc_str(name);
        if same_name && expr.is_identifier_reference() {
            ObjProp::Shorthand(name_alloc)
        } else {
            ObjProp::KeyValue(name_alloc, expr)
        }
    }

    pub fn bind_property_call_stmt(
        &self,
        property_name: &'static str,
        event_name: &'static str,
        target_ident: &str,
        setter: Expression<'a>,
        getter: Option<Expression<'a>>,
    ) -> Statement<'a> {
        let mut args = vec![
            Arg::StrRef(property_name),
            Arg::StrRef(event_name),
            Arg::Ident(target_ident),
            Arg::Expr(setter),
        ];
        if let Some(getter) = getter {
            args.push(Arg::Expr(getter));
        }
        self.call_stmt("$.bind_property", args)
    }

    pub fn bind_this_getter_arrow(&self, var_name: &str, is_rune: bool) -> Expression<'a> {
        let body = if is_rune {
            self.call_expr("$.get", [Arg::Ident(var_name)])
        } else {
            self.rid_expr(var_name)
        };
        self.arrow_expr(self.no_params(), [self.expr_stmt(body)])
    }

    pub fn bind_this_setter_arrow(
        &self,
        var_name: &str,
        is_rune: bool,
        set_marker: bool,
    ) -> Expression<'a> {
        let body = if is_rune {
            let mut args: Vec<Arg<'_, '_>> = vec![Arg::Ident(var_name), Arg::Ident("$$value")];
            if set_marker {
                args.push(Arg::Expr(self.bool_expr(true)));
            }
            self.call_expr("$.set", args)
        } else {
            self.assign_expr(
                AssignLeft::Ident(var_name.to_string()),
                self.rid_expr("$$value"),
            )
        };
        self.arrow_expr(self.params(["$$value"]), [self.expr_stmt(body)])
    }

    pub fn rewrap_arrow_body(&self, expr: Expression<'a>) -> Expression<'a> {
        let Expression::ArrowFunctionExpression(arrow) = expr else {
            return expr;
        };
        let arrow = arrow.unbox();
        let body_stmts: Vec<_> = arrow.body.unbox().statements.into_iter().collect();
        self.arrow_expr(self.no_params(), body_stmts)
    }

    pub fn rewrap_arrow_body_with_first_param(&self, expr: Expression<'a>) -> Expression<'a> {
        let Expression::ArrowFunctionExpression(arrow) = expr else {
            return expr;
        };
        let arrow = arrow.unbox();
        let body_stmts: Vec<_> = arrow.body.unbox().statements.into_iter().collect();
        let first_param = arrow.params.unbox().items.into_iter().next();
        let params = if let Some(param) = first_param {
            let mut items = self.ast.vec();
            items.push(param);
            self.ast.formal_parameters(
                SPAN,
                ast::FormalParameterKind::ArrowFormalParameters,
                items,
                NONE,
            )
        } else {
            self.params(["_"])
        };
        self.arrow_expr(params, body_stmts)
    }

    pub fn try_extract_expression_stmt_expr(expr: Expression<'a>) -> Option<Expression<'a>> {
        let Expression::ArrowFunctionExpression(arrow) = expr else {
            return None;
        };
        let arrow = arrow.unbox();
        let mut stmts = arrow.body.unbox().statements.into_iter();
        let first = stmts.next()?;
        match first {
            Statement::ExpressionStatement(es) => Some(es.unbox().expression),
            _ => None,
        }
    }
}
