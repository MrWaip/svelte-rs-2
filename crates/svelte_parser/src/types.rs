use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;

use rustc_hash::FxHashMap;
use svelte_span::Span;

pub struct JsAst<'a> {
    pub program: Option<oxc_ast::ast::Program<'a>>,

    pub module_program: Option<oxc_ast::ast::Program<'a>>,

    pending_exprs: FxHashMap<u32, Expression<'a>>,

    pending_stmts: FxHashMap<u32, Statement<'a>>,

    exprs: Vec<Option<Expression<'a>>>,

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

    pub fn alloc_expr(&mut self, offset: u32, expr: Expression<'a>) {
        self.pending_exprs.insert(offset, expr);
    }

    pub fn alloc_stmt(&mut self, offset: u32, stmt: Statement<'a>) {
        self.pending_stmts.insert(offset, stmt);
    }

    pub fn has_pending_expr(&self, offset: u32) -> bool {
        self.pending_exprs.contains_key(&offset)
    }

    pub fn has_pending_stmt(&self, offset: u32) -> bool {
        self.pending_stmts.contains_key(&offset)
    }

    pub fn pending_expr(&self, offset: u32) -> Option<&Expression<'a>> {
        self.pending_exprs.get(&offset)
    }

    pub fn pending_stmt(&self, offset: u32) -> Option<&Statement<'a>> {
        self.pending_stmts.get(&offset)
    }

    pub fn take_pending_expr(&mut self, offset: u32) -> Option<Expression<'a>> {
        self.pending_exprs.remove(&offset)
    }

    pub fn expr_at_offset(&self, offset: u32) -> Option<&Expression<'a>> {
        self.pending_exprs.get(&offset)
    }

    pub fn stmt_at_offset(&self, offset: u32) -> Option<&Statement<'a>> {
        self.pending_stmts.get(&offset)
    }

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
        for (offset, id) in expr_ids {
            if let Some(expr) = self.pending_exprs.remove(offset) {
                self.exprs[id.index()] = Some(expr);
            }
        }
        for (offset, id) in stmt_ids {
            if let Some(stmt) = self.pending_stmts.remove(offset) {
                self.stmts[id.index()] = Some(stmt);
            }
        }
        debug_assert!(
            self.pending_exprs
                .keys()
                .all(|offset| !expr_ids.contains_key(offset)),
            "drain_pending left bound expressions in pending_exprs",
        );
        debug_assert!(
            self.pending_stmts
                .keys()
                .all(|offset| !stmt_ids.contains_key(offset)),
            "drain_pending left bound statements in pending_stmts",
        );
    }

    pub fn expr(&self, id: OxcNodeId) -> Option<&Expression<'a>> {
        self.exprs.get(id.index()).and_then(Option::as_ref)
    }

    pub fn stmt(&self, id: OxcNodeId) -> Option<&Statement<'a>> {
        self.stmts.get(id.index()).and_then(Option::as_ref)
    }

    pub fn take_expr(&mut self, id: OxcNodeId) -> Option<Expression<'a>> {
        self.exprs.get_mut(id.index()).and_then(Option::take)
    }

    pub fn take_stmt(&mut self, id: OxcNodeId) -> Option<Statement<'a>> {
        self.stmts.get_mut(id.index()).and_then(Option::take)
    }

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

    pub fn iter_exprs(&self) -> impl Iterator<Item = &Expression<'a>> {
        self.exprs.iter().filter_map(Option::as_ref)
    }

    pub fn iter_stmts(&self) -> impl Iterator<Item = &Statement<'a>> {
        self.stmts.iter().filter_map(Option::as_ref)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CeShadowMode {
    Open,
    None,
}

#[derive(Debug, Clone)]
pub struct CePropConfig {
    pub name: String,
    pub attribute: Option<String>,
    pub reflect: bool,
    pub prop_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedCeConfig {
    pub tag: Option<String>,
    pub shadow: CeShadowMode,
    pub delegates_focus: bool,

    pub props: Vec<CePropConfig>,

    pub extend_span: Option<Span>,
}
