pub(crate) mod attributes;
pub(crate) mod events;
pub(crate) mod expression_await;
pub(crate) mod html_tree_validation;
pub(crate) mod ident_gen;
pub(crate) mod legacy_slot;
pub(crate) mod node_id_utils;
pub(crate) mod property_key;
pub(crate) mod simple_expression;
pub(crate) mod snippet;
pub(crate) mod var_decl_kind;

pub use attributes::{
    collapse_attribute_whitespace, concat_single_dynamic_expr, emit_html_attribute_name,
    event_attribute, is_dom_boolean_attribute, is_regular_dom_property,
    normalize_regular_attribute_name,
};
pub use events::is_simple_identifier;
pub(crate) use events::{is_delegatable_event, is_passive_event, strip_capture_event};
pub use expression_await::expression_calls_or_awaits;
pub(crate) use expression_await::{expression_has_await, statement_has_await};
pub use ident_gen::{IdentGen, IdentGenSnapshot};
pub use property_key::property_key_static_name;
pub use simple_expression::is_simple_expression;
pub use var_decl_kind::is_let_or_var;
