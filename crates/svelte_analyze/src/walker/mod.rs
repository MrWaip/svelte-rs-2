use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::ScopeId;
use rustc_hash::FxHashSet;
use svelte_ast::{
    AnimateDirective, AttachTag, Attribute, AwaitBlock, BindDirective, ClassDirective,
    ConcatPart, ConcatenationAttribute, ComponentNode, ConstTag, DebugTag, EachBlock, Element,
    ExpressionAttribute, ExpressionTag, Fragment, HtmlTag, IfBlock, KeyBlock, Node, NodeId,
    OnDirectiveLegacy, RenderTag, Shorthand, SnippetBlock, SpreadAttribute, StyleDirective,
    StyleDirectiveValue, SvelteBody, SvelteBoundary, SvelteDocument, SvelteElement, SvelteWindow,
    TransitionDirective, UseDirective,
};
use svelte_diagnostics::{extract_svelte_ignore, Diagnostic};
use svelte_span::Span;

use crate::types::data::{AnalysisData, FragmentKey, ParserResult};

mod context;
mod dispatch;
mod traverse;
mod visitor;

pub(crate) use context::{ParentKind, ParentRef, VisitContext};
pub(crate) use traverse::walk_template;
pub(crate) use visitor::TemplateVisitor;
