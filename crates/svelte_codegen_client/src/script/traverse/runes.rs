use oxc_traverse::{Ancestor, TraverseCtx};

use svelte_analyze::RuneKind;

use crate::builder::Arg;
use crate::script::AsyncDerivedMode;

use super::super::{PropKind, ScriptTransformer};

impl<'a> ScriptTransformer<'_, 'a> {
    pub(super) fn rewrite_variable_rune_init(
        &mut self,
        node: &mut oxc_ast::ast::VariableDeclarator<'a>,
    ) {
        let rune_info = match &node.id {
            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => self.rune_for_binding(id),
            _ => return,
        };

        let Some((kind, mutated)) = rune_info else {
            return;
        };

        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);

        if let oxc_ast::ast::Expression::CallExpression(mut call) = init_expr {
            match kind {
                RuneKind::Derived => {
                    call.callee = self.b.rid_expr("$.derived");
                    if let oxc_ast::ast::BindingPattern::BindingIdentifier(bid) = &node.id {
                        if let Some(sym_id) = bid.symbol_id.get() {
                            self.derived_pending.insert(sym_id);
                            // Track async derived BEFORE `rewrite_dev_await_tracking` can
                            // transform the `await` inside to `$.track_reactivity_loss` form.
                            let is_async_init = call
                                .arguments
                                .first()
                                .and_then(|a| a.as_expression())
                                .is_some_and(|e| {
                                    matches!(e, oxc_ast::ast::Expression::AwaitExpression(_))
                                });
                            if is_async_init {
                                let mode =
                                    if self.strip_exports && self.function_info_stack.len() > 1 {
                                        AsyncDerivedMode::Save
                                    } else {
                                        AsyncDerivedMode::Await
                                    };
                                self.async_derived_pending.insert(sym_id, mode);
                            }
                        }
                    }
                    node.init = Some(oxc_ast::ast::Expression::CallExpression(call));
                }
                RuneKind::DerivedBy => {
                    call.callee = self.b.rid_expr("$.derived");
                    let derived_expr = oxc_ast::ast::Expression::CallExpression(call);
                    node.init = if self.dev {
                        let var_name = match &node.id {
                            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => id.name.as_str(),
                            _ => "",
                        };
                        Some(
                            self.b.call_expr(
                                "$.tag",
                                [Arg::Expr(derived_expr), Arg::StrRef(var_name)],
                            ),
                        )
                    } else {
                        Some(derived_expr)
                    };
                }
                RuneKind::State | RuneKind::StateRaw => {
                    if mutated {
                        call.callee = self.b.rid_expr("$.state");

                        if call.arguments.is_empty() {
                            let void_zero = self.b.ast.expression_unary(
                                oxc_span::SPAN,
                                oxc_ast::ast::UnaryOperator::Void,
                                self.b.num_expr(0.0),
                            );
                            call.arguments.push(void_zero.into());
                        } else if kind == RuneKind::State {
                            let needs_proxy = call.arguments[0]
                                .as_expression()
                                .is_some_and(svelte_transform::rune_refs::should_proxy);
                            if needs_proxy {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut call.arguments[0], &mut dummy);
                                let inner = dummy.into_expression();
                                let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                                call.arguments[0] = oxc_ast::ast::Argument::from(proxied);
                            }
                        }

                        let state_expr = oxc_ast::ast::Expression::CallExpression(call);
                        node.init =
                            if self.dev {
                                let var_name = match &node.id {
                                    oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                                        id.name.as_str()
                                    }
                                    _ => "state",
                                };
                                Some(self.b.call_expr(
                                    "$.tag",
                                    [Arg::Expr(state_expr), Arg::StrRef(var_name)],
                                ))
                            } else {
                                Some(state_expr)
                            };
                    } else {
                        let value = if call.arguments.is_empty() {
                            self.b.ast.expression_unary(
                                oxc_span::SPAN,
                                oxc_ast::ast::UnaryOperator::Void,
                                self.b.num_expr(0.0),
                            )
                        } else {
                            let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                            std::mem::swap(&mut call.arguments[0], &mut dummy);
                            dummy.into_expression()
                        };
                        let is_proxy = kind == RuneKind::State
                            && svelte_transform::rune_refs::should_proxy(&value);
                        let value = if is_proxy {
                            self.b.call_expr("$.proxy", [Arg::Expr(value)])
                        } else {
                            value
                        };
                        let value = if self.dev && is_proxy {
                            let var_name = match &node.id {
                                oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                                    id.name.as_str()
                                }
                                _ => "",
                            };
                            self.b
                                .call_expr("$.tag_proxy", [Arg::Expr(value), Arg::StrRef(var_name)])
                        } else {
                            value
                        };
                        node.init = Some(value);
                    }
                }
                RuneKind::StateEager => {
                    let arg = call.arguments.remove(0).into_expression();
                    node.init = Some(self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(arg))]));
                }
                RuneKind::EffectPending => {
                    let pending_call = self
                        .b
                        .call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
                    node.init = Some(
                        self.b
                            .call_expr("$.eager", [Arg::Expr(self.b.thunk(pending_call))]),
                    );
                }
                _ => {
                    node.init = Some(oxc_ast::ast::Expression::CallExpression(call));
                }
            }
        }
    }

    pub(super) fn rewrite_call_expression(&mut self, node: &mut oxc_ast::ast::Expression<'a>) {
        let oxc_ast::ast::Expression::CallExpression(call) = node else {
            return;
        };

        if let oxc_ast::ast::Expression::Identifier(id) = &call.callee {
            if id.name.as_str() == "$host" {
                *node = self
                    .b
                    .static_member_expr(self.b.rid_expr("$$props"), "$$host");
                return;
            }
        }

        if let oxc_ast::ast::Expression::StaticMemberExpression(member) = &call.callee {
            if let oxc_ast::ast::Expression::Identifier(obj) = &member.object {
                match (obj.name.as_str(), member.property.name.as_str()) {
                    ("$state", "eager") => {
                        let oxc_ast::ast::Expression::CallExpression(mut call) =
                            self.b.move_expr(node)
                        else {
                            unreachable!()
                        };
                        let arg = call.arguments.remove(0).into_expression();
                        *node = self.b.call_expr("$.eager", [Arg::Expr(self.b.thunk(arg))]);
                        return;
                    }
                    ("$effect", "pending") => {
                        let pending_call = self
                            .b
                            .call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
                        *node = self
                            .b
                            .call_expr("$.eager", [Arg::Expr(self.b.thunk(pending_call))]);
                        return;
                    }
                    _ => {}
                }
            }
        }

        let mut is_snapshot = false;
        let new_callee = match &call.callee {
            oxc_ast::ast::Expression::Identifier(id) if id.name.as_str() == "$effect" => {
                Some("$.user_effect")
            }
            oxc_ast::ast::Expression::StaticMemberExpression(member) => {
                if let oxc_ast::ast::Expression::Identifier(obj) = &member.object {
                    match (obj.name.as_str(), member.property.name.as_str()) {
                        ("$effect", "pre") => Some("$.user_pre_effect"),
                        ("$effect", "root") => Some("$.effect_root"),
                        ("$state", "snapshot") => {
                            is_snapshot = true;
                            Some("$.snapshot")
                        }
                        ("$effect", "tracking") => Some("$.effect_tracking"),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(callee_name) = new_callee {
            let oxc_ast::ast::Expression::CallExpression(call) = node else {
                unreachable!()
            };
            call.callee = self.b.rid_expr(callee_name);

            // $.snapshot(val, true) when svelte-ignore state_snapshot_uncloneable is active
            if is_snapshot && self.dev && self.is_in_ignored_stmt("state_snapshot_uncloneable") {
                call.arguments
                    .push(oxc_ast::ast::Argument::from(self.b.bool_expr(true)));
            }
        }
    }

    pub(super) fn rewrite_static_member_expression(
        &mut self,
        node: &mut oxc_ast::ast::Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let oxc_ast::ast::Expression::StaticMemberExpression(member) = node else {
            return;
        };
        if let oxc_ast::ast::Expression::Identifier(id) = &member.object {
            if self.is_rest_prop_ref(id)
                && !self
                    .component_scoping
                    .is_rest_prop_excluded(&member.property.name)
            {
                let is_lhs = matches!(
                    ctx.parent(),
                    Ancestor::AssignmentExpressionLeft(_) | Ancestor::UpdateExpressionArgument(_)
                );
                if !is_lhs {
                    let oxc_ast::ast::Expression::StaticMemberExpression(member) = node else {
                        unreachable!()
                    };
                    member.object = self.b.rid_expr("$$props");
                }
            }
        }
    }

    pub(super) fn identifier_read_expr(
        &self,
        id: &oxc_ast::ast::IdentifierReference<'a>,
    ) -> Option<oxc_ast::ast::Expression<'a>> {
        if let Some(prop_kind) = self.prop_kind_for_ref(id) {
            return Some(match prop_kind {
                PropKind::Source => {
                    let name = id.name.as_str().to_string();
                    self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>())
                }
                PropKind::NonSource(prop_name) => self
                    .b
                    .static_member_expr(self.b.rid_expr("$$props"), &prop_name),
            });
        }
        let id_name = id.name.as_str();
        if id.reference_id.get().is_some() && self.component_scoping.is_store_ref(id_name) {
            let name = id_name.to_string();
            return Some(self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>()));
        }
        let Some(ref_id) = id.reference_id.get() else {
            return None;
        };
        let Some(sym_id) = self.component_scoping.get_reference(ref_id).symbol_id() else {
            return None;
        };
        let Some(kind) = self.component_scoping.rune_kind(sym_id) else {
            return None;
        };
        let mutated = self.component_scoping.is_mutated(sym_id);
        let needs_get = mutated || kind.is_derived();
        if !needs_get {
            return None;
        }

        let name = id.name.as_str().to_string();
        let alloc = self.b.ast.allocator;
        Some(if self.component_scoping.is_var_declared_state(sym_id) {
            svelte_transform::rune_refs::make_rune_safe_get(alloc, &name)
        } else {
            svelte_transform::rune_refs::make_rune_get(alloc, &name)
        })
    }

    pub(super) fn rewrite_identifier_expression(
        &mut self,
        node: &mut oxc_ast::ast::Expression<'a>,
    ) {
        let oxc_ast::ast::Expression::Identifier(id) = node else {
            return;
        };
        let Some(replacement) = self.identifier_read_expr(id) else {
            return;
        };
        *node = replacement;
    }
}
