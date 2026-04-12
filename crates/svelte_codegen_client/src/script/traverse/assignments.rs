use oxc_ast::ast::Expression;
use oxc_traverse::{Ancestor, TraverseCtx};
use svelte_analyze::RuneKind;

use crate::builder::Arg;

use super::super::{PropKind, ScriptTransformer};

impl<'a> ScriptTransformer<'_, 'a> {
    fn prop_mutation_info_from_member(
        &self,
        target: &oxc_ast::ast::MemberExpression<'a>,
    ) -> Option<(String, String, Vec<String>)> {
        if !self.dev || self.is_in_ignored_stmt("ownership_invalid_mutation") {
            return None;
        }

        let mut root = target.object();
        let mut segments_rev: Vec<String> = vec![match target {
            oxc_ast::ast::MemberExpression::StaticMemberExpression(member) => {
                member.property.name.as_str().to_string()
            }
            oxc_ast::ast::MemberExpression::ComputedMemberExpression(member) => {
                let Expression::StringLiteral(lit) = &member.expression else {
                    return None;
                };
                lit.value.as_str().to_string()
            }
            oxc_ast::ast::MemberExpression::PrivateFieldExpression(_) => return None,
        }];
        loop {
            match root {
                Expression::StaticMemberExpression(member) => {
                    segments_rev.push(member.property.name.as_str().to_string());
                    root = &member.object;
                }
                Expression::ComputedMemberExpression(member) => {
                    let Expression::StringLiteral(lit) = &member.expression else {
                        return None;
                    };
                    segments_rev.push(lit.value.as_str().to_string());
                    root = &member.object;
                }
                _ => break,
            }
        }
        let Expression::Identifier(root_id) = root else {
            return None;
        };
        segments_rev.reverse();

        if root_id.name.as_str() == "$$props" {
            let prop_alias = segments_rev.first()?.clone();
            return Some((prop_alias, "$$props".to_string(), segments_rev));
        }

        let (prop_alias, root_name) = match self.prop_kind_for_ref(root_id) {
            Some(PropKind::Source) => (
                self.prop_source_alias_for_ref(root_id)?,
                root_id.name.to_string(),
            ),
            Some(PropKind::NonSource(prop_name)) => (prop_name, root_id.name.to_string()),
            None => (
                self.prop_alias_for_local_name(root_id.name.as_str())?,
                root_id.name.to_string(),
            ),
        };
        Some((prop_alias, root_name, segments_rev))
    }

    fn wrap_prop_mutation_validation_from_info(
        &mut self,
        node: &mut Expression<'a>,
        prop_alias: String,
        root_name: String,
        segments: Vec<String>,
        span_start: u32,
    ) {
        self.needs_ownership_validator = true;

        let offset = self.script_content_start + span_start;
        let (line, col) = crate::script::location::compute_line_col(self.component_source, offset);

        let mut path: Vec<Expression<'a>> = Vec::with_capacity(1 + segments.len());
        path.push(self.b.str_expr(&root_name));
        for seg in segments {
            path.push(self.b.str_expr(&seg));
        }

        let expr = self.b.move_expr(node);
        let wrapped = self.b.call_expr(
            "$$ownership_validator.mutation",
            [
                Arg::Str(prop_alias),
                Arg::Expr(self.b.array_expr(path)),
                Arg::Expr(expr),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        );
        *node = wrapped;
    }

    pub(super) fn transform_assignment(
        &mut self,
        node: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let Expression::AssignmentExpression(assign) = node else {
            return;
        };

        if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left {
            if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                if matches!(prop_kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);
                    *node = self.b.call_expr(&name, [Arg::Expr(right)]);
                    return;
                }
            }
            let id_name = id.name.as_str();
            if let Some(base) = self.component_scoping.store_base_name(id_name) {
                let base_name: &str = self.b.alloc_str(base);
                let dollar_name: &str = self.b.alloc_str(id_name);
                let right = self.b.move_expr(&mut assign.right);

                let value = if assign.operator.is_assign() {
                    right
                } else {
                    let current = self
                        .b
                        .call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                    if let Some(bin_op) = assign.operator.to_binary_operator() {
                        self.b
                            .ast
                            .expression_binary(oxc_span::SPAN, current, bin_op, right)
                    } else if let Some(log_op) = assign.operator.to_logical_operator() {
                        self.b
                            .ast
                            .expression_logical(oxc_span::SPAN, current, log_op, right)
                    } else {
                        unreachable!(
                            "all compound assignment operators are either binary or logical"
                        )
                    }
                };

                *node = self
                    .b
                    .call_expr("$.store_set", [Arg::Ident(base_name), Arg::Expr(value)]);
                return;
            }
            if let Some((kind, mutated)) = self.rune_for_ref(id) {
                if mutated {
                    let name = id.name.as_str().to_string();
                    // Resolve sym_id while `id` borrow is still available (before move_expr borrows assign).
                    let is_var_state = id
                        .reference_id
                        .get()
                        .and_then(|r| self.component_scoping.get_reference(r).symbol_id())
                        .is_some_and(|s| self.component_scoping.is_var_declared_state(s));
                    let right = self.b.move_expr(&mut assign.right);

                    let value = if assign.operator.is_assign() {
                        right
                    } else {
                        let left_get = if is_var_state {
                            svelte_transform::rune_refs::make_rune_safe_get(
                                self.b.ast.allocator,
                                &name,
                            )
                        } else {
                            svelte_transform::rune_refs::make_rune_get(self.b.ast.allocator, &name)
                        };
                        if let Some(bin_op) = assign.operator.to_binary_operator() {
                            self.b
                                .ast
                                .expression_binary(oxc_span::SPAN, left_get, bin_op, right)
                        } else if let Some(log_op) = assign.operator.to_logical_operator() {
                            self.b
                                .ast
                                .expression_logical(oxc_span::SPAN, left_get, log_op, right)
                        } else {
                            unreachable!(
                                "all compound assignment operators are either binary or logical"
                            )
                        }
                    };

                    let needs_proxy = kind != svelte_analyze::RuneKind::StateRaw
                        && svelte_transform::rune_refs::should_proxy(&value);
                    *node = svelte_transform::rune_refs::make_rune_set(
                        self.b.ast.allocator,
                        &name,
                        value,
                        needs_proxy,
                    );
                    return;
                }
            }
        }

        if let Some((root_name, base)) = self.extract_assign_member_store_root(&assign.left) {
            let root_name = root_name.to_string();
            let base_name = base.to_string();
            let alloc = self.b.ast.allocator;
            svelte_transform::rune_refs::replace_expr_root_in_assign_target(
                &mut assign.left,
                svelte_transform::rune_refs::make_untrack(alloc, &root_name),
            );
            let mutation = self.b.move_expr(node);
            let untracked = svelte_transform::rune_refs::make_untrack(alloc, &root_name);
            *node = svelte_transform::rune_refs::make_store_mutate(
                alloc, &base_name, mutation, untracked,
            );
            return;
        }

        // Dev-mode: rewrite non-statement member-expression assignments to $.assign_*(obj, key, rhs, loc)
        if !self.dev {
            return;
        }
        let Expression::AssignmentExpression(assign) = &*node else {
            return;
        };
        let mutation_info = assign
            .left
            .as_member_expression()
            .and_then(|m| self.prop_mutation_info_from_member(m));
        let left_span_start = assign.span.start;
        let is_expr_stmt = matches!(ctx.parent(), Ancestor::ExpressionStatementExpression(_));
        let fn_name = match assign.operator {
            oxc_ast::ast::AssignmentOperator::Assign => "$.assign",
            oxc_ast::ast::AssignmentOperator::LogicalAnd => "$.assign_and",
            oxc_ast::ast::AssignmentOperator::LogicalOr => "$.assign_or",
            oxc_ast::ast::AssignmentOperator::LogicalNullish => "$.assign_nullish",
            _ => return,
        };
        if is_expr_stmt {
            if let Some((prop_alias, root_name, segments)) = mutation_info.clone() {
                self.wrap_prop_mutation_validation_from_info(
                    node,
                    prop_alias,
                    root_name,
                    segments,
                    left_span_start,
                );
            }
            return;
        }
        if !svelte_transform::rune_refs::should_proxy(&assign.right) {
            return;
        }
        let is_static = matches!(
            &assign.left,
            oxc_ast::ast::AssignmentTarget::StaticMemberExpression(_)
        );
        let is_computed = matches!(
            &assign.left,
            oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(_)
        );
        if !is_static && !is_computed {
            return;
        }

        // Capture location before moving (spans are Copy)
        let offset = self.script_content_start + left_span_start;
        let (line, col) = crate::script::location::compute_line_col(self.component_source, offset);
        let loc = format!(
            "{}:{}:{}",
            crate::script::location::sanitize_location(self.filename),
            line,
            col
        );

        // Move whole node to obtain ownership, then destructure
        let whole = self.b.move_expr(node);
        let Expression::AssignmentExpression(assign_box) = whole else {
            unreachable!()
        };
        let assign = assign_box.unbox();

        if is_static {
            let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(m) = assign.left else {
                unreachable!()
            };
            let m = m.unbox();
            let key = self.b.str_expr(m.property.name.as_str());
            *node = self.b.call_expr(
                fn_name,
                [
                    Arg::Expr(m.object),
                    Arg::Expr(key),
                    Arg::Expr(assign.right),
                    Arg::Str(loc),
                ],
            );
        } else {
            let oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(m) = assign.left else {
                unreachable!()
            };
            let m = m.unbox();
            *node = self.b.call_expr(
                fn_name,
                [
                    Arg::Expr(m.object),
                    Arg::Expr(m.expression),
                    Arg::Expr(assign.right),
                    Arg::Str(loc),
                ],
            );
        }
        if let Some((prop_alias, root_name, segments)) = mutation_info {
            self.wrap_prop_mutation_validation_from_info(
                node,
                prop_alias,
                root_name,
                segments,
                left_span_start,
            );
        }
    }

    pub(super) fn transform_update(
        &mut self,
        node: &mut Expression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };

        if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument
        {
            if let Some(prop_kind) = self.prop_kind_for_ref(id) {
                if matches!(prop_kind, PropKind::Source) {
                    let name = id.name.as_str().to_string();
                    let fn_name = if upd.prefix {
                        "$.update_pre_prop"
                    } else {
                        "$.update_prop"
                    };
                    let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(&name)];
                    if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                        args.push(Arg::Num(-1.0));
                    }
                    *node = self.b.call_expr(fn_name, args);
                    return;
                }
            }
            let id_name = id.name.as_str();
            if let Some(base) = self.component_scoping.store_base_name(id_name) {
                let base_name: &str = self.b.alloc_str(base);
                let dollar_name: &str = self.b.alloc_str(id_name);
                let fn_name = if upd.prefix {
                    "$.update_pre_store"
                } else {
                    "$.update_store"
                };
                let thunk_call = self
                    .b
                    .call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident(base_name), Arg::Expr(thunk_call)];
                if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                    args.push(Arg::Num(-1.0));
                }
                *node = self.b.call_expr(fn_name, args);
                return;
            }
            if let Some((_, mutated)) = self.rune_for_ref(id) {
                if mutated {
                    let name = id.name.as_str().to_string();
                    let is_increment = upd.operator == oxc_ast::ast::UpdateOperator::Increment;
                    *node = svelte_transform::rune_refs::make_rune_update(
                        self.b.ast.allocator,
                        &name,
                        upd.prefix,
                        is_increment,
                    );
                    return;
                }
            }
        }

        if let oxc_ast::ast::SimpleAssignmentTarget::PrivateFieldExpression(pfe) = &upd.argument {
            if matches!(&pfe.object, Expression::ThisExpression(_))
                && self.is_private_state_field(pfe.field.name.as_str())
            {
                let field_name = pfe.field.name.as_str();
                let fn_name = if upd.prefix {
                    "$.update_pre"
                } else {
                    "$.update"
                };
                let field_expr = self.b.this_private_member(field_name);
                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(field_expr)];
                if upd.operator == oxc_ast::ast::UpdateOperator::Decrement {
                    args.push(Arg::Num(-1.0));
                }
                *node = self.b.call_expr(fn_name, args);
                return;
            }
        }

        if let Some((root_name, base)) = self.extract_simple_member_store_root(&upd.argument) {
            let root_name = root_name.to_string();
            let base_name = base.to_string();
            let alloc = self.b.ast.allocator;
            svelte_transform::rune_refs::replace_expr_root_in_simple_target(
                &mut upd.argument,
                svelte_transform::rune_refs::make_untrack(alloc, &root_name),
            );
            let mutation = self.b.move_expr(node);
            let untracked = svelte_transform::rune_refs::make_untrack(alloc, &root_name);
            *node = svelte_transform::rune_refs::make_store_mutate(
                alloc, &base_name, mutation, untracked,
            );
        }
    }

    pub(super) fn rewrite_prop_update_ownership_exit(&mut self, node: &mut Expression<'a>) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };
        let span_start = upd.span.start;
        let Some(member) = upd.argument.as_member_expression() else {
            return;
        };
        let Some((prop_alias, root_name, segments)) = self.prop_mutation_info_from_member(member)
        else {
            return;
        };
        self.wrap_prop_mutation_validation_from_info(
            node,
            prop_alias,
            root_name,
            segments,
            span_start,
        );
    }

    pub(super) fn rewrite_private_assignment_exit(&self, node: &mut Expression<'a>) -> bool {
        if let Expression::AssignmentExpression(assign) = node {
            if let oxc_ast::ast::AssignmentTarget::PrivateFieldExpression(pfe) = &assign.left {
                if matches!(&pfe.object, Expression::ThisExpression(_)) {
                    let field_name = pfe.field.name.as_str();
                    if self.is_private_state_field(field_name) {
                        let left_expr = self.b.this_private_member(field_name);
                        let right = self.b.move_expr(&mut assign.right);
                        let operator = assign.operator;

                        let value = if operator.is_assign() {
                            right
                        } else {
                            let get_expr = self.b.this_private_member(field_name);
                            let left_read = self.b.call_expr("$.get", [Arg::Expr(get_expr)]);
                            if let Some(bin_op) = operator.to_binary_operator() {
                                self.b.ast.expression_binary(
                                    oxc_span::SPAN,
                                    left_read,
                                    bin_op,
                                    right,
                                )
                            } else if let Some(log_op) = operator.to_logical_operator() {
                                self.b.ast.expression_logical(
                                    oxc_span::SPAN,
                                    left_read,
                                    log_op,
                                    right,
                                )
                            } else {
                                unreachable!(
                                    "all compound assignment operators are either binary or logical"
                                )
                            }
                        };

                        *node = self
                            .b
                            .call_expr("$.set", [Arg::Expr(left_expr), Arg::Expr(value)]);
                        return true;
                    }
                }
            }
        }
        false
    }

    pub(super) fn rewrite_private_read_exit(&self, node: &mut Expression<'a>) -> bool {
        if let Expression::PrivateFieldExpression(pfe) = node {
            if matches!(&pfe.object, Expression::ThisExpression(_)) {
                let rune_kind = self.private_state_field_rune_kind(pfe.field.name.as_str());
                if let Some(kind) = rune_kind {
                    if self.in_constructor() && matches!(kind, RuneKind::State | RuneKind::StateRaw)
                    {
                        // Inside constructor, $state/$state.raw: this.#field → this.#field.v
                        let field_expr = self.b.move_expr(node);
                        *node = self.b.static_member_expr(field_expr, "v");
                    } else {
                        // Outside constructor or $derived: this.#field → $.get(this.#field)
                        let field_expr = self.b.move_expr(node);
                        *node = self.b.call_expr("$.get", [Arg::Expr(field_expr)]);
                    }
                    return true;
                }
            }
        }
        false
    }

    pub(super) fn rewrite_dev_await_tracking(&self, node: &mut Expression<'a>) {
        if let Expression::AwaitExpression(await_expr) = node {
            if self.is_in_ignored_stmt("await_reactivity_loss") {
                return;
            }
            let arg = self.b.move_expr(&mut await_expr.argument);
            let track_call = self
                .b
                .call_expr("$.track_reactivity_loss", [Arg::Expr(arg)]);
            let awaited = self.b.await_expr(track_call);
            *node = self
                .b
                .call_expr_callee(awaited, std::iter::empty::<Arg<'a, '_>>());
        }
    }
}
