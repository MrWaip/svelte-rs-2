use smallvec::SmallVec;
use svelte_ast::{Attribute, NodeId};

use super::super::ElementAsyncKind;
use crate::attribute_semantics::{
    AttributeSemantics, AttributeSemanticsStore, ElementBindPropertyKind, ElementBindSemantics,
};
use crate::expression_semantics::{ExpressionData, ExpressionSemantics, ExpressionSemanticsStore};
use crate::types::data::BlockerData;

pub(super) struct HtmlAttributeSite<'e> {
    pub(super) attributes: &'e AttributeSemanticsStore,
    pub(super) element_name: &'e str,
    pub(super) siblings: &'e [Attribute],
    pub(super) source: &'e str,
    pub(super) value_nodes: SmallVec<[NodeId; 2]>,
}

pub(super) fn from_attributes(
    expressions: &ExpressionSemanticsStore,
    blockers: &BlockerData,
    site: &HtmlAttributeSite<'_>,
) -> ElementAsyncKind {
    collect_html_attributes(expressions, blockers, site, site.siblings.iter())
}

pub(super) fn from_svelte_element_attributes(
    expressions: &ExpressionSemanticsStore,
    blockers: &BlockerData,
    site: &HtmlAttributeSite<'_>,
) -> ElementAsyncKind {
    collect_html_attributes(
        expressions,
        blockers,
        site,
        site.siblings.iter().filter(|a| !a.is_svelte_element_this()),
    )
}

fn collect_html_attributes<'a>(
    expressions: &ExpressionSemanticsStore,
    blocker_data: &BlockerData,
    site: &HtmlAttributeSite<'_>,
    attributes: impl Iterator<Item = &'a Attribute>,
) -> ElementAsyncKind {
    let mut blockers: SmallVec<[u32; 2]> = SmallVec::new();
    let mut seen: SmallVec<[u32; 2]> = SmallVec::new();
    let mut awaited = false;
    for node in &site.value_nodes {
        let ExpressionSemantics::Expression(data) = expressions.get(*node) else {
            continue;
        };
        absorb_blockers(&mut blockers, &mut seen, blocker_data, data);
        if data.volatility.is_asynchronous() {
            awaited = true;
        }
    }
    for attr in attributes {
        if !renders_to_html(site, attr) {
            continue;
        }
        let ExpressionSemantics::Expression(data) = expressions.get(attr.id()) else {
            continue;
        };
        absorb_blockers(&mut blockers, &mut seen, blocker_data, data);
        if data.volatility.is_asynchronous() {
            awaited = true;
        }
    }
    make(blockers, awaited)
}

fn absorb_blockers(
    blockers: &mut SmallVec<[u32; 2]>,
    seen: &mut SmallVec<[u32; 2]>,
    blocker_data: &BlockerData,
    data: &ExpressionData,
) {
    for sym in &data.blocker_references {
        let Some(slot) = blocker_data.symbol_blocker(*sym) else {
            continue;
        };
        if seen.contains(&slot.member) {
            continue;
        }
        seen.push(slot.member);
        blockers.push(slot.entry);
    }
}

pub(super) fn from_component(
    expressions: &ExpressionSemanticsStore,
    blocker_data: &BlockerData,
    attributes: &[Attribute],
    name_id: Option<NodeId>,
) -> ElementAsyncKind {
    let mut blockers: SmallVec<[u32; 2]> = SmallVec::new();
    let mut seen: SmallVec<[u32; 2]> = SmallVec::new();
    let mut awaited = false;
    let mut absorb = |id: NodeId, contributes_value: bool| {
        let ExpressionSemantics::Expression(data) = expressions.get(id) else {
            return;
        };
        for sym in &data.blocker_references {
            let Some(slot) = blocker_data.symbol_blocker(*sym) else {
                continue;
            };
            if seen.contains(&slot.member) {
                continue;
            }
            seen.push(slot.member);
            blockers.push(slot.entry);
        }
        if contributes_value && data.volatility.is_asynchronous() {
            awaited = true;
        }
    };
    if let Some(id) = name_id {
        absorb(id, false);
    }
    for attr in attributes {
        match attr {
            Attribute::ExpressionAttribute(_)
            | Attribute::ConcatenationAttribute(_)
            | Attribute::SpreadAttribute(_) => absorb(attr.id(), true),
            Attribute::AttachTag(_) | Attribute::BindDirective(_) => absorb(attr.id(), false),
            _ => {}
        }
    }
    make(blockers, awaited)
}

pub(super) fn from_slot(
    expressions: &ExpressionSemanticsStore,
    blocker_data: &BlockerData,
    attributes: &[Attribute],
) -> ElementAsyncKind {
    from_component(expressions, blocker_data, attributes, None)
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

fn renders_to_html(site: &HtmlAttributeSite<'_>, attr: &Attribute) -> bool {
    match attr {
        Attribute::ExpressionAttribute(_)
        | Attribute::StringAttribute(_)
        | Attribute::ConcatenationAttribute(_)
        | Attribute::BooleanAttribute(_) => plain_renders_to_html(attr),
        Attribute::BindDirective(_) => match site.attributes.get(attr.id()) {
            AttributeSemantics::ElementBind(bind) => bind_renders_to_html(site, bind),
            _ => false,
        },
        Attribute::SpreadAttribute(_)
        | Attribute::ClassDirective(_)
        | Attribute::StyleDirective(_) => true,
        Attribute::LetDirectiveLegacy(_)
        | Attribute::UseDirective(_)
        | Attribute::OnDirectiveLegacy(_)
        | Attribute::TransitionDirective(_)
        | Attribute::AnimateDirective(_)
        | Attribute::AttachTag(_) => false,
    }
}

fn plain_renders_to_html(attr: &Attribute) -> bool {
    let Some(name) = attr.name() else {
        return true;
    };
    if svelte_ast::is_event_attribute_name(name) {
        return false;
    }
    if name == "defaultValue" || name == "defaultChecked" {
        return false;
    }
    true
}

fn bind_renders_to_html(site: &HtmlAttributeSite<'_>, bind: &ElementBindSemantics) -> bool {
    if !bind.property.reflects_in_html() {
        return false;
    }
    match bind.property {
        ElementBindPropertyKind::Value => site.element_name != "select" && !is_file_input(site),
        ElementBindPropertyKind::Checked
        | ElementBindPropertyKind::Group
        | ElementBindPropertyKind::Open
        | ElementBindPropertyKind::Focused
        | ElementBindPropertyKind::ContentEditable(_)
        | ElementBindPropertyKind::Files
        | ElementBindPropertyKind::Indeterminate
        | ElementBindPropertyKind::This
        | ElementBindPropertyKind::ElementSize(_)
        | ElementBindPropertyKind::ResizeObserver(_)
        | ElementBindPropertyKind::Media(_)
        | ElementBindPropertyKind::ImageNaturalSize(_) => true,
    }
}

fn is_file_input(site: &HtmlAttributeSite<'_>) -> bool {
    if site.element_name != "input" {
        return false;
    }
    for attr in site.siblings {
        let Attribute::StringAttribute(a) = attr else {
            continue;
        };
        if a.name == "type" && a.value(site.source) == "file" {
            return true;
        }
    }
    false
}
