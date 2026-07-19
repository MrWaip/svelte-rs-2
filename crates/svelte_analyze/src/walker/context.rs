use std::mem;

use super::*;
use svelte_diagnostics::DiagnosticKind;

pub(crate) struct VisitContext<'d, 'a> {
    pub scope: ScopeId,
    pub data: &'d mut AnalysisData<'a>,
    pub(crate) parsed: Option<&'d JsAst<'a>>,
    pub store: &'d svelte_ast::AstStore,
    parents: Vec<ParentRef>,
    element_name_id: Option<NodeId>,
    pub source: &'d str,
    pub runes: bool,
    component_name: &'d str,
    filename_basename: &'d str,
    ignore_current: FxHashSet<String>,
    ignore_current_idx: Option<u32>,
    ignore_stack: Vec<(FxHashSet<String>, Option<u32>)>,
    warnings: Vec<Diagnostic>,
    current_fragment_id: Option<svelte_ast::FragmentId>,
}

impl<'d, 'a> VisitContext<'d, 'a> {
    pub fn with_parsed(
        scope: ScopeId,
        data: &'d mut AnalysisData<'a>,
        store: &'d svelte_ast::AstStore,
        parsed: &'d JsAst<'a>,
        source: &'d str,
        runes: bool,
        component_name: &'d str,
        filename_basename: &'d str,
    ) -> Self {
        Self {
            scope,
            data,
            parsed: Some(parsed),
            store,
            parents: Vec::with_capacity(32),
            element_name_id: None,
            source,
            runes,
            component_name,
            filename_basename,
            ignore_current: FxHashSet::default(),
            ignore_current_idx: None,
            ignore_stack: Vec::new(),
            warnings: Vec::new(),
            current_fragment_id: None,
        }
    }

    pub(crate) fn set_current_fragment(
        &mut self,
        id: svelte_ast::FragmentId,
    ) -> Option<svelte_ast::FragmentId> {
        self.current_fragment_id.replace(id)
    }

    pub(crate) fn restore_current_fragment(&mut self, prev: Option<svelte_ast::FragmentId>) {
        self.current_fragment_id = prev;
    }

    pub fn current_fragment_id(&self) -> svelte_ast::FragmentId {
        self.current_fragment_id
            .expect("current_fragment_id queried outside walk_template")
    }

    pub fn parsed(&self) -> Option<&JsAst<'a>> {
        self.parsed
    }

    pub fn parent(&self) -> Option<ParentRef> {
        self.parents.last().copied()
    }

    pub fn ancestors(&self) -> impl Iterator<Item = &ParentRef> {
        self.parents.iter().rev()
    }

    pub fn element_name(&self) -> Option<&str> {
        let id = self.element_name_id?;
        match self.store.get(id) {
            svelte_ast::Node::Element(el) => Some(el.name.as_str()),
            _ => None,
        }
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
        let prev = mem::take(&mut self.ignore_current);
        let mut next = prev.clone();
        next.extend(codes);
        self.ignore_stack.push((prev, self.ignore_current_idx));
        self.ignore_current = next;
        self.ignore_current_idx = None;
    }

    pub(crate) fn child_scope_by_id(
        &self,
        fragment_id: svelte_ast::FragmentId,
        parent_scope: ScopeId,
    ) -> ScopeId {
        self.data
            .effective_fragment_scope(fragment_id, parent_scope)
    }

    pub fn pop_ignore(&mut self) {
        if let Some((prev, prev_idx)) = self.ignore_stack.pop() {
            self.ignore_current = prev;
            self.ignore_current_idx = prev_idx;
        }
    }

    pub fn record_ignore_for_node(&mut self, node_id: NodeId) {
        if self.ignore_current.is_empty() {
            return;
        }
        let idx = match self.ignore_current_idx {
            Some(idx) => idx,
            None => {
                let idx = self
                    .data
                    .output
                    .ignore_data
                    .intern_snapshot(&self.ignore_current);
                self.ignore_current_idx = Some(idx);
                idx
            }
        };
        self.data.output.ignore_data.set_snapshot(node_id, idx);
    }

    pub fn take_warnings(&mut self) -> Vec<Diagnostic> {
        mem::take(&mut self.warnings)
    }

    pub(crate) fn push_warning_if_not_ignored(
        &mut self,
        node_id: NodeId,
        kind: DiagnosticKind,
        span: Span,
    ) {
        if self
            .data
            .output
            .ignore_data
            .is_ignored(node_id, kind.code())
        {
            return;
        }

        self.warnings.push(Diagnostic::warning(kind, span));
    }

    pub(crate) fn push(&mut self, r: ParentRef) {
        self.parents.push(r);
    }

    pub(crate) fn pop(&mut self) {
        self.parents.pop();
    }

    pub(crate) fn replace_element_name(&mut self, id: NodeId) -> Option<NodeId> {
        self.element_name_id.replace(id)
    }

    pub(crate) fn set_element_name(&mut self, id: Option<NodeId>) {
        self.element_name_id = id;
    }

    pub(crate) fn warnings_mut(&mut self) -> &mut Vec<Diagnostic> {
        &mut self.warnings
    }
}
