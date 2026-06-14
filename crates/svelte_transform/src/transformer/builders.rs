use oxc_allocator::Box as OxcBox;
use oxc_ast::NONE;
use oxc_ast::ast::{Argument, ComputedMemberExpression, Expression, NumberBase};
use oxc_span::SPAN;
use oxc_syntax::operator::AssignmentOperator;
use oxc_traverse::TraverseCtx;
use svelte_emit_builders::props::{props_computed_access, props_member};
use svelte_emit_builders::runes::{member_get_via_get, rune_get, rune_safe_get, rune_set};
use svelte_emit_builders::runtime::{thunk_call, untrack_ident};

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn make_rune_get(&self, name: &str) -> Expression<'a> {
        rune_get(self.b, name)
    }

    pub(crate) fn make_rune_safe_get(&self, name: &str) -> Expression<'a> {
        rune_safe_get(self.b, name)
    }

    pub(crate) fn make_rune_set(
        &self,
        name: &str,
        value: Expression<'a>,
        proxy: bool,
    ) -> Expression<'a> {
        rune_set(self.b, name, value, proxy)
    }

    pub(crate) fn make_rune_update(
        &self,
        name: &str,
        is_prefix: bool,
        is_increment: bool,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let fn_name = if is_prefix { "update_pre" } else { "update" };
        let callee = self.make_dollar_member(fn_name);
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));

        let args = if is_increment {
            ast.vec1(name_arg)
        } else {
            let delta = Argument::from(ast.expression_numeric_literal(
                SPAN,
                -1.0,
                None,
                NumberBase::Decimal,
            ));
            ast.vec_from_array([name_arg, delta])
        };

        ast.expression_call(SPAN, callee, NONE, args, false)
    }

    pub(crate) fn make_thunk_call(&self, name: &str) -> Expression<'a> {
        thunk_call(self.b, name)
    }

    pub(crate) fn make_member_get(&self, signal_name: &str, prop: &str) -> Expression<'a> {
        member_get_via_get(self.b, signal_name, prop)
    }

    pub(crate) fn make_props_access(&self, prop_name: &str) -> Expression<'a> {
        props_member(self.b, prop_name)
    }

    pub(crate) fn make_props_computed_access(&self, prop_name: &str) -> Expression<'a> {
        props_computed_access(self.b, prop_name)
    }

    pub(crate) fn make_eager_thunk(&self, arg: Expression<'a>) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("eager");
        let thunked = self.b.thunk(arg);
        ast.expression_call(SPAN, callee, NONE, ast.vec1(Argument::from(thunked)), false)
    }

    pub(crate) fn make_eager_pending(&self) -> Expression<'a> {
        let ast = self.b.ast;
        let eager_callee = self.make_dollar_member("eager");
        let pending_ref = self.make_dollar_member("pending");
        ast.expression_call(
            SPAN,
            eager_callee,
            NONE,
            ast.vec1(Argument::from(pending_ref)),
            false,
        )
    }

    pub(crate) fn make_store_unsub(
        &self,
        inner: Expression<'a>,
        dollar_name: &str,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("store_unsub");
        let inner_arg = Argument::from(inner);
        let label_arg =
            Argument::from(ast.expression_string_literal(SPAN, ast.atom(dollar_name), None));
        let stores_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom("$$stores")));
        ast.expression_call(
            SPAN,
            callee,
            NONE,
            ast.vec_from_array([inner_arg, label_arg, stores_arg]),
            false,
        )
    }

    pub(crate) fn make_untrack(&self, dollar_name: &str) -> Expression<'a> {
        untrack_ident(self.b, dollar_name)
    }

    pub(crate) fn make_invalidate_store_seq(
        &self,
        mutation: Expression<'a>,
        dollar_name: &str,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let stores_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom("$$stores")));
        let label_arg =
            Argument::from(ast.expression_string_literal(SPAN, ast.atom(dollar_name), None));
        let invalidate = ast.expression_call(
            SPAN,
            self.make_dollar_member("invalidate_store"),
            NONE,
            ast.vec_from_array([stores_arg, label_arg]),
            false,
        );
        ast.expression_sequence(SPAN, ast.vec_from_array([mutation, invalidate]))
    }

    pub(crate) fn make_source_read(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        sym: svelte_component_semantics::SymbolId,
    ) -> Expression<'a> {
        let name = self.component_scoping.symbol_name(sym);
        let semantics = analysis.binding_semantics(sym);
        if semantics.reads_via_thunk() {
            self.make_thunk_call(name)
        } else {
            self.make_rune_get(name)
        }
    }

    pub(crate) fn build_each_item_indexed_member_legacy(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        item_sym: svelte_component_semantics::SymbolId,
        index_sym: Option<svelte_component_semantics::SymbolId>,
    ) -> Option<OxcBox<'a, ComputedMemberExpression<'a>>> {
        let ast = self.b.ast;
        let &source_sym = analysis.each_item_indirect_sources(item_sym)?.first()?;
        let collection = self.make_source_read(analysis, source_sym);
        let index_name = match index_sym {
            Some(index_sym) => self.component_scoping.symbol_name(index_sym),
            None => self
                .transform_data
                .each_synthetic_index_names_legacy
                .get(&item_sym)?
                .as_str(),
        };
        let property = ast.expression_identifier(SPAN, ast.atom(index_name));
        Some(ast.alloc(ast.computed_member_expression(SPAN, collection, property, false)))
    }

    pub(crate) fn make_each_item_indexed_read_legacy(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        item_sym: svelte_component_semantics::SymbolId,
        index_sym: Option<svelte_component_semantics::SymbolId>,
    ) -> Option<Expression<'a>> {
        Some(Expression::ComputedMemberExpression(
            self.build_each_item_indexed_member_legacy(analysis, item_sym, index_sym)?,
        ))
    }

    pub(crate) fn make_each_item_invalidate_seq(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        mutation: Expression<'a>,
        source_syms: &[svelte_component_semantics::SymbolId],
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let body_expr = match source_syms {
            [single] => self.make_source_read(analysis, *single),
            many => {
                let mut elems = ast.vec_with_capacity(many.len());
                for &sym in many {
                    elems.push(self.make_source_read(analysis, sym));
                }
                ast.expression_sequence(SPAN, elems)
            }
        };
        let thunk = self.b.thunk_in_scope(body_expr, ctx.current_scope_id());
        let invalidate = ast.expression_call(
            SPAN,
            self.make_dollar_member("invalidate_inner_signals"),
            NONE,
            ast.vec1(Argument::from(thunk)),
            false,
        );
        let mut seq_elems = ast.vec_from_array([mutation, invalidate]);
        if let Some(store_sym) = source_syms
            .iter()
            .copied()
            .find(|&sym| analysis.binding_semantics(sym).is_store())
        {
            let store_name = self.component_scoping.symbol_name(store_sym);
            let stores_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom("$$stores")));
            let label_arg =
                Argument::from(ast.expression_string_literal(SPAN, ast.atom(store_name), None));
            let invalidate_store = ast.expression_call(
                SPAN,
                self.make_dollar_member("invalidate_store"),
                NONE,
                ast.vec_from_array([stores_arg, label_arg]),
                false,
            );
            seq_elems.push(invalidate_store);
        }
        ast.expression_sequence(SPAN, seq_elems)
    }

    pub(crate) fn make_legacy_state_mutate(
        &self,
        name: &str,
        mutation: Expression<'a>,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member("mutate");
        let name_arg = Argument::from(ast.expression_identifier(SPAN, ast.atom(name)));
        let mutation_arg = Argument::from(mutation);
        ast.expression_call(
            SPAN,
            callee,
            NONE,
            ast.vec_from_array([name_arg, mutation_arg]),
            false,
        )
    }

    pub(crate) fn build_compound_value(
        &self,
        operator: AssignmentOperator,
        left_read: Expression<'a>,
        right: Expression<'a>,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        if operator.is_assign() {
            return right;
        }
        if let Some(bin_op) = operator.to_binary_operator() {
            ast.expression_binary(SPAN, left_read, bin_op, right)
        } else if let Some(log_op) = operator.to_logical_operator() {
            ast.expression_logical(SPAN, left_read, log_op, right)
        } else {
            unreachable!("all compound assignment operators are either binary or logical")
        }
    }

    pub(crate) fn make_dollar_member(&self, method: &str) -> Expression<'a> {
        let ast = self.b.ast;
        let object = ast.expression_identifier(SPAN, ast.atom("$"));
        let property = ast.identifier_name(SPAN, ast.atom(method));
        Expression::StaticMemberExpression(
            ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
        )
    }

    pub(crate) fn make_dollar_call(&self, method: &str, arg: Expression<'a>) -> Expression<'a> {
        let ast = self.b.ast;
        let callee = self.make_dollar_member(method);
        ast.expression_call(SPAN, callee, NONE, ast.vec1(Argument::from(arg)), false)
    }
}
