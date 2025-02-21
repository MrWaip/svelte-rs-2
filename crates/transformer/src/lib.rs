use std::vec;

use ast::Ast;
use ast_builder::{Builder, BuilderExpression};
use oxc_ast::ast::{ExportDefaultDeclarationKind, Program};
use rccell::RcCell;
use scope::Scope;
use transform_script::TransformScript;
use transform_template::TransformTemplate;

use analyzer::AnalyzeResult;

pub mod scope;
pub mod transform_script;
pub mod transform_template;

mod client;

pub fn transform_client<'a>(ast: Ast<'a>, b: &'a Builder<'a>, analyze: AnalyzeResult) -> String {
    let svelte_table = analyze.svelte_table;
    let script_transformer = TransformScript::new(b, &svelte_table);
    let root_scope = RcCell::new(Scope::new(None));

    let mut imports = vec![b.import_all("$", "svelte/internal/client")];
    let mut program_body = vec![];
    let mut component_body = vec![];

    if svelte_table.need_binding_group() {
        let ident = root_scope.borrow_mut().generate("binding_group");

        component_body.push(
            b.const_stmt(&ident, BuilderExpression::Array(b.array([])))
        );
    }

    if let Some(mut script) = ast.script {
        let script_result = script_transformer.transform(&mut script.program);

        for stmt in script_result.imports {
            imports.push(stmt);
        }

        for stmt in script.program.body {
            component_body.push(stmt);
        }
    }

    let mut template_transformer = TransformTemplate::new(b, &script_transformer, &svelte_table, root_scope.clone());
    let mut template_result = template_transformer.transform(&mut ast.template.borrow_mut());

    program_body.append(&mut imports);
    program_body.append(&mut template_result.hoisted);
    component_body.append(&mut template_result.body);

    let component_params = b.params(["$$anchor"]);
    let component = b.function_declaration(b.bid("App"), component_body, component_params);

    program_body.push(
        b.export_default(ExportDefaultDeclarationKind::FunctionDeclaration(
            b.alloc(component),
        )),
    );

    return print(b.program(program_body));
}

fn print(program: Program) -> String {
    let codegen = oxc_codegen::Codegen::default();
    let result = codegen.build(&program);

    return result.code;
}
