use oxc_ast::ast::{
    AssignmentOperator, AssignmentTarget, Expression, IdentifierReference, SimpleAssignmentTarget,
    UpdateOperator,
};
use oxc_ast_visit::VisitMut;
use oxc_span::SPAN;
use svelte_ast_builder::Builder;
use svelte_component_semantics::SymbolId;
use svelte_emit_builders::server_refs;

use crate::model::ServerTransform;
use svelte_analyze::ReferenceSemantics;

fn find_member_root<'b, 'a>(mut expr: &'b Expression<'a>) -> Option<&'b IdentifierReference<'a>> {
    loop {
        match expr {
            Expression::Identifier(id) => return Some(id),
            Expression::StaticMemberExpression(member) => expr = &member.object,
            Expression::ComputedMemberExpression(member) => expr = &member.object,
            Expression::ParenthesizedExpression(paren) => expr = &paren.expression,
            _ => return None,
        }
    }
}

fn build_compound_value<'a>(
    b: &Builder<'a>,
    operator: AssignmentOperator,
    left_read: Expression<'a>,
    right: Expression<'a>,
) -> Expression<'a> {
    if operator.is_assign() {
        return right;
    }
    if let Some(bin_op) = operator.to_binary_operator() {
        b.ast.expression_binary(SPAN, left_read, bin_op, right)
    } else if let Some(log_op) = operator.to_logical_operator() {
        b.ast.expression_logical(SPAN, left_read, log_op, right)
    } else {
        unreachable!("all compound assignment operators are either binary or logical")
    }
}

impl<'a> ServerTransform<'_, 'a> {
    pub(crate) fn rewrite_store_mutation(&mut self, node: &mut Expression<'a>) -> bool {
        match node {
            Expression::AssignmentExpression(_) => self.rewrite_store_assignment(node),
            Expression::UpdateExpression(_) => self.rewrite_store_update(node),
            _ => false,
        }
    }

    pub(crate) fn rewrite_store_assignment(&mut self, node: &mut Expression<'a>) -> bool {
        let (store_symbol, operator) = {
            let Expression::AssignmentExpression(assign) = &*node else {
                return false;
            };
            let AssignmentTarget::AssignmentTargetIdentifier(id) = &assign.left else {
                return false;
            };
            let Some(ref_id) = id.reference_id.get() else {
                return false;
            };
            match self.analysis.reference_semantics(ref_id) {
                ReferenceSemantics::StoreWrite { symbol }
                | ReferenceSemantics::StoreUpdate { symbol } => (symbol, assign.operator),
                _ => return false,
            }
        };
        let Some(base_sym) = server_refs::store_base_symbol(self.analysis, store_symbol) else {
            return false;
        };

        {
            let Expression::AssignmentExpression(assign) = &mut *node else {
                unreachable!()
            };
            self.visit_expression(&mut assign.right);
        }
        let right = {
            let Expression::AssignmentExpression(assign) = &mut *node else {
                unreachable!()
            };
            self.b.move_expr(&mut assign.right)
        };

        let dollar_name = self.analysis.scoping.symbol_name(store_symbol).to_string();
        let value = if operator.is_assign() {
            right
        } else {
            let base_read = server_refs::server_store_base_read(self.b, self.analysis, base_sym);
            let left_read = server_refs::server_store_get(self.b, &dollar_name, base_read);
            build_compound_value(self.b, operator, left_read, right)
        };
        let base_name: &str = self
            .b
            .alloc_str(self.analysis.scoping.symbol_name(base_sym));
        let base = self.b.rid_expr(base_name);
        *node = server_refs::server_store_set(self.b, base, value);
        true
    }

    fn rewrite_store_update(&mut self, node: &mut Expression<'a>) -> bool {
        let (store_symbol, is_prefix, is_decrement) = {
            let Expression::UpdateExpression(upd) = &*node else {
                return false;
            };
            let SimpleAssignmentTarget::AssignmentTargetIdentifier(id) = &upd.argument else {
                return false;
            };
            let Some(ref_id) = id.reference_id.get() else {
                return false;
            };
            match self.analysis.reference_semantics(ref_id) {
                ReferenceSemantics::StoreUpdate { symbol } => (
                    symbol,
                    upd.prefix,
                    upd.operator == UpdateOperator::Decrement,
                ),
                _ => return false,
            }
        };
        let Some(base_sym) = server_refs::store_base_symbol(self.analysis, store_symbol) else {
            return false;
        };
        let dollar_name = self.analysis.scoping.symbol_name(store_symbol).to_string();
        let base = server_refs::server_store_base_read(self.b, self.analysis, base_sym);
        *node =
            server_refs::server_store_update(self.b, &dollar_name, base, is_prefix, is_decrement);
        true
    }

    pub(crate) fn detect_store_member_mutation(
        &self,
        node: &Expression<'a>,
    ) -> Option<(String, SymbolId)> {
        let Expression::AssignmentExpression(assign) = node else {
            return None;
        };
        let member = assign.left.as_member_expression()?;
        let root = find_member_root(member.object())?;
        let ref_id = root.reference_id.get()?;
        if let ReferenceSemantics::StoreRead { symbol } = self.analysis.reference_semantics(ref_id)
        {
            let base_sym = server_refs::store_base_symbol(self.analysis, symbol)?;
            let dollar_name = self.analysis.scoping.symbol_name(symbol).to_string();
            return Some((dollar_name, base_sym));
        }
        let base_sym = self.analysis.scoping.shadowed_store_member_root(ref_id)?;
        let dollar_name = format!("${}", self.analysis.scoping.symbol_name(base_sym));
        Some((dollar_name, base_sym))
    }

    pub(crate) fn apply_store_member_mutation(
        &self,
        node: &mut Expression<'a>,
        dollar_name: &str,
        base_sym: SymbolId,
    ) {
        let base_name: &str = self
            .b
            .alloc_str(self.analysis.scoping.symbol_name(base_sym));
        let base = self.b.rid_expr(base_name);
        let mutation = self.b.move_expr(node);
        *node = server_refs::server_store_mutate(self.b, dollar_name, base, mutation);
    }
}
