use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Text,
    StartTag(StartTag),
    EndTag,
    Interpolation,
    StartIfTag(StartIfTag),
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartTag {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub self_closing: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HTMLAttribute {
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    HTMLAttribute(HTMLAttribute),
    ExpressionTag(ExpressionTag),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeValue {
    String(String),
    ExpressionTag(ExpressionTag),
    Concatenation(Concatenation),
    Empty,
}

pub struct JsExpression {
    pub start: usize,
    pub end: usize,
    pub expression: String,
}

impl fmt::Display for AttributeValue {
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
pub struct ExpressionTag {
    pub start: usize,
    pub end: usize,
    pub expression: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Concatenation {
    pub start: usize,
    pub end: usize,
    pub parts: Vec<ConcatenationPart>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConcatenationPart {
    String(String),
    Expression(ExpressionTag),
}

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub line: usize,
    pub lexeme: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StartIfTag {
    pub expression: String,
}
