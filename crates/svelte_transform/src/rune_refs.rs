use oxc_ast::ast::*;

pub fn is_non_coercive_operator(op: AssignmentOperator) -> bool {
    matches!(
        op,
        AssignmentOperator::Assign
            | AssignmentOperator::LogicalAnd
            | AssignmentOperator::LogicalOr
            | AssignmentOperator::LogicalNullish
    )
}

pub fn should_proxy(e: &Expression) -> bool {
    let e = e.get_inner_expression();
    if e.is_literal() {
        return false;
    }
    if matches!(
        e,
        Expression::TemplateLiteral(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::BinaryExpression(_)
    ) {
        return false;
    }
    if let Expression::Identifier(id) = e
        && id.name == "undefined"
    {
        return false;
    }
    true
}

pub fn find_expr_root_name<'b>(expr: &'b Expression) -> Option<&'b str> {
    find_expr_root_identifier(expr).map(|id| id.name.as_str())
}

pub fn find_expr_root_identifier<'b, 'a>(
    expr: &'b Expression<'a>,
) -> Option<&'b IdentifierReference<'a>> {
    let mut current = expr.get_inner_expression();
    loop {
        if let Expression::Identifier(id) = current {
            return Some(id);
        }
        current = current
            .as_member_expression()?
            .object()
            .get_inner_expression();
    }
}

pub fn replace_expr_root<'a>(expr: &mut Expression<'a>, replacement: Expression<'a>) {
    let mut current = expr.get_inner_expression_mut();
    loop {
        if let Expression::Identifier(_) = current {
            *current = replacement;
            return;
        }
        match current.as_member_expression_mut() {
            Some(member) => current = member.object_mut().get_inner_expression_mut(),
            None => return,
        }
    }
}

pub fn replace_expr_root_in_assign_target<'a>(
    target: &mut AssignmentTarget<'a>,
    replacement: Expression<'a>,
) {
    if let Some(member) = target.as_member_expression_mut() {
        replace_expr_root(member.object_mut(), replacement);
    }
}

pub fn replace_expr_root_in_simple_target<'a>(
    target: &mut SimpleAssignmentTarget<'a>,
    replacement: Expression<'a>,
) {
    if let Some(member) = target.as_member_expression_mut() {
        replace_expr_root(member.object_mut(), replacement);
    }
}
