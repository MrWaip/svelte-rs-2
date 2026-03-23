//! HTML template string builders.

use std::fmt::Write;

use svelte_analyze::{LoweredTextPart, ContentStrategy, FragmentItem, FragmentKey};
use svelte_ast::{is_void, Attribute, Element};

use crate::context::Ctx;

/// Build the HTML string for a fragment (used in `$.template(...)`).
/// Returns `(html, needs_import_node)` — the flag is true when the fragment
/// contains a `<video>` element (which requires `importNode` instead of `cloneNode`).
pub(crate) fn fragment_html(ctx: &Ctx<'_>, key: FragmentKey) -> (String, bool) {
    let Some(lf) = ctx.analysis.fragments.lowered(&key) else {
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
                        if let Some(t) = part.text_value(&ctx.component.source) {
                            html.push_str(t);
                        }
                    }
                }
            }
            FragmentItem::Element(id) => {
                let el = ctx.element(*id);
                let (el_html, el_import) = element_html(ctx, el);
                html.push_str(&el_html);
                import_node |= el_import;
            }
            FragmentItem::ComponentNode(_) | FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) | FragmentItem::RenderTag(_) | FragmentItem::HtmlTag(_) | FragmentItem::KeyBlock(_) | FragmentItem::SvelteElement(_) | FragmentItem::SvelteBoundary(_) | FragmentItem::AwaitBlock(_) => html.push_str("<!>"),
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

    let mut html = String::new();
    let mut import_node = el.name == "video";
    write!(html, "<{}", el.name).unwrap();

    // When spread attrs are present, skip all attributes from HTML template
    if !has_spread {
        for attr in &el.attributes {
            match attr {
                Attribute::StringAttribute(a) => {
                    // Skip class attr when class directives or class expression present — set_class handles it
                    if a.name == "class" && (has_class_directives || has_class_attribute) {
                        continue;
                    }
                    // bind:group uses __value pattern — value attr is set in JS, not template
                    if a.name == "value" && ctx.has_bind_group(el.id) {
                        continue;
                    }
                    write!(
                        html,
                        " {}=\"{}\"",
                        a.name,
                        ctx.component.source_text(a.value_span)
                    )
                    .unwrap();
                }
                Attribute::BooleanAttribute(a) => {
                    write!(html, " {}=\"\"", a.name).unwrap();
                }
                _ => {}
            }
        }
    }

    if is_void(&el.name) {
        html.push_str("/>");
        return (html, import_node);
    }
    html.push('>');

    // noscript content is stripped in the template
    if el.name == "noscript" {
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
        }
        ContentStrategy::DynamicText => {
            // space placeholder for the text node
            html.push(' ');
        }
        ContentStrategy::SingleBlock(FragmentItem::EachBlock(_))
        | ContentStrategy::SingleBlock(FragmentItem::HtmlTag(_)) => {
            // Controlled block: element itself is the anchor, no <!> needed
        }
        ContentStrategy::SingleElement(_) | ContentStrategy::SingleBlock(_) | ContentStrategy::Mixed { .. } => {
            let (child_html, child_import) = fragment_html(ctx, child_key);
            html.push_str(&child_html);
            import_node |= child_import;
        }
    }

    write!(html, "</{}>", el.name).unwrap();
    (html, import_node)
}
