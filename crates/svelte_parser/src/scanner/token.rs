use core::fmt;
use svelte_span::{GetSpan, Span};
use svelte_diagnostics::Diagnostic;

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
    StartSnippetTag(StartSnippetTag<'a>),
    EndSnippetTag,
    RenderTag(RenderTagToken<'a>),
    HtmlTag(HtmlTagToken<'a>),
    StyleTag(StyleTag<'a>),
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScriptTag<'a> {
    pub source: &'a str,
    pub attributes: Vec<Attribute<'a>>,
}

impl ScriptTag<'_> {
    pub fn is_typescript(&self) -> bool {
        self.attributes.iter().any(|item| match item {
            Attribute::HTMLAttribute(attr) => {
                attr.name == "lang"
                    && matches!(attr.value, AttributeValue::String(v) if v == "ts")
            }
            _ => false,
        })
    }

    pub fn is_module(&self) -> bool {
        self.attributes.iter().any(|item| match item {
            Attribute::HTMLAttribute(attr) => {
                (attr.name == "module" && attr.value == AttributeValue::Empty)
                    || (attr.name == "context"
                        && matches!(attr.value, AttributeValue::String(v) if v == "module"))
            }
            _ => false,
        })
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

/// Any expression in curly braces `{ 1 + 1 }` or `{ name }` in the template.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExpressionTag<'a> {
    pub span: Span,
    pub expression: JsExpression<'a>,
}

/// The JS expression itself.
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
                for part in &concatenation.parts {
                    match part {
                        ConcatenationPart::String(v) => write!(f, "({})", v)?,
                        ConcatenationPart::Expression(et) => {
                            write!(f, "({{{}}})", et.expression.value)?
                        }
                    }
                }
                Ok(())
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

#[derive(Debug, PartialEq, Eq)]
pub struct StyleTag<'a> {
    pub source: &'a str,
    pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StartSnippetTag<'a> {
    pub name: &'a str,
    pub params: Option<JsExpression<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RenderTagToken<'a> {
    pub expression: JsExpression<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HtmlTagToken<'a> {
    pub expression: JsExpression<'a>,
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
