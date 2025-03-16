use ast_builder::{BuilderFunctionArgument, TemplateLiteralPart};

use super::{context::OwnerContext, template_transformer::TemplateTransformer};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_attributes<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        if element.has_spread {
            self.attributes_spread_shortcut();
        } else {
            self.attributes_common(element, ctx);
        }
    }

    fn attributes_common<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        if !element.class_directives.is_empty() {
            todo!();
        }

        if !element.style_directives.is_empty() {
            todo!()
        }

        if !element.directives.is_empty() {
            todo!()
        }

        for attribute in element.attributes.iter() {
            match attribute {
                hir::Attribute::StringAttribute(it) => self.transform_string_attribute(it, ctx),
                hir::Attribute::BooleanAttribute(it) => self.transform_boolean_attribute(it, ctx),
                hir::Attribute::ExpressionAttribute(it) => {
                    self.transform_expression_attribute(it, ctx)
                }
                hir::Attribute::ConcatenationAttribute(it) => {
                    self.transform_concatenation_attribute(it, ctx)
                }
                hir::Attribute::SpreadAttribute(_) => unreachable!(),
            }
        }
    }

    fn attributes_spread_shortcut(&mut self) {
        //
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

    fn transform_expression_attribute<'short>(
        &self,
        attr: &hir::ExpressionAttribute<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        let mut expression = self.store.get_expression_mut(attr.expression_id);
        let expression = self.b.move_expr(&mut expression);

        let call = self.b.call_stmt(
            "$.set_attribute",
            [
                BuilderFunctionArgument::Expr(ctx.anchor()),
                BuilderFunctionArgument::Str(attr.name.into()),
                BuilderFunctionArgument::Expr(expression),
            ],
        );

        ctx.push_init(call);
    }

    fn transform_concatenation_attribute<'short>(
        &self,
        attr: &hir::ConcatenationAttribute<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        let mut parts = Vec::new();

        for part in attr.parts.iter() {
            match part {
                hir::ConcatenationAttributePart::String(value) => {
                    parts.push(TemplateLiteralPart::String(value));
                }
                hir::ConcatenationAttributePart::Expression(expression_id) => {
                    let mut expr = self.store.get_expression_mut(*expression_id);

                    let expr = self.b.move_expr(&mut *expr);

                    parts.push(TemplateLiteralPart::Expression(expr));
                }
            }
        }

        let expression = self.b.template_literal2_expr(parts);

        let call = self.b.call_stmt(
            "$.set_attribute",
            [
                BuilderFunctionArgument::Expr(ctx.anchor()),
                BuilderFunctionArgument::Str(attr.name.into()),
                BuilderFunctionArgument::Expr(expression),
            ],
        );

        ctx.push_init(call);
    }

    fn transform_class_directive<'short>(
        &self,
        _attr: &hir::ClassDirective<'hir>,
        _ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        // https://github.com/sveltejs/svelte/blob/cf56973bf0f8b2c0e9c87a1ae5393edd42911b90/packages/svelte/src/compiler/phases/3-transform/client/visitors/shared/element.js#L206
        todo!();
    }

    fn transform_bind_directive<'short>(
        &self,
        _attr: &hir::BindDirective<'hir>,
        _ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        // https://github.com/sveltejs/svelte/blob/61a0da8a5fdf5ac86431ceadfae0f54d38dc9a66/packages/svelte/src/compiler/phases/3-transform/client/visitors/BindDirective.js#L15
        todo!()
    }
}
