use std::collections::HashMap;
use std::mem;

use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, BindingPattern, Expression, Statement, VariableDeclarationKind, VariableDeclarator,
};
use oxc_span::SPAN;

use svelte_analyze::{
    BindingSemantics, DeclaratorSemantics, RuneKind, StateBindingSemantics, StateKind,
};
use svelte_ast_builder::Arg;
use svelte_component_semantics::{Access, SymbolId, walk_bindings};
use svelte_emit_builders::binding_pattern as bp;

use crate::rune_refs::should_proxy;

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn binding_pattern_supported(&self, declarator: &VariableDeclarator<'a>) -> bool {
        let node = declarator.node_id();
        matches!(
            self.analysis.map(|a| a.declarator_semantics(node)),
            Some(DeclaratorSemantics::RuneStateDestructure { .. })
        )
    }

    pub(crate) fn rewrite_binding_pattern(
        &mut self,
        decl_kind: VariableDeclarationKind,
        declarator: VariableDeclarator<'a>,
    ) -> Statement<'a> {
        let node = declarator.node_id();
        let semantics = self
            .analysis
            .map(|a| a.declarator_semantics(node))
            .unwrap_or(DeclaratorSemantics::None);

        match semantics {
            DeclaratorSemantics::RuneStateDestructure { kind } => {
                self.rewrite_state_destructure(decl_kind, declarator, kind)
            }
            DeclaratorSemantics::EachItem { .. }
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::LetCarrier { .. } => {
                unreachable!("template-stage declarator kind reached the transform unfold door")
            }
            DeclaratorSemantics::None
            | DeclaratorSemantics::PropsIdentifier { .. }
            | DeclaratorSemantics::PropsObject { .. }
            | DeclaratorSemantics::LegacyStateDestructure { .. }
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => {
                unimplemented!("script declarator kind not yet routed through rewrite_binding_pattern")
            }
        }
    }

    fn rewrite_state_destructure(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        state_kind: StateKind,
    ) -> Statement<'a> {
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
        let leaf_semantics = self.state_leaf_semantics(&declarator.id);

        let tmp_name = self.gen_unique_name("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut carriers: HashMap<String, &'a str> = HashMap::new();
        let mut carrier_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_declarators: Vec<VariableDeclarator<'a>> = Vec::new();
        let mut leaf_index = 0usize;

        walk_bindings(&declarator.id, |v| {
            let mut expr = self.b.rid_expr(tmp_name_str);

            for (i, step) in v.path.iter().enumerate() {
                match step.access {
                    Access::Key { key, computed } => {
                        expr = bp::member_access(self.b, expr, key, computed);
                    }
                    Access::Index { index, len, has_rest } => {
                        let prefix = bp::serialize_prefix(&v.path[..i]);
                        let name = self.ensure_carrier_declarator(
                            &mut carriers,
                            &mut carrier_declarators,
                            &prefix,
                            expr,
                            carrier_count(len, has_rest),
                            dev_label,
                            decl_kind,
                        );
                        let get = self.b.call_expr("$.get", [Arg::Ident(name)]);
                        expr = self.b.computed_member_expr(get, self.b.num_expr(index as f64));
                    }
                    Access::Slice { from } => {
                        let prefix = bp::serialize_prefix(&v.path[..i]);
                        let name = self.ensure_carrier_declarator(
                            &mut carriers,
                            &mut carrier_declarators,
                            &prefix,
                            expr,
                            None,
                            dev_label,
                            decl_kind,
                        );
                        let get = self.b.call_expr("$.get", [Arg::Ident(name)]);
                        let slice = self.b.static_member_expr(get, "slice");
                        expr = self.b.call_expr_callee(slice, [Arg::Num(from as f64)]);
                    }
                }
                if let Some(default) = step.default {
                    expr = bp::fallback(self.b, expr, default);
                }
            }

            if v.is_rest {
                expr = bp::exclude_from_object(self.b, expr, v.excluded);
            }

            let binding_semantic = leaf_semantics
                .get(leaf_index)
                .copied()
                .expect("per-leaf state binding semantics present for every destructure leaf");
            leaf_index += 1;
            leaf_declarators.push(self.build_state_leaf(
                v.symbol,
                expr,
                rune_kind,
                binding_semantic,
                decl_kind,
            ));
        });

        let mut declarators = Vec::with_capacity(1 + carrier_declarators.len() + leaf_declarators.len());
        declarators.push(self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(tmp_name_str)),
            NONE,
            Some(value),
            false,
        ));
        declarators.extend(carrier_declarators);
        declarators.extend(leaf_declarators);

        let decl = self.b.ast.variable_declaration(
            SPAN,
            decl_kind,
            self.b.ast.vec_from_iter(declarators),
            false,
        );
        Statement::VariableDeclaration(self.b.alloc(decl))
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

    fn state_leaf_semantics(&self, pattern: &BindingPattern<'a>) -> Vec<StateBindingSemantics> {
        let mut first = None;
        walk_bindings(pattern, |v| {
            if first.is_none() {
                first = Some(v.symbol);
            }
        });
        match first.and_then(|sym| self.analysis.map(|a| a.binding_semantics(sym))) {
            Some(BindingSemantics::State(state)) => state.binding_semantics.to_vec(),
            _ => Vec::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn ensure_carrier_declarator(
        &mut self,
        carriers: &mut HashMap<String, &'a str>,
        carrier_declarators: &mut Vec<VariableDeclarator<'a>>,
        prefix: &str,
        source: Expression<'a>,
        count: Option<u32>,
        dev_label: Option<&'static str>,
        decl_kind: VariableDeclarationKind,
    ) -> &'a str {
        if let Some(name) = carriers.get(prefix) {
            return name;
        }
        let name_owned = self.ident_gen.generate("$$array");
        let name: &'a str = self.b.alloc_str(&name_owned);
        let derived = bp::to_array_derived(self.b, source, count);
        let derived = match dev_label.filter(|_| self.dev) {
            Some(label) => self
                .b
                .call_expr("$.tag", [Arg::Expr(derived), Arg::Str(label.to_string())]),
            None => derived,
        };
        let declarator = self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
            NONE,
            Some(derived),
            false,
        );
        carrier_declarators.push(declarator);
        carriers.insert(prefix.to_string(), name);
        name
    }

    fn build_state_leaf(
        &self,
        symbol: SymbolId,
        accessor: Expression<'a>,
        rune_kind: RuneKind,
        binding_semantic: StateBindingSemantics,
        decl_kind: VariableDeclarationKind,
    ) -> VariableDeclarator<'a> {
        let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(symbol));
        let is_signal_source = matches!(
            binding_semantic,
            StateBindingSemantics::StateSignal { .. } | StateBindingSemantics::StateRawSignal
        );
        let is_proxy = matches!(rune_kind, RuneKind::State) && should_proxy(&accessor);
        let final_value = self.wrap_state_value(accessor, rune_kind, is_signal_source);
        let final_value = if self.dev {
            if is_signal_source {
                self.b
                    .call_expr("$.tag", [Arg::Expr(final_value), Arg::Str(name.to_string())])
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

fn carrier_count(len: u32, has_rest: bool) -> Option<u32> {
    if has_rest { None } else { Some(len) }
}
