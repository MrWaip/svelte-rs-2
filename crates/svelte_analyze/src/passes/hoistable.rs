//! HoistableSnippetsVisitor — detect which top-level snippets can be hoisted.
//!
//! A snippet is hoistable if its body doesn't reference any script-declared variables.

use oxc_semantic::SymbolId;
use rustc_hash::FxHashSet;
use svelte_ast::NodeId;
use svelte_span::Span;

use crate::walker::{ParentKind, TemplateVisitor, VisitContext};

pub(crate) struct HoistableSnippetsVisitor {
    script_syms: FxHashSet<SymbolId>,
    top_level_ids: FxHashSet<NodeId>,
    tainted: FxHashSet<NodeId>,
}

impl HoistableSnippetsVisitor {
    pub(crate) fn new(
        script_syms: FxHashSet<SymbolId>,
        top_level_ids: FxHashSet<NodeId>,
    ) -> Self {
        Self {
            script_syms,
            top_level_ids,
            tainted: FxHashSet::default(),
        }
    }

    /// Write results: mark untainted top-level snippets as hoistable.
    pub(crate) fn finish(self, data: &mut crate::types::data::AnalysisData) {
        for id in &self.top_level_ids {
            if !self.tainted.contains(id) {
                data.snippets.hoistable.insert(*id);
            }
        }
    }
}

impl TemplateVisitor for HoistableSnippetsVisitor {
    fn visit_expression(&mut self, node_id: NodeId, _span: Span, ctx: &mut VisitContext<'_>) {
        let root = ctx
            .ancestors()
            .find(|p| p.kind == ParentKind::SnippetBlock && self.top_level_ids.contains(&p.id));
        let Some(root) = root else { return };
        let root_id = root.id;

        let info = if ctx.parent().is_some_and(|p| p.kind.is_attr()) {
            ctx.data.attr_expressions.get(node_id)
        } else {
            ctx.data.expressions.get(node_id)
        };
        if let Some(info) = info {
            if info
                .references
                .iter()
                .any(|r| r.symbol_id.is_some_and(|s| self.script_syms.contains(&s)))
            {
                self.tainted.insert(root_id);
            }
        }
    }
}
