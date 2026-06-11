use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

pub fn strip_js_comments(js: &str) -> String {
    let allocator = Allocator::default();
    let source_type = SourceType::default().with_module(true);
    let parsed = Parser::new(&allocator, js, source_type).parse();

    let mut spans: Vec<(usize, usize)> = parsed
        .program
        .comments
        .iter()
        .map(|c| (c.span.start as usize, c.span.end as usize))
        .collect();
    if spans.is_empty() {
        return js.to_string();
    }
    spans.sort_unstable_by_key(|&(start, _)| start);

    let bytes = js.as_bytes();
    let mut out = String::with_capacity(js.len());
    let mut cursor = 0usize;

    for (start, end) in spans {
        if start < cursor {
            continue;
        }
        let line_start = js[..start].rfind('\n').map_or(0, |i| i + 1);
        let before = &js[line_start..start];
        let before_all_ws = before.bytes().all(|b| b == b' ' || b == b'\t');

        let mut after = end;
        while after < bytes.len() && (bytes[after] == b' ' || bytes[after] == b'\t') {
            after += 1;
        }
        let after_is_eol = after >= bytes.len() || bytes[after] == b'\n';

        if before_all_ws && after_is_eol {
            out.push_str(&js[cursor..line_start]);
            cursor = if after < bytes.len() {
                after + 1
            } else {
                after
            };
        } else if after_is_eol {
            let code_end = line_start + before.trim_end_matches([' ', '\t']).len();
            out.push_str(&js[cursor..code_end]);
            cursor = after;
        } else {
            out.push_str(&js[cursor..start]);
            cursor = after;
        }
    }
    out.push_str(&js[cursor..]);
    out
}

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

    #[track_caller]
    fn assert_stripped(input: &str, expected: &str) {
        let got = strip_js_comments(input);
        assert_eq!(
            got, expected,
            "strip_js_comments: expected {expected:?}, got {got:?}"
        );
    }

    #[test]
    fn drops_whole_line_comment_with_its_newline() {
        assert_stripped("\t/* a */\n\t/* b */\n\trefresh();\n", "\trefresh();\n");
    }

    #[test]
    fn drops_trailing_comment_and_preceding_space_keeps_code() {
        assert_stripped("\tg(); // call reset\n", "\tg();\n");
    }

    #[test]
    fn drops_inline_block_comment_with_following_space() {
        assert_stripped(
            "\t\t/* listen */ scrollToIndex();\n",
            "\t\tscrollToIndex();\n",
        );
    }

    #[test]
    fn drops_multiline_block_comment_leaving_no_blank_line() {
        assert_stripped("a;\n/**\n * doc\n */\nconst b = 1;\n", "a;\nconst b = 1;\n");
    }

    #[test]
    fn preserves_non_comment_formatting_byte_for_byte() {
        assert_stripped(
            "function f() {\n\treturn  1;\n}\n",
            "function f() {\n\treturn  1;\n}\n",
        );
    }

    #[test]
    fn keeps_comment_like_text_inside_strings() {
        assert_stripped(
            "const s = \"/* not a comment */\";\n",
            "const s = \"/* not a comment */\";\n",
        );
    }
}
