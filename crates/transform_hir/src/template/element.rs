use std::borrow::Cow;

use ast_builder::BuilderFunctionArgument;
use hir::OwnerId;

use super::{
    context::{FragmentContext, OwnerContext},
    interpolation::TransformInterpolationOptions,
    template_transformer::TemplateTransformer,
};

impl<'hir> TemplateTransformer<'hir> {
    pub(crate) fn transform_element<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
    ) {
        if element.is_noscript() {
            ctx.push_template("<noscript></noscript>".into());
            return;
        }

        if element.is_video() || element.is_custom_element() {
            todo!()
        }

        if element.is_script() {
            todo!();
        }

        let self_owner_id = self.store.node_to_owner(&element.node_id);
        let content_type = self.analyses.get_common_content_type(&self_owner_id);
        ctx.push_template(Cow::Owned(format!("<{}", &element.name)));

        self.transform_attributes(&element, ctx);

        ctx.push_template(Cow::Borrowed(">"));

        if content_type.any_interpolation_like() {
            self.element_text_shortcut(element, ctx, self_owner_id);
        } else {
            self.element_common(element, ctx, self_owner_id);
        }

        if !element.self_closing {
            ctx.push_template(Cow::Owned(format!("</{}>", &element.name)));
        }
    }

    fn element_common<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
        self_owner_id: OwnerId,
    ) {
        let is_dynamic = self.analyses.is_dynamic(&element.node_id);
        let content_type = self.analyses.get_common_content_type(&self_owner_id);
        let anchor = self
            .b
            .call_expr("$.child", [BuilderFunctionArgument::Expr(ctx.anchor())]);

        let mut fragment_ctx = FragmentContext::new();
        let owner_ctx = OwnerContext::new(&mut fragment_ctx, anchor, self.b, self_owner_id);

        self.transform_nodes(&element.node_ids, owner_ctx);

        if is_dynamic {
            ctx.append_template(&mut fragment_ctx.template);
            ctx.append_before_init(&mut fragment_ctx.before_init);
            ctx.append_init(&mut fragment_ctx.init);
            ctx.append_update(&mut fragment_ctx.update);
            ctx.append_after_update(&mut fragment_ctx.after_update);
        } else {
            ctx.append_template(&mut fragment_ctx.template);
        }

        if is_dynamic && content_type.none_text() {
            ctx.push_init(self.b.call_stmt(
                "$.reset",
                [BuilderFunctionArgument::Expr(
                    self.b.clone_expr(&ctx.anchor()),
                )],
            ));
        }
    }

    fn element_text_shortcut<'short>(
        &mut self,
        element: &hir::Element<'hir>,
        ctx: &mut OwnerContext<'hir, 'short>,
        self_owner_id: OwnerId,
    ) {
        let node = self.store.get_node(*element.node_ids.first().unwrap());

        let anchor = ctx.anchor();
        let mut owner_ctx = OwnerContext::new(&mut ctx.fragment, anchor, self.b, self_owner_id);

        let opts = TransformInterpolationOptions {
            need_empty_template: false,
            property: super::interpolation::InterpolationProperty::TextContent,
        };

        match node {
            hir::Node::Interpolation(interpolation) => {
                self.transform_interpolation(interpolation, &mut owner_ctx, opts);
            }

            hir::Node::Concatenation(concatenation) => {
                self.transform_concatenation(concatenation, &mut owner_ctx, opts);
            }
            _ => unreachable!(),
        };
    }
}
