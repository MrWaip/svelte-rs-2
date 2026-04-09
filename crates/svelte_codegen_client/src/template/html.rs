//! HTML template string builders.

use std::fmt::Write;

use svelte_analyze::{ContentStrategy, FragmentItem, FragmentKey, NamespaceKind};
use svelte_ast::{Attribute, Element};

use super::expression::item_has_local_blockers;
use super::slot::is_legacy_slot_element;
use crate::context::Ctx;

/// Build the HTML string for a fragment (used in `$.template(...)`).
/// Returns `(html, needs_import_node)` — the flag is true when the fragment
/// contains a `<video>` element (which requires `importNode` instead of `cloneNode`).
pub(crate) fn fragment_html(ctx: &Ctx<'_>, key: FragmentKey) -> (String, bool) {
    let Some(lf) = ctx.query.maybe_lowered_fragment(&key) else {
        return (String::new(), false);
    };
    let mut html = String::new();
    let mut import_node = false;
    for item in &lf.items {
        match item {
            FragmentItem::TextConcat { parts, has_expr } => {
                let has_expr = *has_expr;
                if has_expr {
                    html.push(' ');
                } else {
                    for part in parts {
                        if let Some(t) = part.text_value(&ctx.query.component.source) {
                            html.push_str(t);
                        }
                    }
                }
            }
            FragmentItem::Element(id) => {
                if is_legacy_slot_element(ctx, *id) {
                    html.push_str("<!>");
                } else {
                    let el = ctx.element(*id);
                    let (el_html, el_import) = element_html(ctx, el);
                    html.push_str(&el_html);
                    import_node |= el_import;
                }
            }
            FragmentItem::ComponentNode(_)
            | FragmentItem::IfBlock(_)
            | FragmentItem::EachBlock(_)
            | FragmentItem::RenderTag(_)
            | FragmentItem::HtmlTag(_)
            | FragmentItem::KeyBlock(_)
            | FragmentItem::SvelteElement(_)
            | FragmentItem::SvelteBoundary(_)
            | FragmentItem::AwaitBlock(_) => html.push_str("<!>"),
        }
    }
    (html, import_node)
}

/// Build the HTML string for a single element (opening tag + attrs + children + closing tag).
/// Returns `(html, needs_import_node)` — true when this element or any descendant is `<video>`.
pub(crate) fn element_html(ctx: &Ctx<'_>, el: &Element) -> (String, bool) {
    let has_spread = ctx.has_spread(el.id);
    let has_class_directives = ctx.has_class_directives(el.id);
    let has_class_attribute = ctx.has_class_attribute(el.id);
    let is_scoped = ctx.is_css_scoped(el.id);
    let css_hash = ctx.css_hash();

    let mut html = String::new();
    let has_is_attr = el
        .attributes
        .iter()
        .any(|a| matches!(a, Attribute::StringAttribute(sa) if sa.name == "is"));
    let mut import_node =
        el.name == "video" || ctx.query.view.is_custom_element(el.id) || has_is_attr;
    // HTML attribute names are case-insensitive; normalize to lowercase for HTML-namespace
    // elements. SVG and MathML attributes are case-sensitive — preserve their case.
    let lowercase_attrs = !matches!(
        ctx.query.view.namespace(el.id),
        Some(NamespaceKind::Svg) | Some(NamespaceKind::MathMl) | Some(NamespaceKind::AnnotationXml)
    );
    write!(html, "<{}", el.name).unwrap();

    // Track whether we emitted a class attribute so we know to add one later.
    let mut wrote_class_attr = false;

    // When spread attrs are present, skip all attributes from HTML template
    if !has_spread {
        for attr in &el.attributes {
            match attr {
                Attribute::StringAttribute(a) => {
                    if a.name == "class" {
                        // Skip class attr when class directives or class expression present —
                        // set_class handles it. Scoped-hash injection for that path is a
                        // later slice; dynamic-class + scoped is excluded for now.
                        if has_class_directives || has_class_attribute {
                            continue;
                        }
                        // Static class: bake scoped hash directly into the template.
                        let val = ctx.query.component.source_text(a.value_span);
                        if is_scoped {
                            write!(html, " class=\"{val} {css_hash}\"").unwrap();
                        } else {
                            write!(html, " class=\"{val}\"").unwrap();
                        }
                        wrote_class_attr = true;
                        continue;
                    }
                    // `<option value=...>` and `<input bind:group ... value=...>` both
                    // emit the value via the `__value` cache pattern in JS, so the literal
                    // attr is dropped from the template HTML.
                    if a.name == "value" && (ctx.has_bind_group(el.id) || el.name == "option") {
                        continue;
                    }
                    let val = ctx.query.component.source_text(a.value_span);
                    if lowercase_attrs {
                        write!(html, " {}=\"{val}\"", a.name.to_lowercase()).unwrap();
                    } else {
                        write!(html, " {}=\"{val}\"", a.name).unwrap();
                    }
                }
                Attribute::BooleanAttribute(a) => {
                    if lowercase_attrs {
                        write!(html, " {}=\"\"", a.name.to_lowercase()).unwrap();
                    } else {
                        write!(html, " {}=\"\"", a.name).unwrap();
                    }
                }
                _ => {}
            }
        }
        // Inject scope class only when set_class won't handle it.
        // When class directives or a dynamic class attr are present, set_class
        // carries the scope hash — don't also bake it into the template HTML.
        if is_scoped && !wrote_class_attr && !has_class_directives && !has_class_attribute {
            write!(html, " class=\"{css_hash}\"").unwrap();
        }
    }

    if ctx.query.view.is_void(el.id) {
        html.push_str("/>");
        return (html, import_node);
    }
    html.push('>');

    // noscript content is stripped in the template
    if el.name == "noscript" {
        write!(html, "</{}>", el.name).unwrap();
        return (html, import_node);
    }

    // Customizable select: children live in a separate hoisted template.
    // Only a hydration comment placeholder goes inside the element itself.
    if ctx.is_customizable_select(el.id) {
        html.push_str("<!>");
        write!(html, "</{}>", el.name).unwrap();
        return (html, import_node);
    }

    let child_key = FragmentKey::Element(el.id);
    let ct = ctx.content_type(&child_key);
    let has_state = ctx.has_dynamic_children(&child_key);

    match ct {
        ContentStrategy::Empty => {}
        ContentStrategy::Static(ref text) => {
            html.push_str(text);
        }
        ContentStrategy::DynamicText if !has_state => {
            // textContent shortcut — no placeholder
            let items = &ctx.lowered_fragment(&child_key).items;
            if item_has_local_blockers(&items[0], ctx) {
                html.push(' ');
            }
        }
        ContentStrategy::DynamicText => {
            // space placeholder for the text node
            html.push(' ');
        }
        ContentStrategy::SingleBlock(FragmentItem::EachBlock(_))
        | ContentStrategy::SingleBlock(FragmentItem::HtmlTag(_)) => {
            // Controlled block: element itself is the anchor, no <!> needed
        }
        ContentStrategy::SingleElement(_)
        | ContentStrategy::SingleBlock(_)
        | ContentStrategy::Mixed { .. } => {
            let (child_html, child_import) = fragment_html(ctx, child_key);
            html.push_str(&child_html);
            import_node |= child_import;
        }
    }

    write!(html, "</{}>", el.name).unwrap();
    (html, import_node)
}
