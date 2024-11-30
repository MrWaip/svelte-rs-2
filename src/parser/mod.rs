use std::mem;

use rccell::RcCell;
use scanner::{
    token::{EndTag, StartTag, Token},
    Scanner,
};

use crate::{
    ast::{AsNode, Ast, Element, Node, Text},
    diagnostics::Diagnostic,
};

pub mod scanner;

pub struct Parser {
    scanner: Scanner,
    stack: Vec<RcCell<Node>>,
    roots: Vec<RcCell<Node>>,
}

impl Parser {
    pub fn new(source: &'static str) -> Parser {
        let scanner = Scanner::new(source);

        return Parser {
            scanner,
            stack: vec![],
            roots: vec![],
        };
    }

    pub fn parse(&mut self) -> Result<Ast, Diagnostic> {
        for token in self.scanner.scan_tokens()?.iter() {
            match &token.r#type {
                scanner::token::TokenType::Text => self.parse_text(token)?,
                scanner::token::TokenType::StartTag(tag) => self.parse_start_tag(tag)?,
                scanner::token::TokenType::EndTag(tag) => self.parse_end_tag(tag)?,
                scanner::token::TokenType::Interpolation => todo!(),
                scanner::token::TokenType::StartIfTag(_start_if_tag) => todo!(),
                scanner::token::TokenType::ElseTag(_else_tag) => todo!(),
                scanner::token::TokenType::EndIfTag => todo!(),
                scanner::token::TokenType::EOF => break,
            }
        }

        if !self.stack.is_empty() {
            return Diagnostic::unclosed_node(0).as_err();
        }

        let template = mem::replace(&mut self.roots, vec![]);

        return Ok(Ast { template });
    }

    fn parse_start_tag(&mut self, tag: &StartTag) -> Result<(), Diagnostic> {
        let name = tag.name.clone();
        let self_closing = tag.self_closing;

        let node = Element {
            name,
            nodes: vec![],
        };

        let node = RcCell::new(node.as_node());

        self.add_node(node)?;

        if self_closing {
            todo!();
        }

        return Ok(());
    }

    /**
     * Открывает новую Node в стэке и добавляет ее родительскую ноду, если имеется
     */
    fn add_node(&mut self, node: RcCell<Node>) -> Result<(), Diagnostic> {
        self.add_child(node.clone())?;

        self.stack.push(node);

        return Ok(());
    }

    /**
     * Добавляет ноду в родителя, если родителя нет то добавляет ее в root
     */
    fn add_leaf(&mut self, node: RcCell<Node>) -> Result<(), Diagnostic> {
        let is_added = self.add_child(node.clone())?;

        if !is_added {
            self.roots.push(node);
        }

        return Ok(());
    }

    fn add_child(&mut self, node: RcCell<Node>) -> Result<bool, Diagnostic> {
        if let Some(parent) = self.stack.last_mut() {
            let mut parent = parent.borrow_mut();

            match &mut *parent {
                Node::Element(element) => {
                    element.nodes.push(node.clone());
                }
                Node::Text(_) => unreachable!(),
            };

            return Ok(true);
        }

        return Ok(false);
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

    fn parse_text(&mut self, token: &Token) -> Result<(), Diagnostic> {
        let node = Text {
            value: token.lexeme.to_string(),
        };

        self.add_leaf(RcCell::new(node.as_node()))?;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::FormatNode;

    use super::*;

    #[test]
    fn smoke() {
        let mut parser = Parser::new("prefix <div>text</div>");

        let ast = parser.parse().unwrap();

        let node1 = ast.template[0].borrow();
        let node2 = ast.template[1].borrow();

        assert_eq!(node1.format_node(), "prefix ");
        assert_eq!(node2.format_node(), "<div>text</div>");
    }
}
