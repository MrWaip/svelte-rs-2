use std::collections::HashMap;
use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{Argument, Expression, VariableDeclarationKind, VariableDeclarator};
use oxc_span::SPAN;

use svelte_analyze::{BindingSemantics, RuneKind, StateKind};
use svelte_ast_builder::Arg;
use svelte_component_semantics::{SymbolId, walk_bindings};

use crate::rune_refs::should_proxy;

use super::super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(super) fn rewrite_state(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        state_kind: StateKind,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        let rune_kind = match state_kind {
            StateKind::State => RuneKind::State,
            StateKind::StateRaw => RuneKind::StateRaw,
            StateKind::StateEager => RuneKind::StateEager,
        };

        let init = declarator
            .init
            .take()
            .expect("$state destructure declarator carries an initializer");
        let value = self.take_state_init_value(init);

        let dev_label = Self::state_destructure_dev_label(&declarator.id, rune_kind);

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
                rune_kind,
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
                    .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(tmp_name_str)),
                NONE,
                Some(value),
                false,
            ),
        );
        out.extend(carrier_declarators);
        out.extend(leaf_declarators);
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
        matches!(
            self.analysis.map(|a| a.binding_semantics(symbol)),
            Some(BindingSemantics::State(state)) if state.is_signal_source
        )
    }

    fn build_state_leaf(
        &self,
        symbol: SymbolId,
        accessor: Expression<'a>,
        rune_kind: RuneKind,
        is_signal_source: bool,
        decl_kind: VariableDeclarationKind,
    ) -> VariableDeclarator<'a> {
        let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(symbol));
        let is_proxy = matches!(rune_kind, RuneKind::State) && should_proxy(&accessor);
        let final_value = self.wrap_state_value(accessor, rune_kind, is_signal_source);
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
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
            NONE,
            Some(final_value),
            false,
        )
    }
}
