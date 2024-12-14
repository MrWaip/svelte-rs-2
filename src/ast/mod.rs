use oxc_ast::ast::Expression;
use rccell::RcCell;

use crate::parser::span::{GetSpan, Span};

pub struct Ast<'a> {
    pub template: Vec<RcCell<Node<'a>>>,
}

pub trait FormatNode {
    fn format_node(&self) -> String;
}

pub trait AsNode<'a> {
    fn as_node(self) -> Node<'a>;
}

#[derive(Debug)]
pub enum Node<'a> {
    Element(Element<'a>),
    Text(Text),
    Interpolation(Interpolation<'a>),
    IfBlock(IfBlock<'a>),
}

impl<'a> Node<'a> {
    pub fn as_rc_cell(self) -> RcCell<Node<'a>> {
        return RcCell::new(self);
    }
}

impl<'a> FormatNode for Node<'a> {
    fn format_node(&self) -> String {
        return match self {
            Node::Element(element) => element.format_node(),
            Node::Text(text) => text.format_node(),
            Node::Interpolation(interpolation) => interpolation.format_node(),
            Node::IfBlock(if_block) => if_block.format_node(),
        };
    }
}

impl<'a> GetSpan for Node<'a> {
    fn span(&self) -> Span {
        match self {
            Node::Element(element) => element.span,
            Node::Text(text) => text.span,
            Node::Interpolation(interpolation) => interpolation.span,
            Node::IfBlock(if_block) => if_block.span,
        }
    }
}

#[derive(Debug)]
pub struct Interpolation<'a> {
    pub expression: Expression<'a>,
    pub span: Span,
}

impl<'a> FormatNode for Interpolation<'a> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("{ ");

        let expr_string = print_expression(&self.expression);
        result.push_str(&expr_string);

        result.push_str(" }");

        return result;
    }
}

impl<'a> AsNode<'a> for Interpolation<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::Interpolation(self);
    }
}

#[derive(Debug)]
pub struct IfBlock<'a> {
    pub span: Span,
    pub test: Expression<'a>,
    pub elseif: bool,
    pub consequent: Vec<RcCell<Node<'a>>>,
    pub alternate: Option<Vec<RcCell<Node<'a>>>>,
}
impl<'a> IfBlock<'a> {
    pub fn push(&mut self, node: RcCell<Node<'a>>) {
        if let Some(alternate) = self.alternate.as_mut() {
            alternate.push(node);
        } else {
            self.consequent.push(node);
        }
    }
}

impl<'a> FormatNode for IfBlock<'a> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!("{{#if {} }}", &print_expression(&self.test)));

        for node in self.consequent.iter() {
            let formatted = &node.borrow().format_node();
            result.push_str(formatted);
        }

        result.push_str("{/if}");

        return result;
    }
}

impl<'a> AsNode<'a> for IfBlock<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::IfBlock(self);
    }
}

#[derive(Debug)]
pub struct Element<'a> {
    pub name: String,
    pub span: Span,
    pub self_closing: bool,
    pub nodes: Vec<RcCell<Node<'a>>>,
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> FormatNode for Element<'a> {
    fn format_node(&self) -> String {
        let mut result = String::new();

        result.push_str("<");
        result.push_str(&self.name);

        if !self.attributes.is_empty() {
            result.push_str(" ");
            let mut attributes = vec![];

            for attr in self.attributes.iter() {
                let mut result = String::new();

                match attr {
                    Attribute::HTMLAttribute(attr) => {
                        result.push_str(attr.name);

                        match &attr.value {
                            AttributeValue::String(value) => {
                                result.push_str(format!("=\"{}\"", value).as_str());
                            }
                            AttributeValue::Boolean => (),
                            AttributeValue::Expression(expression) => {
                                let expr_string = print_expression(expression);
                                result.push_str(format!("={{{}}}", expr_string).as_str());
                            }
                            AttributeValue::Concatenation(concatenation) => {
                                result.push_str("=\"");

                                for part in concatenation.parts.iter() {
                                    match part {
                                        ConcatenationPart::String(value) => result.push_str(value),
                                        ConcatenationPart::Expression(expression) => {
                                            let expr_string = print_expression(expression);
                                            result
                                                .push_str(format!("{{{}}}", expr_string).as_str());
                                        }
                                    }
                                }

                                result.push_str("\"");
                            }
                        }
                    }
                    Attribute::Expression(expression) => {
                        let expr_string = print_expression(expression);
                        result.push_str(format!("{{{}}}", expr_string).as_str());
                    }
                }

                attributes.push(result);
            }

            result.push_str(attributes.join(" ").as_str());
        }

        if self.self_closing {
            result.push_str("/>");
            return result;
        } else {
            result.push_str(">");
        }

        for node in self.nodes.iter() {
            let formatted = node.borrow().format_node();
            result.push_str(&formatted);
        }

        result.push_str("</");
        result.push_str(&self.name);
        result.push_str(">");

        return result;
    }
}

impl<'a> AsNode<'a> for Element<'a> {
    fn as_node(self) -> Node<'a> {
        return Node::Element(self);
    }
}

#[derive(Debug)]
pub struct Text {
    pub value: String,
    pub span: Span,
}

impl FormatNode for Text {
    fn format_node(&self) -> String {
        return self.value.clone();
    }
}

impl<'a> AsNode<'a> for Text {
    fn as_node(self) -> Node<'a> {
        return Node::Text(self);
    }
}

#[derive(Debug)]
pub enum Attribute<'a> {
    HTMLAttribute(HTMLAttribute<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug)]
pub struct HTMLAttribute<'a> {
    pub name: &'a str,
    pub value: AttributeValue<'a>,
}

#[derive(Debug)]
pub enum AttributeValue<'a> {
    String(&'a str),
    Expression(Expression<'a>),
    Boolean,
    Concatenation(Concatenation<'a>),
}

#[derive(Debug)]
pub struct Concatenation<'a> {
    pub parts: Vec<ConcatenationPart<'a>>,
    pub span: Span,
}

#[derive(Debug)]
pub enum ConcatenationPart<'a> {
    String(&'a str),
    Expression(Expression<'a>),
}

fn print_expression<'a>(expression: &Expression<'a>) -> String {
    let mut codegen = oxc_codegen::Codegen::default();
    codegen.print_expression(expression);
    return codegen.into_source_text();
}
