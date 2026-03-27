use oxc_ast::ast::{Expression, Statement};

use rustc_hash::FxHashMap;
use svelte_span::Span;

pub struct ParserResult<'a> {
    pub program: Option<oxc_ast::ast::Program<'a>>,
    pub exprs: FxHashMap<u32, Expression<'a>>,
    pub stmts: FxHashMap<u32, Statement<'a>>,
    pub script_content_span: Option<Span>,
    pub typescript: bool,
}

impl<'a> ParserResult<'a> {
    pub fn new() -> Self {
        Self {
            program: None,
            exprs: FxHashMap::default(),
            stmts: FxHashMap::default(),
            script_content_span: None,
            typescript: false,
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
