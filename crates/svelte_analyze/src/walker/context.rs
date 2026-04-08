use super::*;

pub(crate) struct VisitContext<'a> {
    pub scope: ScopeId,
    pub data: &'a mut AnalysisData,
    parsed: Option<&'a ParserResult<'a>>,
    pub store: &'a svelte_ast::AstStore,
    parents: Vec<ParentRef>,
    element_name: Option<String>,
    pub source: &'a str,
    pub runes: bool,
    component_name: &'a str,
    filename_basename: &'a str,
    ignore_current: FxHashSet<String>,
    ignore_stack: Vec<FxHashSet<String>>,
    warnings: Vec<Diagnostic>,
}

impl<'a> VisitContext<'a> {
    pub fn new(
        scope: ScopeId,
        data: &'a mut AnalysisData,
        store: &'a svelte_ast::AstStore,
        source: &'a str,
        runes: bool,
        component_name: &'a str,
        filename_basename: &'a str,
    ) -> Self {
        Self {
            scope,
            data,
            parsed: None,
            store,
            parents: Vec::new(),
            element_name: None,
            source,
            runes,
            component_name,
            filename_basename,
            ignore_current: FxHashSet::default(),
            ignore_stack: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn with_parsed(
        scope: ScopeId,
        data: &'a mut AnalysisData,
        store: &'a svelte_ast::AstStore,
        parsed: &'a ParserResult<'a>,
        source: &'a str,
        runes: bool,
        component_name: &'a str,
        filename_basename: &'a str,
    ) -> Self {
        Self {
            scope,
            data,
            parsed: Some(parsed),
            store,
            parents: Vec::new(),
            element_name: None,
            source,
            runes,
            component_name,
            filename_basename,
            ignore_current: FxHashSet::default(),
            ignore_stack: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn parsed(&self) -> Option<&'a ParserResult<'a>> {
        self.parsed
    }

    pub fn parent(&self) -> Option<ParentRef> {
        self.parents.last().copied()
    }

    pub fn ancestors(&self) -> impl Iterator<Item = &ParentRef> {
        self.parents.iter().rev()
    }

    pub fn element_name(&self) -> Option<&str> {
        self.element_name.as_deref()
    }

    pub fn component_name(&self) -> &str {
        self.component_name
    }

    pub fn filename_basename(&self) -> &str {
        self.filename_basename
    }

    pub fn nearest_element(&self) -> Option<NodeId> {
        self.ancestors().find(|p| p.kind.is_element()).map(|p| p.id)
    }

    pub fn push_ignore(&mut self, codes: Vec<String>) {
        let prev = std::mem::take(&mut self.ignore_current);
        let mut next = prev.clone();
        next.extend(codes);
        self.ignore_stack.push(prev);
        self.ignore_current = next;
    }

    pub(crate) fn child_scope(&self, key: FragmentKey, parent_scope: ScopeId) -> ScopeId {
        match key {
            FragmentKey::IfConsequent(id) => self.data.if_consequent_scope(id, parent_scope),
            FragmentKey::IfAlternate(id) => self.data.if_alternate_scope(id, parent_scope),
            FragmentKey::EachBody(id) => self.data.each_body_scope(id, parent_scope),
            FragmentKey::SnippetBody(id) => self.data.snippet_body_scope(id, parent_scope),
            FragmentKey::KeyBlockBody(id) => self.data.key_block_body_scope(id, parent_scope),
            FragmentKey::SvelteHeadBody(id) => self.data.svelte_head_body_scope(id, parent_scope),
            FragmentKey::SvelteElementBody(id) => {
                self.data.svelte_element_body_scope(id, parent_scope)
            }
            FragmentKey::SvelteBoundaryBody(id) => {
                self.data.svelte_boundary_body_scope(id, parent_scope)
            }
            FragmentKey::AwaitPending(id) => self.data.await_pending_scope(id, parent_scope),
            FragmentKey::AwaitThen(id) => self.data.await_then_scope(id, parent_scope),
            FragmentKey::AwaitCatch(id) => self.data.await_catch_scope(id, parent_scope),
            _ => parent_scope,
        }
    }

    pub fn pop_ignore(&mut self) {
        if let Some(prev) = self.ignore_stack.pop() {
            self.ignore_current = prev;
        }
    }

    pub fn record_ignore_for_node(&mut self, node_id: NodeId) {
        if !self.ignore_current.is_empty() {
            let idx = self
                .data
                .output
                .ignore_data
                .intern_snapshot(&self.ignore_current);
            self.data.output.ignore_data.set_snapshot(node_id, idx);
        }
    }

    pub fn take_warnings(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.warnings)
    }

    pub(crate) fn push(&mut self, r: ParentRef) {
        self.parents.push(r);
    }

    pub(crate) fn pop(&mut self) {
        self.parents.pop();
    }

    pub(crate) fn replace_element_name(&mut self, name: String) -> Option<String> {
        self.element_name.replace(name)
    }

    pub(crate) fn set_element_name(&mut self, name: Option<String>) {
        self.element_name = name;
    }

    pub(crate) fn warnings_mut(&mut self) -> &mut Vec<Diagnostic> {
        &mut self.warnings
    }
}
