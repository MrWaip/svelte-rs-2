use smallvec::SmallVec;
use svelte_ast::{Attribute, NodeId};

use super::super::ElementAsyncKind;
use crate::expression_semantics::{ExpressionSemantics, ExpressionSemanticsStore};

pub(super) fn from_attributes(
    expressions: &ExpressionSemanticsStore,
    attributes: &[Attribute],
) -> ElementAsyncKind {
    let mut blockers: SmallVec<[u32; 2]> = SmallVec::new();
    let mut awaited = false;
    for attr in attributes {
        if !renders_to_html(attr) {
            continue;
        }
        let ExpressionSemantics::Expression(data) = expressions.get(attr.id()) else {
            continue;
        };
        for &blocker in &data.blockers {
            if !blockers.contains(&blocker) {
                blockers.push(blocker);
            }
        }
        if data.volatility.is_asynchronous() {
            awaited = true;
        }
    }
    make(blockers, awaited)
}

pub(super) fn from_tag(expressions: &ExpressionSemanticsStore, id: NodeId) -> ElementAsyncKind {
    let ExpressionSemantics::Expression(data) = expressions.get(id) else {
        return ElementAsyncKind::Sync;
    };
    make(data.blockers.clone(), data.volatility.is_asynchronous())
}

fn make(blockers: SmallVec<[u32; 2]>, awaited: bool) -> ElementAsyncKind {
    if awaited {
        ElementAsyncKind::Awaited { blockers }
    } else if !blockers.is_empty() {
        ElementAsyncKind::Deferred { blockers }
    } else {
        ElementAsyncKind::Sync
    }
}

fn renders_to_html(attr: &Attribute) -> bool {
    matches!(
        attr,
        Attribute::ExpressionAttribute(_)
            | Attribute::StringAttribute(_)
            | Attribute::ConcatenationAttribute(_)
            | Attribute::BooleanAttribute(_)
            | Attribute::SpreadAttribute(_)
            | Attribute::BindDirective(_)
            | Attribute::ClassDirective(_)
            | Attribute::StyleDirective(_)
    )
}
