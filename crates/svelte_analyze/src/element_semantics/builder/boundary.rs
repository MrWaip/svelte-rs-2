use svelte_ast::{Component, Node, SvelteBoundary};

use super::super::{BoundaryBranch, BoundarySemantics};
use crate::AnalysisData;
use crate::types::data::JsAst;
use crate::utils::snippet::snippet_name_symbol;
use crate::value_evaluation::{Evaluation, KnownValue};

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
    let pending_needs_nullish_guard = match (pending_attr, pending_snippet) {
        (BoundaryBranch::Attribute(attr_id), BoundaryBranch::None) => {
            !attribute_provably_defined(data, attr_id)
        }
        _ => false,
    };

    BoundarySemantics {
        failed,
        pending,
        pending_needs_nullish_guard,
        failed_snippet: snippet_node(failed_snippet),
        pending_snippet: snippet_node(pending_snippet),
    }
}

fn snippet_node(branch: BoundaryBranch) -> Option<svelte_ast::NodeId> {
    match branch {
        BoundaryBranch::Snippet(id) => Some(id),
        BoundaryBranch::None | BoundaryBranch::Attribute(_) => None,
    }
}

fn attribute_provably_defined(data: &AnalysisData<'_>, attr_id: svelte_ast::NodeId) -> bool {
    let Some(expr_data) = data.expression_data(attr_id) else {
        return false;
    };
    match &expr_data.declared_evaluation {
        Evaluation::MaybeNullish { .. } => return false,
        Evaluation::Known(value) => match value {
            KnownValue::Null | KnownValue::Undefined => return false,
            KnownValue::Bool(_)
            | KnownValue::Num(_)
            | KnownValue::Str(_)
            | KnownValue::Regex(_)
            | KnownValue::BigInt => {}
        },
        Evaluation::Defined { .. } => {}
    }
    for sym in expr_data.references.iter() {
        if data.template.snippets.snippet_by_symbol(*sym).is_some() {
            return false;
        }
    }
    true
}

fn pick(primary: BoundaryBranch, fallback: BoundaryBranch) -> BoundaryBranch {
    match primary {
        BoundaryBranch::None => fallback,
        other => other,
    }
}
