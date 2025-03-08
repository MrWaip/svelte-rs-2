use hir::AttributeId;

use super::{context::OwnerContext, template_transformer::TemplateTransformer};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_attributes<'short>(
        &mut self,
        attributes: &Vec<AttributeId>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        for attribute_id in attributes.iter() {
            let attribute = self.store.get_attribute(*attribute_id);

            match attribute {
                hir::Attribute::StringAttribute(it) => self.transform_string_attribute(it, ctx),
                hir::Attribute::ExpressionAttribute(it) => todo!(),
                hir::Attribute::ClassDirective(it) => todo!(),
                hir::Attribute::BindDirective(it) => todo!(),
                hir::Attribute::BooleanAttribute(it) => self.transform_boolean_attribute(it, ctx),
                hir::Attribute::ConcatenationAttribute(it) => todo!(),
            }
        }
    }

    fn transform_string_attribute<'short>(
        &mut self,
        attr: &hir::StringAttribute<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        ctx.push_template(" ".into());
        ctx.push_template(attr.name.into());
        let value = attr.value;
        ctx.push_template(format!("=\"{value}\"").into());
    }

    fn transform_boolean_attribute<'short>(
        &self,
        attr: &hir::BooleanAttribute<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        ctx.push_template(" ".into());
        ctx.push_template(attr.name.into());
        ctx.push_template("=\"\"".into());
    }
}
