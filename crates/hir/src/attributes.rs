use crate::ExpressionId;

#[derive(Debug)]
pub enum Attribute<'hir> {
    StringAttribute(&'hir StringAttribute<'hir>),
    ExpressionAttribute(&'hir ExpressionAttribute<'hir>),
    ClassDirective(&'hir ClassDirective<'hir>),
    BindDirective(&'hir BindDirective<'hir>),
    BooleanAttribute(&'hir BooleanAttribute<'hir>),
    ConcatenationAttribute(&'hir ConcatenationAttribute<'hir>),
}

impl<'hir> Attribute<'hir> {
    pub fn name(&self) -> &'hir str {
        match self {
            Attribute::StringAttribute(it) => it.name,
            Attribute::ExpressionAttribute(it) => it.name,
            Attribute::ClassDirective(it) => it.name,
            Attribute::BindDirective(it) => it.name,
            Attribute::BooleanAttribute(it) => it.name,
            Attribute::ConcatenationAttribute(it) => it.name,
        }
    }

    pub fn contains_expression(&self) -> bool {
        return matches!(
            self,
            Self::BindDirective(_)
                | Self::ClassDirective(_)
                | Self::ConcatenationAttribute(_)
                | Self::ExpressionAttribute(_)
        );
    }
}

#[derive(Debug)]
pub struct StringAttribute<'hir> {
    pub name: &'hir str,
    pub value: &'hir str,
}

#[derive(Debug)]
pub struct ExpressionAttribute<'hir> {
    pub shorthand: bool,
    pub name: &'hir str,
    pub expression_id: ExpressionId,
}

#[derive(Debug)]
pub struct ClassDirective<'hir> {
    pub shorthand: bool,
    pub name: &'hir str,
    pub expression_id: ExpressionId,
}

#[derive(Debug)]
pub struct BindDirective<'hir> {
    pub shorthand: bool,
    pub name: &'hir str,
    pub expression_id: ExpressionId,
}

#[derive(Debug)]
pub struct BooleanAttribute<'hir> {
    pub name: &'hir str,
}

#[derive(Debug)]
pub struct ConcatenationAttribute<'hir> {
    pub parts: Vec<ConcatenationAttributePart<'hir>>,
    pub name: &'hir str,
}

#[derive(Debug)]
pub enum ConcatenationAttributePart<'hir> {
    String(&'hir str),
    Expression(ExpressionId),
}
