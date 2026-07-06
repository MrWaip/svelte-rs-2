use std::borrow::Cow;

use memchr::{memchr2, memchr3};

pub(crate) fn escape_text(value: &str) -> Cow<'_, str> {
    escape(value, false)
}

pub(crate) fn escape_attribute(value: &str) -> Cow<'_, str> {
    escape(value, true)
}

fn escape(value: &str, is_attr: bool) -> Cow<'_, str> {
    let bytes = value.as_bytes();
    let find = |haystack: &[u8]| {
        if is_attr {
            memchr3(b'&', b'<', b'"', haystack)
        } else {
            memchr2(b'&', b'<', haystack)
        }
    };
    let Some(first) = find(bytes) else {
        return Cow::Borrowed(value);
    };

    let mut out = String::with_capacity(value.len() + 8);
    out.push_str(&value[..first]);
    let mut i = first;
    while i < bytes.len() {
        let Some(rel) = find(&bytes[i..]) else {
            out.push_str(&value[i..]);
            break;
        };
        let pos = i + rel;
        out.push_str(&value[i..pos]);
        let replacement = match bytes[pos] {
            b'&' => "&amp;",
            b'<' => "&lt;",
            _ => "&quot;",
        };
        out.push_str(replacement);
        i = pos + 1;
    }
    Cow::Owned(out)
}
