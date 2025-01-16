use std::vec;

use builder::Builder;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Program};
use transform_script::TransformScript;
use transform_template::TransformTemplate;

use crate::ast::Ast;

pub mod builder;
pub mod scope;
pub mod transform_script;
pub mod transform_template;

pub fn transform_client<'a>(ast: Ast<'a>, b: &'a Builder<'a>) -> String {
    let mut template_transformer = TransformTemplate::new(b);
    let mut script_transformer = TransformScript::new(b);

    let mut imports = vec![b.import_all("$", "svelte/internal/client")];
    let mut program_body = vec![];
    let mut component_body = vec![];

    let mut template_result = template_transformer.transform(&ast);

    if let Some(script) = ast.script {
        let mut script_result = script_transformer.transform(script);

        component_body.append(&mut script_result.body);
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

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;
    use oxc_ast::AstBuilder;

    use crate::parser::Parser;

    use super::*;

    #[test]
    fn smoke() {
        let allocator = Allocator::default();
        let mut parser = Parser::new(
            "<script>const i = 10;</script>",
            // "<br/>{#if true}<div>{title}</div><br/>{:else if false}<div>{number}</div><br/>{:else}<div {id}>4444</div><br/>{/if}",
            &allocator,
        );
        let builder = Builder::new(AstBuilder::new(&allocator));
        let ast = parser.parse().unwrap();

        let code = transform_client(ast, &builder);

        print!("{}", code);
    }
}
