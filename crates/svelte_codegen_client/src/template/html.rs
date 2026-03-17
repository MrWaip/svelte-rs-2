//! HTML template string builders.

use std::fmt::Write;

use svelte_analyze::{ConcatPart, ContentType, FragmentItem, FragmentKey};
use svelte_ast::{is_void, Attribute, Element};

use crate::context::Ctx;

use super::expression::static_text_of;

/// Build the HTML string for a fragment (used in `$.template(...)`).
pub(crate) fn fragment_html(ctx: &Ctx<'_>, key: FragmentKey) -> String {
    let Some(lf) = ctx.analysis.fragments.lowered(&key) else {
        return String::new();
    };
    let mut html = String::new();
    for item in &lf.items {
        match item {
            FragmentItem::TextConcat { parts, has_expr } => {
                let has_expr = *has_expr;
                if has_expr {
                    html.push(' ');
                } else {
                    for part in parts {
                        if let ConcatPart::Text(t) = part {
                            html.push_str(t);
                        }
                    }
                }
            }
            FragmentItem::Element(id) => {
                let el = ctx.element(*id);
                html.push_str(&element_html(ctx, el));
            }
            FragmentItem::ComponentNode(_) | FragmentItem::IfBlock(_) | FragmentItem::EachBlock(_) | FragmentItem::RenderTag(_) | FragmentItem::HtmlTag(_) | FragmentItem::KeyBlock(_) | FragmentItem::SvelteElement(_) => html.push_str("<!>"),
        }
    }
    html
}

/// Build the HTML string for a single element (opening tag + attrs + children + closing tag).
pub(crate) fn element_html(ctx: &Ctx<'_>, el: &Element) -> String {
    let has_spread = ctx.has_spread(el.id);
    let has_class_directives = ctx.has_class_directives(el.id);
    let has_class_attribute = ctx.has_class_attribute(el.id);

    let mut html = String::new();
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
        return html;
    }
    html.push('>');

    // noscript content is stripped in the template
    if el.name == "noscript" {
        write!(html, "</{}>", el.name).unwrap();
        return html;
    }

    let child_key = FragmentKey::Element(el.id);
    let ct = ctx.content_type(&child_key);
    let has_state = ctx.has_dynamic_children(&child_key);

    match ct {
        ContentType::Empty => {}
        ContentType::StaticText => {
            let lf = ctx.lowered_fragment(&child_key);
            html.push_str(&static_text_of(&lf.items[0]));
        }
        ContentType::DynamicText if !has_state => {
            // textContent shortcut — no placeholder
        }
        ContentType::DynamicText => {
            // space placeholder for the text node
            html.push(' ');
        }
        ContentType::SingleBlock if matches!(
            ctx.lowered_fragment(&child_key).items.first(),
            Some(FragmentItem::EachBlock(_))
        ) => {
            // Controlled each block: no <!> anchor in template
        }
        ContentType::SingleElement | ContentType::SingleBlock | ContentType::Mixed => {
            html.push_str(&fragment_html(ctx, child_key));
        }
    }

    write!(html, "</{}>", el.name).unwrap();
    html
}

