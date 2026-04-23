use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;

use rustc_hash::FxHashMap;
use svelte_span::Span;

pub struct JsAst<'a> {
    /// Parsed program for the instance `<script>` block.
    pub program: Option<oxc_ast::ast::Program<'a>>,
    /// Parsed program for the `<script module>` block.
    pub module_program: Option<oxc_ast::ast::Program<'a>>,
    /// Parser staging area: span.start → owned Expression. Drained by
    /// `svelte_analyze::build_component_semantics` after the semantic pass
    /// fills each `ExprRef.oxc_id`. Empty after analyze.
    pending_exprs: FxHashMap<u32, Expression<'a>>,
    /// Parser staging area: span.start → owned Statement. Drained alongside
    /// `pending_exprs`.
    pending_stmts: FxHashMap<u32, Statement<'a>>,
    /// Final storage: `OxcNodeId.index()` → owned Expression. Filled during
    /// drain. The slot becomes `None` while transform/codegen take/put
    /// expressions out for traversal.
    exprs: Vec<Option<Expression<'a>>>,
    /// Final storage: `OxcNodeId.index()` → owned Statement.
    stmts: Vec<Option<Statement<'a>>>,
    pub script_content_span: Option<Span>,
    pub module_script_content_span: Option<Span>,
    pub typescript: bool,
}

impl<'a> Default for JsAst<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> JsAst<'a> {
    pub fn new() -> Self {
        Self {
            program: None,
            module_program: None,
            pending_exprs: FxHashMap::default(),
            pending_stmts: FxHashMap::default(),
            exprs: Vec::new(),
            stmts: Vec::new(),
            script_content_span: None,
            module_script_content_span: None,
            typescript: false,
        }
    }

    // -- Parser staging API (write-only) --

    /// Parser stages an expression by source offset. Drained by analyze.
    pub fn alloc_expr(&mut self, offset: u32, expr: Expression<'a>) {
        self.pending_exprs.insert(offset, expr);
    }

    /// Parser stages a statement by source offset.
    pub fn alloc_stmt(&mut self, offset: u32, stmt: Statement<'a>) {
        self.pending_stmts.insert(offset, stmt);
    }

    // -- Drain bridge (used only by build_component_semantics) --

    /// True iff an expression is staged at this source offset. Used by
    /// `build_component_semantics` to decide whether to run the visitor.
    pub fn has_pending_expr(&self, offset: u32) -> bool {
        self.pending_exprs.contains_key(&offset)
    }

    pub fn has_pending_stmt(&self, offset: u32) -> bool {
        self.pending_stmts.contains_key(&offset)
    }

