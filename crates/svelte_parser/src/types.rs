use oxc_ast::ast::{Expression, Statement};

use rustc_hash::FxHashMap;
use svelte_span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprHandle(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StmtHandle(pub u32);

pub struct ParserResult<'a> {
    pub program: Option<oxc_ast::ast::Program<'a>>,
    pub exprs: FxHashMap<ExprHandle, Expression<'a>>,
    pub stmts: FxHashMap<StmtHandle, Statement<'a>>,
    expr_by_offset: FxHashMap<u32, ExprHandle>,
    stmt_by_offset: FxHashMap<u32, StmtHandle>,
    next_expr: u32,
    next_stmt: u32,
    pub script_content_span: Option<Span>,
    pub typescript: bool,
}

impl<'a> ParserResult<'a> {
    pub fn new() -> Self {
        Self {
            program: None,
            exprs: FxHashMap::default(),
            stmts: FxHashMap::default(),
            expr_by_offset: FxHashMap::default(),
            stmt_by_offset: FxHashMap::default(),
            next_expr: 0,
            next_stmt: 0,
            script_content_span: None,
            typescript: false,
        }
    }

    pub fn alloc_expr(&mut self, offset: u32, expr: Expression<'a>) -> ExprHandle {
        let handle = ExprHandle(self.next_expr);
        self.next_expr += 1;
        self.expr_by_offset.insert(offset, handle);
        self.exprs.insert(handle, expr);
        handle
    }

    pub fn alloc_stmt(&mut self, offset: u32, stmt: Statement<'a>) -> StmtHandle {
        let handle = StmtHandle(self.next_stmt);
        self.next_stmt += 1;
        self.stmt_by_offset.insert(offset, handle);
        self.stmts.insert(handle, stmt);
        handle
    }

    pub fn expr_handle(&self, offset: u32) -> Option<ExprHandle> {
        self.expr_by_offset.get(&offset).copied()
    }

    pub fn stmt_handle(&self, offset: u32) -> Option<StmtHandle> {
        self.stmt_by_offset.get(&offset).copied()
    }

    pub fn expr(&self, handle: ExprHandle) -> Option<&Expression<'a>> {
        self.exprs.get(&handle)
    }

    pub fn expr_mut(&mut self, handle: ExprHandle) -> Option<&mut Expression<'a>> {
        self.exprs.get_mut(&handle)
    }

    pub fn take_expr(&mut self, handle: ExprHandle) -> Option<Expression<'a>> {
        self.exprs.remove(&handle)
    }

    pub fn stmt(&self, handle: StmtHandle) -> Option<&Statement<'a>> {
        self.stmts.get(&handle)
    }

    pub fn stmt_mut(&mut self, handle: StmtHandle) -> Option<&mut Statement<'a>> {
        self.stmts.get_mut(&handle)
    }

    pub fn take_stmt(&mut self, handle: StmtHandle) -> Option<Statement<'a>> {
        self.stmts.remove(&handle)
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
