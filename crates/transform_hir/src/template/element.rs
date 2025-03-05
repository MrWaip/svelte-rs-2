use std::borrow::Cow;

use ast_builder::{BuilderExpression as BExpr, BuilderFunctionArgument as BArg};
use hir::{NodeId, OwnerId};
use oxc_ast::ast::Statement;

use super::{
    context::{FragmentContext, OwnerContext},
    template_transformer::TemplateTransformer,
};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_element<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        ctx.push_template(Cow::Owned(format!("<{}", &element.name)));

        if !element.attributes.is_empty() {
            self.transform_attributes(&element.attributes, ctx);
        }

        ctx.push_template(Cow::Borrowed(">"));

        if !element.self_closing {
            ctx.push_template(Cow::Owned(format!("</{}>", &element.name)));
        }
    }
}
