use std::borrow::Cow;

use svelte_ast::{Attribute, ConcatPart, ConcatenationAttribute, ExprRef};

pub fn is_dom_boolean_attribute(name: &str) -> bool {
    matches!(
        name,
        "allowfullscreen"
            | "async"
            | "autofocus"
            | "autoplay"
            | "checked"
            | "controls"
            | "default"
            | "disabled"
            | "formnovalidate"
            | "indeterminate"
            | "inert"
            | "ismap"
            | "loop"
            | "multiple"
            | "muted"
            | "nomodule"
            | "novalidate"
            | "open"
            | "playsinline"
            | "readonly"
            | "required"
            | "reversed"
            | "seamless"
            | "selected"
            | "webkitdirectory"
            | "defer"
            | "disablepictureinpicture"
            | "disableremoteplayback"
    )
}

pub fn emit_html_attribute_name(name: &str, namespaced: bool) -> Cow<'_, str> {
    if namespaced || !name.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(name.to_ascii_lowercase())
    }
}

pub fn collapse_attribute_whitespace(value: &str) -> Cow<'_, str> {
    let mut prev_ws = false;
    let already_normalized = value.chars().all(|ch| {
        let ws = ch.is_whitespace();
        let ok = !ws || (ch == ' ' && !prev_ws);
        prev_ws = ws;
        ok
    });
    if already_normalized {
        return Cow::Borrowed(value);
    }

    let mut out = String::with_capacity(value.len());
    let mut in_ws = false;
    for ch in value.chars() {
        if ch.is_whitespace() {
            if !in_ws {
                out.push(' ');
                in_ws = true;
            }
        } else {
            out.push(ch);
            in_ws = false;
        }
    }
    Cow::Owned(out)
}

pub fn normalize_regular_attribute_name(name: &str, html_attr_namespace: bool) -> String {
    if !html_attr_namespace {
        return name.to_string();
    }

    match name.to_ascii_lowercase().as_str() {
        "formnovalidate" => "formNoValidate".to_string(),
        "ismap" => "isMap".to_string(),
        "nomodule" => "noModule".to_string(),
        "playsinline" => "playsInline".to_string(),
        "readonly" => "readOnly".to_string(),
        "defaultvalue" => "defaultValue".to_string(),
        "defaultchecked" => "defaultChecked".to_string(),
        "srcobject" => "srcObject".to_string(),
        "novalidate" => "noValidate".to_string(),
        "allowfullscreen" => "allowFullscreen".to_string(),
        "disablepictureinpicture" => "disablePictureInPicture".to_string(),
        "disableremoteplayback" => "disableRemotePlayback".to_string(),
        lower => lower.to_string(),
    }
}

pub fn concat_single_dynamic_expr(ca: &ConcatenationAttribute) -> Option<&ExprRef> {
    match ca.parts.as_slice() {
        [ConcatPart::Dynamic { expr, .. }] => Some(expr),
        _ => None,
    }
}

pub fn event_attribute(attr: &Attribute) -> Option<(&str, &ExprRef)> {
    let (name, expr) = match attr {
        Attribute::ExpressionAttribute(ea) => (ea.name.as_str(), &ea.expression),
        Attribute::ConcatenationAttribute(ca) => {
            (ca.name.as_str(), concat_single_dynamic_expr(ca)?)
        }
        _ => return None,
    };

    let event_name = name.strip_prefix("on")?;
    Some((event_name, expr))
}

pub fn is_regular_dom_property(name: &str) -> bool {
    matches!(
        name,
        "allowfullscreen"
            | "async"
            | "autofocus"
            | "autoplay"
            | "checked"
            | "controls"
            | "default"
            | "disabled"
            | "formnovalidate"
            | "indeterminate"
            | "inert"
            | "ismap"
            | "loop"
            | "multiple"
            | "muted"
            | "nomodule"
            | "novalidate"
            | "open"
            | "playsinline"
            | "readonly"
            | "required"
            | "reversed"
            | "seamless"
            | "selected"
            | "webkitdirectory"
            | "defer"
            | "disablepictureinpicture"
            | "disableremoteplayback"
            | "formNoValidate"
            | "isMap"
            | "noModule"
            | "playsInline"
            | "readOnly"
            | "value"
            | "volume"
            | "defaultValue"
            | "defaultChecked"
            | "srcObject"
            | "noValidate"
            | "allowFullscreen"
            | "disablePictureInPicture"
            | "disableRemotePlayback"
    )
}

#[cfg(test)]
mod tests {
    use super::{is_regular_dom_property, normalize_regular_attribute_name};

    #[test]
    fn normalizes_regular_html_attribute_aliases() {
        assert_eq!(
            normalize_regular_attribute_name("readonly", true),
            "readOnly"
        );
        assert_eq!(
            normalize_regular_attribute_name("disablepictureinpicture", true),
            "disablePictureInPicture"
        );
        assert_eq!(
            normalize_regular_attribute_name("readonly", false),
            "readonly"
        );
    }

    #[test]
    fn classifies_regular_dom_properties() {
        assert!(is_regular_dom_property("disabled"));
        assert!(is_regular_dom_property("readOnly"));
        assert!(is_regular_dom_property("defaultChecked"));
        assert!(!is_regular_dom_property("placeholder"));
    }
}
