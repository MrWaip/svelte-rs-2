pub(crate) mod builder_v2;
pub(crate) mod data;
pub mod legacy_reactive;
mod mode_resolution;

pub(crate) const SVELTE_STORE_MODULE: &str = "svelte/store";

pub(crate) use builder_v2::{
    ReactivityInputs, build_v2, detect_rune_from_call, finalize_proxy, finalize_reactivity,
};
