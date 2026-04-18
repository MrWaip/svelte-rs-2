use svelte_ast::{Attribute, Element, SlotElementLegacy, SvelteFragmentLegacy};

use crate::types::data::{
    AnalysisData, ContentStrategy, FragmentItem, FragmentKey, LoweredTextPart,
};
use crate::walker::TemplateVisitor;

/// Visitor that computes content_type + has_dynamic_children + needs_var
/// for element fragments during the main composite walk.
///
/// Must be LAST in the composite tuple: reads `dynamic_nodes` and `needs_ref`
/// populated by ReactivityVisitor earlier in the same walk.
pub(crate) struct ContentAndVarVisitor<'s> {
    pub source: &'s str,
}

impl TemplateVisitor for ContentAndVarVisitor<'_> {
    fn leave_element(&mut self, el: &Element, ctx: &mut crate::walker::VisitContext<'_, '_>) {
        let key = FragmentKey::Element(el.id);

        // Compute content_type + has_dynamic for this element's fragment
        if let Some(lf) = ctx.data.template.fragments.lowered.get(&key) {
            let cs = classify_items(&lf.items, self.source);
            let has_dynamic = lf
                .items
                .iter()
                .any(|item| item_is_dynamic(item, ctx.data, &ctx.data.dynamism));
            ctx.data.template.fragments.content_types.insert(key, cs);
            if has_dynamic {
                ctx.data.template.fragments.has_dynamic_children.insert(key);
            }
        }

        // Compute needs_var (same logic as former NeedsVarVisitor)
        if element_needs_var(el, ctx.data) {
            ctx.data.elements.flags.needs_var.insert(el.id);
        }
    }

    fn leave_slot_element_legacy(
        &mut self,
        el: &SlotElementLegacy,
        ctx: &mut crate::walker::VisitContext<'_, '_>,
    ) {
        let key = FragmentKey::Element(el.id);
        if let Some(lf) = ctx.data.template.fragments.lowered.get(&key) {
            let cs = classify_items(&lf.items, self.source);
            let has_dynamic = lf
                .items
                .iter()
                .any(|item| item_is_dynamic(item, ctx.data, &ctx.data.dynamism));
            ctx.data.template.fragments.content_types.insert(key, cs);
            if has_dynamic {
                ctx.data.template.fragments.has_dynamic_children.insert(key);
            }
        }
    }

    fn leave_svelte_fragment_legacy(
        &mut self,
        el: &SvelteFragmentLegacy,
        ctx: &mut crate::walker::VisitContext<'_, '_>,
    ) {
        let key = FragmentKey::Element(el.id);
        if let Some(lf) = ctx.data.template.fragments.lowered.get(&key) {
            let cs = classify_items(&lf.items, self.source);
            let has_dynamic = lf
                .items
                .iter()
                .any(|item| item_is_dynamic(item, ctx.data, &ctx.data.dynamism));
            ctx.data.template.fragments.content_types.insert(key, cs);
            if has_dynamic {
                ctx.data.template.fragments.has_dynamic_children.insert(key);
            }
        }
    }
}

/// Classify non-element fragments after the composite walk completes.
/// Element fragments are already classified by ContentAndVarVisitor::leave_element.
pub fn classify_remaining_fragments(data: &mut AnalysisData, source: &str) {
    let keys: Vec<_> = data
        .template
        .fragments
        .lowered
        .keys()
        .filter(|k| !matches!(k, FragmentKey::Element(_)))
        .copied()
        .collect();
    for key in keys {
        let lf = &data.template.fragments.lowered[&key];
        let cs = classify_items(&lf.items, source);
        let has_dynamic = lf
            .items
            .iter()
            .any(|item| item_is_dynamic(item, data, &data.dynamism));
        data.template.fragments.content_types.insert(key, cs);
        if has_dynamic {
            data.template.fragments.has_dynamic_children.insert(key);
        }
    }
}

fn element_needs_var(el: &Element, data: &AnalysisData) -> bool {
    let id = el.id;

    if data.elements.flags.needs_ref.contains(&id) {
        return true;
    }

    // Customizable select elements require a JS var so codegen can call $.customizable_select(el, ...).
    if data.elements.flags.is_customizable_select(id) {
        return true;
    }

    // <selectedcontent> requires a JS var so codegen can call $.selectedcontent(el, setter).
    if data.elements.flags.is_selectedcontent(id) {
        return true;
    }

    if data.has_runtime_attrs(id) {
        return true;
    }

    // `<option value="...">` always emits a JS-side `el.value = el.__value = "<lit>"`
    // initializer (matches reference compiler `needs_special_value_handling`), so the
    // option element needs a DOM ref even if nothing else is dynamic.
    if el.name == "option"
        && el
            .attributes
            .iter()
            .any(|a| matches!(a, Attribute::StringAttribute(s) if s.name == "value"))
    {
        return true;
    }

    let key = FragmentKey::Element(id);
    let ct = data
        .template
        .fragments
        .content_types
        .get(&key)
        .cloned()
        .unwrap_or(ContentStrategy::Empty);
    match ct {
        ContentStrategy::Empty | ContentStrategy::Static(_) => false,
        ContentStrategy::DynamicText => true,
        ContentStrategy::SingleBlock(_) => true,
        ContentStrategy::SingleElement(_) | ContentStrategy::Mixed { .. } => {
            let Some(lf) = data.template.fragments.lowered.get(&key) else {
                return false;
            };
            lf.items.iter().any(|item| item_needs_var(item, data))
        }
    }
}

