use ast_builder::Builder;
use oxc_allocator::Allocator;
use oxc_ast::ast::Program;

mod script;
mod template;

pub fn transform_hir<'hir>(allocator: &'hir Allocator) -> Program<'hir> {
    let builder = Builder::new_with_ast(allocator);

    // let svelte_table = analyze.svelte_table;
    // let script_transformer = TransformScript::new(b, &svelte_table);
    // let root_scope = RcCell::new(Scope::new(None));

    // let mut imports = vec![b.import_all("$", "svelte/internal/client")];
    // let mut program_body = vec![];
    // let mut component_body = vec![];

    // if svelte_table.need_binding_group() {
    //     let ident = root_scope.borrow_mut().generate("binding_group");

    //     component_body.push(b.const_stmt(&ident, BuilderExpression::Array(b.array([]))));
    // }

    // if let Some(mut script) = ast.script {
    //     let script_result = script_transformer.transform(&mut script.program);

    //     for stmt in script_result.imports {
    //         imports.push(stmt);
    //     }

    //     for stmt in script.program.body {
    //         component_body.push(stmt);
    //     }
    // }

    // let mut template_transformer =
    //     TransformTemplate::new(b, &script_transformer, &svelte_table, root_scope.clone());
    // let mut template_result = template_transformer.transform(&mut ast.template.borrow_mut());

    // program_body.append(&mut imports);
    // program_body.append(&mut template_result.hoisted);
    // component_body.append(&mut template_result.body);

    // let component_params = b.params(["$$anchor"]);
    // let component = b.function_declaration(b.bid("App"), component_body, component_params);

    // program_body.push(
    //     b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
    //         b.alloc(component),
    //     )),
    // );

    return builder.program(vec![]);
}
