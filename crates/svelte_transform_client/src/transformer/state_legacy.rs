use std::{iter, mem};

use oxc_allocator::{CloneIn, Vec as OxcVec};
use oxc_ast::NONE;
use oxc_ast::ast::{Argument, AssignmentOperator, AssignmentTarget, Expression, Statement};
use oxc_span::{GetSpan, SPAN};
use oxc_traverse::{Ancestor, TraverseCtx};
use svelte_ast_builder::Arg;
use svelte_component_semantics::{WriteAccess, WriteStep, WriteTarget};

use super::async_check::is_expression_async;
use super::model::ComponentTransformer;
use crate::is_simple_expression;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_destructure_assignment_exit(
        &mut self,
        node: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> bool {
        let Expression::AssignmentExpression(assign_box) = node else {
            return false;
        };
        if !matches!(
            &assign_box.left,
            AssignmentTarget::ArrayAssignmentTarget(_)
                | AssignmentTarget::ObjectAssignmentTarget(_)
        ) {
            return false;
        }
        self.destructure_lhs_depth = self.destructure_lhs_depth.saturating_sub(1);
        if assign_box.operator != AssignmentOperator::Assign {
            return false;
        }

        let is_standalone = destructure_assignment_is_standalone(ctx);

        let placeholder = self.b.cheap_expr();
        let owned = mem::replace(node, placeholder);
        let Expression::AssignmentExpression(assign_box) = owned else {
            unreachable!();
        };
        let assign = assign_box.unbox();

        let should_cache = !matches!(&assign.right, Expression::Identifier(_));
        let param_name: String = match &assign.right {
            Expression::Identifier(id) if !should_cache => id.name.to_string(),
            _ => "$$value".to_string(),
        };

        let mut decls: Vec<Statement<'a>> = Vec::new();
        let mut setters: Vec<Expression<'a>> = Vec::new();
        let mut temps: Vec<(String, &'a str)> = Vec::new();
        let mut changed = false;

        svelte_component_semantics::walk_assignment_targets(&assign.left, |v| {
            let access = self.build_destructure_access(
                v.path,
                v.excluded,
                &param_name,
                &mut decls,
                &mut temps,
            );
            match v.target {
                WriteTarget::Identifier(id) => {
                    let idref = self.b.rid(id.name.as_str());
                    idref.reference_id.set(id.reference_id.get());
                    let target =
                        AssignmentTarget::AssignmentTargetIdentifier(self.b.ast.alloc(idref));
                    let mut setter = self.b.assign_expr_raw(target, access);
                    if self.dispatch_identifier_assignment(&mut setter, ctx) {
                        changed = true;
                    }
                    setters.push(setter);
                }
                WriteTarget::Member(m) => {
                    let mut cloned = m.clone_in(self.b.ast.allocator);
                    copy_member_root_ref(&mut cloned, m);
                    let mut setter = self.b.assign_expr_raw(cloned, access);
                    if self.dispatch_member_assignment(&mut setter, is_standalone, ctx) {
                        changed = true;
                    }
                    setters.push(setter);
                }
            }
        });

        if !changed {
            *node = Expression::AssignmentExpression(self.b.ast.alloc(assign));
            return true;
        }

        let rhs = assign.right;
        let use_iife = !decls.is_empty() || should_cache;
        let is_async = is_expression_async(&rhs) || setters.iter().any(|s| is_expression_async(s));

        if use_iife {
            let param: &'a str = self.b.alloc_str(&param_name);
            let mut body: Vec<Statement<'a>> = Vec::with_capacity(decls.len() + setters.len() + 1);
            body.extend(decls);
            body.extend(setters.into_iter().map(|e| self.b.expr_stmt(e)));
            if !is_standalone {
                body.push(self.b.return_stmt(self.b.rid_expr(param)));
            }
            let body_fn =
                self.b
                    .ast
                    .function_body(SPAN, self.b.ast.vec(), self.b.ast.vec_from_iter(body));
            let arrow = Expression::ArrowFunctionExpression(self.b.ast.alloc(
                self.b.ast.arrow_function_expression(
                    SPAN,
                    false,
                    is_async,
                    NONE,
                    self.b.params([param]),
                    NONE,
                    body_fn,
                ),
            ));
            let call = self.b.ast.expression_call(
                SPAN,
                arrow,
                NONE,
                self.b.ast.vec_from_iter(iter::once(Argument::from(rhs))),
                false,
            );
            *node = if is_async {
                self.b.await_expr(call)
            } else {
                call
            };
        } else {
            debug_assert!(decls.is_empty());
            let mut seq: OxcVec<'a, Expression<'a>> =
                OxcVec::with_capacity_in(setters.len() + 1, &self.b.ast.allocator);
            for setter in setters {
                seq.push(setter);
            }
            if !is_standalone {
                seq.push(rhs);
            }
            *node = self.b.ast.expression_sequence(SPAN, seq);
        }
        true
    }

    fn build_destructure_access<'w>(
        &mut self,
        path: &[WriteStep<'w>],
        excluded: &[&'w str],
        param_name: &str,
        decls: &mut Vec<Statement<'a>>,
        temps: &mut Vec<(String, &'a str)>,
    ) -> Expression<'a> {
        let mut current = self.b.rid_expr(param_name);
        for (i, step) in path.iter().enumerate() {
            match step.access {
                WriteAccess::Index {
                    index,
                    len,
                    has_rest,
                } => {
                    let temp = self.array_temp(&path[..i], current, has_rest, len, decls, temps);
                    current = self
                        .b
                        .computed_member_expr(self.b.rid_expr(temp), self.b.num_expr(index as f64));
                }
                WriteAccess::Slice { from } => {
                    let temp = self.array_temp(&path[..i], current, true, 0, decls, temps);
                    current = self.b.call_expr_callee(
                        self.b.static_member_expr(self.b.rid_expr(temp), "slice"),
                        [Arg::Num(from as f64)],
                    );
                }
                WriteAccess::Key { name } => {
                    let name: &'a str = self.b.alloc_str(name);
                    current = self.b.static_member_expr(current, name);
                }
                WriteAccess::Computed { key } => {
                    let key = key.clone_in(self.b.ast.allocator);
                    current = self.b.computed_member_expr(current, key);
                }
            }
            if let Some(default) = step.default {
                current = self.build_destructure_fallback(current, default);
            }
        }
        if !excluded.is_empty() {
            let keys = self
                .b
                .array_from_args(excluded.iter().map(|k| Arg::StrRef(k)));
            current = self.b.call_expr(
                "$.exclude_from_object",
                [Arg::Expr(current), Arg::Expr(keys)],
            );
        }
        current
    }

    fn array_temp<'w>(
        &mut self,
        prefix: &[WriteStep<'w>],
        source: Expression<'a>,
        has_rest: bool,
        len: u32,
        decls: &mut Vec<Statement<'a>>,
        temps: &mut Vec<(String, &'a str)>,
    ) -> &'a str {
        let key = serialize_prefix(prefix);
        if let Some((_, name)) = temps.iter().find(|(k, _)| *k == key) {
            return name;
        }
        let owned = self.ident_gen.generate("$$array");
        let name: &'a str = self.b.alloc_str(&owned);
        let to_array = if has_rest {
            self.b.call_expr("$.to_array", [Arg::Expr(source)])
        } else {
            self.b
                .call_expr("$.to_array", [Arg::Expr(source), Arg::Num(len as f64)])
        };
        decls.push(self.b.var_stmt(name, to_array));
        temps.push((key, name));
        name
    }

    fn build_destructure_fallback<'w>(
        &mut self,
        expr: Expression<'a>,
        fallback: &Expression<'w>,
    ) -> Expression<'a> {
        if fallback_is_simple(fallback) {
            let fallback = fallback.clone_in(self.b.ast.allocator);
            return self
                .b
                .call_expr("$.fallback", [Arg::Expr(expr), Arg::Expr(fallback)]);
        }
        if let Some(inner) = extract_dev_tracked_await(fallback) {
            let call = self.build_await_fallback_call(expr, inner);
            return self.await_fallback_tracked(call);
        }
        if let Expression::AwaitExpression(aw) = fallback {
            let call = self.build_await_fallback_call(expr, &aw.argument);
            return self.await_fallback_tracked(call);
        }
        if is_expression_async(fallback) {
            let thunk = self
                .b
                .async_arrow_expr_body(fallback.clone_in(self.b.ast.allocator));
            let call = self.b.call_expr(
                "$.fallback",
                [Arg::Expr(expr), Arg::Expr(thunk), Arg::Bool(true)],
            );
            return self.await_fallback_tracked(call);
        }
        let thunk = self.b.thunk(fallback.clone_in(self.b.ast.allocator));
        self.b.call_expr(
            "$.fallback",
            [Arg::Expr(expr), Arg::Expr(thunk), Arg::Bool(true)],
        )
    }

    fn await_fallback_tracked(&self, call: Expression<'a>) -> Expression<'a> {
        if !self.dev {
            return self.b.await_expr(call);
        }
        let track_call = self
            .b
            .call_expr("$.track_reactivity_loss", [Arg::Expr(call)]);
        let awaited = self.b.await_expr(track_call);
        self.b
            .call_expr_callee(awaited, iter::empty::<Arg<'a, '_>>())
    }

    fn build_await_fallback_call<'w>(
        &mut self,
        expr: Expression<'a>,
        arg: &Expression<'w>,
    ) -> Expression<'a> {
        if fallback_is_simple(arg) {
            let arg = arg.clone_in(self.b.ast.allocator);
            return self
                .b
                .call_expr("$.fallback", [Arg::Expr(expr), Arg::Expr(arg)]);
        }
        let thunk = self.b.thunk(arg.clone_in(self.b.ast.allocator));
        self.b.call_expr(
            "$.fallback",
            [Arg::Expr(expr), Arg::Expr(thunk), Arg::Bool(true)],
        )
    }
}

