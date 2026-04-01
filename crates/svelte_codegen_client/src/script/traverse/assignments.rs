use oxc_ast::ast::Expression;
use oxc_traverse::TraverseCtx;
use svelte_analyze::RuneKind;

use crate::builder::Arg;

use super::super::{PropKind, ScriptTransformer};

impl<'a> ScriptTransformer<'_, 'a> {
    pub(super) fn transform_assignment(
        &self,
        node: &mut Expression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
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
                    let current = self.b.call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
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

                *node = self.b.call_expr(
                    "$.store_set",
                    [Arg::Ident(base_name), Arg::Expr(value)],
                );
                return;
            }
            if let Some((kind, mutated)) = self.rune_for_ref(id) {
                if mutated {
                    let name = id.name.as_str().to_string();
                    let right = self.b.move_expr(&mut assign.right);

                    let value = if assign.operator.is_assign() {
                        right
                    } else {
                        let left_get =
                            svelte_transform::rune_refs::make_rune_get(self.b.ast.allocator, &name);
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
        }
    }

    pub(super) fn transform_update(
        &self,
        node: &mut Expression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let Expression::UpdateExpression(upd) = node else {
            return;
        };

        if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument {
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
                let thunk_call = self.b.call_expr(dollar_name, std::iter::empty::<Arg<'a, '_>>());
                let mut args: Vec<Arg<'a, '_>> =
                    vec![Arg::Ident(base_name), Arg::Expr(thunk_call)];
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
                let fn_name = if upd.prefix { "$.update_pre" } else { "$.update" };
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
                    if self.in_constructor()
                        && matches!(kind, RuneKind::State | RuneKind::StateRaw)
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
