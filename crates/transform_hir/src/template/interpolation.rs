use std::borrow::Cow;

use ast_builder::{
    BuilderAssignmentLeft, BuilderAssignmentRight, BuilderExpression as BExpr, TemplateLiteralPart,
};

use crate::context::OwnerContext;

use super::{ template_transformer::TemplateTransformer};

#[derive(Debug, Default)]
pub enum InterpolationProperty {
    #[default]
    NodeValue,
    TextContent,
}

impl InterpolationProperty {
    pub fn to_str(&self) -> &str {
        match self {
            InterpolationProperty::NodeValue => "nodeValue",
            InterpolationProperty::TextContent => "textContent",
        }
    }
}

#[derive(Debug)]
pub struct TransformInterpolationOptions {
    pub property: InterpolationProperty,
    pub need_empty_template: bool,
}

impl Default for TransformInterpolationOptions {
    fn default() -> Self {
        Self {
            property: Default::default(),
            need_empty_template: true,
        }
    }
}

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_interpolation<'short>(
        &mut self,
        node: &hir::Interpolation,
        ctx: &mut OwnerContext<'hir, 'short>,
        options: TransformInterpolationOptions,
    ) {
        let mut expression = self.store.get_expression_mut(node.expression_id);
        let expression = self.b.move_expr(&mut expression);
        let anchor = ctx.anchor();

        let member = self.b.static_member_expr(anchor, options.property.to_str());

        let set_text = self.b.assignment_expression_stmt(
            BuilderAssignmentLeft::StaticMemberExpression(member),
            BuilderAssignmentRight::Expr(expression),
        );

        if options.need_empty_template {
            ctx.push_template(Cow::Borrowed(" "));
        }

        ctx.push_init(set_text);
    }

    pub(crate) fn transform_concatenation<'short>(
        &mut self,
        node: &hir::Concatenation<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
        options: TransformInterpolationOptions,
    ) {
        let mut parts = Vec::new();
        let anchor = ctx.anchor();

        for part in node.parts.iter() {
            match part {
                hir::ConcatenationPart::Text(value) => {
                    parts.push(TemplateLiteralPart::String(value));
                }
                hir::ConcatenationPart::Expression(expression_id) => {
                    let mut expr = self.store.get_expression_mut(*expression_id);

                    let expr = self.b.move_expr(&mut *expr);

                    parts.push(TemplateLiteralPart::Expression(expr));
                }
            }
        }

        let expression = self.b.template_literal2(parts);

        let member = self.b.static_member_expr(anchor, options.property.to_str());

        let set_text = self.b.assignment_expression_stmt(
            BuilderAssignmentLeft::StaticMemberExpression(member),
            BuilderAssignmentRight::Expr(self.b.expr(BExpr::TemplateLiteral(expression))),
        );

        if options.need_empty_template {
            ctx.push_template(Cow::Borrowed(" "));
        }

        ctx.push_init(set_text);
    }
}
