//! Shared types and OXC utilities for the Svelte compiler.
//!
//! Leaf crate providing parsing result types (`ParsedExprs`, `JsParseResult`, etc.)
//! and OXC parsing helpers used across parser, analyze, transform, and codegen.

use oxc_ast::ast::Expression;

use compact_str::CompactString;
use rustc_hash::FxHashMap;
use svelte_span::Span;

/// Convert OXC Atom to CompactString without intermediate String allocation.
#[inline]
fn compact(s: &str) -> CompactString {
    CompactString::from(s)
}

// ---------------------------------------------------------------------------
// ParsedExprs — parsed JS expression ASTs in a shared OXC allocator
// ---------------------------------------------------------------------------

/// Parsed JS expression ASTs, stored in a shared OXC allocator.
/// Separate from AnalysisData to avoid lifetime propagation.
/// All expressions keyed by source offset (span.start).
pub struct ParsedExprs<'a> {
    /// Pre-parsed script Program AST. Consumed by codegen via `Option::take()`.
    pub program: Option<oxc_ast::ast::Program<'a>>,
    /// All parsed expressions keyed by source byte offset.
    pub exprs: FxHashMap<u32, Expression<'a>>,
    /// Parsed each-block destructuring contexts, keyed by context_span.start.
    pub each_contexts: FxHashMap<u32, EachContextBinding<'a>>,
}

impl<'a> ParsedExprs<'a> {
    pub fn new() -> Self {
        Self {
            program: None,
            exprs: FxHashMap::default(),
            each_contexts: FxHashMap::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// JsParseResult — intermediate JS parsing results passed from parser to analyze
// ---------------------------------------------------------------------------

/// All data produced by JS expression parsing.
/// Created by `svelte_parser::parse_with_js()`, consumed by `svelte_analyze::analyze()`.
pub struct JsParseResult<'a> {
    pub parsed: ParsedExprs<'a>,
    /// Script content span for analyze to re-parse metadata.
    pub script_content_span: Option<Span>,
    /// Whether the script block uses TypeScript.
    pub typescript: bool,
    /// ConstTag binding names extracted during OXC parsing, keyed by expression_span.start.
    /// Stored here (not ParsedExprs) because these are extracted metadata, not OXC AST.
    pub const_tag_names: FxHashMap<u32, Vec<compact_str::CompactString>>,
}

impl<'a> JsParseResult<'a> {
    pub fn new() -> Self {
        Self {
            parsed: ParsedExprs::new(),
            script_content_span: None,
            typescript: false,
            const_tag_names: FxHashMap::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Each context binding info (used by codegen, not stored in ParsedExprs)
// ---------------------------------------------------------------------------

/// Parsed destructuring context for `{#each items as { name, value }}` or `[a, b]`.
pub struct EachContextBinding<'a> {
    pub is_array: bool,
    pub bindings: Vec<EachBindingEntry<'a>>,
}

/// Single entry in a parsed destructuring context pattern.
pub struct EachBindingEntry<'a> {
    /// Binding name (alias if renamed, e.g. `alias` for `{ prop: alias }`).
    pub name: CompactString,
    /// Property key for object patterns (None = shorthand, i.e. `name == key_name`).
    pub key_name: Option<CompactString>,
    /// Pre-parsed default expression (e.g. `'N/A'` for `{ value = 'N/A' }`).
    pub default_expr: Option<Expression<'a>>,
}

// ---------------------------------------------------------------------------
// Await binding info
// ---------------------------------------------------------------------------

/// Destructuring kind for await block bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestructureKind {
    Array,
    Object,
}

/// Parsed binding pattern for `{:then value}` / `{:catch error}`.
#[derive(Debug, Clone, PartialEq)]
pub enum AwaitBindingInfo {
    /// Simple identifier: `{:then value}`
    Simple(String),
    /// Destructured: `{:then { name, age }}` or `{:then [a, b]}`
    Destructured {
        kind: DestructureKind,
        names: Vec<String>,
    },
}

impl AwaitBindingInfo {
    /// All binding names regardless of variant.
    pub fn names(&self) -> Vec<&str> {
        match self {
            Self::Simple(name) => vec![name.as_str()],
            Self::Destructured { names, .. } => names.iter().map(|s| s.as_str()).collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// Custom element config parsing
// ---------------------------------------------------------------------------

/// Shadow root mode for custom elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CeShadowMode {
    Open,
    None,
}

/// Single prop definition within a custom element config.
#[derive(Debug, Clone)]
pub struct CePropConfig {
    pub name: String,
    pub attribute: Option<String>,
    pub reflect: bool,
    pub prop_type: Option<String>,
}

/// Parsed custom element config from `<svelte:options customElement={{ ... }}>`.
#[derive(Debug, Clone)]
pub struct ParsedCeConfig {
    pub tag: Option<String>,
    pub shadow: CeShadowMode,
    /// Ordered list of prop definitions, preserving config order.
    pub props: Vec<CePropConfig>,
    /// Span of the `extend` expression value (absolute, within original source).
    pub extend_span: Option<Span>,
}

pub fn extract_all_binding_names(pattern: &oxc_ast::ast::BindingPattern<'_>, names: &mut Vec<CompactString>) {
    use oxc_ast::ast::BindingPattern;
    match pattern {
        BindingPattern::BindingIdentifier(id) => names.push(compact(&id.name)),
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                extract_all_binding_names(&prop.value, names);
            }
            if let Some(rest) = &obj.rest {
                extract_all_binding_names(&rest.argument, names);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                extract_all_binding_names(elem, names);
            }
            if let Some(rest) = &arr.rest {
                extract_all_binding_names(&rest.argument, names);
            }
        }
        BindingPattern::AssignmentPattern(assign) => extract_all_binding_names(&assign.left, names),
    }
}

pub fn is_simple_expr(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::Identifier(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_) => true,
        Expression::ConditionalExpression(c) => {
            is_simple_expr(&c.test) && is_simple_expr(&c.consequent) && is_simple_expr(&c.alternate)
        }
        Expression::BinaryExpression(b) => {
            is_simple_expr(&b.left) && is_simple_expr(&b.right)
        }
        Expression::LogicalExpression(l) => {
            is_simple_expr(&l.left) && is_simple_expr(&l.right)
        }
        _ => false,
    }
}
