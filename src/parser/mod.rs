use std::{cell::RefCell, mem, rc::Rc, thread::panicking};

use scanner::{
    token::{EndTag, StartTag},
    Scanner,
};

use crate::{
    ast::{Ast, AstNode, Element, Node, RcWrap},
    diagnostics::Diagnostic,
};

pub mod scanner;

pub struct Parser {
    scanner: Scanner,
    source: &'static str,
    stack: Vec<RcWrap<Node>>,
    roots: Vec<RcWrap<Node>>,
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
                scanner::token::TokenType::EndTag(tag) => self.parse_end_tag(tag)?,
                scanner::token::TokenType::Interpolation => todo!(),
                scanner::token::TokenType::StartIfTag(_start_if_tag) => todo!(),
                scanner::token::TokenType::ElseTag(_else_tag) => todo!(),
                scanner::token::TokenType::EndIfTag => todo!(),
                scanner::token::TokenType::EOF => break,
            }
        }

        let template = mem::replace(&mut self.roots, vec![]);

        return Ok(Ast { template });
    }

    fn parse_start_tag(&mut self, tag: &StartTag) -> Result<(), Diagnostic> {
        let name = tag.name.clone();
        let self_closing = tag.self_closing;

        let node = Node::Element(Element {
            name,
            nodes: vec![],
        });

        let node = Rc::from(RefCell::from(node));

        if let Some(last) = self.stack.last_mut() {
            last.borrow_mut().push(node.clone());
        }

        if self_closing {
            todo!();
        }

        self.stack.push(node);

        return Ok(());
    }

    fn parse_end_tag(&mut self, tag: &EndTag) -> Result<(), Diagnostic> {
        let closed_node_ref = if let Some(closed_node) = self.stack.pop() {
            closed_node
        } else {
            return Err(Diagnostic::no_element_to_close(0));
        };

        let closed_node = &*closed_node_ref.borrow();

        let element = if let Node::Element(element) = closed_node {
            element
        } else {
            return Err(Diagnostic::no_element_to_close(0));
        };

        if element.name != tag.name {
            return Err(Diagnostic::no_element_to_close(0));
        }

        if self.stack.is_empty() {
            self.roots.push(closed_node_ref.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::FormatNode;

    use super::*;

    #[test]
    fn smoke() {
        let mut parser = Parser::new("<div>text</div>");

        let ast = parser.parse().unwrap();

        let node = ast.template[0].borrow();

        assert_eq!(node.format_node(), "<div></div>")
    }
}
