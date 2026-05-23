use std::borrow::Cow;

use oxc_ast::ast::PropertyKey;

pub fn property_key_static_name<'a>(key: &'a PropertyKey<'a>) -> Option<Cow<'a, str>> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(Cow::Borrowed(id.name.as_str())),
        PropertyKey::StringLiteral(s) => Some(Cow::Borrowed(s.value.as_str())),
        PropertyKey::NumericLiteral(n) => Some(Cow::Owned(n.value.to_string())),
        _ => None,
    }
}
