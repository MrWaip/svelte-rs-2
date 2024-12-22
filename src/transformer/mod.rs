use std::cell::Cell;

use oxc_allocator::{Allocator, Vec};
use oxc_ast::{
    ast::{Program, Statement},
    AstBuilder,
};
use oxc_span::{SourceType, SPAN};

use crate::ast::Ast;

pub mod builder;
pub mod visitor;

pub fn transform_client<'a>(ast: &Ast<'a>) -> String {
    let allocator = Allocator::default();
    let b = AstBuilder::new(&allocator);
    let mut body: Vec<Statement<'a>> = b.vec();

    let program = Program {
        body,
        span: SPAN,
        comments: b.vec(),
        directives: b.vec(),
        hashbang: None,
        source_text: "",
        source_type: SourceType::default(),
        scope_id: Cell::from(None),
    };

    let codegen = oxc_codegen::Codegen::default();

    let result = codegen.build(&program);

    return result.code;
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;

    use crate::parser::Parser;

    use super::*;

    #[test]
    fn smoke() {
        let allocator = Allocator::default();
        let mut parser = Parser::new("prefix <div>text</div>", &allocator);
        let ast = parser.parse().unwrap();

        let code = transform_client(&ast);

        dbg!(code);
    }
}
