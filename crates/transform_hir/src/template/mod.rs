mod attributes;
mod element;
mod fragment;
mod interpolation;
mod is_static;
mod nodes;
mod template_transformer;
mod if_block;
mod expression;

use analyze_hir::HirAnalyses;
use ast_builder::Builder;
use hir::HirStore;
use oxc_ast::ast::Statement;
use template_transformer::TemplateTransformer;

pub struct TransformRet<'hir> {
    pub template_body: Vec<Statement<'hir>>,
    pub hoisted: Vec<Statement<'hir>>,
}

pub fn transform_template<'hir>(
    analyses: &'hir HirAnalyses,
    builder: &'hir Builder<'hir>,
    store: &'hir HirStore<'hir>,
) -> TransformRet<'hir> {
    let mut transformer = TemplateTransformer::new(analyses, builder, store);
    let template_body = transformer.transform();

    return TransformRet {
        template_body,
        hoisted: transformer.hoisted,
    };
}
