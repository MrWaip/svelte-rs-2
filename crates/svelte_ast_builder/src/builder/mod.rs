use oxc_allocator::{Allocator, Box, CloneIn};
use oxc_ast::{
    AstBuilder, NONE,
    ast::{
        self, Argument, ArrowFunctionExpression, AssignmentTarget, BindingIdentifier,
        CallExpression, ChainElement, ComputedMemberExpression, ExportDefaultDeclarationKind,
        Expression, FormalParameters, Function, FunctionType, IdentifierReference,
        ImportDeclarationSpecifier, ImportOrExportKind, ModuleDeclaration, NumericLiteral, Program,
        Statement, StaticMemberExpression, StringLiteral, TemplateElementValue, TemplateLiteral,
        VariableDeclarationKind,
    },
};
use oxc_parser::Parser as OxcParser;
use oxc_span::{Atom, SPAN, SourceType, Span};
use oxc_syntax::node::NodeId as OxcNodeId;
use oxc_syntax::scope::ScopeId;
use std::cell::Cell;

mod base;
mod calls;
mod classes;
mod functions;
mod members;
mod modules;
mod objects;
mod statements;
mod svelte_patterns;
mod templates;

pub enum Arg<'a, 'short> {
    Str(String),

    StrRef(&'short str),
    Num(f64),
    Ident(&'short str),
    Expr(Expression<'a>),
    Arrow(ArrowFunctionExpression<'a>),
    Bool(bool),
    Spread(Expression<'a>),
}

pub enum AssignLeft<'a> {
    StaticMember(StaticMemberExpression<'a>),
    ComputedMember(ComputedMemberExpression<'a>),
    Ident(String),
}

pub enum TemplatePart<'a> {
    Str(String),

    Expr(Expression<'a>, /* defined */ bool),
}

pub struct Builder<'a> {
    pub ast: AstBuilder<'a>,
}

pub enum ObjProp<'a> {
    KeyValue(&'a str, Expression<'a>),

    Method(&'a str, Expression<'a>),

    Shorthand(&'a str),

    Spread(Expression<'a>),

    Getter(&'a str, Expression<'a>),

    GetterBody(&'a str, Vec<Statement<'a>>),

    Setter(&'a str, &'a str, Option<Expression<'a>>, Vec<Statement<'a>>),

    Computed(Expression<'a>, Expression<'a>),
}