    /// Read the staged expression by source offset without consuming it.
    /// Used by `build_component_semantics::AnalyzeTemplateWalker` to feed
    /// the JS semantic visitor before drain.
    pub fn pending_expr(&self, offset: u32) -> Option<&Expression<'a>> {
        self.pending_exprs.get(&offset)
    }

    pub fn pending_stmt(&self, offset: u32) -> Option<&Statement<'a>> {
        self.pending_stmts.get(&offset)
    }

    /// Consume a parser-staged expression by source offset. Used by
    /// codegen sites whose expression isn't routed through the template
    /// AST (e.g. `<svelte:options customElement={...}>` extend value).
    pub fn take_pending_expr(&mut self, offset: u32) -> Option<Expression<'a>> {
        self.pending_exprs.remove(&offset)
    }

    /// Span-keyed lookup — works during the parser-staging window (before
    /// `drain_pending`). Read-only consumers throughout analyze use this
    /// while the staging map is alive. After drain, these return `None`;
    /// post-drain callers should use `expr(OxcNodeId)`.
    pub fn expr_at_offset(&self, offset: u32) -> Option<&Expression<'a>> {
        self.pending_exprs.get(&offset)
    }

    pub fn stmt_at_offset(&self, offset: u32) -> Option<&Statement<'a>> {
        self.pending_stmts.get(&offset)
    }

    /// Move all staged expressions into the final `OxcNodeId`-indexed
    /// storage. `expr_ids`/`stmt_ids` map source offset → bound OxcNodeId
    /// (filled by `ExprRef.bind` during semantic pass). Called once at the
    /// end of `build_component_semantics::build`.
    pub fn drain_pending(
        &mut self,
        expr_ids: &FxHashMap<u32, OxcNodeId>,
        stmt_ids: &FxHashMap<u32, OxcNodeId>,
    ) {
        let max_expr_id = expr_ids.values().map(|id| id.index()).max().unwrap_or(0);
        let max_stmt_id = stmt_ids.values().map(|id| id.index()).max().unwrap_or(0);
        if !expr_ids.is_empty() {
            self.exprs.resize_with(max_expr_id + 1, || None);
        }
        if !stmt_ids.is_empty() {
            self.stmts.resize_with(max_stmt_id + 1, || None);
        }
        // Drain only entries with bound OxcNodeId. Entries without a binding
        // (e.g. `<svelte:options customElement={...}>` extend expression that
        // is never visited by the template walker) stay in pending storage
        // and remain accessible via `pending_expr` / `pending_stmt`.
        let mut drained_expr = Vec::with_capacity(expr_ids.len());
        for offset in expr_ids.keys() {
            if let Some(expr) = self.pending_exprs.remove(offset) {
                drained_expr.push((*offset, expr));
            }
        }
        for (offset, expr) in drained_expr {
            if let Some(id) = expr_ids.get(&offset) {
                self.exprs[id.index()] = Some(expr);
            }
        }
        let mut drained_stmt = Vec::with_capacity(stmt_ids.len());
        for offset in stmt_ids.keys() {
            if let Some(stmt) = self.pending_stmts.remove(offset) {
                drained_stmt.push((*offset, stmt));
            }
        }
        for (offset, stmt) in drained_stmt {
            if let Some(id) = stmt_ids.get(&offset) {
                self.stmts[id.index()] = Some(stmt);
            }
        }
    }

    // -- OxcNodeId-keyed read/mutate API (the only API after drain) --

    /// Read the expression for this `OxcNodeId`. Returns `None` if the slot
    /// was taken by transform/codegen and not yet returned.
    pub fn expr(&self, id: OxcNodeId) -> Option<&Expression<'a>> {
        self.exprs.get(id.index()).and_then(Option::as_ref)
    }

    /// Read the statement for this `OxcNodeId`.
    pub fn stmt(&self, id: OxcNodeId) -> Option<&Statement<'a>> {
        self.stmts.get(id.index()).and_then(Option::as_ref)
    }

    /// Consume the stored expression. The slot is left empty until
    /// `replace_expr` puts it back. Used by transform's traverse cycle and
    /// by codegen to splice the pre-transformed expression.
    pub fn take_expr(&mut self, id: OxcNodeId) -> Option<Expression<'a>> {
        self.exprs.get_mut(id.index()).and_then(Option::take)
    }

    pub fn take_stmt(&mut self, id: OxcNodeId) -> Option<Statement<'a>> {
        self.stmts.get_mut(id.index()).and_then(Option::take)
    }

    /// Put a (possibly rewritten) expression back under its id.
    pub fn replace_expr(&mut self, id: OxcNodeId, expr: Expression<'a>) -> Option<Expression<'a>> {
        let idx = id.index();
        if self.exprs.len() <= idx {
            self.exprs.resize_with(idx + 1, || None);
        }
        self.exprs[idx].replace(expr)
    }

    pub fn replace_stmt(&mut self, id: OxcNodeId, stmt: Statement<'a>) -> Option<Statement<'a>> {
        let idx = id.index();
        if self.stmts.len() <= idx {
            self.stmts.resize_with(idx + 1, || None);
        }
        self.stmts[idx].replace(stmt)
    }

    /// Iterate all parsed template/attribute expressions in OxcNodeId order.
    /// Skips empty slots (in-flight take_expr).
    pub fn iter_exprs(&self) -> impl Iterator<Item = &Expression<'a>> {
        self.exprs.iter().filter_map(Option::as_ref)
    }

    /// Iterate all parsed template statements.
    pub fn iter_stmts(&self) -> impl Iterator<Item = &Statement<'a>> {
        self.stmts.iter().filter_map(Option::as_ref)
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
