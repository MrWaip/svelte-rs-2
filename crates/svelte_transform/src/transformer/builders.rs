use oxc_ast::NONE;
use oxc_ast::ast::{Argument, Expression, NumberBase};
use oxc_span::SPAN;
use oxc_syntax::operator::AssignmentOperator;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn make_rune_get(&self, name: &str) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("get");
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
        ast.expression_call(SPAN, callee, NONE, ast.vec1(name_arg), false)
    }

    pub(crate) fn make_rune_safe_get(&self, name: &str) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("safe_get");
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
        ast.expression_call(SPAN, callee, NONE, ast.vec1(name_arg), false)
    }

    pub(crate) fn make_rune_set(
        &self,
        name: &str,
        value: Expression<'a>,
        proxy: bool,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("set");
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
        let value_arg = Argument::from(value);
        if proxy {
            let true_arg = Argument::from(ast.expression_boolean_literal(SPAN, true));
            ast.expression_call(
                SPAN,
                callee,
                NONE,
                ast.vec_from_array([name_arg, value_arg, true_arg]),
                false,
            )
        } else {
            ast.expression_call(
                SPAN,
                callee,
                NONE,
                ast.vec_from_array([name_arg, value_arg]),
                false,
            )
        }
    }

    pub(crate) fn make_rune_update(
        &self,
        name: &str,
        is_prefix: bool,
        is_increment: bool,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let fn_name = if is_prefix { "update_pre" } else { "update" };
        let callee = self.make_dollar_member(fn_name);
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));

        let args = if is_increment {
            ast.vec1(name_arg)
        } else {
            let delta = Argument::from(ast.expression_numeric_literal(
                SPAN,
                -1.0,
                None,
                NumberBase::Decimal,
            ));
            ast.vec_from_array([name_arg, delta])
        };

        ast.expression_call(SPAN, callee, NONE, args, false)
    }

    pub(crate) fn make_thunk_call(&self, name: &str) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = ast.expression_identifier(SPAN, ast.atom(name));
        ast.expression_call(SPAN, callee, NONE, ast.vec(), false)
    }

    pub(crate) fn make_member_get(&self, signal_name: &str, prop: &str) -> Expression<'a> {
        let ast = self.b.ast;
        let get_call = self.make_rune_get(signal_name);
        let property = ast.identifier_name(SPAN, ast.atom(prop));
        Expression::StaticMemberExpression(
            ast.alloc(ast.static_member_expression(SPAN, get_call, property, false)),
        )
    }

    pub(crate) fn make_props_access(&self, prop_name: &str) -> Expression<'a> {
        let ast = self.b.ast;
        let object = ast.expression_identifier(SPAN, ast.atom("$$props"));
        let property = ast.identifier_name(SPAN, ast.atom(prop_name));
        Expression::StaticMemberExpression(
            ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
        )
    }

    pub(crate) fn make_eager_thunk(&self, arg: Expression<'a>) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("eager");
        let thunked = self.b.thunk(arg);
        ast.expression_call(SPAN, callee, NONE, ast.vec1(Argument::from(thunked)), false)
    }

    pub(crate) fn make_eager_pending(&self) -> Expression<'a> {
        let ast = self.b.ast;
        let eager_callee = self.make_dollar_member("eager");
        let pending_ref = self.make_dollar_member("pending");
        ast.expression_call(
            SPAN,
            eager_callee,
            NONE,
            ast.vec1(Argument::from(pending_ref)),
            false,
        )
    }

    pub(crate) fn make_store_set(&self, base_name: &str, value: Expression<'a>) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("store_set");
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(base_name)));
        let value_arg = Argument::from(value);
        ast.expression_call(
            SPAN,
            callee,
            NONE,
            ast.vec_from_array([name_arg, value_arg]),
            false,
        )
    }

    pub(crate) fn make_store_update(
        &self,
        base_name: &str,
        dollar_name: &str,
        is_prefix: bool,
        is_increment: bool,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let fn_name = if is_prefix {
            "update_pre_store"
        } else {
            "update_store"
        };
        let callee = self.make_dollar_member(fn_name);
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(base_name)));
        let thunk_call = self.make_thunk_call(dollar_name);
        let thunk_arg = Argument::from(thunk_call);

        let args = if is_increment {
            ast.vec_from_array([name_arg, thunk_arg])
        } else {
            let delta = Argument::from(ast.expression_numeric_literal(
                SPAN,
                -1.0,
                None,
                NumberBase::Decimal,
            ));
            ast.vec_from_array([name_arg, thunk_arg, delta])
        };

        ast.expression_call(SPAN, callee, NONE, args, false)
    }

    pub(crate) fn make_untrack(&self, dollar_name: &str) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("untrack");
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(dollar_name)));
        ast.expression_call(SPAN, callee, NONE, ast.vec1(name_arg), false)
    }

    pub(crate) fn make_legacy_state_mutate(
        &self,
        name: &str,
        mutation: Expression<'a>,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("mutate");
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
        let mutation_arg = Argument::from(mutation);
        ast.expression_call(
            SPAN,
            callee,
            NONE,
            ast.vec_from_array([name_arg, mutation_arg]),
            false,
        )
    }

    pub(crate) fn make_store_mutate(
        &self,
        base_name: &str,
        mutation: Expression<'a>,
        untracked: Expression<'a>,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("store_mutate");
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(base_name)));
        let mutation_arg = Argument::from(mutation);
        let untracked_arg = Argument::from(untracked);
        ast.expression_call(
            SPAN,
            callee,
            NONE,
            ast.vec_from_array([name_arg, mutation_arg, untracked_arg]),
            false,
        )
    }

    pub(crate) fn build_compound_value(
        &self,
        operator: AssignmentOperator,
        left_read: Expression<'a>,
        right: Expression<'a>,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        if operator.is_assign() {
            return right;
        }
        if let Some(bin_op) = operator.to_binary_operator() {
            ast.expression_binary(SPAN, left_read, bin_op, right)
        } else if let Some(log_op) = operator.to_logical_operator() {
            ast.expression_logical(SPAN, left_read, log_op, right)
        } else {
            unreachable!("all compound assignment operators are either binary or logical")
        }
    }

    pub(crate) fn make_dollar_member(&self, method: &str) -> Expression<'a> {
        let ast = self.b.ast;
        let object = ast.expression_identifier(SPAN, ast.atom("$"));
        let property = ast.identifier_name(SPAN, ast.atom(method));
        Expression::StaticMemberExpression(
            ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
        )
    }

    pub(crate) fn make_dollar_call(&self, method: &str, arg: Expression<'a>) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member(method);
        ast.expression_call(SPAN, callee, NONE, ast.vec1(Argument::from(arg)), false)
    }
}
