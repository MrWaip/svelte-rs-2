use std::{iter, mem};

use oxc_allocator::{CloneIn, Vec as OxcVec};
use oxc_ast::NONE;
use oxc_ast::ast::{Argument, AssignmentOperator, AssignmentTarget, Expression, Statement};
use oxc_ast_visit::{VisitMut, walk_mut};
use oxc_span::{GetSpan, SPAN};
use svelte_ast_builder::Arg;
use svelte_component_semantics::{WriteAccess, WriteStep, WriteTarget, walk_assignment_targets};
use svelte_emit_builders::binding_pattern as bp;

use crate::model::ServerTransform;

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_store_destructure_assignment(
        &mut self,
        node: &mut Expression<'a>,
        is_standalone: bool,
    ) -> bool {
        {
            let Expression::AssignmentExpression(assign) = &*node else {
                return false;
            };
            if !matches!(
                &assign.left,
                AssignmentTarget::ArrayAssignmentTarget(_)
                    | AssignmentTarget::ObjectAssignmentTarget(_)
            ) {
                return false;
            }
            if assign.operator != AssignmentOperator::Assign {
                return false;
            }
        }

        {
            let Expression::AssignmentExpression(assign) = &mut *node else {
                unreachable!()
            };
            self.visit_expression(&mut assign.right);
        }

        let placeholder = self.b.cheap_expr();
        let owned = mem::replace(node, placeholder);
        let Expression::AssignmentExpression(assign_box) = owned else {
            unreachable!()
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

        walk_assignment_targets(&assign.left, |v| {
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
                    if self.rewrite_store_assignment(&mut setter) {
                        changed = true;
                    }
                    setters.push(setter);
                }
                WriteTarget::Member(m) => {
                    let mut cloned = m.clone_in(self.b.ast.allocator);
                    copy_member_root_ref(&mut cloned, m);
                    let setter = self.b.assign_expr_raw(cloned, access);
                    setters.push(setter);
                }
            }
        });

        if !changed {
            *node = Expression::AssignmentExpression(self.b.ast.alloc(assign));
            if let Expression::AssignmentExpression(restored) = node {
                walk_mut::walk_assignment_target(self, &mut restored.left);
            }
            return true;
        }

        let rhs = assign.right;
        let use_iife = !decls.is_empty() || should_cache;

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
                    false,
                    NONE,
                    self.b.params([param]),
                    NONE,
                    body_fn,
                ),
            ));
            *node = self.b.ast.expression_call(
                SPAN,
                arrow,
                NONE,
                self.b.ast.vec_from_iter(iter::once(Argument::from(rhs))),
                false,
            );
        } else {
            let mut seq: OxcVec<'a, Expression<'a>> =
                OxcVec::with_capacity_in(setters.len() + 1, self.b.ast.allocator);
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
                current = bp::fallback(self.b, current, default, None);
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
        let count = if has_rest { None } else { Some(len) };
        let to_array = bp::to_array(self.b, source, count);
        decls.push(self.b.var_stmt(name, to_array));
        temps.push((key, name));
        name
    }
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
