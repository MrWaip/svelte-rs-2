/// Compute the Svelte scoping class for a component's CSS source.
///
/// Algorithm: djb2 iterated in reverse over char code points (as u32), kept as
/// a wrapping u32, then formatted as `"svelte-{n_base36}"`.
///
/// This matches Svelte 5's default `cssHash` option exactly:
///   `svelte-${hash(filename === '(unknown)' ? css : filename ?? css)}`
/// where `hash` is the same djb2-reverse function.
pub(crate) fn css_component_hash(css: &str) -> String {
    let mut h: u32 = 5381;
    for ch in css.chars().rev() {
        h = h.wrapping_shl(5).wrapping_sub(h) ^ (ch as u32);
    }
    format!("svelte-{}", to_base36(h))
}

fn to_base36(mut n: u32) -> String {
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    if n == 0 {
        return "0".to_string();
    }
    let mut buf = Vec::with_capacity(8);
    while n > 0 {
        buf.push(DIGITS[(n % 36) as usize]);
        n /= 36;
    }
    buf.reverse();
    String::from_utf8(buf).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_component_hash_basic() {
        // Verified against Svelte 5.53.9 reference compiler:
        // input = CSS content of css_scoped_basic/case.svelte
        let css = "\n\tp {\n\t\tcolor: red;\n\t}\n";
        assert_eq!(css_component_hash(css), "svelte-1a7i8ec");
    }

    #[test]
    fn css_component_hash_empty() {
        // Empty string should not panic
        let h = css_component_hash("");
        assert!(h.starts_with("svelte-"));
    }
}
