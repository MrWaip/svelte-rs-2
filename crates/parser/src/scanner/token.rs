use core::fmt;

use diagnostics::Diagnostic;
use oxc_span::Language;

use span::{GetSpan, Span};

#[derive(Debug, PartialEq, Eq)]

pub enum TokenType<'a> {
    Text,
    Comment,
    StartTag(StartTag<'a>),
    EndTag(EndTag<'a>),
    Interpolation(ExpressionTag<'a>),
    StartIfTag(StartIfTag<'a>),
    ElseTag(ElseTag<'a>),
    ScriptTag(ScriptTag<'a>),
    EndIfTag,
    StartEachTag(StartEachTag<'a>),
    EndEachTag,
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptTag<'a> {
    pub source: &'a str,
    pub attributes: Vec<Attribute<'a>>,
}

impl ScriptTag<'_> {
    pub fn language(&self) -> Language {
        let lang_attr = self.attributes.iter().find_map(|item| match item {
            Attribute::HTMLAttribute(attr) => {
                if attr.name == "lang" {
                    return Some(attr);
                }

                None
            }
            _ => None,
        });

        if let Some(attr) = lang_attr {
            if let AttributeValue::String(value) = attr.value {
                if value == "ts" {
                    return Language::TypeScript;
                }
            };
        }

        Language::JavaScript
    }
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
pub struct StartEachTag<'a> {
    pub collection: JsExpression<'a>,
    pub item: JsExpression<'a>,
    pub index: Option<JsExpression<'a>>,
    pub key: Option<JsExpression<'a>>,
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
    ClassDirective(ClassDirective<'a>),
    BindDirective(BindDirective<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClassDirective<'a> {
    pub shorthand: bool,
    pub name: &'a str,
    pub expression: JsExpression<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BindDirective<'a> {
    pub shorthand: bool,
    pub name: &'a str,
    pub expression: JsExpression<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeValue<'a> {
    String(&'a str),
    ExpressionTag(ExpressionTag<'a>),
    Concatenation(Concatenation<'a>),
    Empty,
}

// Любое выражение в фигурных скобках { 1 + 1 } или { name } в шаблоне
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionTag<'a> {
    pub span: Span,
    pub expression: JsExpression<'a>,
}

// Самое js выражение
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JsExpression<'a> {
    pub span: Span,
    pub value: &'a str,
}

impl fmt::Display for AttributeValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttributeValue::String(value) => write!(f, "{}", value),
            AttributeValue::ExpressionTag(value) => write!(f, "{}", value.expression.value),
            AttributeValue::Empty => write!(f, ""),
            AttributeValue::Concatenation(concatenation) => {
                let mut result = String::new();

                concatenation.parts.iter().for_each(|value| match value {
                    ConcatenationPart::String(v) => {
                        let part = format!("({})", v).to_string();
                        result.push_str(&part);
                    }
                    ConcatenationPart::Expression(expression_tag) => {
                        let part = format!("({{{}}})", expression_tag.expression.value).to_string();
                        result.push_str(&part);
                    }
                });

                write!(f, "{}", result)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Concatenation<'a> {
    pub start: usize,
    pub end: usize,
    pub parts: Vec<ConcatenationPart<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConcatenationPart<'a> {
    String(&'a str),
    Expression(ExpressionTag<'a>),
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub span: Span,
    pub lexeme: &'a str,
}

impl GetSpan for Token<'_> {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StartIfTag<'a> {
    pub expression: JsExpression<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElseTag<'a> {
    pub elseif: bool,
    pub expression: Option<JsExpression<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AttributeIdentifierType<'a> {
    HTMLAttribute(&'a str),
    ClassDirective(&'a str),
    BindDirective(&'a str),
    None,
}

impl<'a> AttributeIdentifierType<'a> {
    pub fn is_class_directive(name: &str) -> bool {
        name == "class"
    }

    pub fn is_bind_directive(name: &str) -> bool {
        name == "bind"
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, AttributeIdentifierType::None)
    }

    pub fn as_ok(self) -> Result<AttributeIdentifierType<'a>, Diagnostic> {
        Ok(self)
    }
}
