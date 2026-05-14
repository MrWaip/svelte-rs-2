use std::{iter, mem};

use oxc_ast::ast::{Argument, BindingPattern, Expression, UnaryOperator, VariableDeclarator};
use oxc_semantic::SymbolId;
use oxc_span::SPAN;
use oxc_traverse::{Ancestor, TraverseCtx};

use svelte_analyze::{
    BindingSemantics, DerivedKind, RuneKind, RuntimeRuneKind, StateBindingSemantics, StateKind,
    detect_rune_from_call,
};

use super::model::AsyncDerivedMode;
use svelte_ast_builder::Arg;

use super::model::ComponentTransformer;
use crate::rune_refs::should_proxy;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_variable_rune_init(&mut self, node: &mut VariableDeclarator<'a>) {
        let (sym_id, binding_name) = {
            let BindingPattern::BindingIdentifier(binding) = &node.id else {
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
                } else {
                    let call = self.b.call_expr("$.mutable_source", iter::empty::<Arg>());
                    node.init = Some(call);
                }
            }
            Some(BindingSemantics::State(state)) => {
                let binding_semantic = state.binding_semantics.first().copied();
                self.rewrite_state_binding_init(node, binding_name, state.kind, binding_semantic);
            }
            Some(BindingSemantics::Derived(derived)) => {
                self.rewrite_derived_binding_init(node, binding_name, derived.kind, sym_id);
            }
            Some(BindingSemantics::OptimizedRune(opt)) => {
                let binding_semantic = match opt.kind {
                    StateKind::State => StateBindingSemantics::NonReactive {
                        proxied: opt.proxy_init,
                    },
                    StateKind::StateRaw | StateKind::StateEager => {
                        StateBindingSemantics::NonReactive { proxied: false }
                    }
                };
                self.rewrite_state_binding_init(
                    node,
                    binding_name,
                    opt.kind,
                    Some(binding_semantic),
                );
            }
            Some(BindingSemantics::RuntimeRune {
                kind: RuntimeRuneKind::EffectPending,
            }) => {
                self.rewrite_effect_pending_init(node);
            }
            _ => {}
        }
    }

    fn rewrite_state_binding_init(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        binding_name: &'a str,
        kind: StateKind,
        binding_semantic: Option<StateBindingSemantics>,
    ) {
        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);

        if matches!(kind, StateKind::StateEager) {
            node.init = None;
            return;
        }

        let Expression::CallExpression(mut call) = init_expr else {
            return;
        };
        let is_state_source = matches!(
            binding_semantic,
            Some(StateBindingSemantics::StateSignal { .. } | StateBindingSemantics::StateRawSignal)
        );
        if is_state_source {
            call.callee = self.b.rid_expr("$.state");

            if call.arguments.is_empty() {
                let void_zero = self.b.ast.expression_unary(
                    SPAN,
                    UnaryOperator::Void,
                    self.b.num_expr(0.0),
                );
                call.arguments.push(void_zero.into());
            } else if matches!(kind, StateKind::State) {
                let needs_proxy = call.arguments[0]
                    .as_expression()
                    .is_some_and(should_proxy);
                if needs_proxy {
                    let mut dummy = Argument::from(self.b.cheap_expr());
                    mem::swap(&mut call.arguments[0], &mut dummy);
                    let inner = dummy.into_expression();
                    let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                    call.arguments[0] = Argument::from(proxied);
                }
            }

            let state_expr = Expression::CallExpression(call);
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
                    SPAN,
                    UnaryOperator::Void,
                    self.b.num_expr(0.0),
                )
            } else {
                let mut dummy = Argument::from(self.b.cheap_expr());
                mem::swap(&mut call.arguments[0], &mut dummy);
                dummy.into_expression()
            };
            let is_proxy = matches!(kind, StateKind::State) && should_proxy(&value);
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
        node: &mut VariableDeclarator<'a>,
        binding_name: &'a str,
        kind: DerivedKind,
        sym_id: Option<SymbolId>,
    ) {
        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);
        let Expression::CallExpression(mut call) = init_expr else {
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
                        .is_some_and(|e| matches!(e, Expression::AwaitExpression(_)));
                    if is_async_init {
                        let mode = if self.strip_exports && self.function_info_stack.len() > 1 {
                            AsyncDerivedMode::Save
                        } else {
                            AsyncDerivedMode::Await
                        };
                        self.async_derived_pending.insert(sym, mode);
                    }
                }
                node.init = Some(Expression::CallExpression(call));
            }
            DerivedKind::DerivedBy => {
                call.callee = self.b.rid_expr("$.derived");
                let derived_expr = Expression::CallExpression(call);
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

    fn rewrite_effect_pending_init(&mut self, node: &mut VariableDeclarator<'a>) {
        let Some(init) = node.init.as_mut() else {
            return;
        };
        let init_expr = self.b.move_expr(init);
        let Expression::CallExpression(_) = init_expr else {
            return;
        };
        let pending_call = self
            .b
            .call_expr("$.pending", iter::empty::<Arg<'a, '_>>());
        node.init = Some(
            self.b
                .call_expr("$.eager", [Arg::Expr(self.b.thunk(pending_call))]),
        );
    }

    pub(crate) fn rewrite_call_expression(&mut self, node: &mut Expression<'a>) {
        if !self
            .analysis
            .as_ref()
            .is_some_and(|a| a.uses_runes())
        {
            return;
        }
        let Expression::CallExpression(call) = &*node else {
            return;
        };
        let Some(rune_kind) = detect_rune_from_call(call) else {
            return;
        };

        if matches!(rune_kind, RuneKind::Host) {
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

        let new_callee = match rune_kind {
            RuneKind::Effect => Some("$.user_effect"),
            RuneKind::EffectPre => Some("$.user_pre_effect"),
            RuneKind::EffectRoot => Some("$.effect_root"),
            RuneKind::EffectTracking => Some("$.effect_tracking"),
            _ => None,
        };
        if let Some(callee_name) = new_callee {
            let Expression::CallExpression(call) = node else {
                unreachable!()
            };
            call.callee = self.b.rid_expr(callee_name);
        }
    }

    pub(crate) fn rewrite_static_member_expression(
        &mut self,
        node: &mut Expression<'a>,
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

    pub(crate) fn rewrite_identifier_expression(&mut self, node: &mut Expression<'a>) {
        let _ = self.dispatch_identifier_read(node);
    }
}
