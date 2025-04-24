use std::borrow::Cow;

use analyze_hir::ExpressionFlags;
use ast_builder::{
    BuilderAssignmentLeft, BuilderAssignmentRight, BuilderExpression as BExpr,
    BuilderFunctionArgument, TemplateLiteralPart,
};

use crate::context::OwnerContext;

use super::template_transformer::TemplateTransformer;

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
        let expression = self.transform_expression_by_id(node.expression_id, ctx);
        let anchor = ctx.anchor();
        let expr_flags = self.analyses.get_expression_flags(node.expression_id);

        if expr_flags.has_rune_reference() {
            ctx.push_template(Cow::Borrowed(" "));

            let call = self.b.call_stmt(
                "$.set_text",
                [
                    BuilderFunctionArgument::Expr(anchor),
                    BuilderFunctionArgument::Expr(expression),
                ],
            );

            ctx.push_update(call);
        } else {
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
    }

    pub(crate) fn transform_concatenation<'short>(
        &mut self,
        node: &hir::Concatenation<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
        options: TransformInterpolationOptions,
    ) {
        let mut parts = Vec::new();
        let anchor = ctx.anchor();
        let mut expr_flags = ExpressionFlags::empty();

        for part in node.parts.iter() {
            match part {
                hir::ConcatenationPart::Text(value) => {
                    parts.push(TemplateLiteralPart::String(value));
                }
                hir::ConcatenationPart::Expression(expression_id) => {
                    let expr = self.transform_expression_by_id(*expression_id, ctx);
                    let flags = self.analyses.get_expression_flags(*expression_id);

                    parts.push(TemplateLiteralPart::Expression(expr));
                    expr_flags = expr_flags | *flags;
                }
            }
        }

        let expression = self.b.template_literal2_expr(parts);

        if expr_flags.has_rune_reference() {
            ctx.push_template(Cow::Borrowed(" "));

            let call = self.b.call_stmt(
                "$.set_text",
                [
                    BuilderFunctionArgument::Expr(anchor),
                    BuilderFunctionArgument::Expr(expression),
                ],
            );

            ctx.push_update(call);
        } else {
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
    }
}
