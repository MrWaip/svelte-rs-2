use scanner::Scanner;

use crate::{
    ast::{Ast, Node},
    diagnostics::Diagnostic,
};

pub mod scanner;

pub struct Parser {
    scanner: Scanner,
    source: &'static str,
    stack: Vec<Node>,
}

impl Parser {
    pub fn new(source: &'static str) -> Parser {
        let scanner = Scanner::new(source);

        return Parser {
            scanner,
            source,
            stack: vec![],
        };
    }

    pub fn parse(&mut self) -> Result<Ast, Diagnostic> {
        for token in self.scanner.scan_tokens()?.iter() {
            match &token.r#type {
                scanner::token::TokenType::Text => todo!(),
                scanner::token::TokenType::StartTag(start_tag) => todo!(),
                scanner::token::TokenType::EndTag => todo!(),
                scanner::token::TokenType::Interpolation => todo!(),
                scanner::token::TokenType::StartIfTag(start_if_tag) => todo!(),
                scanner::token::TokenType::ElseTag(else_tag) => todo!(),
                scanner::token::TokenType::EndIfTag => todo!(),
                scanner::token::TokenType::EOF => todo!(),
            }
        }

        return Ok(Ast { template: vec![] });
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::FormatNode;

    use super::*;

    #[test]
    fn smoke() {
        let mut parser = Parser::new("<div></div>");

        let ast = parser.parse().unwrap();

        assert_eq!(ast.template[0].format_node(), "kek")
    }
}