fn extract_dev_tracked_await<'x, 'w>(expr: &'x Expression<'w>) -> Option<&'x Expression<'w>> {
    let Expression::CallExpression(call) = expr else {
        return None;
    };
    if !call.arguments.is_empty() {
        return None;
    }
    let Expression::AwaitExpression(aw) = call.callee.get_inner_expression() else {
        return None;
    };
    let Expression::CallExpression(track) = aw.argument.get_inner_expression() else {
        return None;
    };
    let Expression::Identifier(callee) = track.callee.get_inner_expression() else {
        return None;
    };
    if callee.name != "$.track_reactivity_loss" {
        return None;
    }
    track.arguments.first().and_then(|arg| arg.as_expression())
}

pub(crate) fn is_destructure_assignment_lhs(node: &Expression<'_>) -> bool {
    matches!(
        node,
        Expression::AssignmentExpression(assign)
            if matches!(
                assign.left,
                AssignmentTarget::ArrayAssignmentTarget(_)
                    | AssignmentTarget::ObjectAssignmentTarget(_)
            )
    )
}

fn destructure_assignment_is_standalone(ctx: &TraverseCtx<'_, ()>) -> bool {
    let mut ancestors = ctx
        .ancestors()
        .filter(|a| !matches!(a, Ancestor::ParenthesizedExpressionExpression(_)));
    let Some(first) = ancestors.next() else {
        return false;
    };
    if !ancestor_is_statement(first) {
        return false;
    }
    if !first.is_expression_statement() {
        return true;
    }
    match ancestors.next() {
        Some(Ancestor::FunctionBodyStatements(_)) => match ancestors.next() {
            Some(Ancestor::ArrowFunctionExpressionBody(arrow)) => !*arrow.expression(),
            _ => true,
        },
        _ => true,
    }
}

