use std::{mem, rc::Rc};

use scanner::{token::StartTag, Scanner};

use crate::{
    ast::{Ast, AstNode, Element, Node},
    diagnostics::Diagnostic,
};

pub mod scanner;

pub struct Parser {
    scanner: Scanner,
    source: &'static str,
    stack: Vec<Rc<Node>>,
    roots: Vec<Rc<Node>>,
}

impl Parser {
    pub fn new(source: &'static str) -> Parser {
        let scanner = Scanner::new(source);

        return Parser {
            scanner,
            source,
            stack: vec![],
            roots: vec![],
        };
    }

    pub fn parse(&mut self) -> Result<Ast, Diagnostic> {
        for token in self.scanner.scan_tokens()?.iter() {
            match &token.r#type {
                scanner::token::TokenType::Text => todo!(),
                scanner::token::TokenType::StartTag(tag) => self.parse_start_tag(tag)?,
                scanner::token::TokenType::EndTag => todo!(),
                scanner::token::TokenType::Interpolation => todo!(),
                scanner::token::TokenType::StartIfTag(_start_if_tag) => todo!(),
                scanner::token::TokenType::ElseTag(_else_tag) => todo!(),
                scanner::token::TokenType::EndIfTag => todo!(),
                scanner::token::TokenType::EOF => todo!(),
            }
        }

        let template = mem::replace(&mut self.roots, vec![]);

        return Ok(Ast { template });
    }

    fn parse_start_tag(&mut self, tag: &StartTag) -> Result<(), Diagnostic> {
        let name = tag.name.clone();
        // let self_closing = tag.self_closing;

        let node = Node::Element(Element {
            name,
            nodes: vec![],
        });

        let node = Rc::new(node);

        if let Some(last) = self.stack.last_mut() {
            last.push(node);
        }

        // self.stack.push(node.clone());

        return Ok(());
    }

    fn parse_end_tag() -> Result<(), Diagnostic> {
        Ok(())
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
