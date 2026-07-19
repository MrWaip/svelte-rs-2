use std::collections::HashMap;
use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, BindingPattern, Expression, UnaryOperator, VariableDeclarationKind,
    VariableDeclarator,
};
use oxc_span::SPAN;

use svelte_analyze::{BindingSemantics, StateKind};

use svelte_ast_builder::Arg;
use svelte_component_semantics::{SymbolId, walk_bindings};

use crate::rune_refs;

use super::super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(super) fn rewrite_state(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        state_kind: StateKind,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        if matches!(&declarator.id, BindingPattern::BindingIdentifier(_)) {
            self.rewrite_single_identifier_state(declarator, state_kind, out);
            return;
        }

        let init = declarator
            .init
            .take()
            .expect("$state destructure declarator carries an initializer");
        let value = self.take_state_init_value(init);

        let dev_label = Self::state_destructure_dev_label(&declarator.id, state_kind);

        let tmp_name = self.ident_gen.generate("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_declarators: Vec<VariableDeclarator<'a>> = Vec::new();

        walk_bindings(&declarator.id, |v| {
            let root = self.b.rid_expr(tmp_name_str);
            let expr = self.unfold_carrier_access(
                root,
                v.path,
                v.is_rest,
                v.excluded,
                &mut carriers,
                &mut carrier_declarators,
                dev_label,
                decl_kind,
            );

            let is_signal_source = self.binding_is_signal_source(v.symbol);
            leaf_declarators.push(self.build_state_leaf(
                v.symbol,
                expr,
                state_kind,
                is_signal_source,
                decl_kind,
            ));
        });

        out.push(
            self.b.ast.variable_declarator(
                SPAN,
                decl_kind,
                self.b
                    .ast
                    .binding_pattern_binding_identifier(SPAN, tmp_name_str),
                NONE,
                Some(value),
                false,
            ),
        );
        out.extend(carrier_declarators);
        out.extend(leaf_declarators);
    }

    fn rewrite_single_identifier_state(
        &mut self,
        mut declarator: VariableDeclarator<'a>,
        state_kind: StateKind,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        let BindingPattern::BindingIdentifier(binding) = &declarator.id else {
            unreachable!("single-identifier state declarator");
        };
        let binding_name: &'a str = self.b.alloc_str(binding.name.as_str());
        let sym = binding.symbol_id.get();
        let semantics = match (self.analysis, sym) {
            (Some(analysis), Some(sym)) => Some(analysis.binding_semantics(sym)),
            _ => None,
        };
        match semantics {
            Some(BindingSemantics::State(state)) => {
                self.rewrite_state_binding_init(
                    &mut declarator,
                    binding_name,
                    state_kind,
                    state.is_signal_source,
                    state.proxied,
                );
            }
            Some(BindingSemantics::OptimizedRune(opt)) => {
                self.rewrite_state_binding_init(
                    &mut declarator,
                    binding_name,
                    state_kind,
                    false,
                    opt.proxy_init,
                );
            }
            _ => {}
        }
        out.push(declarator);
    }

    fn rewrite_state_binding_init(
        &mut self,
        node: &mut VariableDeclarator<'a>,
        binding_name: &'a str,
        kind: StateKind,
        is_signal_source: bool,
        proxied: bool,
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
        if is_signal_source {
            call.callee = self.b.rid_expr("$.state");

            if call.arguments.is_empty() {
                let void_zero =
                    self.b
                        .ast
                        .expression_unary(SPAN, UnaryOperator::Void, self.b.num_expr(0.0));
                call.arguments.push(void_zero.into());
            } else if matches!(kind, StateKind::State) {
                let needs_proxy = proxied;
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
                self.b
                    .ast
                    .expression_unary(SPAN, UnaryOperator::Void, self.b.num_expr(0.0))
            } else {
                let mut dummy = Argument::from(self.b.cheap_expr());
                mem::swap(&mut call.arguments[0], &mut dummy);
                dummy.into_expression().into_inner_expression()
            };
            let is_proxy = matches!(kind, StateKind::State) && proxied;
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

    fn wrap_state_value(
        &self,
        value: Expression<'a>,
        state_kind: StateKind,
        is_signal_source: bool,
        proxied: bool,
    ) -> Expression<'a> {
        let value = value.into_inner_expression();
        match state_kind {
            StateKind::State => {
                let proxied = if proxied {
                    self.b.call_expr("$.proxy", [Arg::Expr(value)])
                } else {
                    value
                };
                if is_signal_source {
                    self.b.call_expr("$.state", [Arg::Expr(proxied)])
                } else {
                    proxied
                }
            }
            StateKind::StateRaw => {
                if is_signal_source {
                    self.b.call_expr("$.state", [Arg::Expr(value)])
                } else {
                    value
                }
            }
            StateKind::StateEager => value,
        }
    }

    fn take_state_init_value(&self, init: Expression<'a>) -> Expression<'a> {
        let Expression::CallExpression(mut call) = init else {
            unreachable!("$state destructure initializer is a $state(...) call");
        };
        if call.arguments.is_empty() {
            self.b.ast.expression_object(SPAN, self.b.ast.vec())
        } else {
            let mut dummy = Argument::from(self.b.cheap_expr());
            mem::swap(&mut call.arguments[0], &mut dummy);
            dummy.into_expression()
        }
    }

    fn binding_is_signal_source(&self, symbol: SymbolId) -> bool {
        self.analysis.is_some_and(|a| {
            a.binding_semantics(symbol)
                .state()
                .is_some_and(|state| state.is_signal_source)
        })
    }

    fn build_state_leaf(
        &self,
        symbol: SymbolId,
        accessor: Expression<'a>,
        state_kind: StateKind,
        is_signal_source: bool,
        decl_kind: VariableDeclarationKind,
    ) -> VariableDeclarator<'a> {
        let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(symbol));
        let is_proxy = matches!(state_kind, StateKind::State) && rune_refs::should_proxy(&accessor);
        let final_value = self.wrap_state_value(accessor, state_kind, is_signal_source, is_proxy);
        let final_value = if self.dev {
            if is_signal_source {
                self.b.call_expr(
                    "$.tag",
                    [Arg::Expr(final_value), Arg::Str(name.to_string())],
                )
            } else if is_proxy {
                self.b.call_expr(
                    "$.tag_proxy",
                    [Arg::Expr(final_value), Arg::Str(name.to_string())],
                )
            } else {
                final_value
            }
        } else {
            final_value
        };
        self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b.ast.binding_pattern_binding_identifier(SPAN, name),
            NONE,
            Some(final_value),
            false,
        )
    }
}
