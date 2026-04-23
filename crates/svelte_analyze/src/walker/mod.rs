use oxc_ast::ast::{Expression, Statement};
use oxc_semantic::ScopeId;
use rustc_hash::FxHashSet;
use svelte_ast::{
    AnimateDirective, AttachTag, Attribute, AwaitBlock, BindDirective, ClassDirective,
    ComponentNode, ConcatPart, ConcatenationAttribute, ConstTag, DebugTag, EachBlock, Element,
    ExpressionAttribute, ExpressionTag, Fragment, HtmlTag, IfBlock, KeyBlock, LetDirectiveLegacy,
    Node, NodeId, OnDirectiveLegacy, RenderTag, SlotElementLegacy, SnippetBlock, SpreadAttribute,
    StyleDirective, StyleDirectiveValue, SvelteBody, SvelteBoundary, SvelteDocument, SvelteElement,
    SvelteFragmentLegacy, SvelteWindow, TransitionDirective, UseDirective,
};
use svelte_diagnostics::{extract_svelte_ignore, Diagnostic};
use svelte_span::Span;

use crate::types::data::{AnalysisData, JsAst};

mod context;
mod dispatch;
mod traverse;
mod visitor;

pub(crate) use crate::types::data::{ParentKind, ParentRef};
pub(crate) use context::VisitContext;
pub(crate) use traverse::walk_template;
pub(crate) use visitor::TemplateVisitor;
