use std::mem;

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_span::SourceType;
use rccell::RcCell;
use scanner::{
    token::{self, EndTag, ExpressionTag, StartTag, Token},
    Scanner,
};
use span::{GetSpan, Span};

use crate::{
    ast::{
        AsNode, Ast, Attribute, AttributeValue, Element, HTMLAttribute, Interpolation, Node, Text,
    },
    diagnostics::Diagnostic,
};

pub mod scanner;
pub mod span;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    allocator: &'a Allocator,
    node_stack: NodeStack<'a>,
}

struct NodeStack<'a> {
    stack: Vec<RcCell<Node<'a>>>,
    roots: Vec<RcCell<Node<'a>>>,
}

impl<'a> NodeStack<'a> {
    fn new() -> NodeStack<'a> {
        return NodeStack {
            roots: vec![],
            stack: vec![],
        };
    }

    /**
     * Открывает новую Node в стэке и добавляет ее родительскую ноду, если имеется
     */
    pub fn add_node(&mut self, node: RcCell<Node<'a>>) -> Result<(), Diagnostic> {
        self.add_child(node.clone())?;

        self.stack.push(node);

        return Ok(());
    }

    /**
     * Добавляет ноду в родителя, если родителя нет то добавляет ее в root
     */
    pub fn add_leaf(&mut self, node: RcCell<Node<'a>>) -> Result<(), Diagnostic> {
        let is_added = self.add_child(node.clone())?;

        if !is_added {
            self.roots.push(node);
        }

        return Ok(());
    }

    pub fn add_to_root(&mut self, node: RcCell<Node<'a>>) {
        self.roots.push(node);
    }

    pub fn pop(&mut self) -> Option<RcCell<Node<'a>>> {
        return self.stack.pop();
    }

    pub fn add_child(&mut self, node: RcCell<Node<'a>>) -> Result<bool, Diagnostic> {
        if let Some(parent) = self.stack.last_mut() {
            let mut parent = parent.borrow_mut();

            match &mut *parent {
                Node::Element(element) => {
                    element.nodes.push(node.clone());
                }
                _ => unreachable!(),
            };

            return Ok(true);
        }

        return Ok(false);
    }

    pub fn is_stack_empty(&self) -> bool {
        return self.stack.is_empty();
    }

    pub fn get_roots(&mut self) -> Vec<RcCell<Node<'a>>> {
        let template = mem::replace(&mut self.roots, vec![]);

        return template;
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, allocator: &'a Allocator) -> Parser<'a> {
        let scanner = Scanner::new(source);

        return Parser {
            scanner,
            allocator,
            node_stack: NodeStack::new(),
        };
    }

    pub fn parse(&mut self) -> Result<Ast<'a>, Diagnostic> {
        let tokens = self.scanner.scan_tokens()?;

        for token in tokens {
            match token.token_type {
                scanner::token::TokenType::Text => self.parse_text(&token)?,
                scanner::token::TokenType::StartTag(tag) => {
                    self.parse_start_tag(&tag, token.span)?
                }
                scanner::token::TokenType::EndTag(tag) => self.parse_end_tag(&tag, token.span)?,
                scanner::token::TokenType::Interpolation(interpolation) => {
                    self.parse_interpolation(interpolation)?;
                }
                scanner::token::TokenType::StartIfTag(_start_if_tag) => todo!(),
                scanner::token::TokenType::ElseTag(_else_tag) => todo!(),
                scanner::token::TokenType::EndIfTag => todo!(),
                token::TokenType::EOF => break,
            }
        }

        if !self.node_stack.is_stack_empty() {
            let node = self.node_stack.pop().unwrap();
            let span = node.borrow().span();
            return Diagnostic::unclosed_node(span).as_err();
        }

        return Ok(Ast {
            template: self.node_stack.get_roots(),
        });
    }

    fn parse_start_tag(&mut self, tag: &StartTag<'a>, start_span: Span) -> Result<(), Diagnostic> {
        let name = tag.name;
        let self_closing = tag.self_closing;
        let attributes = self.parse_attributes(&tag.attributes)?;

        let element = Element {
            name: name.to_string(),
            self_closing,
            nodes: vec![],
            span: start_span,
            attributes,
        };

        let node = element.as_node().as_rc_cell();

        if self_closing {
            self.node_stack.add_leaf(node)?;
        } else {
            self.node_stack.add_node(node)?;
        }

        return Ok(());
    }

    fn parse_end_tag(&mut self, tag: &EndTag<'a>, end_tag_span: Span) -> Result<(), Diagnostic> {
        let closed_node = if let Some(closed_node) = self.node_stack.pop() {
            closed_node.unwrap()
        } else {
            return Err(Diagnostic::no_element_to_close(end_tag_span));
        };

        let mut element = if let Node::Element(element) = closed_node {
            element
        } else {
            return Err(Diagnostic::no_element_to_close(end_tag_span));
        };

        if element.name != tag.name {
            return Err(Diagnostic::no_element_to_close(end_tag_span));
        }

        let full_span = element.span.merge(&end_tag_span);
        element.span = full_span;

        if self.node_stack.is_stack_empty() {
            self.node_stack.add_to_root(element.as_node().as_rc_cell())
        }

        Ok(())
    }

    fn parse_text(&mut self, token: &Token) -> Result<(), Diagnostic> {
        let node = Text {
            value: token.lexeme.to_string(),
            span: token.span,
        };

        self.node_stack.add_leaf(node.as_node().as_rc_cell())?;

        return Ok(());
    }

    fn parse_interpolation(&mut self, interpolation: ExpressionTag<'a>) -> Result<(), Diagnostic> {
        let expression =
            self.parse_js_expression(&interpolation.expression.value, interpolation.span)?;

        let node = Interpolation {
            expression,
            span: interpolation.span,
        };

        self.node_stack.add_leaf(node.as_node().as_rc_cell())?;

        return Ok(());
    }

    fn parse_js_expression(
        &self,
        source: &'a str,
        span: Span,
    ) -> Result<Expression<'a>, Diagnostic> {
        let oxc_parser = oxc_parser::Parser::new(&self.allocator, source, SourceType::default());

        let expression = oxc_parser
            .parse_expression()
            .map_err(|_| Diagnostic::invalid_expression(span))?;

        return Ok(expression);
    }

    fn parse_attributes(
        &self,
        token_attrs: &Vec<token::Attribute<'a>>,
    ) -> Result<Vec<Attribute<'a>>, Diagnostic> {
        let mut attributes: Vec<Attribute<'a>> = vec![];

        for attribute in token_attrs.iter() {
            match attribute {
                token::Attribute::HTMLAttribute(token_attr) => {
                    let value: AttributeValue = match &token_attr.value {
                        token::AttributeValue::String(value) => AttributeValue::String(value),
                        token::AttributeValue::ExpressionTag(expression_tag) => {
                            let expression = self.parse_js_expression(
                                &expression_tag.expression.value,
                                expression_tag.span,
                            )?;

                            AttributeValue::Expression(expression)
                        }
                        token::AttributeValue::Concatenation(_) => todo!(),
                        token::AttributeValue::Empty => AttributeValue::Boolean,
                    };

                    let html_attr = HTMLAttribute {
                        name: token_attr.name,
                        value,
                    };

                    attributes.push(Attribute::HTMLAttribute(html_attr));
                }
                token::Attribute::ExpressionTag(_) => todo!(),
            }
        }

        return Ok(attributes);
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::FormatNode;

    use super::*;

    #[test]
    fn smoke() {
        let allocator = Allocator::default();
        let mut parser = Parser::new("prefix <div>text</div>", &allocator);

        let ast = parser.parse().unwrap().template;

        assert_node(&ast[0], "prefix ");
        assert_node(&ast[1], "<div>text</div>");
    }

    #[test]
    fn self_closed_element() {
        let allocator = Allocator::default();
        let mut parser = Parser::new("<img /><body><input/></body>", &allocator);
        let ast = parser.parse().unwrap().template;

        assert_node(&ast[0], "<img/>");
        assert_node(&ast[1], "<body><input/></body>");
    }

    fn assert_node<'a>(node: &'a RcCell<Node<'a>>, expected: &'a str) {
        let node = node.borrow();
        assert_eq!(node.format_node(), expected);
    }

    #[test]
    fn smoke_interpolation() {
        let allocator = Allocator::default();
        let mut parser = Parser::new("{ id - 22 + 1 }", &allocator);
        let ast = parser.parse().unwrap().template;

        assert_node(&ast[0], "{ id - 22 + 1 }");
    }

    #[test]
    fn smoke_tag_with_attributes() {
        let allocator = Allocator::default();
        let mut parser = Parser::new(
            r#"<script lang="ts" disabled  value={value}>source</script>"#,
            &allocator,
        );
        let ast = parser.parse().unwrap().template;

        assert_node(&ast[0], r#"<script lang="ts" disabled value={value} label="at: {date} time">source</script>"#);
    }
}
