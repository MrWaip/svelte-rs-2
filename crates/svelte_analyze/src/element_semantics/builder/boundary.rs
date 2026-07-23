use svelte_ast::{Component, Node, SvelteBoundary};

use super::super::{BoundaryBranch, BoundarySemantics};
use crate::AnalysisData;
use crate::types::data::JsAst;
use crate::utils::snippet::snippet_name_symbol;

pub(super) fn classify(
    component: &Component,
    parsed: &JsAst<'_>,
    data: &AnalysisData<'_>,
    boundary: &SvelteBoundary,
) -> BoundarySemantics {
    let mut failed_snippet = BoundaryBranch::None;
    let mut pending_snippet = BoundaryBranch::None;
    let mut failed_attr = BoundaryBranch::None;
    let mut pending_attr = BoundaryBranch::None;

    for &nid in component.store.fragment_nodes(boundary.fragment) {
        let Node::SnippetBlock(block) = component.store.get(nid) else {
            continue;
        };
        let name =
            snippet_name_symbol(parsed, block).map(|sym| data.scoping.semantics().symbol_name(sym));
        match name {
            Some("failed") => failed_snippet = BoundaryBranch::Snippet(nid),
            Some("pending") => pending_snippet = BoundaryBranch::Snippet(nid),
            _ => {}
        }
    }

    for attr in &boundary.attributes {
        match attr.name() {
            Some("failed") => failed_attr = BoundaryBranch::Attribute(attr.id()),
            Some("pending") => pending_attr = BoundaryBranch::Attribute(attr.id()),
            _ => {}
        }
    }

    let failed = pick(failed_snippet, failed_attr);
    let pending = pick(pending_attr, pending_snippet);

    BoundarySemantics { failed, pending }
}

fn pick(primary: BoundaryBranch, fallback: BoundaryBranch) -> BoundaryBranch {
    match primary {
        BoundaryBranch::None => fallback,
        other => other,
    }
}
