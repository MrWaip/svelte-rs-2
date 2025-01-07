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
        AsNode, Ast, Attribute, AttributeValue, Concatenation, ConcatenationPart, Element,
        HTMLAttribute, IfBlock, Interpolation, Node, Text,
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

    pub fn last_mut(&mut self) -> Option<&mut RcCell<Node<'a>>> {
        return self.stack.last_mut();
    }

    pub fn add_child(&mut self, node: RcCell<Node<'a>>) -> Result<bool, Diagnostic> {
        if let Some(parent) = self.stack.last_mut() {
            let mut parent = parent.borrow_mut();

            match &mut *parent {
                Node::Element(element) => {
                    element.push(node.clone());
                }
                Node::IfBlock(if_block) => {
                    if_block.push(node.clone());
                }
                Node::Text(_) => unreachable!(),
                Node::Interpolation(_) => unreachable!(),
                Node::VirtualConcatenation(_) => unreachable!(),
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
                scanner::token::TokenType::StartIfTag(start_if_tag) => {
                    self.parse_start_if_tag(&start_if_tag, token.span)?
                }
                scanner::token::TokenType::ElseTag(else_tag) => {
                    self.parse_else_tag(&else_tag, token.span)?
                }
                scanner::token::TokenType::EndIfTag => self.parse_end_if_tag(token.span)?,
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
        let mut option = self.node_stack.pop();

        let cell = if let Some(cell) = option.as_mut() {
            cell
        } else {
            return Err(Diagnostic::no_element_to_close(end_tag_span));
        };

        let mut borrow = cell.borrow_mut();

        let element = if let Node::Element(element) = &mut *borrow {
            element
        } else {
            return Err(Diagnostic::no_element_to_close(end_tag_span));
        };

        if element.name != tag.name {
            return Err(Diagnostic::no_element_to_close(end_tag_span));
        }

        element.span = element.span.merge(&end_tag_span);

        if self.node_stack.is_stack_empty() {
            self.node_stack.add_to_root(cell.clone())
        }

        Ok(())
    }

    fn parse_text(&mut self, token: &Token<'a>) -> Result<(), Diagnostic> {
        let node = Text {
            value: token.lexeme,
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
                        token::AttributeValue::Concatenation(token) => {
                            let mut parts: Vec<ConcatenationPart<'a>> = vec![];

                            for part in token.parts.iter() {
                                match part {
                                    token::ConcatenationPart::String(value) => {
                                        parts.push(ConcatenationPart::String(value));
                                    }
                                    token::ConcatenationPart::Expression(expression_tag) => {
                                        let expression = self.parse_js_expression(
                                            &expression_tag.expression.value,
                                            expression_tag.span,
                                        )?;

                                        parts.push(ConcatenationPart::Expression(expression));
                                    }
                                }
                            }

                            AttributeValue::Concatenation(Concatenation {
                                parts,
                                span: Span::new(token.start, token.end),
                            })
                        }
                        token::AttributeValue::Empty => AttributeValue::Boolean,
                    };

                    let html_attr = HTMLAttribute {
                        name: token_attr.name,
                        value,
                    };

                    attributes.push(Attribute::HTMLAttribute(html_attr));
                }
                token::Attribute::ExpressionTag(expression_tag) => {
                    let expression = self.parse_js_expression(
                        &expression_tag.expression.value,
                        expression_tag.span,
                    )?;

                    attributes.push(Attribute::Expression(expression));
                }
            }
        }

        return Ok(attributes);
    }

    fn parse_start_if_tag(
        &mut self,
        start_if_tag: &token::StartIfTag<'a>,
        span: Span,
    ) -> Result<(), Diagnostic> {
        let if_block = IfBlock {
            span,
            is_elseif: false,
            alternate: None,
            consequent: vec![],
            test: self.parse_js_expression(
                &start_if_tag.expression.value,
                start_if_tag.expression.span,
            )?,
        };

        let node = if_block.as_node().as_rc_cell();

        self.node_stack.add_node(node)?;

        return Ok(());
    }

    fn parse_else_tag(
        &mut self,
        else_tag: &token::ElseTag<'a>,
        span: Span,
    ) -> Result<(), Diagnostic> {
        let expression = else_tag
            .expression
            .as_ref()
            .map(|v| self.parse_js_expression(&v.value, span));

        let cell = if else_tag.elseif {
            let else_if_block = IfBlock {
                alternate: None,
                consequent: vec![],
                is_elseif: true,
                span,
                test: expression.unwrap()?,
            };

            let cell = else_if_block.as_node().as_rc_cell();

            Some(cell)
        } else {
            None
        };

        {
            let option = self.node_stack.last_mut();
            let node = &mut *Node::from_option_mut(option)?;
            let if_block: &mut IfBlock<'a> = node.try_into()?;

            if_block.alternate = Some(vec![]);
            if_block.span = if_block.span.merge(&span);
        }

        if let Some(cell) = cell {
            self.node_stack.add_node(cell)?;
        }

        return Ok(());
    }

    fn parse_end_if_tag(&mut self, span: Span) -> Result<(), Diagnostic> {
        loop {
            if self
                .node_stack
                .last_mut()
                .is_some_and(|v| !v.borrow().is_if_block())
            {
                break;
            }

            let mut option = self.node_stack.pop();

            let cell = if let Some(cell) = option.as_mut() {
                cell
            } else {
                return Err(Diagnostic::no_if_block_to_close(span));
            };

            let mut borrow = cell.borrow_mut();

            let if_block = if let Node::IfBlock(if_block) = &mut *borrow {
                if_block
            } else {
                return Err(Diagnostic::no_if_block_to_close(span));
            };

            if_block.span = if_block.span.merge(&span);

            if self.node_stack.is_stack_empty() && !if_block.is_elseif {
                self.node_stack.add_to_root(cell.clone())
            }

            if !if_block.is_elseif {
                break;
            }
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
            r#"<script lang="ts" {id} disabled  value={value} label="at: {date} time">source</script>"#,
            &allocator,
        );
        let ast = parser.parse().unwrap().template;

        assert_node(
            &ast[0],
            r#"<script lang="ts" {id} disabled value={value} label="at: {date} time">source</script>"#,
        );
    }

    #[test]
    fn smoke_if_tag() {
        let allocator = Allocator::default();
        let mut parser = Parser::new(r#"{#if true }<div>title</div>{/if}"#, &allocator);
        let ast = parser.parse().unwrap().template;

        assert_node(&ast[0], r#"{#if true}<div>title</div>{/if}"#);
    }

    #[test]
    fn smoke_if_else_tag() {
        let allocator = Allocator::default();
        let mut parser = Parser::new(
            r#"{#if true }<div>title</div>{:else}<h1>big title</h1>{/if}"#,
            &allocator,
        );
        let ast = parser.parse().unwrap().template;

        assert_node(
            &ast[0],
            r#"{#if true}<div>title</div>{:else}<h1>big title</h1>{/if}"#,
        );
    }

    #[test]
    fn smoke_if_elseif_tag() {
        let allocator = Allocator::default();
        let mut parser = Parser::new(
            r#"<div>{#if false }one{:else if true}two{:else}three{/if}</div>{#if false}one{:else if true}two{:else if 1 == 1}<h1>three</h1>{:else}four{/if}"#,
            &allocator,
        );
        let ast = parser.parse().unwrap().template;

        assert_node(
            &ast[0],
            r#"<div>{#if false}one{:else if true}two{:else}three{/if}</div>"#,
        );

        assert_node(
            &ast[1],
            r#"{#if false}one{:else if true}two{:else if 1 == 1}<h1>three</h1>{:else}four{/if}"#,
        );
    }

    #[test]
    fn if_else_in_if_else() {
        let allocator = Allocator::default();
        let mut parser = Parser::new(
            r#"{#if 1 === 1}{#if 2 === 2}inside{/if}{:else}{#if 3 === 3}alternate inside{/if}{/if}"#,
            &allocator,
        );
        let ast = parser.parse().unwrap().template;

        assert_node(
            &ast[0],
            r#"{#if 1 === 1}{#if 2 === 2}inside{/if}{:else}{#if 3 === 3}alternate inside{/if}{/if}"#,
        );
    }
}
