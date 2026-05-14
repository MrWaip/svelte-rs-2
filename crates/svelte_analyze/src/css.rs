pub(crate) fn css_component_hash(filename: &str, root_dir: Option<&str>, css: &str) -> String {
    let effective = effective_hash_input(filename, root_dir, css);
    let mut h: u32 = 5381;
    for ch in effective.chars().rev() {
        h = h.wrapping_shl(5).wrapping_sub(h) ^ (ch as u32);
    }
    format!("svelte-{}", to_base36(h))
}

fn effective_hash_input<'a>(filename: &'a str, root_dir: Option<&str>, css: &'a str) -> String {
    if filename == "(unknown)" {
        return css.to_string();
    }
    let normalized = filename.replace('\\', "/");
    let Some(rd) = root_dir else {
        return normalized;
    };
    let rd_norm = rd.replace('\\', "/");
    if let Some(rest) = normalized.strip_prefix(&rd_norm) {
        rest.trim_start_matches('/').to_string()
    } else {
        normalized
    }
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
        assert_eq!(css_component_hash("(unknown)", None, css), "svelte-1a7i8ec");
    }

    #[test]
    fn css_component_hash_empty() {
        let h = css_component_hash("(unknown)", None, "");
        assert!(h.starts_with("svelte-"));
    }

    #[test]
    fn uses_filename_when_known() {
        let h1 = css_component_hash("App.svelte", None, "p { color: red; }");
        let h2 = css_component_hash("App.svelte", None, "div { color: blue; }");
        assert_eq!(h1, h2, "hash must depend on filename only when filename is known");
    }

    #[test]
    fn different_filenames_yield_different_hashes() {
        let css = "p { color: red; }";
        assert_ne!(
            css_component_hash("A.svelte", None, css),
            css_component_hash("B.svelte", None, css)
        );
    }

    #[test]
    fn root_dir_prefix_is_stripped_before_hashing() {
        let css = "p { color: red; }";
        let with_root = css_component_hash(
            "/home/user/proj/src/A.svelte",
            Some("/home/user/proj"),
            css,
        );
        let relative = css_component_hash("src/A.svelte", None, css);
        assert_eq!(with_root, relative);
    }

    #[test]
    fn backslashes_normalize_to_forward_slashes() {
        let css = "p { color: red; }";
        let windows = css_component_hash(
            "C:\\Users\\proj\\src\\A.svelte",
            Some("C:\\Users\\proj"),
            css,
        );
        let posix = css_component_hash("src/A.svelte", None, css);
        assert_eq!(windows, posix);
    }

    #[test]
    fn root_dir_not_a_prefix_leaves_filename_intact() {
        let css = "p { color: red; }";
        let unstripped = css_component_hash("/foo/A.svelte", Some("/bar"), css);
        let plain = css_component_hash("/foo/A.svelte", None, css);
        assert_eq!(unstripped, plain);
    }
}
