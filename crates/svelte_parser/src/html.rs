use crate::html_entities::NAMED_ENTITIES;

const WINDOWS_1252: [u32; 32] = [
    8364, 129, 8218, 402, 8222, 8230, 8224, 8225, 710, 8240, 352, 8249, 338, 141, 381, 143, 144,
    8216, 8217, 8220, 8221, 8226, 8211, 8212, 732, 8482, 353, 8250, 339, 157, 382, 376,
];

const MAX_ENTITY_LEN: usize = 40;

pub(crate) fn decode_text(input: &str) -> Option<String> {
    decode_with_mode(input, false)
}

pub(crate) fn decode_attribute_value(input: &str) -> Option<String> {
    decode_with_mode(input, true)
}

fn decode_with_mode(input: &str, is_attribute_value: bool) -> Option<String> {
    let bytes = input.as_bytes();
    let first = memchr::memchr(b'&', bytes)?;

    let mut out = String::with_capacity(input.len());
    out.push_str(&input[..first]);
    let mut cursor = first;
    let mut changed = false;

    loop {
        if let Some((decoded, consumed)) = decode_entity(&input[cursor + 1..], is_attribute_value) {
            out.push(decoded);
            cursor += 1 + consumed;
            changed = true;
        } else {
            out.push('&');
            cursor += 1;
        }

        match memchr::memchr(b'&', &bytes[cursor..]) {
            Some(rel) => {
                out.push_str(&input[cursor..cursor + rel]);
                cursor += rel;
            }
            None => break,
        }
    }

    if !changed {
        return None;
    }

    out.push_str(&input[cursor..]);
    Some(out)
}

fn decode_entity(rest: &str, is_attribute_value: bool) -> Option<(char, usize)> {
    if let Some(decoded) = decode_numeric_entity(rest) {
        return Some(decoded);
    }

    decode_named_entity(rest, is_attribute_value)
}

fn decode_numeric_entity(rest: &str) -> Option<(char, usize)> {
    let bytes = rest.as_bytes();
    if bytes.first().copied() != Some(b'#') {
        return None;
    }

    let (radix, digits_start) = match bytes.get(1).copied() {
        Some(b'x') | Some(b'X') => (16, 2),
        _ => (10, 1),
    };

    let mut end = digits_start;
    while end < bytes.len()
        && match radix {
            16 => bytes[end].is_ascii_hexdigit(),
            _ => bytes[end].is_ascii_digit(),
        }
    {
        end += 1;
    }

    if end == digits_start {
        return None;
    }

    let has_semicolon = bytes.get(end).copied() == Some(b';');
    let consumed = end + usize::from(has_semicolon);
    let code = u32::from_str_radix(&rest[digits_start..end], radix).ok()?;
    let validated = validate_code(code);
    let decoded = char::from_u32(validated)?;
    Some((decoded, consumed))
}

fn decode_named_entity(rest: &str, is_attribute_value: bool) -> Option<(char, usize)> {
    let bytes = rest.as_bytes();
    let limit = bytes.len().min(MAX_ENTITY_LEN);
    let ascii_len = bytes[..limit].iter().take_while(|b| b.is_ascii()).count();
    let mut best = None;
    let mut lo = 0;
    let mut hi = NAMED_ENTITIES.len();

    for end in 1..=ascii_len {
        let candidate = &rest[..end];
        lo += NAMED_ENTITIES[lo..hi].partition_point(|(name, _)| *name < candidate);
        hi = lo + NAMED_ENTITIES[lo..hi].partition_point(|(name, _)| name.starts_with(candidate));
        if lo == hi {
            break;
        }
        if NAMED_ENTITIES[lo].0 == candidate {
            let name = NAMED_ENTITIES[lo].0;
            if is_attribute_value && !name.ends_with(';') {
                let next = bytes.get(end).copied();
                let blocks = match next {
                    None => false,
                    Some(b) => b == b'=' || b.is_ascii_alphanumeric(),
                };
                if blocks {
                    continue;
                }
            }
            best = Some((NAMED_ENTITIES[lo].1, end));
        }
    }

    let (code, consumed) = best?;
    let validated = validate_code(code);
    let decoded = char::from_u32(validated)?;
    Some((decoded, consumed))
}

fn validate_code(code: u32) -> u32 {
    match code {
        10 => 32,
        0..=127 => code,
        128..=159 => WINDOWS_1252[(code - 128) as usize],
        160..=55295 => code,
        55296..=57343 => 0,
        57344..=65535 => code,
        65536..=131071 => code,
        131072..=196607 => code,
        917504..=917631 | 917760..=917999 => code,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_attribute_value, decode_text};

    #[test]
    fn decodes_named_entities_in_text() {
        assert_eq!(decode_text("&amp; &lt;"), Some("& <".into()));
    }

    #[test]
    fn decodes_numeric_entities_in_text() {
        assert_eq!(decode_text("&#38; &#x3c;"), Some("& <".into()));
    }

    #[test]
    fn returns_none_when_text_is_unchanged() {
        assert_eq!(decode_text("plain text"), None);
    }

    #[test]
    fn decodes_named_entity_followed_by_multibyte_char() {
        assert_eq!(decode_text("&nbsp;\u{42f}"), Some("\u{a0}\u{42f}".into()));
    }

    #[test]
    fn leaves_ampersand_when_multibyte_char_follows_directly() {
        assert_eq!(decode_text("&\u{42f}"), None);
    }

    #[test]
    fn decodes_named_entity_in_mixed_multibyte_text() {
        assert_eq!(
            decode_text("a&nbsp;\u{42f}b\u{2603}c"),
            Some("a\u{a0}\u{42f}b\u{2603}c".into()),
        );
    }

    #[test]
    fn attr_decodes_terminated_entities() {
        assert_eq!(
            decode_attribute_value("a&nbsp;b&amp;c&lt;d"),
            Some("a\u{a0}b&c<d".into()),
        );
    }

    #[test]
    fn attr_keeps_unterminated_entity_when_followed_by_equals() {
        assert_eq!(decode_attribute_value("&amp=q"), None);
        assert_eq!(decode_attribute_value("&nbsp=q"), None);
    }

    #[test]
    fn attr_keeps_unterminated_entity_when_followed_by_alnum() {
        assert_eq!(decode_attribute_value("&ampoule"), None);
    }

    #[test]
    fn attr_decodes_unterminated_entity_at_word_boundary() {
        assert_eq!(decode_attribute_value("&lt foo"), Some("< foo".into()));
        assert_eq!(
            decode_attribute_value("&copy bar"),
            Some("\u{a9} bar".into())
        );
    }

    #[test]
    fn attr_decodes_unterminated_entity_at_end_of_input() {
        assert_eq!(decode_attribute_value("&nbsp"), Some("\u{a0}".into()));
    }

    #[test]
    fn attr_decodes_numeric_entity_without_semicolon() {
        assert_eq!(decode_attribute_value("&#38"), Some("&".into()));
        assert_eq!(decode_attribute_value("&#x3c;"), Some("<".into()));
    }
}
