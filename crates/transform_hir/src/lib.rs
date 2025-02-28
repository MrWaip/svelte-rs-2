use analyze_hir::HirAnalyses;
use ast_builder::Builder;
use hir::HirStore;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Program};
use script::transform_script;
use template::{TransformRet, transform_template};

mod script;
mod template;

pub fn transform_hir<'hir>(
    analyses: &'hir HirAnalyses,
    store: &'hir mut HirStore<'hir>,
    b: &'hir Builder<'hir>,
) -> Program<'hir> {
    transform_script(&analyses, &b, &mut store.program);
    let TransformRet { mut template_body } = transform_template(&analyses, &b, store);

    let mut program_body = vec![];
    let mut imports = vec![b.import_all("$", "svelte/internal/client")];

    let mut component_body = vec![];
    let component_params = b.params(["$$anchor"]);

    component_body.append(&mut template_body);

    let component = b.function_declaration(b.bid("App"), component_body, component_params);
    program_body.append(&mut imports);
    program_body.push(
        b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
            b.alloc(component),
        )),
    );

    return b.program(program_body);
}
