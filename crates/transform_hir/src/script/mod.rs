use analyze_hir::HirAnalyses;
use ast_builder::Builder;

pub fn transform_script<'hir>(
    analyses: &HirAnalyses,
    builder: &Builder,
    program: &mut hir::Program<'hir>,
) {
}
