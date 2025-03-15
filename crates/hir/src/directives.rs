use crate::ExpressionId;

#[derive(Debug)]
pub enum Directive<'hir> {
    Use(&'hir UseDirective<'hir>),
    Animation(&'hir AnimationDirective<'hir>),
    Bind(&'hir BindDirective<'hir>),
    Let(&'hir LetDirective<'hir>),
    On(&'hir OnDirective<'hir>),
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