fn ancestor_is_statement(a: Ancestor<'_, '_>) -> bool {
    a.is_expression_statement()
        || a.is_for_statement()
        || a.is_for_in_statement()
        || a.is_for_of_statement()
        || a.is_if_statement()
        || a.is_while_statement()
        || a.is_do_while_statement()
        || a.is_return_statement()
        || a.is_throw_statement()
        || a.is_switch_statement()
        || a.is_with_statement()
        || a.is_labeled_statement()
        || a.is_block_statement()
        || a.is_try_statement()
}

fn fallback_is_simple(expr: &Expression<'_>) -> bool {
    if is_simple_expression(expr) {
        return true;
    }
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
        return false;
    };
    if call.arguments.len() != 1 {
        return false;
    }
    let Expression::StaticMemberExpression(member) = call.callee.get_inner_expression() else {
        return false;
    };
    let Expression::Identifier(object) = member.object.get_inner_expression() else {
        return false;
    };
    if object.name != "$" || member.property.name != "get" {
        return false;
    }
    call.arguments[0]
        .as_expression()
        .is_some_and(is_simple_expression)
}

fn serialize_prefix(prefix: &[WriteStep<'_>]) -> String {
    let mut out = String::new();
    for step in prefix {
        match step.access {
            WriteAccess::Index { index, .. } => {
                out.push('i');
                out.push_str(&index.to_string());
            }
            WriteAccess::Slice { from } => {
                out.push('s');
                out.push_str(&from.to_string());
            }
            WriteAccess::Key { name } => {
                out.push('k');
                out.push_str(name);
            }
            WriteAccess::Computed { key } => {
                let span = key.span();
                out.push('c');
                out.push_str(&span.start.to_string());
                out.push('_');
                out.push_str(&span.end.to_string());
            }
        }
        out.push('/');
    }
    out
}

fn copy_member_root_ref<'x, 'y>(cloned: &mut AssignmentTarget<'x>, orig: &AssignmentTarget<'y>) {
    let (cloned_obj, orig_obj) = match (cloned, orig) {
        (
            AssignmentTarget::StaticMemberExpression(c),
            AssignmentTarget::StaticMemberExpression(o),
        ) => (&mut c.object, &o.object),
        (
            AssignmentTarget::ComputedMemberExpression(c),
            AssignmentTarget::ComputedMemberExpression(o),
        ) => (&mut c.object, &o.object),
        _ => return,
    };
    copy_root_ref_expr(cloned_obj, orig_obj);
}

fn copy_root_ref_expr<'x, 'y>(cloned: &mut Expression<'x>, orig: &Expression<'y>) {
    match (cloned, orig) {
        (Expression::Identifier(c), Expression::Identifier(o)) => {
            c.reference_id.set(o.reference_id.get());
        }
        (Expression::StaticMemberExpression(c), Expression::StaticMemberExpression(o)) => {
            copy_root_ref_expr(&mut c.object, &o.object)
        }
        (Expression::ComputedMemberExpression(c), Expression::ComputedMemberExpression(o)) => {
            copy_root_ref_expr(&mut c.object, &o.object)
        }
        _ => {}
    }
}
