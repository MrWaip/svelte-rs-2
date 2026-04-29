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
    String::from_utf8(buf).expect("DIGITS contains only ASCII bytes — always valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_component_hash_basic() {
        let css = "\n\tp {\n\t\tcolor: red;\n\t}\n";
        assert_eq!(css_component_hash(css), "svelte-1a7i8ec");
    }

    #[test]
    fn css_component_hash_empty() {
        let h = css_component_hash("");
        assert!(h.starts_with("svelte-"));
    }
}
