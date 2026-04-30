use oxc_traverse::{Ancestor, TraverseCtx};

use svelte_analyze::{BindingSemantics, DerivedKind, RuneKind, RuntimeRuneKind, StateKind};

use super::model::AsyncDerivedMode;
use svelte_ast_builder::Arg;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_variable_rune_init(
        &mut self,
        node: &mut oxc_ast::ast::VariableDeclarator<'a>,
    ) {
        let (sym_id, binding_name) = {
            let oxc_ast::ast::BindingPattern::BindingIdentifier(binding) = &node.id else {
                return;
            };
            (
                binding.symbol_id.get(),
                self.b.alloc_str(binding.name.as_str()),
            )
        };
        let semantics = match (self.analysis.as_ref(), sym_id) {
            (Some(analysis), Some(sym)) => Some(analysis.binding_semantics(sym)),
            _ => None,
        };

        match semantics {
            Some(BindingSemantics::LegacyState(state)) => {
                if let Some(init) = node.init.as_mut() {
                    let init_expr = self.b.move_expr(init);
                    let call = if state.immutable {
                        self.b
                            .call_expr("$.mutable_source", [Arg::Expr(init_expr), Arg::Bool(true)])
                    } else {
                        self.b.call_expr("$.mutable_source", [Arg::Expr(init_expr)])
                    };
                    node.init = Some(call);
                }
            }
            Some(BindingSemantics::State(state)) => {
                self.rewrite_state_binding_init(node, binding_name, state.kind, sym_id);
            }
            Some(BindingSemantics::Derived(derived)) => {
                self.rewrite_derived_binding_init(node, binding_name, derived.kind, sym_id);
            }
            Some(BindingSemantics::RuntimeRune {
                kind: RuntimeRuneKind::EffectPending,
            }) => {
                self.rewrite_effect_pending_init(node);
            }
            _ => {
                let Some(init) = node.init.as_mut() else {
                    return;
                };
                let Some(kind) = Self::detect_class_field_rune_kind(init) else {
                    return;
                };
                self.rewrite_class_field_rune_init(node, binding_name, kind, sym_id);
            }
        }
    }

    fn rewrite_state_binding_init(
        &mut self,
        node: &mut oxc_ast::ast::VariableDeclarator<'a>,
        binding_name: &'a str,
        kind: StateKind,
        sym_id: Option<oxc_semantic::SymbolId>,
    ) {
        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);

        if matches!(kind, StateKind::StateEager) {
            node.init = None;
            return;
        }

        let oxc_ast::ast::Expression::CallExpression(mut call) = init_expr else {
            return;
        };
        let mutated = sym_id.is_some_and(|sym| self.component_scoping.is_mutated(sym));
        if mutated {
            call.callee = self.b.rid_expr("$.state");

            if call.arguments.is_empty() {
                let void_zero = self.b.ast.expression_unary(
                    oxc_span::SPAN,
                    oxc_ast::ast::UnaryOperator::Void,
                    self.b.num_expr(0.0),
                );
                call.arguments.push(void_zero.into());
            } else if matches!(kind, StateKind::State) {
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
            node.init = if self.dev {
                Some(
                    self.b
                        .call_expr("$.tag", [Arg::Expr(state_expr), Arg::StrRef(binding_name)]),
                )
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
                matches!(kind, StateKind::State) && crate::rune_refs::should_proxy(&value);
            let value = if is_proxy {
                self.b.call_expr("$.proxy", [Arg::Expr(value)])
            } else {
                value
            };
            let value = if self.dev && is_proxy {
                self.b
                    .call_expr("$.tag_proxy", [Arg::Expr(value), Arg::StrRef(binding_name)])
            } else {
                value
            };
            node.init = Some(value);
        }
    }

    fn rewrite_derived_binding_init(
        &mut self,
        node: &mut oxc_ast::ast::VariableDeclarator<'a>,
        binding_name: &'a str,
        kind: DerivedKind,
        sym_id: Option<oxc_semantic::SymbolId>,
    ) {
        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);
        let oxc_ast::ast::Expression::CallExpression(mut call) = init_expr else {
            return;
        };

        match kind {
            DerivedKind::Derived => {
                call.callee = self.b.rid_expr("$.derived");
                if let Some(sym) = sym_id {
                    self.derived_pending.insert(sym);

                    let is_async_init = call
                        .arguments
                        .first()
                        .and_then(|a| a.as_expression())
                        .is_some_and(|e| matches!(e, oxc_ast::ast::Expression::AwaitExpression(_)));
                    if is_async_init {
                        let mode = if self.strip_exports && self.function_info_stack.len() > 1 {
                            AsyncDerivedMode::Save
                        } else {
                            AsyncDerivedMode::Await
                        };
                        self.async_derived_pending.insert(sym, mode);
                    }
                }
                node.init = Some(oxc_ast::ast::Expression::CallExpression(call));
            }
            DerivedKind::DerivedBy => {
                call.callee = self.b.rid_expr("$.derived");
                let derived_expr = oxc_ast::ast::Expression::CallExpression(call);
                node.init = if self.dev {
                    Some(self.b.call_expr(
                        "$.tag",
                        [Arg::Expr(derived_expr), Arg::StrRef(binding_name)],
                    ))
                } else {
                    Some(derived_expr)
                };
            }
        }
    }

    fn rewrite_effect_pending_init(&mut self, node: &mut oxc_ast::ast::VariableDeclarator<'a>) {
        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);
        let oxc_ast::ast::Expression::CallExpression(_) = init_expr else {
            return;
        };
        let pending_call = self
            .b
            .call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
        node.init = Some(
            self.b
                .call_expr("$.eager", [Arg::Expr(self.b.thunk(pending_call))]),
        );
    }

    fn rewrite_class_field_rune_init(
        &mut self,
        node: &mut oxc_ast::ast::VariableDeclarator<'a>,
        binding_name: &'a str,
        kind: RuneKind,
        sym_id: Option<oxc_semantic::SymbolId>,
    ) {
        match kind {
            RuneKind::State | RuneKind::StateRaw => {
                let state_kind = if matches!(kind, RuneKind::State) {
                    StateKind::State
                } else {
                    StateKind::StateRaw
                };
                self.rewrite_state_binding_init(node, binding_name, state_kind, sym_id);
            }
            RuneKind::Derived => {
                self.rewrite_derived_binding_init(node, binding_name, DerivedKind::Derived, sym_id);
            }
            RuneKind::DerivedBy => {
                self.rewrite_derived_binding_init(
                    node,
                    binding_name,
                    DerivedKind::DerivedBy,
                    sym_id,
                );
            }
            _ => {}
        }
    }

    pub(crate) fn rewrite_call_expression(&mut self, node: &mut oxc_ast::ast::Expression<'a>) {
        let oxc_ast::ast::Expression::CallExpression(call) = node else {
            return;
        };

        if let oxc_ast::ast::Expression::Identifier(id) = &call.callee
            && id.name.as_str() == "$host"
        {
            *node = self
                .b
                .static_member_expr(self.b.rid_expr("$$props"), "$$host");
            return;
        }

        let dev_snapshot_ignored =
            self.dev && self.is_in_ignored_stmt("state_snapshot_uncloneable");
        if self.rewrite_shared_call(node, dev_snapshot_ignored) {
            return;
        }

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
        if self.analysis.is_none() {
            return;
        }
        let is_lhs = matches!(
            ctx.parent(),
            Ancestor::AssignmentExpressionLeft(_) | Ancestor::UpdateExpressionArgument(_)
        );
        self.rewrite_rest_prop_member(node, is_lhs);
    }

    pub(crate) fn rewrite_identifier_expression(
        &mut self,
        node: &mut oxc_ast::ast::Expression<'a>,
    ) {
        let _ = self.dispatch_identifier_read(node);
    }
}
