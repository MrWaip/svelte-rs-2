use crate::html_entities::NAMED_ENTITIES;

const WINDOWS_1252: [u32; 32] = [
    8364, 129, 8218, 402, 8222, 8230, 8224, 8225, 710, 8240, 352, 8249, 338, 141, 381, 143, 144,
    8216, 8217, 8220, 8221, 8226, 8211, 8212, 732, 8482, 353, 8250, 339, 157, 382, 376,
];

const MAX_ENTITY_LEN: usize = 40;

pub(crate) fn decode_text(input: &str) -> Option<String> {
    let mut out = String::new();
    let mut cursor = 0;
    let mut changed = false;

    while let Some(rel) = input[cursor..].find('&') {
        let start = cursor + rel;
        out.push_str(&input[cursor..start]);

        if let Some((decoded, consumed)) = decode_entity(&input[start + 1..]) {
            out.push(decoded);
            cursor = start + 1 + consumed;
            changed = true;
        } else {
            out.push('&');
            cursor = start + 1;
        }
    }

    if !changed {
        return None;
    }

    out.push_str(&input[cursor..]);
    Some(out)
}

fn decode_entity(rest: &str) -> Option<(char, usize)> {
    if let Some(decoded) = decode_numeric_entity(rest) {
        return Some(decoded);
    }

    decode_named_entity(rest)
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

fn decode_named_entity(rest: &str) -> Option<(char, usize)> {
    let limit = rest.len().min(MAX_ENTITY_LEN);
    let mut best = None;

    for end in 1..=limit {
        let candidate = &rest[..end];
        if let Ok(index) = NAMED_ENTITIES.binary_search_by_key(&candidate, |(name, _)| *name) {
            best = Some((NAMED_ENTITIES[index].1, end));
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
    use super::decode_text;

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
}
