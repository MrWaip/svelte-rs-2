use oxc_allocator::Box as OxcBox;
use oxc_allocator::CloneIn;
use oxc_ast::NONE;
use oxc_ast::ast::{Argument, ComputedMemberExpression, Expression, NumberBase, Statement};
use oxc_span::SPAN;
use oxc_syntax::operator::AssignmentOperator;
use oxc_traverse::TraverseCtx;
use svelte_analyze::EachIndexStrategy;
use svelte_ast::Node;
use svelte_component_semantics::ReferenceId;
use svelte_emit_builders::each_item;
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

    pub(crate) fn each_item_member_root_read_legacy(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        item_sym: svelte_component_semantics::SymbolId,
        name: &str,
    ) -> Expression<'a> {
        if analysis
            .binding_semantics(item_sym)
            .reads_via_each_item_accessor()
        {
            self.make_thunk_call(name)
        } else {
            self.make_rune_get(name)
        }
    }

    pub(crate) fn each_item_collection_read_legacy(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        item_sym: svelte_component_semantics::SymbolId,
        source_sym: svelte_component_semantics::SymbolId,
    ) -> Expression<'a> {
        let hoisted = self
            .transform_data
            .each_collection_block_by_item_legacy
            .get(&item_sym)
            .and_then(|block_id| {
                self.transform_data
                    .each_collection_internal_names_legacy
                    .get(block_id)
            })
            .map(String::as_str);
        each_item::each_item_collection_read_legacy(self.b, analysis, source_sym, hoisted)
    }

    fn rebuilt_each_collection_legacy(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        item_sym: svelte_component_semantics::SymbolId,
    ) -> Option<Expression<'a>> {
        let hoisted = self
            .transform_data
            .each_collection_block_by_item_legacy
            .get(&item_sym)
            .and_then(|block_id| {
                self.transform_data
                    .each_collection_internal_names_legacy
                    .get(block_id)
            });
        if hoisted.is_some() {
            return None;
        }
        let &block_id = self
            .transform_data
            .each_index_block_by_item
            .get(&item_sym)?;
        let Node::EachBlock(block) = self.component?.store.get(block_id) else {
            return None;
        };
        let oxc_id = block.expression.id();
        let raw = self
            .parsed
            .as_ref()?
            .expr(oxc_id)?
            .clone_in_with_semantic_ids(self.b.ast.allocator);
        let expr_data = analysis.expression_data_by_oxc(oxc_id)?;
        Some(each_item::wrap_each_collection_legacy(
            self.b,
            analysis,
            raw,
            expr_data,
            self.component_scoping.root_scope_id(),
        ))
    }

    pub(crate) fn build_each_item_indexed_member_legacy(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        item_sym: svelte_component_semantics::SymbolId,
        index_sym: Option<svelte_component_semantics::SymbolId>,
        index_read: EachIndexStrategy,
    ) -> Option<OxcBox<'a, ComputedMemberExpression<'a>>> {
        let ast = self.b.ast;
        let &source_sym = analysis.each_item_indirect_sources(item_sym)?.first()?;
        let collection = self
            .rebuilt_each_collection_legacy(analysis, item_sym)
            .unwrap_or_else(|| {
                self.each_item_collection_read_legacy(analysis, item_sym, source_sym)
            });
        let index_name = match index_sym {
            Some(index_sym) => self.component_scoping.symbol_name(index_sym),
            None => {
                let block_id = self
                    .transform_data
                    .each_index_block_by_item
                    .get(&item_sym)?;
                self.transform_data
                    .each_index_internal_names
                    .get(block_id)?
                    .as_str()
            }
        };
        let property = match index_read {
            EachIndexStrategy::Signal => self.make_rune_get(index_name),
            EachIndexStrategy::Direct => ast.expression_identifier(SPAN, ast.atom(index_name)),
        };
        Some(ast.alloc(ast.computed_member_expression(SPAN, collection, property, false)))
    }

    pub(crate) fn make_each_item_indexed_read_legacy(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        item_sym: svelte_component_semantics::SymbolId,
        index_sym: Option<svelte_component_semantics::SymbolId>,
        index_read: EachIndexStrategy,
    ) -> Option<Expression<'a>> {
        Some(Expression::ComputedMemberExpression(
            self.build_each_item_indexed_member_legacy(analysis, item_sym, index_sym, index_read)?,
        ))
    }

    pub(crate) fn make_each_item_invalidate_seq(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        mutation: Expression<'a>,
        source_syms: &[svelte_component_semantics::SymbolId],
        item_sym: svelte_component_semantics::SymbolId,
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> Expression<'a> {
        let ast = self.b.ast;
        let body_expr = match source_syms {
            [single] => self.each_item_collection_read_legacy(analysis, item_sym, *single),
            many => {
                let mut elems = ast.vec_with_capacity(many.len());
                for &sym in many {
                    elems.push(self.each_item_collection_read_legacy(analysis, item_sym, sym));
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

    pub(crate) fn maybe_wrap_legacy_indirect_invalidate(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        expr: Expression<'a>,
        root_ref_id: ReferenceId,
        ctx: &mut TraverseCtx<'a, ()>,
    ) -> Expression<'a> {
        let Some(root_sym) = analysis.symbol_for_reference(root_ref_id) else {
            return expr;
        };
        let Some(indirect_syms) = analysis.legacy_indirect_bindings(root_sym) else {
            return expr;
        };
        if indirect_syms.is_empty() {
            return expr;
        }
        let ast = self.b.ast;
        let mut statements: Vec<Statement<'a>> = Vec::with_capacity(indirect_syms.len());
        for &sym in indirect_syms {
            let getter = self.make_legacy_indirect_getter(analysis, root_sym, sym);
            statements.push(self.b.expr_stmt(getter));
        }
        let thunk = self.b.thunk_block(statements);
        self.b
            .seed_arrow_scope(&thunk, Some(ctx.current_scope_id()));
        let invalidate = ast.expression_call(
            SPAN,
            self.make_dollar_member("invalidate_inner_signals"),
            NONE,
            ast.vec1(Argument::from(thunk)),
            false,
        );
        ast.expression_sequence(SPAN, ast.vec_from_array([expr, invalidate]))
    }

    fn make_legacy_indirect_getter(
        &self,
        analysis: &svelte_analyze::AnalysisData<'_>,
        item_sym: svelte_component_semantics::SymbolId,
        sym: svelte_component_semantics::SymbolId,
    ) -> Expression<'a> {
        if analysis.binding_semantics(sym).is_reactive() {
            return self.each_item_collection_read_legacy(analysis, item_sym, sym);
        }
        let name = self.component_scoping.symbol_name(sym);
        self.b
            .ast
            .expression_identifier(SPAN, self.b.ast.atom(name))
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