fn item_needs_var(item: &FragmentItem, data: &AnalysisData) -> bool {
    match item {
        FragmentItem::TextConcat { has_expr, .. } => *has_expr,
        FragmentItem::Element(id) => {
            // Bottom-up: child elements' needs_var already computed via leave_element
            data.elements.flags.needs_var.contains(id)
        }
        FragmentItem::SlotElementLegacy(_) => true,
        FragmentItem::SvelteFragmentLegacy(id) => data
            .template
            .fragments
            .lowered(&FragmentKey::Element(*id))
            .is_some_and(|fragment| fragment.items.iter().any(|item| item_needs_var(item, data))),
        FragmentItem::ComponentNode(_)
        | FragmentItem::IfBlock(_)
        | FragmentItem::EachBlock(_)
        | FragmentItem::RenderTag(_)
        | FragmentItem::HtmlTag(_)
        | FragmentItem::KeyBlock(_)
        | FragmentItem::SvelteElement(_)
        | FragmentItem::SvelteBoundary(_)
        | FragmentItem::AwaitBlock(_) => true,
    }
}

fn item_is_dynamic(
    item: &FragmentItem,
    data: &AnalysisData,
    dynamism: &crate::passes::dynamism::DynamismData,
) -> bool {
    match item {
        FragmentItem::TextConcat { parts, .. } => parts
            .iter()
            .any(|p| matches!(p, LoweredTextPart::Expr(id) if dynamism.is_dynamic_node(*id))),
        FragmentItem::Element(id)
        | FragmentItem::SlotElementLegacy(id)
        | FragmentItem::ComponentNode(id)
        | FragmentItem::IfBlock(id)
        | FragmentItem::EachBlock(id)
        | FragmentItem::RenderTag(id)
        | FragmentItem::HtmlTag(id)
        | FragmentItem::KeyBlock(id)
        | FragmentItem::SvelteElement(id)
        | FragmentItem::SvelteBoundary(id)
        | FragmentItem::AwaitBlock(id) => dynamism.is_dynamic_node(*id),
        FragmentItem::SvelteFragmentLegacy(id) => {
            dynamism.is_dynamic_node(*id)
                || data
                    .template
                    .fragments
                    .lowered(&FragmentKey::Element(*id))
                    .is_some_and(|fragment| {
                        fragment
                            .items
                            .iter()
                            .any(|item| item_is_dynamic(item, data, dynamism))
                    })
        }
    }
}

fn classify_items(items: &[FragmentItem], source: &str) -> ContentStrategy {
    if items.is_empty() {
        return ContentStrategy::Empty;
    }

    if items.len() == 1 {
        match &items[0] {
            FragmentItem::Element(id)
            | FragmentItem::SlotElementLegacy(id)
            | FragmentItem::SvelteFragmentLegacy(id) => {
                return ContentStrategy::SingleElement(*id);
            }
            FragmentItem::TextConcat { .. } => {}
            item => return ContentStrategy::SingleBlock(item.clone()),
        }
    }

    let mut has_element = false;
    let mut has_block = false;
    let mut has_static_text = false;
    let mut has_dynamic_text = false;

    for item in items {
        match item {
            FragmentItem::Element(_)
            | FragmentItem::SlotElementLegacy(_)
            | FragmentItem::SvelteFragmentLegacy(_) => has_element = true,
            FragmentItem::ComponentNode(_)
            | FragmentItem::IfBlock(_)
            | FragmentItem::EachBlock(_)
            | FragmentItem::RenderTag(_)
            | FragmentItem::HtmlTag(_)
            | FragmentItem::KeyBlock(_)
            | FragmentItem::SvelteElement(_)
            | FragmentItem::SvelteBoundary(_)
            | FragmentItem::AwaitBlock(_) => has_block = true,
            FragmentItem::TextConcat { has_expr, .. } => {
                if *has_expr {
                    has_dynamic_text = true;
                } else {
                    has_static_text = true;
                }
            }
        }
    }

    if has_element || has_block {
        let has_text = has_static_text || has_dynamic_text;
        return ContentStrategy::Mixed {
            has_elements: has_element,
            has_blocks: has_block,
            has_text,
        };
    }

    if has_dynamic_text {
        ContentStrategy::DynamicText
    } else if has_static_text {
        let text = if let FragmentItem::TextConcat { parts, .. } = &items[0] {
            parts
                .iter()
                .map(|p| match p {
                    LoweredTextPart::TextSpan(span) => span.source_text(&source),
                    LoweredTextPart::TextOwned(t) => t.as_str(),
                    LoweredTextPart::Expr(_) => "",
                })
                .collect::<String>()
        } else {
            String::new()
        };
        ContentStrategy::Static(text)
    } else {
        ContentStrategy::Empty
    }
}
