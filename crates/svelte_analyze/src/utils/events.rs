//! Shared utilities for analyze and codegen (event classification, identifier checks).

/// Events that Svelte delegates to the document root.
pub fn is_delegatable_event(name: &str) -> bool {
    matches!(
        name,
        "beforeinput"
            | "click"
            | "change"
            | "dblclick"
            | "contextmenu"
            | "focusin"
            | "focusout"
            | "input"
            | "keydown"
            | "keyup"
            | "mousedown"
            | "mousemove"
            | "mouseout"
            | "mouseover"
            | "mouseup"
            | "pointerdown"
            | "pointermove"
            | "pointerout"
            | "pointerover"
            | "pointerup"
            | "touchend"
            | "touchmove"
            | "touchstart"
    )
}

pub fn is_capture_event(name: &str) -> bool {
    name.ends_with("capture") && name != "gotpointercapture" && name != "lostpointercapture"
}

pub fn strip_capture_event(name: &str) -> Option<&str> {
    if is_capture_event(name) {
        Some(&name[..name.len() - 7])
    } else {
        None
    }
}

pub fn is_passive_event(name: &str) -> bool {
    matches!(name, "touchstart" | "touchmove")
}

/// Check if a string is a simple JS identifier (no member access, no computed access).
pub fn is_simple_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .next()
            .is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$')
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}
