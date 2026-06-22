use oxc_allocator::CloneIn;
use oxc_ast::ast::Expression;
use svelte_analyze::scope::SymbolId;
use svelte_analyze::{Evaluation, KnownValue};
use svelte_ast::{ExprRef, Node, NodeId};
use svelte_emit_builders::binding::{LegacyStateSafety, read_binding};
use svelte_emit_builders::each_item::{
    each_item_collection_read_legacy, each_item_indexed_member_legacy,
};
use svelte_emit_builders::legacy_wrap;
use svelte_emit_builders::runes::rune_get;

use super::{Codegen, CodegenError, Result};
use crate::context::Ctx;

pub(crate) fn evaluation_is_defined(eval: &Evaluation) -> bool {
    match eval {
        Evaluation::Known(KnownValue::Null | KnownValue::Undefined) => false,
        Evaluation::Known(_) | Evaluation::Defined { .. } => true,
        Evaluation::MaybeNullish { .. } => false,
    }
}

fn expr_ref_for_node(node: &Node) -> Option<&ExprRef> {
    match node {
        Node::ExpressionTag(t) => Some(&t.expression),
        Node::HtmlTag(t) => Some(&t.expression),
        Node::RenderTag(t) => Some(&t.expression),
        Node::IfBlock(b) => Some(&b.test),
        Node::EachBlock(b) => Some(&b.expression),
        Node::KeyBlock(b) => Some(&b.expression),
        Node::AwaitBlock(b) => Some(&b.expression),
        Node::SvelteElement(el) => el.this_expr(),
        _ => None,
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn take_node_expr(&mut self, id: NodeId) -> Result<Expression<'a>> {
        let node = self.ctx.query.component.store.get(id);
        let Some(expr_ref) = expr_ref_for_node(node) else {
            return CodegenError::missing_expression(id);
        };
        let expr = match self.ctx.state.parsed.take_expr(expr_ref.id()) {
            Some(expr) => expr,
            None => return CodegenError::missing_expression(id),
        };
        Ok(expr)
    }

    pub(in crate::codegen) fn clone_node_expr(&self, id: NodeId) -> Result<Expression<'a>> {
        let node = self.ctx.query.component.store.get(id);
        let Some(expr_ref) = expr_ref_for_node(node) else {
            return CodegenError::missing_expression(id);
        };
        let Some(expr) = self.ctx.state.parsed.expr(expr_ref.id()) else {
            return CodegenError::missing_expression(id);
        };
        Ok(expr.clone_in(self.ctx.b.ast.allocator))
    }

    pub(super) fn take_attr_expr(
        &mut self,
        attr_id: NodeId,
        expr_ref: &ExprRef,
    ) -> Result<Expression<'a>> {
        let expr = match self.ctx.state.parsed.take_expr(expr_ref.id()) {
            Some(expr) => expr,
            None => return CodegenError::missing_expression(attr_id),
        };
        Ok(expr)
    }

    pub(super) fn take_expr_by_ref(&mut self, expr_ref: &ExprRef) -> Option<Expression<'a>> {
        self.ctx.state.parsed.take_expr(expr_ref.id())
    }
}

pub(in crate::codegen) fn coarse_wrap<'a>(
    ctx: &Ctx<'a>,
    expr: Expression<'a>,
    data: Option<&svelte_analyze::ExpressionData>,
) -> Expression<'a> {
    legacy_wrap::maybe(&ctx.b, expr, data, |sym| legacy_dep_expr(ctx, sym))
}

pub(in crate::codegen) fn legacy_dep_expr<'a>(
    ctx: &Ctx<'a>,
    sym: SymbolId,
) -> Option<Expression<'a>> {
    use svelte_ast_builder::Arg;
    let getter = build_reactive_dep_expr_legacy(ctx, sym)?;
    Some(if uses_deep_read_state(ctx, sym) {
        ctx.b.call_expr("$.deep_read_state", [Arg::Expr(getter)])
    } else {
        getter
    })
}

fn uses_deep_read_state(ctx: &Ctx<'_>, sym: SymbolId) -> bool {
    use svelte_analyze::LegacyDependency;
    match ctx.query.view.binding_semantics(sym).legacy_dependency() {
        LegacyDependency::Deep => true,
        LegacyDependency::SelfTracked | LegacyDependency::Shallow => false,
    }
}

pub(in crate::codegen) fn build_reactive_dep_expr_legacy<'a>(
    ctx: &Ctx<'a>,
    sym: SymbolId,
) -> Option<Expression<'a>> {
    use svelte_analyze::{BindingSemantics, ConstBindingSemantics, LegacyDependency};
    if let BindingSemantics::Const(ConstBindingSemantics::ConstTag {
        destructured: true,
        owner_node,
        ..
    }) = ctx.query.view.binding_semantics(sym)
    {
        let tmp = ctx.transform_data.const_tag_tmp_names.get(&owner_node)?;
        let tmp_ref: &str = ctx.b.alloc_str(tmp);
        let field = ctx.query.symbol_name(sym);
        return Some(ctx.b.static_member_expr(rune_get(&ctx.b, tmp_ref), field));
    }
    if ctx
        .query
        .view
        .binding_semantics(sym)
        .is_each_item_indexed_legacy()
        && let Some(expr) = build_each_item_indexed_dep_legacy(ctx, sym)
    {
        return Some(expr);
    }
    let reads_directly = match ctx.query.view.binding_semantics(sym).legacy_dependency() {
        LegacyDependency::SelfTracked => true,
        LegacyDependency::Shallow | LegacyDependency::Deep => false,
    };
    if reads_directly {
        return None;
    }
    read_binding(
        &ctx.b,
        ctx.query.analysis,
        sym,
        LegacyStateSafety::FromVarDeclared,
    )
}

fn build_each_item_indexed_dep_legacy<'a>(
    ctx: &Ctx<'a>,
    item_sym: SymbolId,
) -> Option<Expression<'a>> {
    let analysis = ctx.query.analysis;
    let &source_sym = analysis.each_item_indirect_sources(item_sym)?.first()?;
    let hoisted = ctx
        .transform_data
        .each_collection_block_by_item_legacy
        .get(&item_sym)
        .and_then(|block_id| {
            ctx.transform_data
                .each_collection_internal_names_legacy
                .get(block_id)
        })
        .map(String::as_str);
    let collection = each_item_collection_read_legacy(&ctx.b, analysis, source_sym, hoisted);
    let block_id = ctx.transform_data.each_index_block_by_item.get(&item_sym)?;
    let index_name = ctx.transform_data.each_index_internal_names.get(block_id)?;
    Some(each_item_indexed_member_legacy(
        &ctx.b,
        collection,
        index_name.as_str(),
    ))
}
