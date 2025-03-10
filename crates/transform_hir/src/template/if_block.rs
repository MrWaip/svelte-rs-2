use ast_builder::{BuilderExpression, BuilderFunctionArgument, TemplateLiteralPart};
use hir::AttributeId;

use super::{context::OwnerContext, template_transformer::TemplateTransformer};

const COMMENT_NODE_ANCHOR: &str = "<!>";

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_if_block<'short>(
        &mut self,
        node: &hir::IfBlock,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        ctx.push_template(COMMENT_NODE_ANCHOR.into());
        let mut statements = vec![];
        let self_owner_id = self.store.node_to_owner(&node.node_id);
        let content_type = self.analyses.get_content_type(&self_owner_id).as_if();

        let mut test = self.store.get_expression_mut(node.test);
        let test = self.b.move_expr(&mut test);

        let consequent_fragment = self.transform_fragment(&node.consequent, self_owner_id, content_type.0);
        let consequent_id = self.analyses.generate_ident("consequent");


        dbg!(&node.consequent);
        
        let consequent = self.b.var(
            &consequent_id,
            BuilderExpression::Arrow(
                self.b
                    .arrow(self.b.params(["$$anchor"]), consequent_fragment),
            ),
        );

        dbg!(&consequent);

        statements.push(consequent);

        // let alternate_stmt = if let Some(alt) = &mut if_block.alternate {
        //     let alternate_fragment = self.transform_fragment(alt);
        //     let alternate_id = ctx.generate("alternate");

        //     let alternate = self.b.var(
        //         &alternate_id,
        //         BExpr::Arrow(
        //             self.b
        //                 .arrow(self.b.params(["$$anchor"]), alternate_fragment.body),
        //         ),
        //     );

        //     statements.push(alternate);

        //     Some(
        //         self.b
        //             .call_stmt("$$render", [BArg::Ident(&alternate_id), BArg::Bool(false)]),
        //     )
        // } else {
        //     None
        // };

        let mut args = vec![BuilderFunctionArgument::Expr(ctx.anchor())];

        let if_stmt = self.b.if_stmt(
            test,
            self.b
                .call_stmt("$$render", [BuilderFunctionArgument::Ident(&consequent_id)]),
            None,
        );

        let render = self.b.arrow(self.b.params(["$$render"]), [if_stmt]);

        args.push(BuilderFunctionArgument::Arrow(render));

        if node.is_elseif {
            args.push(BuilderFunctionArgument::Bool(true));
        }

        let if_call = self.b.call_stmt("$.if", args);

        statements.push(if_call);

        ctx.push_init(self.b.block(statements));
    }
}
