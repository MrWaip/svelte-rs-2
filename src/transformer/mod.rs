use std::vec;

use builder::Builder;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Program};
use transform_script::TransformScript;
use transform_template::TransformTemplate;

use crate::{
    analyze::{AnalyzeResult, ScriptResult},
    ast::Ast,
};

pub mod builder;
pub mod scope;
pub mod transform_script;
pub mod transform_template;

pub fn transform_client<'a>(ast: Ast<'a>, b: &'a Builder<'a>, analyze: AnalyzeResult) -> String {
    let mut template_transformer = TransformTemplate::new(b);
    let script_transformer = TransformScript::new(b);

    let mut imports = vec![b.import_all("$", "svelte/internal/client")];
    let mut program_body = vec![];
    let mut component_body = vec![];

    let mut template_result = template_transformer.transform(&ast);

    if let Some(script) = ast.script {
        let ScriptResult { scopes, symbols } = analyze.script.unwrap();
        let script_result = script_transformer.transform(script, symbols, scopes, &analyze.runes);

        for stmt in script_result.program.body {
            component_body.push(stmt);
        }
    }

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
