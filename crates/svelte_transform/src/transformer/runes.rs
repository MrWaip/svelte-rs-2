use oxc_traverse::{Ancestor, TraverseCtx};

use svelte_analyze::{DeclarationSemantics, DerivedKind, DerivedLowering, RuneKind, StateKind};

use super::model::AsyncDerivedMode;
use svelte_ast_builder::Arg;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_variable_rune_init(
        &mut self,
        node: &mut oxc_ast::ast::VariableDeclarator<'a>,
    ) {
        let oxc_ast::ast::BindingPattern::BindingIdentifier(binding) = &node.id else {
            return;
        };
        let Some(sym_id) = binding.symbol_id.get() else {
            return;
        };
        if self.analysis.is_some() {
            let root_node = node.node_id();
            let Some(init) = node.init.as_mut() else {
                return;
            };
            let semantics = self
                .analysis
                .map(|analysis| analysis.declaration_semantics(root_node))
                .unwrap_or(DeclarationSemantics::NonReactive);
            match semantics {
                DeclarationSemantics::State(state) if state.kind == StateKind::State => {
                    let init_expr = self.b.move_expr(init);
                    let oxc_ast::ast::Expression::CallExpression(mut call) = init_expr else {
                        node.init = Some(init_expr);
                        return;
                    };

                    call.callee = self.b.rid_expr("$.state");

                    if call.arguments.is_empty() {
                        let void_zero = self.b.ast.expression_unary(
                            oxc_span::SPAN,
                            oxc_ast::ast::UnaryOperator::Void,
                            self.b.num_expr(0.0),
                        );
                        call.arguments.push(void_zero.into());
                    } else if state.proxied {
                        let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                        std::mem::swap(&mut call.arguments[0], &mut dummy);
                        let inner = dummy.into_expression();
                        let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                        call.arguments[0] = oxc_ast::ast::Argument::from(proxied);
                    }

                    let state_expr = oxc_ast::ast::Expression::CallExpression(call);
                    node.init = if self.dev {
                        let var_name = binding.name.as_str();
                        Some(
                            self.b
                                .call_expr("$.tag", [Arg::Expr(state_expr), Arg::StrRef(var_name)]),
                        )
                    } else {
                        Some(state_expr)
                    };
                    return;
                }
                DeclarationSemantics::State(state) if state.kind == StateKind::StateRaw => {
                    let init_expr = self.b.move_expr(init);
                    let oxc_ast::ast::Expression::CallExpression(mut call) = init_expr else {
                        node.init = Some(init_expr);
                        return;
                    };

                    call.callee = self.b.rid_expr("$.state");

                    if call.arguments.is_empty() {
                        let void_zero = self.b.ast.expression_unary(
                            oxc_span::SPAN,
                            oxc_ast::ast::UnaryOperator::Void,
                            self.b.num_expr(0.0),
                        );
                        call.arguments.push(void_zero.into());
                    }

                    let state_expr = oxc_ast::ast::Expression::CallExpression(call);
                    node.init = if self.dev {
                        let var_name = binding.name.as_str();
                        Some(
                            self.b
                                .call_expr("$.tag", [Arg::Expr(state_expr), Arg::StrRef(var_name)]),
                        )
                    } else {
                        Some(state_expr)
                    };
                    return;
                }
                DeclarationSemantics::Derived(derived) => {
                    let init_expr = self.b.move_expr(init);
                    let oxc_ast::ast::Expression::CallExpression(mut call) = init_expr else {
                        node.init = Some(init_expr);
                        return;
                    };

                    call.callee = self.b.rid_expr("$.derived");

                    match derived.kind {
                        DerivedKind::Derived => {
                            self.derived_pending.insert(sym_id);
                            // `wrap_derived_thunks` runs later and needs to know which pending
                            // `$derived(...)` declarations must lower through `$.async_derived`
                            // instead of the ordinary thunk-wrapping path.
                            if matches!(derived.lowering, DerivedLowering::Async) {
                                let mode =
                                    if self.strip_exports && self.function_info_stack.len() > 1 {
                                        AsyncDerivedMode::Save
                                    } else {
                                        AsyncDerivedMode::Await
                                    };
                                self.async_derived_pending.insert(sym_id, mode);
                            }
                            node.init = Some(oxc_ast::ast::Expression::CallExpression(call));
                        }
                        DerivedKind::DerivedBy => {
                            let derived_expr = oxc_ast::ast::Expression::CallExpression(call);
                            node.init = if self.dev {
                                let var_name = binding.name.as_str();
                                Some(self.b.call_expr(
                                    "$.tag",
                                    [Arg::Expr(derived_expr), Arg::StrRef(var_name)],
                                ))
                            } else {
                                Some(derived_expr)
                            };
                        }
                    }
                    return;
                }
                DeclarationSemantics::NonReactive => {}
                _ => return,
            }
        }

        let Some(init) = node.init.as_mut() else {
            return;
        };
        let kind = self
            .rune_for_binding(binding)
            .or_else(|| Self::detect_class_field_rune_kind(init));
        let Some(kind) = kind else {
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
                    let mutated = binding
                        .symbol_id
                        .get()
                        .is_some_and(|sym_id| self.component_scoping.is_mutated(sym_id));
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
                                .is_some_and(crate::rune_refs::should_proxy);
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
                                let var_name = binding.name.as_str();
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
                        let is_proxy =
                            kind == RuneKind::State && crate::rune_refs::should_proxy(&value);
                        let value = if is_proxy {
                            self.b.call_expr("$.proxy", [Arg::Expr(value)])
                        } else {
                            value
                        };
                        let value = if self.dev && is_proxy {
                            let var_name = binding.name.as_str();
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

    pub(crate) fn rewrite_call_expression(&mut self, node: &mut oxc_ast::ast::Expression<'a>) {
        let oxc_ast::ast::Expression::CallExpression(call) = node else {
            return;
        };

        // Script-only: `$host()` → `$$props.$$host`.
        if let oxc_ast::ast::Expression::Identifier(id) = &call.callee {
            if id.name.as_str() == "$host" {
                *node = self
                    .b
                    .static_member_expr(self.b.rid_expr("$$props"), "$$host");
                return;
            }
        }

        // Shared with Template: $state.eager, $state.snapshot, $effect.pending.
        let dev_snapshot_ignored =
            self.dev && self.is_in_ignored_stmt("state_snapshot_uncloneable");
        if super::rewrites::rewrite_shared_call(self.b.ast.allocator, node, dev_snapshot_ignored) {
            return;
        }

        // Script-only callee renames: $effect / $effect.pre / $effect.root / $effect.tracking.
        let oxc_ast::ast::Expression::CallExpression(call) = &*node else {
            return;
        };
        let new_callee = match &call.callee {
            oxc_ast::ast::Expression::Identifier(id) if id.name.as_str() == "$effect" => {
                Some("$.user_effect")
            }
            oxc_ast::ast::Expression::StaticMemberExpression(member) => {
                if let oxc_ast::ast::Expression::Identifier(obj) = &member.object {
                    match (obj.name.as_str(), member.property.name.as_str()) {
                        ("$effect", "pre") => Some("$.user_pre_effect"),
                        ("$effect", "root") => Some("$.effect_root"),
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
        }
    }

    pub(crate) fn rewrite_static_member_expression(
        &mut self,
        node: &mut oxc_ast::ast::Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let Some(analysis) = self.analysis else {
            return;
        };
        let is_lhs = matches!(
            ctx.parent(),
            Ancestor::AssignmentExpressionLeft(_) | Ancestor::UpdateExpressionArgument(_)
        );
        super::rewrites::rewrite_rest_prop_member(analysis, self.b.ast.allocator, node, is_lhs);
    }

    pub(crate) fn rewrite_identifier_expression(
        &mut self,
        node: &mut oxc_ast::ast::Expression<'a>,
    ) {
        if let Some(analysis) = self.analysis {
            if super::rewrites::rewrite_identifier_read(
                analysis,
                self.b.ast.allocator,
                &self.transform_data,
                node,
            ) {}
        }
    }
}
