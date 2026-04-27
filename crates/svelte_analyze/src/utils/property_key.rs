//! Single source of truth for "extract a static name from an OXC `PropertyKey`".
//! Returns `None` for computed keys (`{ [expr]: b }`).

use oxc_ast::ast::PropertyKey;

/// Static name behind a non-computed `PropertyKey`.
///
/// Examples:
/// - `{ a: 1 }`        -> `Some("a")` (StaticIdentifier)
/// - `{ "a": 1 }`      -> `Some("a")` (StringLiteral)
/// - `{ [expr]: 1 }`   -> `None`      (computed)
pub fn property_key_static_name<'a>(key: &'a PropertyKey<'a>) -> Option<&'a str> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str()),
        PropertyKey::StringLiteral(s) => Some(s.value.as_str()),
        _ => None,
    }
}
