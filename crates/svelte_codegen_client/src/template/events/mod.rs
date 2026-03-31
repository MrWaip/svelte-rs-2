//! Event handler, use:action, transition, animate, and legacy on:directive codegen.

mod actions;
mod emit;
mod handlers;

pub(crate) use actions::{
    gen_animate_directive, gen_attach_tag, gen_transition_directive, gen_use_directive,
    gen_use_directive_on,
};
pub(crate) use emit::{gen_event_attr_on, gen_legacy_event_on, gen_on_directive_legacy};
pub(crate) use handlers::{build_event_handler_s5, dev_event_handler};
