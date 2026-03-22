//! Shared types and OXC utilities for the Svelte compiler.
//!
//! Leaf crate providing domain types (`RuneKind`, `ScriptInfo`, etc.)
//! and OXC parsing helpers used across parser, analyze, transform, and codegen.

use oxc_ast::ast::Expression;

use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};
use svelte_ast::NodeId;
use svelte_span::Span;

/// Convert OXC Atom to CompactString without intermediate String allocation.
#[inline]
fn compact(s: &str) -> CompactString {
    CompactString::from(s)
}

// ---------------------------------------------------------------------------
// Public types (owned, no lifetime)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PropInfo {
    pub local_name: CompactString,
    pub prop_name: CompactString,
    pub default_span: Option<Span>,
    /// The raw text of the default expression (for codegen to parse).
    pub default_text: Option<String>,
    pub is_bindable: bool,
    pub is_rest: bool,
    /// True when the default expression is simple (literal, identifier, arrow).
    /// Pre-computed to avoid re-parsing in analyze.
    pub is_simple_default: bool,
}

#[derive(Debug, Clone)]
pub struct PropsDeclaration {
    pub props: Vec<PropInfo>,
}

#[derive(Debug, Clone)]
pub struct ExportInfo {
    pub name: CompactString,
    pub alias: Option<CompactString>,
}

#[derive(Debug, Clone)]
pub struct ScriptInfo {
    pub declarations: Vec<DeclarationInfo>,
    pub props_declaration: Option<PropsDeclaration>,
    pub exports: Vec<ExportInfo>,
    /// Base names of `$`-prefixed identifiers found in the script body
    /// (e.g. `"count"` for `$count`). Used to detect store subscriptions.
    pub store_candidates: Vec<CompactString>,
}

#[derive(Debug, Clone)]
pub struct DeclarationInfo {
    pub name: CompactString,
    pub span: Span,
    pub kind: DeclarationKind,
    pub init_span: Option<Span>,
    pub is_rune: Option<RuneKind>,
    /// For $derived/$derived.by: names referenced in the init expression.
    pub rune_init_refs: Vec<CompactString>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    Let,
    Const,
    Var,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuneKind {
    State,
    StateRaw,
    Derived,
    DerivedBy,
    Effect,
    EffectTracking,
    Props,
    Bindable,
    StateEager,
    EffectPending,
    Inspect,
    Host,
    PropsId,
}

impl RuneKind {
    pub fn is_derived(&self) -> bool {
        matches!(self, RuneKind::Derived | RuneKind::DerivedBy)
    }
}

// ---------------------------------------------------------------------------
// ParsedExprs — parsed JS expression ASTs in a shared OXC allocator
// ---------------------------------------------------------------------------

/// Parsed JS expression ASTs, stored in a shared OXC allocator.
/// Separate from AnalysisData to avoid lifetime propagation.
pub struct ParsedExprs<'a> {
    /// Template expressions: ExpressionTag, IfBlock test, EachBlock expr, RenderTag, HtmlTag.
    pub exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Attribute expressions, keyed by attribute NodeId.
    pub attr_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// ConcatenationAttribute dynamic parts: (attr_id, part_index).
    pub concat_part_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,
    /// EachBlock key expressions: keyed by EachBlock NodeId.
    pub key_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Pre-parsed script Program AST. Consumed by codegen via `Option::take()`.
    pub script_program: Option<oxc_ast::ast::Program<'a>>,
    /// DebugTag identifier expressions: (debug_tag_id, identifier_index) → transformed expression.
    pub debug_tag_exprs: FxHashMap<(NodeId, usize), Expression<'a>>,
    /// Pre-parsed custom element `extend` expression. Consumed by codegen via `Option::take()`.
    pub ce_extend_expr: Option<Expression<'a>>,
    /// Pre-parsed prop default expressions, indexed by prop position in PropsDeclaration.
    /// Consumed by codegen via clone/take.
    pub prop_default_exprs: Vec<Option<Expression<'a>>>,
    /// Pre-parsed each-block destructuring context bindings, keyed by EachBlock NodeId.
    /// Consumed by codegen via `remove()`.
    pub each_context_bindings: FxHashMap<NodeId, EachContextBinding<'a>>,
    /// Pre-parsed directive name expressions (use:, transition:, animate:).
    /// Keyed by directive NodeId. Consumed by codegen via `remove()`.
    pub directive_name_exprs: FxHashMap<NodeId, Expression<'a>>,
    /// Offsets for template expressions (needed by analyze to extract ExpressionInfo).
    pub expr_offsets: FxHashMap<NodeId, u32>,
    /// Offsets for attribute expressions.
    pub attr_expr_offsets: FxHashMap<NodeId, u32>,
    /// Offsets for concatenation part expressions.
    pub concat_part_offsets: FxHashMap<(NodeId, usize), u32>,
    /// Offsets for each-block key expressions.
    pub key_expr_offsets: FxHashMap<NodeId, u32>,
}

