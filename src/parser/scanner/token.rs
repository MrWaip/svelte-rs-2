use core::fmt;

use crate::parser::span::{GetSpan, Span};

#[derive(Debug, PartialEq, Eq)]

pub enum TokenType<'a> {
    Text,
    StartTag(StartTag<'a>),
    EndTag(EndTag<'a>),
    Interpolation(Interpolation<'a>),
    StartIfTag(StartIfTag<'a>),
    ElseTag(ElseTag<'a>),
    EndIfTag,
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartTag<'a> {
    pub attributes: Vec<Attribute<'a>>,
    pub name: &'a str,
    pub self_closing: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EndTag<'a> {
    pub name: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HTMLAttribute<'a> {
    pub name: &'a str,
    pub value: AttributeValue<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute<'a> {
    HTMLAttribute(HTMLAttribute<'a>),
    ExpressionTag(ExpressionTag<'a>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeValue<'a> {
    String(String),
    ExpressionTag(ExpressionTag<'a>),
    Concatenation(Concatenation<'a>),
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interpolation<'a> {
    pub expression: &'a str,
}

pub struct JsExpression<'a> {
    pub start: usize,
    pub end: usize,
    pub expression: &'a str,
}

impl<'a> fmt::Display for AttributeValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttributeValue::String(value) => write!(f, "{}", value),
            AttributeValue::ExpressionTag(value) => write!(f, "{}", value.expression),
            AttributeValue::Empty => write!(f, ""),
            AttributeValue::Concatenation(concatenation) => {
                let mut result = String::new();

                concatenation.parts.iter().for_each(|value| match value {
                    ConcatenationPart::String(v) => {
                        let part = format!("({})", v).to_string();
                        result.push_str(&part);
                    }
                    ConcatenationPart::Expression(expression_tag) => {
                        let part = format!("({{{}}})", expression_tag.expression).to_string();
                        result.push_str(&part);
                    }
                });

                return write!(f, "{}", result);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionTag<'a> {
    pub start: usize,
    pub end: usize,
    pub expression: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Concatenation<'a> {
    pub start: usize,
    pub end: usize,
    pub parts: Vec<ConcatenationPart<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConcatenationPart<'a> {
    String(String),
    Expression(ExpressionTag<'a>),
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub span: Span,
    pub lexeme: &'a str,
}

impl<'a> GetSpan for Token<'a> {
    fn span(&self) -> Span {
        return self.span;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StartIfTag<'a> {
    pub expression: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElseTag<'a> {
    pub elseif: bool,
    pub expression: Option<&'a str>,
}
