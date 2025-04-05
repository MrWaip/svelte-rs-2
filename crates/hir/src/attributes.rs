use crate::ExpressionId;

#[derive(Debug)]
pub enum AnyAttribute<'hir> {
    StringAttribute(&'hir StringAttribute<'hir>),
    ExpressionAttribute(&'hir ExpressionAttribute<'hir>),
    SpreadAttribute(&'hir SpreadAttribute),
    BooleanAttribute(&'hir BooleanAttribute<'hir>),
    ConcatenationAttribute(&'hir ConcatenationAttribute<'hir>),
    Use(&'hir UseDirective<'hir>),
    Animation(&'hir AnimationDirective<'hir>),
    Bind(&'hir BindDirective<'hir>),
    On(&'hir OnDirective<'hir>),
    Transition(&'hir TransitionDirective<'hir>),
    Class(&'hir ClassDirective<'hir>),
    Style(&'hir StyleDirective<'hir>),
    Let(&'hir LetDirective<'hir>),
}

impl<'hir> AnyAttribute<'hir> {
    pub fn from_attr(attr: &Attribute<'hir>) -> Self {
        match attr {
            Attribute::StringAttribute(it) => Self::StringAttribute(it),
            Attribute::ExpressionAttribute(it) => Self::ExpressionAttribute(it),
            Attribute::SpreadAttribute(it) => Self::SpreadAttribute(it),
            Attribute::BooleanAttribute(it) => Self::BooleanAttribute(it),
            Attribute::ConcatenationAttribute(it) => Self::ConcatenationAttribute(it),
        }
    }

    pub fn is_dynamic(&self) -> bool {
        !matches!(self, Self::StringAttribute(_) | Self::BooleanAttribute(_))
    }
}

#[derive(Debug)]
pub enum Attribute<'hir> {
    StringAttribute(&'hir StringAttribute<'hir>),
    ExpressionAttribute(&'hir ExpressionAttribute<'hir>),
    SpreadAttribute(&'hir SpreadAttribute),
    BooleanAttribute(&'hir BooleanAttribute<'hir>),
    ConcatenationAttribute(&'hir ConcatenationAttribute<'hir>),
}

impl<'hir> Attribute<'hir> {
    pub fn name(&self) -> Option<&'hir str> {
        match self {
            Attribute::StringAttribute(it) => it.name.into(),
            Attribute::ExpressionAttribute(it) => it.name.into(),
            Attribute::SpreadAttribute(_) => None,
            Attribute::BooleanAttribute(it) => it.name.into(),
            Attribute::ConcatenationAttribute(it) => it.name.into(),
        }
    }

    pub fn contains_expression(&self) -> bool {
        matches!(
            self,
            Self::ConcatenationAttribute(_) | Self::ExpressionAttribute(_)
        )
    }

    pub fn is_spread(&self) -> bool {
        matches!(self, Self::SpreadAttribute(_))
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
pub struct SpreadAttribute {
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

//  directives -------------------------------------------

#[derive(Debug)]
pub enum Directives<'hir> {
    Use(&'hir UseDirective<'hir>),
    Animation(&'hir AnimationDirective<'hir>),
    Bind(&'hir BindDirective<'hir>),
    On(&'hir OnDirective<'hir>),
    Transition(&'hir TransitionDirective<'hir>),
    Class(&'hir ClassDirective<'hir>),
    Style(&'hir StyleDirective<'hir>),
    Let(&'hir LetDirective<'hir>),
}

#[derive(Debug)]
pub struct StyleDirective<'hir> {
    pub name: &'hir str,
}

#[derive(Debug)]
pub struct ClassDirective<'hir> {
    pub shorthand: bool,
    pub name: &'hir str,
    pub expression_id: ExpressionId,
}

#[derive(Debug)]
pub struct AnimationDirective<'hir> {
    pub name: &'hir str,
}

#[derive(Debug)]
pub struct BindDirective<'hir> {
    pub shorthand: bool,
    pub name: &'hir str,
    pub expression_id: ExpressionId,
}
#[derive(Debug)]
pub struct LetDirective<'hir> {
    pub name: &'hir str,
}

#[derive(Debug)]
pub struct OnDirective<'hir> {
    pub name: &'hir str,
}

#[derive(Debug)]
pub struct TransitionDirective<'hir> {
    pub name: &'hir str,
}

#[derive(Debug)]
pub struct UseDirective<'hir> {
    pub name: &'hir str,
}
