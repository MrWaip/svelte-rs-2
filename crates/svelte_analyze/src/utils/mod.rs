pub(crate) mod attributes;
pub(crate) mod binding_pattern;
pub(crate) mod ce_config;
pub(crate) mod events;
pub(crate) mod ident_gen;
pub(crate) mod legacy_slot;
pub(crate) mod script_info;
pub(crate) mod simple_expression;

pub use attributes::{is_regular_dom_property, normalize_regular_attribute_name};
pub use events::{
    is_capture_event, is_delegatable_event, is_passive_event, is_simple_identifier,
    strip_capture_event,
};
pub use ident_gen::IdentGen;
pub use simple_expression::is_simple_expression;
