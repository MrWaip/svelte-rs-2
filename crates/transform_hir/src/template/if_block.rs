use ast_builder::{BuilderExpression, BuilderFunctionArgument};

use crate::context::OwnerContext;

use super::template_transformer::TemplateTransformer;

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_if_block<'short>(
        &mut self,
        node: &hir::IfBlock,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        ctx.push_template(Self::COMMENT_NODE_ANCHOR.into());
        let mut statements = vec![];
        let self_owner_id = self.store.node_to_owner(&node.node_id);
        let content_type = self.analyses.get_content_type(&self_owner_id).as_if();

        let mut test = self.store.get_expression_mut(node.test);
        let test = self.b.move_expr(&mut test);

        let consequent_fragment =
            self.transform_fragment(&node.consequent, self_owner_id, content_type.0);
        let consequent_id = self.analyses.generate_ident("consequent");

        let consequent = self.b.var(
            &consequent_id,
            BuilderExpression::Arrow(
                self.b
                    .arrow(self.b.params(["$$anchor"]), consequent_fragment),
            ),
        );

        statements.push(consequent);

        let alternate_stmt = if let Some(alt) = &node.alternate {
            let alternate_id = self.analyses.generate_ident("alternate");
            let alternate_fragment = self.transform_fragment(alt, self_owner_id, content_type.1);

            let mut args = vec!["$$anchor"];

            if alt.len() == 1
                && self
                    .store
                    .lookup_node(alt[0], |node| node.is_elseif_block())
            {
                args.push("$$elseif")
            }

            let alternate = self.b.var(
                &alternate_id,
                BuilderExpression::Arrow(self.b.arrow(self.b.params(args), alternate_fragment)),
            );

            statements.push(alternate);

            Some(self.b.call_stmt(
                "$$render",
                [
                    BuilderFunctionArgument::Ident(&alternate_id),
                    BuilderFunctionArgument::Bool(false),
                ],
            ))
        } else {
            None
        };

        let mut args = vec![];

        if node.is_elseif {
            args.push(BuilderFunctionArgument::Ident("$$anchor"));
        } else {
            args.push(BuilderFunctionArgument::Expr(ctx.anchor()));
        }

        let if_stmt = self.b.if_stmt(
            test,
            self.b
                .call_stmt("$$render", [BuilderFunctionArgument::Ident(&consequent_id)]),
            alternate_stmt,
        );

        let render = self.b.arrow(self.b.params(["$$render"]), [if_stmt]);

        args.push(BuilderFunctionArgument::Arrow(render));

        if node.is_elseif {
            args.push(BuilderFunctionArgument::Ident("$$elseif"));
        }

        let if_call = self.b.call_stmt("$.if", args);

        statements.push(if_call);

        ctx.push_init(self.b.block(statements));
    }
}
