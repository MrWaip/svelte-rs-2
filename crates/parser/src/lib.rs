use std::mem;

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_parser::Parser as OxcParser;
use oxc_span::{Language, SourceType};
use rccell::RcCell;
use scanner::{
    token::{self, EndTag, ExpressionTag, StartTag, Token, TokenType},
    Scanner,
};
use span::{GetSpan, Span};

use ast::{
    AsNode, Ast, Attribute, AttributeValue, BindDirective, BindDirectiveKind, ClassDirective,
    Concatenation, ConcatenationPart, Element, ExpressionAttribute, ExpressionAttributeValue,
    Fragment, HTMLAttribute, IfBlock, Interpolation, Node, ScriptTag, Template, Text,
};

use diagnostics::Diagnostic;

pub mod scanner;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    allocator: &'a Allocator,
    node_stack: NodeStack<'a>,
}

struct NodeStack<'a> {
    stack: Vec<Node<'a>>,
    roots: Vec<Node<'a>>,
    scripts: Vec<Node<'a>>,
}

impl<'a> NodeStack<'a> {
    fn new() -> NodeStack<'a> {
        return NodeStack {
            roots: vec![],
            stack: vec![],
            scripts: vec![],
        };
    }

    /**
     * Открывает новую Node в стэке и добавляет ее родительскую ноду, если имеется
     */
    pub fn add_node(&mut self, node: Node<'a>) -> Result<(), Diagnostic> {
        self.add_child(node.clone())?;

        self.stack.push(node);

        return Ok(());
    }

    /**
     * Добавляет ноду в родителя, если родителя нет то добавляет ее в root
     */
    pub fn add_leaf(&mut self, node: Node<'a>) -> Result<(), Diagnostic> {
        let is_added = self.add_child(node.clone())?;

        if !is_added {
            self.roots.push(node);
        }

        return Ok(());
    }

    pub fn add_script(&mut self, node: Node<'a>) -> Result<(), Diagnostic> {
        let is_added = self.add_child(node.clone())?;

        if !is_added {
            self.scripts.push(node);
        }

        return Ok(());
    }

    pub fn add_to_root(&mut self, node: Node<'a>) {
        self.roots.push(node);
    }

    pub fn pop(&mut self) -> Option<Node<'a>> {
        return self.stack.pop();
    }

    pub fn last_mut(&mut self) -> Option<&mut Node<'a>> {
        return self.stack.last_mut();
    }

    pub fn add_child(&mut self, node: Node<'a>) -> Result<bool, Diagnostic> {
        if let Some(parent) = self.stack.last_mut() {
            match &mut *parent {
                Node::Element(element) => {
                    element.borrow_mut().push(node.clone());
                }
                Node::IfBlock(if_block) => {
                    if_block.borrow_mut().push(node.clone());
                }
                Node::ScriptTag(_) => unreachable!(),
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

    pub fn take_nodes(&mut self) -> (Vec<Node<'a>>, Vec<Node<'a>>) {
        let template = mem::replace(&mut self.roots, vec![]);
        let scripts = mem::replace(&mut self.scripts, vec![]);

        return (template, scripts);
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
                TokenType::Text => self.parse_text(&token)?,
                TokenType::StartTag(tag) => self.parse_start_tag(&tag, token.span)?,
                TokenType::EndTag(tag) => self.parse_end_tag(&tag, token.span)?,
                TokenType::Interpolation(interpolation) => {
                    self.parse_interpolation(interpolation)?;
                }
                TokenType::StartIfTag(start_if_tag) => {
                    self.parse_start_if_tag(&start_if_tag, token.span)?
                }
                TokenType::ElseTag(else_tag) => self.parse_else_tag(&else_tag, token.span)?,
                TokenType::EndIfTag => self.parse_end_if_tag(token.span)?,
                TokenType::EOF => break,
                TokenType::ScriptTag(script_tag) => {
                    self.parse_script_tag(script_tag, token.span)?
                }
            }
        }

        if !self.node_stack.is_stack_empty() {
            let node = self.node_stack.pop().unwrap();
            let span = node.span();
            return Diagnostic::unclosed_node(span).as_err();
        }

        let (nodes, mut scripts) = self.node_stack.take_nodes();
        let mut script: Option<ScriptTag<'a>> = None;

        if scripts.len() > 1 {
            let span_of_last = scripts.last().unwrap().span();
            return Diagnostic::only_single_top_level_script(span_of_last).as_err();
        } else if scripts.len() == 1 {
            script = Some(scripts.remove(0).into());
        }

        return Ok(Ast {
            template: RcCell::new(Template {
                nodes: Fragment::from(nodes),
            }),
            script,
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
            metadata: None,
            node_id: None,
        };

        let node = element.as_node();

        if self_closing {
            self.node_stack.add_leaf(node)?;
        } else {
            self.node_stack.add_node(node)?;
        }

        return Ok(());
    }

    fn parse_end_tag(&mut self, tag: &EndTag<'a>, end_tag_span: Span) -> Result<(), Diagnostic> {
        let mut option = self.node_stack.pop();

        let node = if let Some(node) = option.as_mut() {
            node
        } else {
            return Err(Diagnostic::no_element_to_close(end_tag_span));
        };

        if let Node::Element(element) = node {
            let mut element = element.borrow_mut();

            if element.name != tag.name {
                return Err(Diagnostic::no_element_to_close(end_tag_span));
            }

            element.span = element.span.merge(&end_tag_span);
        } else {
            return Err(Diagnostic::no_element_to_close(end_tag_span));
        };

        if self.node_stack.is_stack_empty() {
            self.node_stack.add_to_root(node.clone())
        }

        Ok(())
    }

    fn parse_text(&mut self, token: &Token<'a>) -> Result<(), Diagnostic> {
        let node = Text {
            value: token.lexeme,
            span: token.span,
        };

        self.node_stack.add_leaf(node.as_node())?;

        return Ok(());
    }

    fn parse_interpolation(&mut self, interpolation: ExpressionTag<'a>) -> Result<(), Diagnostic> {
        let expression =
            self.parse_js_expression(&interpolation.expression.value, interpolation.span)?;

        let node = Interpolation {
            expression,
            metadata: None,
            span: interpolation.span,
        };

        self.node_stack.add_leaf(node.as_node())?;

        return Ok(());
    }

    fn parse_js_expression(
        &self,
        source: &'a str,
        span: Span,
    ) -> Result<Expression<'a>, Diagnostic> {
        let oxc_parser = OxcParser::new(&self.allocator, source, SourceType::default());

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

                            AttributeValue::Expression(ExpressionAttributeValue {
                                expression,
                                metadata: None,
                            })
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
                                metadata: None,
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

                    attributes.push(Attribute::Expression(ExpressionAttribute {
                        expression,
                        metadata: None,
                    }));
                }
                token::Attribute::ClassDirective(token) => {
                    attributes.push(Attribute::ClassDirective(ClassDirective {
                        shorthand: token.shorthand,
                        metadata: None,
                        name: token.name,
                        expression: self
                            .parse_js_expression(token.expression.value, token.expression.span)?,
                    }));
                }
                token::Attribute::BindDirective(token) => {
                    attributes.push(Attribute::BindDirective(BindDirective {
                        shorthand: token.shorthand,
                        metadata: None,
                        name: token.name,
                        kind: BindDirectiveKind::from_str(token.name),
                        expression: self
                            .parse_js_expression(token.expression.value, token.expression.span)?,
                    }));
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
            consequent: Fragment::empty(),
            test: self.parse_js_expression(
                &start_if_tag.expression.value,
                start_if_tag.expression.span,
            )?,
        };

        let node = if_block.as_node();

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
                consequent: Fragment::empty(),
                is_elseif: true,
                span,
                test: expression.unwrap()?,
            };

            let cell = else_if_block.as_node();

            Some(cell)
        } else {
            None
        };

        {
            let option = self.node_stack.last_mut();
            let node = &mut *Node::from_option_mut(option)?;

            let Node::IfBlock(cell) = node else {
                todo!();
            };

            let mut if_block = cell.borrow_mut();

            if_block.alternate = Some(Fragment::empty());
            if_block.span = if_block.span.merge(&span);
        }

        if let Some(cell) = cell {
            self.node_stack.add_node(cell)?;
        }

        return Ok(());
    }

    fn parse_end_if_tag(&mut self, span: Span) -> Result<(), Diagnostic> {
        loop {
            if self.node_stack.last_mut().is_some_and(|v| !v.is_if_block()) {
                break;
            }

            let option = self.node_stack.pop();

            let node = if let Some(node) = &option {
                node
            } else {
                return Err(Diagnostic::no_if_block_to_close(span));
            };

            let is_elseif = false;

            let mut if_block = if let Node::IfBlock(if_block) = node {
                if_block.borrow_mut()
            } else {
                return Err(Diagnostic::no_if_block_to_close(span));
            };

            if_block.span = if_block.span.merge(&span);

            if self.node_stack.is_stack_empty() && !is_elseif {
                self.node_stack.add_to_root(node.clone())
            }

            if !if_block.is_elseif {
                break;
            }
        }

        Ok(())
    }

    fn parse_script_tag(
        &mut self,
        script_tag: token::ScriptTag<'a>,
        span: Span,
    ) -> Result<(), Diagnostic> {
        let language = script_tag.language();
        let source_type = SourceType::default().with_typescript(language == Language::TypeScript);
        let oxc_parser = OxcParser::new(&self.allocator, &script_tag.source, source_type);

        let program_result = oxc_parser.parse();

        if !program_result.errors.is_empty() {
            return Diagnostic::invalid_expression(span).as_err();
        }

        self.node_stack.add_script(
            ScriptTag {
                program: program_result.program,
                language,
                span,
            }
            .as_node(),
        )?;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {

    use ast::format::FormatNode;

    use super::*;

    fn setup_template<'a>(source: &'a str, allocator: &'a Allocator) -> Fragment<'a> {
        let mut parser = Parser::new(source, &allocator);

        let ast = parser.parse().unwrap();

        return ast.template.unwrap().nodes;
    }

    #[test]
    fn smoke() {
        let allocator = Allocator::default();
        let nodes = setup_template("prefix <div>text</div>", &allocator);

        assert_node_cell(&nodes[0], "prefix ");
        assert_node_cell(&nodes[1], "<div>text</div>");
    }

    #[test]
    fn self_closed_element() {
        let allocator = Allocator::default();
        let ast = setup_template("<img /><body><input/></body>", &allocator);

        assert_node_cell(&ast[0], "<img/>");
        assert_node_cell(&ast[1], "<body><input/></body>");
    }

    fn assert_node_cell<'a>(node: &'a Node<'a>, expected: &'a str) {
        assert_eq!(node.format_node(), expected);
    }

    fn assert_script<'a>(node: ScriptTag<'a>, expected: &'a str) {
        assert_eq!(node.format_node(), expected);
    }

    #[test]
    fn smoke_interpolation() {
        let allocator = Allocator::default();
        let ast = setup_template("{ id - 22 + 1 }", &allocator);

        assert_node_cell(&ast[0], "{ id - 22 + 1 }");
    }

    #[test]
    fn smoke_tag_with_attributes() {
        let allocator = Allocator::default();
        let ast = setup_template(
            r#"<div lang="ts" {id} disabled  value={value} label="at: {date} time">source</div>"#,
            &allocator,
        );

        assert_node_cell(
            &ast[0],
            r#"<div lang="ts" {id} disabled value={value} label="at: {date} time">source</div>"#,
        );
    }

    #[test]
    fn smoke_if_tag() {
        let allocator = Allocator::default();
        let ast = setup_template(r#"{#if true }<div>title</div>{/if}"#, &allocator);

        assert_node_cell(&ast[0], r#"{#if true}<div>title</div>{/if}"#);
    }

    #[test]
    fn smoke_if_else_tag() {
        let allocator = Allocator::default();
        let ast = setup_template(
            r#"{#if true }<div>title</div>{:else}<h1>big title</h1>{/if}"#,
            &allocator,
        );

        assert_node_cell(
            &ast[0],
            r#"{#if true}<div>title</div>{:else}<h1>big title</h1>{/if}"#,
        );
    }

    #[test]
    fn smoke_if_elseif_tag() {
        let allocator = Allocator::default();
        let ast = setup_template(
            r#"<div>{#if false }one{:else if true}two{:else}three{/if}</div>{#if false}one{:else if true}two{:else if 1 == 1}<h1>three</h1>{:else}four{/if}"#,
            &allocator,
        );

        assert_node_cell(
            &ast[0],
            r#"<div>{#if false}one{:else if true}two{:else}three{/if}</div>"#,
        );

        assert_node_cell(
            &ast[1],
            r#"{#if false}one{:else if true}two{:else if 1 == 1}<h1>three</h1>{:else}four{/if}"#,
        );
    }

    #[test]
    fn if_else_in_if_else() {
        let allocator = Allocator::default();
        let ast = setup_template(
            r#"{#if 1 === 1}{#if 2 === 2}inside{/if}{:else}{#if 3 === 3}alternate inside{/if}{/if}"#,
            &allocator,
        );

        assert_node_cell(
            &ast[0],
            r#"{#if 1 === 1}{#if 2 === 2}inside{/if}{:else}{#if 3 === 3}alternate inside{/if}{/if}"#,
        );
    }

    #[test]
    fn script_tag() {
        let allocator = Allocator::default();
        let mut parser = Parser::new(r#"<script>const i = 10;</script>"#, &allocator);

        let script = parser.parse().unwrap().script.unwrap();

        assert_script(script, "<script>const i = 10;\n</script>");
    }

    #[test]
    fn class_directive() {
        let allocator = Allocator::default();
        let ast = setup_template(
            r#"<input class:visible class:toggled={true} />"#,
            &allocator,
        );

        assert_node_cell(&ast[0], r#"<input class:visible class:toggled={true}/>"#);
    }

    #[test]
    fn bind_directives() {
        let allocator = Allocator::default();
        let ast = setup_template(r#"<input bind:value bind:toggled={true} />"#, &allocator);

        assert_node_cell(&ast[0], r#"<input bind:value bind:toggled={true}/>"#);
    }

    #[test]
    fn script_tag_lang_ts() {
        let allocator = Allocator::default();
        let mut parser = Parser::new(
            r#"<script lang="ts">const i: number = 10;</script>"#,
            &allocator,
        );

        let script = parser.parse().unwrap().script.unwrap();

        assert_script(
            script,
            "<script lang=\"ts\">const i: number = 10;\n</script>",
        );
    }
}