impl<'a> ParsedExprs<'a> {
    pub fn new() -> Self {
        Self {
            exprs: FxHashMap::default(),
            attr_exprs: FxHashMap::default(),
            concat_part_exprs: FxHashMap::default(),
            key_exprs: FxHashMap::default(),
            script_program: None,
            debug_tag_exprs: FxHashMap::default(),
            ce_extend_expr: None,
            prop_default_exprs: Vec::new(),
            each_context_bindings: FxHashMap::default(),
            directive_name_exprs: FxHashMap::default(),
            expr_offsets: FxHashMap::default(),
            attr_expr_offsets: FxHashMap::default(),
            concat_part_offsets: FxHashMap::default(),
            key_expr_offsets: FxHashMap::default(),
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
    /// Const tag declared binding names.
    pub const_tag_names: FxHashMap<NodeId, Vec<String>>,
    /// Await block then-binding patterns.
    pub await_values: FxHashMap<NodeId, AwaitBindingInfo>,
    /// Await block catch-binding patterns.
    pub await_errors: FxHashMap<NodeId, AwaitBindingInfo>,
    /// Render tags with ChainExpression callee.
    pub render_tag_is_chain: FxHashSet<NodeId>,
    /// Callee identifier name for render tags.
    pub render_tag_callee_name: FxHashMap<NodeId, String>,
    /// Attribute/directive whose expression is a simple identifier matching the name.
    pub expression_shorthand: FxHashSet<NodeId>,
    /// class={expr} attributes that need clsx resolution.
    pub needs_clsx: FxHashSet<NodeId>,
    /// Parsed custom element config.
    pub ce_config: Option<ParsedCeConfig>,
    /// Pre-computed snippet parameter names, keyed by SnippetBlock NodeId.
    pub snippet_param_names: FxHashMap<NodeId, Vec<String>>,
    /// Script content span for analyze to re-parse metadata.
    pub script_content_span: Option<Span>,
    /// Whether the script block uses TypeScript.
    pub typescript: bool,
    /// Pre-extracted script metadata (declarations, props, exports).
    /// Populated by parser from the script Program AST.
    pub script_info: Option<ScriptInfo>,
}

impl<'a> JsParseResult<'a> {
    pub fn new() -> Self {
        Self {
            parsed: ParsedExprs::new(),
            const_tag_names: FxHashMap::default(),
            await_values: FxHashMap::default(),
            await_errors: FxHashMap::default(),
            render_tag_is_chain: FxHashSet::default(),
            render_tag_callee_name: FxHashMap::default(),
            expression_shorthand: FxHashSet::default(),
            needs_clsx: FxHashSet::default(),
            ce_config: None,
            snippet_param_names: FxHashMap::default(),
            script_content_span: None,
            typescript: false,
            script_info: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Each context binding info
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




