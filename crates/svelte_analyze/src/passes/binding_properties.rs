#[derive(Clone, Copy)]
pub(crate) struct BindingProperty {
    pub(crate) valid_elements: &'static [&'static str],
    pub(crate) invalid_elements: &'static [&'static str],
}

impl BindingProperty {
    pub(crate) fn allows(self, element: &str) -> bool {
        if !self.valid_elements.is_empty() {
            self.valid_elements.contains(&element)
        } else {
            !self.invalid_elements.contains(&element)
        }
    }
}

pub(crate) const BINDING_NAMES: &[&str] = &[
    "currentTime",
    "duration",
    "focused",
    "paused",
    "buffered",
    "seekable",
    "played",
    "volume",
    "muted",
    "playbackRate",
    "seeking",
    "ended",
    "readyState",
    "videoHeight",
    "videoWidth",
    "naturalWidth",
    "naturalHeight",
    "activeElement",
    "fullscreenElement",
    "pointerLockElement",
    "visibilityState",
    "innerWidth",
    "innerHeight",
    "outerWidth",
    "outerHeight",
    "scrollX",
    "scrollY",
    "online",
    "devicePixelRatio",
    "clientWidth",
    "clientHeight",
    "offsetWidth",
    "offsetHeight",
    "contentRect",
    "contentBoxSize",
    "borderBoxSize",
    "devicePixelContentBoxSize",
    "indeterminate",
    "checked",
    "group",
    "this",
    "innerText",
    "innerHTML",
    "textContent",
    "open",
    "value",
    "files",
];

pub(crate) fn binding_property(name: &str) -> Option<BindingProperty> {
    match name {
        "currentTime" | "duration" | "paused" | "buffered" | "seekable" | "played" | "volume"
        | "muted" | "playbackRate" | "seeking" | "ended" | "readyState" => Some(BindingProperty {
            valid_elements: &["audio", "video"],
            invalid_elements: &[],
        }),
        "focused" | "this" => Some(BindingProperty {
            valid_elements: &[],
            invalid_elements: &[],
        }),
        "videoHeight" | "videoWidth" => Some(BindingProperty {
            valid_elements: &["video"],
            invalid_elements: &[],
        }),
        "naturalWidth" | "naturalHeight" => Some(BindingProperty {
            valid_elements: &["img"],
            invalid_elements: &[],
        }),
        "activeElement" | "fullscreenElement" | "pointerLockElement" | "visibilityState" => {
            Some(BindingProperty {
                valid_elements: &["svelte:document"],
                invalid_elements: &[],
            })
        }
        "innerWidth" | "innerHeight" | "outerWidth" | "outerHeight" | "scrollX" | "scrollY"
        | "online" | "devicePixelRatio" => Some(BindingProperty {
            valid_elements: &["svelte:window"],
            invalid_elements: &[],
        }),
        "clientWidth"
        | "clientHeight"
        | "offsetWidth"
        | "offsetHeight"
        | "contentRect"
        | "contentBoxSize"
        | "borderBoxSize"
        | "devicePixelContentBoxSize"
        | "innerText"
        | "innerHTML"
        | "textContent" => Some(BindingProperty {
            valid_elements: &[],
            invalid_elements: &["svelte:window", "svelte:document"],
        }),
        "indeterminate" | "checked" | "group" | "files" => Some(BindingProperty {
            valid_elements: &["input"],
            invalid_elements: &[],
        }),
        "open" => Some(BindingProperty {
            valid_elements: &["details"],
            invalid_elements: &[],
        }),
        "value" => Some(BindingProperty {
            valid_elements: &["input", "textarea", "select"],
            invalid_elements: &[],
        }),
        _ => None,
    }
}
