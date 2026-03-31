use super::*;

#[derive(Debug, Default)]
pub struct BlockerData {
    pub(crate) symbol_blockers: FxHashMap<SymbolId, u32>,
    pub(crate) async_thunk_count: u32,
    pub(crate) has_async: bool,
    pub(crate) first_await_index: Option<usize>,
    pub(crate) stmt_metas: Vec<AsyncStmtMeta>,
}

#[derive(Debug, Clone)]
pub struct AsyncStmtMeta {
    pub(crate) has_await: bool,
    pub(crate) hoist_names: Vec<String>,
}

impl AsyncStmtMeta {
    pub fn has_await(&self) -> bool {
        self.has_await
    }
    pub fn hoist_names(&self) -> &[String] {
        &self.hoist_names
    }
}

impl BlockerData {
    pub fn has_async(&self) -> bool {
        self.has_async
    }

    pub fn symbol_blocker(&self, sym: SymbolId) -> Option<u32> {
        self.symbol_blockers.get(&sym).copied()
    }

    pub fn first_await_index(&self) -> Option<usize> {
        self.first_await_index
    }

    pub fn stmt_meta(&self, stmt_index: usize) -> Option<&AsyncStmtMeta> {
        let first = self.first_await_index?;
        self.stmt_metas.get(stmt_index - first)
    }
}
