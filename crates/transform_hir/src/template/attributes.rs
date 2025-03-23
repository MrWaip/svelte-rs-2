use ast_builder::{BuilderExpression, BuilderFunctionArgument, TemplateLiteralPart};

use super::{context::OwnerContext, template_transformer::TemplateTransformer};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_attributes<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        self.element_specific(element, ctx);

        if element.attributes.has_spread() {
            self.attributes_spread_shortcut(element, ctx);
        } else {
            self.attributes_common(element, ctx);
        }
    }

    fn element_specific<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        if element.is_input() {
            let value_attr = element
                .attributes
                .get_attribute_by_name("value")
                .or_else(|| element.attributes.get_attribute_by_name("checked"));

            let default_value_attr = element
                .attributes
                .get_attribute_by_name("defaultValue")
                .or_else(|| element.attributes.get_attribute_by_name("defaultChecked"));

            let has_value_attribute = value_attr.is_some_and(|attr| attr.contains_expression());
            let has_default_value_attribute = default_value_attr.is_some();

            if !has_default_value_attribute
                && (element.attributes.has_spread()
                    || element.attributes.has_binding("value")
                    || element.attributes.has_binding("checked")
                    || element.attributes.has_binding("group")
                    || (!element.attributes.has_binding("group") && has_value_attribute))
            {
                ctx.push_init(self.b.call_stmt(
                    "$.remove_input_defaults",
                    [BuilderFunctionArgument::Expr(ctx.anchor())],
                ));
            }
        }
    }

    fn attributes_common<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        for attribute in element.attributes.iter_attrs() {
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

    fn attributes_spread_shortcut<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        let attributes_id = self.analyses.generate_ident("attributes");
        let mut props = vec![];

        for attribute in element.attributes.iter_attrs() {
            match attribute {
                hir::Attribute::StringAttribute(attr) => {
                    props.push(self.b.init_prop(
                        attr.name,
                        self.b.expr(BuilderExpression::Str(attr.value.into())),
                    ));
                }
                hir::Attribute::ExpressionAttribute(attr) => {
                    props.push(
                        self.b
                            .init_prop(attr.name, self.take_expression(attr.expression_id)),
                    );
                }
                hir::Attribute::SpreadAttribute(attr) => {
                    props.push(self.b.spread_prop(self.take_expression(attr.expression_id)));
                }
                hir::Attribute::BooleanAttribute(attr) => {
                    props.push(self.b.init_prop(attr.name, self.b.bool_expr(true)));
                }
                hir::Attribute::ConcatenationAttribute(attr) => {
                    let parts = self.concatenation_to_template(&attr.parts);
                    props.push(
                        self.b
                            .init_prop(attr.name, self.b.template_literal2_expr(parts)),
                    );
                }
            }
        }

        let args = vec![
            BuilderFunctionArgument::Expr(ctx.anchor()),
            BuilderFunctionArgument::Ident(&attributes_id),
            BuilderFunctionArgument::Expr(self.b.object_expr(props)),
        ];
        let call = self.b.call_expr("$.set_attributes", args);

        ctx.push_init(self.b.let_stmt(&attributes_id, None));

        let update = self.b.assignment_expression_stmt(
            ast_builder::BuilderAssignmentLeft::Ident(&attributes_id),
            ast_builder::BuilderAssignmentRight::Expr(call),
        );

        ctx.push_update(update);
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
        let parts = self.concatenation_to_template(&attr.parts);
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

    fn concatenation_to_template(
        &self,
        in_parts: &Vec<hir::ConcatenationAttributePart<'hir>>,
    ) -> Vec<TemplateLiteralPart<'hir>> {
        let mut parts = Vec::new();

        for part in in_parts.iter() {
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

        return parts;
    }
}
