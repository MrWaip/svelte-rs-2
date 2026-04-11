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
