use oxc_ast::ast::Expression;
use svelte_ast::{Component, NodeId};

use crate::types::data::{
    AnalysisData, ParserResult, RenderTagArgPlan, RenderTagCalleeMode, RenderTagPlan,
};

pub(crate) fn classify_render_tags(
    parsed: &mut ParserResult<'_>,
    component: &Component,
    data: &mut AnalysisData<'_>,
    source: &str,
    runes: bool,
) {
    let root = data.scoping.root_scope_id();
    let mut visitor = RenderTagClassifier { parsed };
    let mut ctx = crate::walker::VisitContext::new(
        root,
        data,
        &component.store,
        source,
        runes,
        "Self",
        "Self.svelte",
    );
    crate::walker::walk_template(&component.fragment, &mut ctx, &mut [&mut visitor]);
}

struct RenderTagClassifier<'a, 'b> {
    parsed: &'b mut ParserResult<'a>,
}

impl crate::walker::TemplateVisitor for RenderTagClassifier<'_, '_> {
    fn visit_render_tag(
        &mut self,
        tag: &svelte_ast::RenderTag,
        ctx: &mut crate::walker::VisitContext<'_, '_>,
    ) {
        let Some(handle) = self.parsed.expr_handle(tag.expression_span.start) else {
            return;
        };
        if matches!(
            self.parsed.expr(handle),
            Some(Expression::ChainExpression(_))
        ) {
            ctx.data.blocks.render_tag_is_chain.insert(tag.id);
            if let Some(Expression::ChainExpression(chain)) = self.parsed.take_expr(handle) {
                if let oxc_ast::ast::ChainElement::CallExpression(call) = chain.unbox().expression {
                    self.parsed
                        .replace_expr(handle, Expression::CallExpression(call));
                }
            }
        }
    }
}

pub(crate) fn classify_render_tag_args(
    expr: &Expression<'_>,
    data: &mut AnalysisData,
    tag_id: NodeId,
) {
    if let Expression::CallExpression(call) = expr {
        let arg_plans: Vec<RenderTagArgPlan> = call
            .arguments
            .iter()
            .map(|arg| RenderTagArgPlan {
                info: crate::passes::collect_symbols::build_expression_info(
                    arg.to_expression(),
                    &mut data.scoping,
                ),
                prop_source: None,
            })
            .collect();
        data.blocks.render_tag_plans.insert(
            tag_id,
            RenderTagPlan {
                callee_mode: RenderTagCalleeMode::Direct,
                arg_plans,
            },
        );
    }
}
