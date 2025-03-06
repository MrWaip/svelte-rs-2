
use hir::AttributeId;

use super::{
    context::OwnerContext,
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
