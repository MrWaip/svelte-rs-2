use oxc_ast::ast::Expression;

use crate::metadata::AttributeMetadata;

#[derive(Debug)]
pub enum AttributeKind {
    Unknown,
    Value,
    Checked,
    Group,
}

impl AttributeKind {
    pub fn from_str(value: &str) -> Self {
        match value {
            "group" => AttributeKind::Group,
            "checked" => AttributeKind::Checked,
            "value" => AttributeKind::Value,
            _ => AttributeKind::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum Attribute<'a> {
    ExpressionAttribute(ExpressionAttribute<'a>),
    ClassDirective(ClassDirective<'a>),
    BindDirective(BindDirective<'a>),
    BooleanAttribute(BooleanAttribute<'a>),
    StringAttribute(StringAttribute<'a>),
    ConcatenationAttribute(ConcatenationAttribute<'a>),
}

#[derive(Debug)]
pub struct ExpressionAttribute<'a> {
    pub shorthand: bool,
    pub name: &'a str,
    pub kind: AttributeKind,
    pub expression: Expression<'a>,
    pub metadata: Option<AttributeMetadata>,
}

impl<'a> ExpressionAttribute<'a> {
    pub fn as_attribute(self) -> Attribute<'a> {
        Attribute::ExpressionAttribute(self)
    }
}

#[derive(Debug)]
pub struct ClassDirective<'a> {
    pub shorthand: bool,
    pub name: &'a str,
    pub expression: Expression<'a>,
    pub metadata: Option<AttributeMetadata>,
}

impl<'a> ClassDirective<'a> {
    pub fn as_attribute(self) -> Attribute<'a> {
        Attribute::ClassDirective(self)
    }
}

#[derive(Debug)]
pub struct BindDirective<'a> {
    pub shorthand: bool,
    pub name: &'a str,
    pub kind: BindDirectiveKind,
    pub expression: Expression<'a>,
    pub metadata: Option<AttributeMetadata>,
}

impl<'a> BindDirective<'a> {
    pub fn as_attribute(self) -> Attribute<'a> {
        Attribute::BindDirective(self)
    }
}

#[derive(Debug)]
pub enum BindDirectiveKind {
    Unknown,
    Value,
    Group,
    Checked,
}

impl BindDirectiveKind {
    pub fn is_group(&self) -> bool {
        matches!(self, BindDirectiveKind::Group)
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "checked" => BindDirectiveKind::Checked,
            "group" => BindDirectiveKind::Group,
            "value" => BindDirectiveKind::Value,
            _ => BindDirectiveKind::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct BooleanAttribute<'a> {
    pub name: &'a str,
    pub kind: AttributeKind,
}

impl<'a> BooleanAttribute<'a> {
    pub fn as_attribute(self) -> Attribute<'a> {
        Attribute::BooleanAttribute(self)
    }
}

#[derive(Debug)]
pub struct StringAttribute<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub kind: AttributeKind,
}

impl<'a> StringAttribute<'a> {
    pub fn as_attribute(self) -> Attribute<'a> {
        Attribute::StringAttribute(self)
    }
}

#[derive(Debug)]
pub struct ConcatenationAttribute<'a> {
    pub parts: Vec<ConcatenationPart<'a>>,
    pub name: &'a str,
    pub kind: AttributeKind,
    pub metadata: Option<AttributeMetadata>,
}

impl<'a> ConcatenationAttribute<'a> {
    pub fn as_attribute(self) -> Attribute<'a> {
        Attribute::ConcatenationAttribute(self)
    }
}

#[derive(Debug)]
pub enum ConcatenationPart<'a> {
    String(&'a str),
    Expression(Expression<'a>),
}
