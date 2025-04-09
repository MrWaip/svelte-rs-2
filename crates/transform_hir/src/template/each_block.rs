use ast_builder::{BuilderExpression, BuilderFunctionArgument, BuilderStatement};

use crate::context::OwnerContext;

use super::template_transformer::TemplateTransformer;

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_each_block<'short>(
        &mut self,
        node: &hir::EachBlock,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        let self_owner_id = self.store.node_to_owner(&node.node_id);
        let collection_expr = self.transform_expression_by_id(node.collection, ctx);
        //

        let collection = self.b.arrow_expr(
            self.b.params([]),
            [self.b.stmt(BuilderStatement::Expr(collection_expr))],
        );
        let index = self.b.rid("$.index");

        let content_type = self
            .analyses
            .get_content_type(&self_owner_id)
            .as_common_or_empty();

        let fragment = self.transform_fragment(&node.node_ids, self_owner_id, content_type);

        let fragment_fn = self
            .b
            .arrow_expr(self.b.params(["$$anchor", "item"]), fragment);

        let res = self.b.call_stmt(
            "$.each",
            [
                BuilderFunctionArgument::Expr(ctx.anchor()),
                BuilderFunctionArgument::Num(16.0),
                BuilderFunctionArgument::Expr(collection),
                BuilderFunctionArgument::IdentRef(index),
                BuilderFunctionArgument::Expr(fragment_fn),
            ],
        );

        ctx.push_init(res);
    }
}
