use std::mem;

use oxc_ast::ast::{Argument, CallExpression, Expression, Statement};
use svelte_analyze::{DeclaratorSemantics, RuntimeRuneKind};
use svelte_ast_builder::Arg;

use crate::model::ServerTransform;

pub(crate) enum RuneStatement<'a> {
    Keep(Statement<'a>),
    Replace(Vec<Statement<'a>>),
}

impl<'a> ServerTransform<'_, 'a> {
    fn statement_holds_promise_slot(&self, stmt: &Statement<'a>) -> bool {
        self.analysis
            .blocker_data()
            .entry_location(stmt.node_id())
            .is_some()
    }

    pub(crate) fn rewrite_rune_statement(&self, stmt: Statement<'a>) -> RuneStatement<'a> {
        let Statement::ExpressionStatement(es) = &stmt else {
            return RuneStatement::Keep(stmt);
        };
        let Expression::CallExpression(call) = &es.expression else {
            return RuneStatement::Keep(stmt);
        };
        let DeclaratorSemantics::RuntimeRuneCall { kind } =
            self.analysis.declarator_semantics(call.node_id())
        else {
            return RuneStatement::Keep(stmt);
        };
        match kind {
            RuntimeRuneKind::Effect
            | RuntimeRuneKind::EffectPre
            | RuntimeRuneKind::EffectRoot
            | RuntimeRuneKind::InspectTrace => {
                if self.statement_holds_promise_slot(&stmt) {
                    let node_id = stmt.node_id();
                    let void = self.b.void_zero_expr();
                    let replacement = self.b.expr_stmt(void);
                    if let Statement::ExpressionStatement(es) = &replacement {
                        es.node_id.set(node_id);
                    }
                    return RuneStatement::Replace(vec![replacement]);
                }
                RuneStatement::Replace(Vec::new())
            }
            RuntimeRuneKind::Inspect => self.rewrite_inspect(stmt, false),
            RuntimeRuneKind::InspectWith => self.rewrite_inspect(stmt, true),
            RuntimeRuneKind::PropsId
            | RuntimeRuneKind::EffectTracking
            | RuntimeRuneKind::EffectPending
            | RuntimeRuneKind::Host
            | RuntimeRuneKind::StateSnapshot
            | RuntimeRuneKind::StateEager
            | RuntimeRuneKind::Bindable => RuneStatement::Keep(stmt),
        }
    }

    fn rewrite_inspect(&self, mut stmt: Statement<'a>, with: bool) -> RuneStatement<'a> {
        if !self.dev {
            return RuneStatement::Replace(vec![self.b.empty_stmt(), self.b.empty_stmt()]);
        }
        let Statement::ExpressionStatement(es) = &mut stmt else {
            return RuneStatement::Keep(stmt);
        };
        let Expression::CallExpression(call) = &mut es.expression else {
            return RuneStatement::Keep(stmt);
        };

        let rewritten = if with {
            let Some(rewritten) = self.rewrite_inspect_with(call) else {
                return RuneStatement::Keep(stmt);
            };
            rewritten
        } else {
            self.rewrite_inspect_log(call)
        };
        RuneStatement::Replace(vec![self.b.expr_stmt(rewritten)])
    }

    fn rewrite_inspect_with(&self, call: &mut CallExpression<'a>) -> Option<Expression<'a>> {
        if call.arguments.is_empty() {
            return None;
        }
        let Expression::StaticMemberExpression(member) = &mut call.callee else {
            return None;
        };
        let Expression::CallExpression(inner) = &mut member.object else {
            return None;
        };
        let inner_args = mem::replace(&mut inner.arguments, self.b.ast.vec());
        let mut taken = Argument::from(self.b.cheap_expr());
        mem::swap(&mut call.arguments[0], &mut taken);
        let inspector = taken.into_expression();
        let mut args = vec![Arg::Str("init".to_string())];
        args.extend(
            inner_args
                .into_iter()
                .map(|arg| Arg::Expr(arg.into_expression())),
        );
        Some(self.b.call_expr_callee(inspector, args))
    }

    fn rewrite_inspect_log(&self, call: &mut CallExpression<'a>) -> Expression<'a> {
        let call_args = mem::replace(&mut call.arguments, self.b.ast.vec());
        let mut args = vec![Arg::Str("$inspect(".to_string())];
        args.extend(
            call_args
                .into_iter()
                .map(|arg| Arg::Expr(arg.into_expression())),
        );
        args.push(Arg::Str(")".to_string()));
        self.b.call_expr("console.log", args)
    }
}
