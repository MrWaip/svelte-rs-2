mod run_reference;
mod rune_assignment;
mod rune_declaration;
mod script_transformer;

use analyze_hir::HirAnalyses;
use ast_builder::Builder;
use hir::HirStore;
use oxc_ast::ast::Statement;
pub use script_transformer::ScriptTransformer;

pub struct TransformRet<'hir> {
    pub body: Vec<Statement<'hir>>,
    pub imports: Vec<Statement<'hir>>,
}

pub fn transform_script<'hir>(
    analyses: &'hir HirAnalyses,
    builder: &'hir Builder,
    store: &'hir HirStore<'hir>,
) -> TransformRet<'hir> {
    let mut transformer =
        ScriptTransformer::new(analyses, builder, store, HirStore::TEMPLATE_OWNER_ID);

    let body = transformer.transform();

    return TransformRet {
        body,
        imports: transformer.imports,
    };
}
