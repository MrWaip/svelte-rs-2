pub fn strip_reference_only_css_markers(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let bytes = css.as_bytes();
    let mut idx = 0;
    while idx < bytes.len() {
        if let Some(skipped) = match_marker(css, idx) {
            idx = skipped;
            continue;
        }
        out.push(bytes[idx] as char);
        idx += 1;
    }
    out
}

fn match_marker(css: &str, idx: usize) -> Option<usize> {
    let tail = css.get(idx..)?;
    if tail.starts_with("/* (unused) ") || tail.starts_with("/* (empty) ") {
        let end_rel = tail.find("*/")?;
        return Some(idx + end_rel + "*/".len());
    }
    if tail.starts_with("/* :global {*/") {
        return Some(idx + "/* :global {*/".len());
    }
    if tail.starts_with("/*}*/") {
        return Some(idx + "/*}*/".len());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_unused_rule() {
        assert_eq!(
            strip_reference_only_css_markers(".a {} /* (unused) .b { color: red; }*/ .c {}"),
            ".a {}  .c {}"
        );
    }

    #[test]
    fn strips_empty_rule() {
        assert_eq!(
            strip_reference_only_css_markers(".a {} /* (empty) .b {}*/"),
            ".a {} "
        );
    }

    #[test]
    fn strips_lone_global_wrapper() {
        assert_eq!(
            strip_reference_only_css_markers("/* :global {*/ .a {} /*}*/"),
            " .a {} "
        );
    }

    #[test]
    fn strips_multiple_markers() {
        let input = ".a { /* :global {*/ p {} /*}*/ /* (unused) .b {}*/ }";
        assert_eq!(strip_reference_only_css_markers(input), ".a {  p {}   }");
    }

    #[test]
    fn preserves_other_comments() {
        assert_eq!(
            strip_reference_only_css_markers(".a { /* note */ color: red; }"),
            ".a { /* note */ color: red; }"
        );
    }
}
