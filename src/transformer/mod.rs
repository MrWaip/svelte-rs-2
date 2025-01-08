use std::vec;

use builder::Builder;
use oxc_ast::ast::{ExportDefaultDeclarationKind, Program};
use transform_template::TransformTemplate;

use crate::ast::Ast;

pub mod builder;
pub mod scope;
pub mod transform_template;

pub fn transform_client<'a>(ast: &'a Ast<'a>, b: &'a Builder<'a>) -> String {
    let mut transformer = TransformTemplate::new(b);

    let mut imports = vec![b.import_all("$", "svelte/internal/client")];
    let mut program_body = vec![];
    let mut component_body = vec![];

    let mut template_result = transformer.transform(ast);

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
            // r#"<h1 {id} >{title}</h1><div checked value="nope" skip id={id + id} label="one_{title}" {title}><br/>{number} text + {number}<br/>123</div>"#,
            "{#if 1 === 1}{title}<div></div>{/if}",
            &allocator,
        );
        let builder = Builder::new(AstBuilder::new(&allocator));
        let ast = parser.parse().unwrap();

        let code = transform_client(&ast, &builder);

        print!("{}", code);
    }
}
