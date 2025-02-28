use analyze_hir::HirAnalises;
use ast_builder::Builder;

pub fn transform_script<'hir>(
    analyses: &HirAnalises,
    builder: &Builder,
    program: &mut hir::Program<'hir>,
) {
}
