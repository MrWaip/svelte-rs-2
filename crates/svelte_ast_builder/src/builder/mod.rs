use oxc_allocator::{Allocator, Box, CloneIn};
use oxc_ast::{
    ast::{
        self, Argument, ArrowFunctionExpression, AssignmentTarget, BindingIdentifier,
        CallExpression, ChainElement, ComputedMemberExpression, ExportDefaultDeclarationKind,
        Expression, FormalParameters, Function, FunctionType, IdentifierReference,
        ImportDeclarationSpecifier, ImportOrExportKind, ModuleDeclaration, NumericLiteral, Program,
        Statement, StaticMemberExpression, StringLiteral, TemplateElementValue, TemplateLiteral,
        VariableDeclarationKind,
    },
    AstBuilder, NONE,
};
use oxc_parser::Parser as OxcParser;
use oxc_span::{Atom, SourceType, Span, SPAN};
use oxc_syntax::node::NodeId as OxcNodeId;
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
    /// Borrowed string literal — avoids heap allocation when a &str is available.
    StrRef(&'short str),
    Num(f64),
    Ident(&'short str),
    #[allow(dead_code)]
    IdentRef(IdentifierReference<'a>),
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
    /// Expression part. The boolean indicates whether the expression is
    /// guaranteed to be defined (non-null/undefined). When `false`, the
    /// builder wraps it with `?? ""` so that interpolating `null`/`undefined`
    /// produces an empty string instead of `"null"`/`"undefined"`.
    Expr(Expression<'a>, /* defined */ bool),
}

pub struct Builder<'a> {
    pub ast: AstBuilder<'a>,
}

/// Property in an object literal expression.
pub enum ObjProp<'a> {
    /// `key: value`
    KeyValue(&'a str, Expression<'a>),
    /// `name(...) { ... }`
    Method(&'a str, Expression<'a>),
    /// `name` (property shorthand, equivalent to `name: name`)
    Shorthand(&'a str),
    /// `...expr`
    Spread(Expression<'a>),
    /// `get name() { return expr }`
    Getter(&'a str, Expression<'a>),
    /// `get name() { stmts... }` — multi-statement getter body
    GetterBody(&'a str, Vec<Statement<'a>>),
    /// `set name(param_name = default?) { body }`
    Setter(&'a str, &'a str, Option<Expression<'a>>, Vec<Statement<'a>>),
    /// `[computed_key]: value` — computed property key
    Computed(Expression<'a>, Expression<'a>),
}
