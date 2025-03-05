use std::borrow::Cow;

use ast_builder::{BuilderExpression as BExpr, BuilderFunctionArgument as BArg};
use hir::{AttributeId, NodeId, OwnerId};
use oxc_ast::ast::Statement;

use super::{
    context::{FragmentContext, OwnerContext},
    template_transformer::TemplateTransformer,
};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_attributes<'short>(
        &mut self,
        attributes: &Vec<AttributeId>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        todo!()
    }
}
