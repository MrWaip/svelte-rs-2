use oxc_traverse::{Ancestor, TraverseCtx};

use svelte_analyze::RuneKind;

use crate::builder::Arg;

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
                        }
                    }
                    node.init = Some(oxc_ast::ast::Expression::CallExpression(call));
                }
                RuneKind::DerivedBy => {
                    call.callee = self.b.rid_expr("$.derived");
                    node.init = Some(oxc_ast::ast::Expression::CallExpression(call));
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
                                let mut dummy =
                                    oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut call.arguments[0], &mut dummy);
                                let inner = dummy.into_expression();
                                let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                                call.arguments[0] = oxc_ast::ast::Argument::from(proxied);
                            }
                        }

                        let state_expr = oxc_ast::ast::Expression::CallExpression(call);
                        node.init = if self.dev {
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
                        let value = if kind == RuneKind::State
                            && svelte_transform::rune_refs::should_proxy(&value)
                        {
                            self.b.call_expr("$.proxy", [Arg::Expr(value)])
                        } else {
                            value
                        };
                        node.init = Some(value);
                    }
                }
                RuneKind::StateEager => {
                    let arg = call.arguments.remove(0).into_expression();
                    node.init = Some(self.b.call_expr(
                        "$.eager",
                        [Arg::Expr(self.b.thunk(arg))],
                    ));
                }
                RuneKind::EffectPending => {
                    let pending_call =
                        self.b.call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
                    node.init = Some(self.b.call_expr(
                        "$.eager",
                        [Arg::Expr(self.b.thunk(pending_call))],
                    ));
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
                *node = self.b.static_member_expr(self.b.rid_expr("$$props"), "$$host");
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
                        let pending_call =
                            self.b.call_expr("$.pending", std::iter::empty::<Arg<'a, '_>>());
                        *node = self.b.call_expr(
                            "$.eager",
                            [Arg::Expr(self.b.thunk(pending_call))],
                        );
                        return;
                    }
                    _ => {}
                }
            }
        }

        let new_callee = match &call.callee {
            oxc_ast::ast::Expression::Identifier(id) if id.name.as_str() == "$effect" => {
                Some("$.user_effect")
            }
            oxc_ast::ast::Expression::StaticMemberExpression(member) => {
                if let oxc_ast::ast::Expression::Identifier(obj) = &member.object {
                    match (obj.name.as_str(), member.property.name.as_str()) {
                        ("$effect", "pre") => Some("$.user_pre_effect"),
                        ("$effect", "root") => Some("$.effect_root"),
                        ("$state", "snapshot") => Some("$.snapshot"),
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

    pub(super) fn rewrite_identifier_expression(
        &mut self,
        node: &mut oxc_ast::ast::Expression<'a>,
    ) {
        let oxc_ast::ast::Expression::Identifier(id) = node else {
            return;
        };
        if let Some(prop_kind) = self.prop_kind_for_ref(id) {
            match prop_kind {
                PropKind::Source => {
                    let name = id.name.as_str().to_string();
                    *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
                }
                PropKind::NonSource(prop_name) => {
                    *node = self.b.static_member_expr(self.b.rid_expr("$$props"), &prop_name);
                }
            }
            return;
        }
        let id_name = id.name.as_str();
        if id.reference_id.get().is_some() && self.component_scoping.is_store_ref(id_name) {
            let name = id_name.to_string();
            *node = self.b.call_expr(&name, std::iter::empty::<Arg<'a, '_>>());
            return;
        }
        let Some((kind, mutated)) = self.rune_for_ref(id) else {
            return;
        };
        let needs_get = mutated || kind.is_derived();
        if needs_get {
            let name = id.name.as_str().to_string();
            *node = svelte_transform::rune_refs::make_rune_get(self.b.ast.allocator, &name);
        }
    }
}
