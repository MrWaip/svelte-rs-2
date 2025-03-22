mod script_transformer;
mod rune_declaration;

use analyze_hir::HirAnalyses;
use ast_builder::Builder;
use hir::HirStore;
use oxc_ast::ast::Statement;
use script_transformer::ScriptTransformer;

pub struct TransformRet<'hir> {
    pub body: Vec<Statement<'hir>>,
    pub imports: Vec<Statement<'hir>>,
}

pub fn transform_script<'hir>(
    analyses: &'hir HirAnalyses,
    builder: &'hir Builder,
    store: &'hir HirStore<'hir>,
) -> TransformRet<'hir> {
    let mut transformer = ScriptTransformer::new(analyses, builder, store);

    let body = transformer.transform();

    return TransformRet {
        body,
        imports: transformer.imports,
    };
}
