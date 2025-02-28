use analyze_hir::HirAnalises;
use ast_builder::Builder;
use hir::HirStore;
use oxc_ast::ast::Statement;

pub struct TransformRet<'hir> {
    pub template_body: Vec<Statement<'hir>>,
}

pub fn transform_template<'hir>(
    analyses: &HirAnalises,
    builder: &Builder,
    store: &HirStore<'hir>,
) -> TransformRet<'hir> {
    return TransformRet {
        template_body: vec![],
    };
}
