use svelte_diagnostics::DiagnosticKind;
use svelte_span::GetSpan;

use super::*;

#[test]
fn smoke() {
    let source = "<div>kek {name} hello</div>";
    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
    assert!(tokens[1].token_type == TokenType::Text);
    assert!(matches!(tokens[2].token_type, TokenType::Interpolation(_)));
    assert!(tokens[3].token_type == TokenType::Text);
    assert!(matches!(tokens[4].token_type, TokenType::EndTag(_)));
    assert!(tokens[5].token_type == TokenType::EOF);
}

#[test]
fn interpolation_with_js_strings() {
    let mut scanner = Scanner::new("{ name + '}' + \"{}\" + `{\n}` }");

    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_js_curly_braces_balance() {
    let mut scanner = Scanner::new("{ { field: 1} + (function(){return {}}) }");

    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_with_regex_literal() {
    let source = r"{ /}/.test(value) }";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    if let TokenType::Interpolation(ref et) = tokens[0].token_type {
        assert_eq!(et.expression_span.source_text(source), "/}/.test(value)");
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_with_regex_character_class() {
    let source = r"{ /[}]/.test(value) }";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    if let TokenType::Interpolation(ref et) = tokens[0].token_type {
        assert_eq!(et.expression_span.source_text(source), "/[}]/.test(value)");
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_with_block_comment() {
    let source = "{ a /* } */ + b }";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    if let TokenType::Interpolation(ref et) = tokens[0].token_type {
        assert_eq!(et.expression_span.source_text(source), "a /* } */ + b");
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_with_line_comment() {
    let source = "{ a // }\n + b }";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    if let TokenType::Interpolation(ref et) = tokens[0].token_type {
        assert_eq!(et.expression_span.source_text(source), "a // }\n + b");
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_with_nested_template_literal() {
    let source = "{ `x ${foo({ bar: `}` })} y` }";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    if let TokenType::Interpolation(ref et) = tokens[0].token_type {
        assert_eq!(
            et.expression_span.source_text(source),
            "`x ${foo({ bar: `}` })} y`"
        );
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn self_closed_start_tag() {
    let source = "<input/>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert_start_tag(source, &tokens[0], "input", vec![], true);
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn attribute_name_with_underscore() {
    let source = "<div foo_bar=\"x\" class:is_expanded={true} on:foo_bar={h} use:foo_bar={a} bind:foo_bar={v} style:foo_bar=\"red\" />";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(
        diagnostics.is_empty(),
        "unexpected diagnostics: {diagnostics:?}"
    );

    let TokenType::StartTag(start_tag) = &tokens[0].token_type else {
        panic!("expected StartTag");
    };
    let names: Vec<&str> = start_tag
        .attributes
        .iter()
        .map(|a| match a {
            Attribute::HTMLAttribute(v) => v.name_span.source_text(source),
            Attribute::ClassDirective(cd) => cd.name_span.source_text(source),
            Attribute::OnDirectiveLegacy(od) => od.name_span.source_text(source),
            Attribute::UseDirective(ud) => ud.name_span.source_text(source),
            Attribute::BindDirective(bd) => bd.name_span.source_text(source),
            Attribute::StyleDirective(sd) => sd.name_span.source_text(source),
            _ => panic!("unexpected attribute variant"),
        })
        .collect();
    assert_eq!(
        names,
        vec![
            "foo_bar",
            "is_expanded",
            "foo_bar",
            "foo_bar",
            "foo_bar",
            "foo_bar",
        ]
    );
}

#[test]
fn start_tag_attributes() {
    let source = "<div valid id=123 touched some=true disabled value=\"333\" class='never' >";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert_start_tag(
        source,
        &tokens[0],
        "div",
        vec![
            ("valid", ""),
            ("id", "123"),
            ("touched", ""),
            ("some", "true"),
            ("disabled", ""),
            ("value", "333"),
            ("class", "never"),
        ],
        false,
    );

    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn unquoted_attribute_value_stops_before_self_close_slash() {
    let source = "<circle cx=50 cy=50 r=50/>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(
        source,
        &tokens[0],
        "circle",
        vec![("cx", "50"), ("cy", "50"), ("r", "50")],
        true,
    );
}

#[test]
fn unquoted_attribute_value_keeps_bare_slash_not_before_gt() {
    let source = "<a href=foo/bar/baz>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(
        source,
        &tokens[0],
        "a",
        vec![("href", "foo/bar/baz")],
        false,
    );
}

#[test]
fn tag_line_comment_between_attributes() {
    let source = "<div\n\ta=\"1\"\n\t// a line comment\n\tb=\"2\"\n>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(
        source,
        &tokens[0],
        "div",
        vec![("a", "1"), ("b", "2")],
        false,
    );
}

#[test]
fn tag_block_comment_between_attributes() {
    let source = "<div a=\"1\" /* a block comment */ b=\"2\">";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(
        source,
        &tokens[0],
        "div",
        vec![("a", "1"), ("b", "2")],
        false,
    );
}

#[test]
fn tag_block_comment_spanning_multiple_lines() {
    let source = "<div a=\"1\" /* one\n\ttwo\n\tthree */ b=\"2\">";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(
        source,
        &tokens[0],
        "div",
        vec![("a", "1"), ("b", "2")],
        false,
    );
}

#[test]
fn tag_comment_right_after_name() {
    let source = "<div /* c */ a=\"1\">";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(source, &tokens[0], "div", vec![("a", "1")], false);
}

#[test]
fn tag_comment_right_before_close() {
    let source = "<div a=\"1\" /* trailing */>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(source, &tokens[0], "div", vec![("a", "1")], false);
}

#[test]
fn tag_comment_before_self_close() {
    let source = "<input /* c */ value=\"a\" />";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(source, &tokens[0], "input", vec![("value", "a")], true);
}

#[test]
fn tag_multiple_consecutive_comments() {
    let source = "<span /* one */ // two\n x=\"1\">";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(source, &tokens[0], "span", vec![("x", "1")], false);
}

#[test]
fn tag_comment_on_component() {
    let source = "<Comp /* c */ prop=\"a\" />";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(source, &tokens[0], "Comp", vec![("prop", "a")], true);
}

#[test]
fn tag_unclosed_block_comment_reaches_end_of_file() {
    let (_, diagnostics) = Scanner::new("<div /* unclosed").scan_tokens();

    assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[test]
fn tag_line_comment_eats_close_to_end_of_file() {
    let (_, diagnostics) = Scanner::new("<div a=\"1\" // no newline before gt>").scan_tokens();

    assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[test]
fn top_level_script_tag_does_not_treat_comment_as_whitespace() {
    let (_, diagnostics) = Scanner::new("<script /* c */>const x = 1;</script>").scan_tokens();

    assert_has_diagnostic(
        &diagnostics,
        DiagnosticKind::ExpectedToken { token: ">".into() },
    );
}

#[test]
fn top_level_style_tag_does_not_treat_comment_as_whitespace() {
    let (_, diagnostics) = Scanner::new("<style /* c */>a{}</style>").scan_tokens();

    assert_has_diagnostic(
        &diagnostics,
        DiagnosticKind::ExpectedToken { token: ">".into() },
    );
}

#[test]
fn nested_script_tag_allows_comment_between_attributes() {
    let source = "<div><script /* c */ src=\"a\"></script></div>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_start_tag(source, &tokens[1], "script", vec![("src", "a")], false);
}

#[test]
fn attribute_tokens_capture_full_source_spans() {
    let source = "<div class='x' {...props} let:item={slotProps} on:click|once={handler} style:color|important {@attach attach} />";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    let TokenType::StartTag(start_tag) = &tokens[0].token_type else {
        panic!("expected StartTag");
    };

    let spans: Vec<_> = start_tag
        .attributes
        .iter()
        .map(|attr| attr.span().source_text(source))
        .collect();

    assert_eq!(
        spans,
        vec![
            "class='x'",
            "{...props}",
            "let:item={slotProps}",
            "on:click|once={handler}",
            "style:color|important",
            "{@attach attach}",
        ]
    );
}

#[test]
fn let_directive_legacy_without_expression() {
    let source = "<Comp let:item></Comp>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    let TokenType::StartTag(start_tag) = &tokens[0].token_type else {
        panic!("expected StartTag");
    };

    let Attribute::LetDirectiveLegacy(dir) = &start_tag.attributes[0] else {
        panic!("expected LetDirectiveLegacy");
    };

    assert_eq!(dir.name_span.source_text(source), "item");
    assert!(!dir.has_expression);
}

#[test]
fn let_directive_legacy_with_expression() {
    let source = "<Comp let:item={processed}></Comp>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    let TokenType::StartTag(start_tag) = &tokens[0].token_type else {
        panic!("expected StartTag");
    };

    let Attribute::LetDirectiveLegacy(dir) = &start_tag.attributes[0] else {
        panic!("expected LetDirectiveLegacy");
    };

    assert_eq!(dir.name_span.source_text(source), "item");
    assert!(dir.has_expression);
    assert_eq!(dir.expression_span.source_text(source), "processed");
}

#[test]
fn comment() {
    let source = "<!-- \nsome comment\n -->";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    let TokenType::Comment { content_span } = tokens[0].token_type else {
        panic!("expected comment token");
    };
    assert_eq!(content_span.source_text(source), " \nsome comment\n ");
    assert_eq!(
        tokens[0].span.source_text(source),
        "<!-- \nsome comment\n -->"
    );
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn each_block() {
    let source = "{#each [1,2,3] as { value, flag }}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert_start_each_tag(source, &tokens[0], "[1,2,3]", "{ value, flag }");
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[track_caller]
fn assert_start_tag(
    source: &str,
    token: &Token,
    expected_name: &str,
    expected_attributes: Vec<(&str, &str)>,
    expected_self_closing: bool,
) {
    let start_tag = match &token.token_type {
        TokenType::StartTag(t) => t,
        _ => panic!("Expected token.type = StartTag."),
    };

    assert_eq!(start_tag.name_span.source_text(source), expected_name);
    assert_eq!(start_tag.self_closing, expected_self_closing);
    assert_attributes(source, &start_tag.attributes, expected_attributes);
}

fn assert_attributes(
    source: &str,
    actual_attributes: &[Attribute],
    expected_attributes: Vec<(&str, &str)>,
) {
    assert_eq!(actual_attributes.len(), expected_attributes.len());

    for (index, (expected_name, expected_value)) in expected_attributes.iter().enumerate() {
        let attribute = &actual_attributes[index];

        let name = match attribute {
            Attribute::HTMLAttribute(value) => value.name_span.source_text(source),
            Attribute::ExpressionTag(_) => "$expression",
            Attribute::ClassDirective(_) => "$classDirective",
            Attribute::StyleDirective(sd) => sd.name_span.source_text(source),
            Attribute::BindDirective(_) => "$bindDirective",
            Attribute::LetDirectiveLegacy(ld) => ld.name_span.source_text(source),
            Attribute::UseDirective(ud) => ud.name_span.source_text(source),
            Attribute::OnDirectiveLegacy(od) => od.name_span.source_text(source),
            Attribute::TransitionDirective(td) => td.name_span.source_text(source),
            Attribute::AnimateDirective(ad) => ad.name_span.source_text(source),
            Attribute::AttachTag(_) => "$attachTag",
        };

        let value: String = match attribute {
            Attribute::HTMLAttribute(value) => match value.value {
                AttributeValue::String(span) => span.source_text(source).to_string(),
                AttributeValue::Empty => String::new(),
                AttributeValue::ExpressionTag(ref et) => {
                    et.expression_span.source_text(source).to_string()
                }
                AttributeValue::Concatenation(ref c) => c
                    .parts
                    .iter()
                    .map(|p| match p {
                        ConcatenationPart::String(span) => {
                            format!("({})", span.source_text(source))
                        }
                        ConcatenationPart::Expression(et) => {
                            format!("({{{}}})", et.expression_span.source_text(source))
                        }
                    })
                    .collect(),
            },
            Attribute::ExpressionTag(value) => {
                value.expression_span.source_text(source).to_string()
            }
            Attribute::ClassDirective(cd) => cd.expression_span.source_text(source).to_string(),
            Attribute::StyleDirective(sd) => match sd.value {
                AttributeValue::String(span) => span.source_text(source).to_string(),
                AttributeValue::Empty => String::new(),
                AttributeValue::ExpressionTag(ref et) => {
                    et.expression_span.source_text(source).to_string()
                }
                AttributeValue::Concatenation(ref c) => c
                    .parts
                    .iter()
                    .map(|p| match p {
                        ConcatenationPart::String(span) => {
                            format!("({})", span.source_text(source))
                        }
                        ConcatenationPart::Expression(et) => {
                            format!("({{{}}})", et.expression_span.source_text(source))
                        }
                    })
                    .collect(),
            },
            Attribute::BindDirective(bd) => bd.expression_span.source_text(source).to_string(),
            Attribute::LetDirectiveLegacy(ld) => {
                if ld.has_expression {
                    ld.expression_span.source_text(source).to_string()
                } else {
                    String::new()
                }
            }
            Attribute::UseDirective(ud) => ud.expression_span.source_text(source).to_string(),
            Attribute::OnDirectiveLegacy(od) => od.expression_span.source_text(source).to_string(),
            Attribute::TransitionDirective(td) => {
                td.expression_span.source_text(source).to_string()
            }
            Attribute::AnimateDirective(ad) => ad.expression_span.source_text(source).to_string(),
            Attribute::AttachTag(at) => at.expression_span.source_text(source).to_string(),
        };

        assert_eq!(name, *expected_name);
        assert_eq!(value, expected_value.to_string());
    }
}

fn assert_start_each_tag(
    source: &str,
    token: &Token,
    expected_collection: &str,
    expected_item: &str,
) {
    let tag = match &token.token_type {
        TokenType::StartEachTag(t) => t,
        _ => panic!("Expected token.type = StartEachTag"),
    };

    assert_eq!(tag.collection_span.source_text(source), expected_collection);
    assert_eq!(
        tag.context_span
            .as_ref()
            .expect("expected item binding")
            .source_text(source),
        expected_item
    );
    assert!(tag.index_span.is_none(), "expected no index");
}

fn assert_start_each_tag_with_index(
    source: &str,
    token: &Token,
    expected_collection: &str,
    expected_item: &str,
    expected_index: &str,
) {
    let tag = match &token.token_type {
        TokenType::StartEachTag(t) => t,
        _ => panic!("Expected token.type = StartEachTag"),
    };

    assert_eq!(tag.collection_span.source_text(source), expected_collection);
    assert_eq!(
        tag.context_span
            .as_ref()
            .expect("expected item binding")
            .source_text(source),
        expected_item
    );
    let index = tag.index_span.as_ref().expect("expected index");
    assert_eq!(index.source_text(source), expected_index);
}

#[test]
fn each_block_with_index() {
    let source = "{#each items as item, i}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert_start_each_tag_with_index(source, &tokens[0], "items", "item", "i");
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn each_block_destructured_with_index() {
    let source = "{#each items as { value, flag }, idx}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert_start_each_tag_with_index(source, &tokens[0], "items", "{ value, flag }", "idx");
    assert!(tokens[1].token_type == TokenType::EOF);
}

fn assert_no_as_each_with_index(
    source: &str,
    token: &Token,
    expected_collection: &str,
    expected_index: &str,
) {
    let tag = match &token.token_type {
        TokenType::StartEachTag(t) => t,
        _ => panic!("Expected token.type = StartEachTag"),
    };
    assert_eq!(tag.collection_span.source_text(source), expected_collection);
    assert!(tag.context_span.is_none(), "expected no context (no `as`)");
    let index = tag.index_span.as_ref().expect("expected index span");
    assert_eq!(index.source_text(source), expected_index);
    assert!(tag.key_span.is_none(), "expected no key");
}

#[test]
fn each_block_no_as_with_index_simple() {
    let source = "{#each items, rank}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_no_as_each_with_index(source, &tokens[0], "items", "rank");
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn each_block_no_as_with_index_object_literal() {
    let source = "{#each { length: 8 }, rank}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_no_as_each_with_index(source, &tokens[0], "{ length: 8 }", "rank");
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn each_block_no_as_with_index_and_key() {
    let source = "{#each items, rank (item.id)}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    let TokenType::StartEachTag(tag) = &tokens[0].token_type else {
        panic!("Expected StartEachTag");
    };
    assert_eq!(tag.collection_span.source_text(source), "items");
    assert_eq!(
        tag.index_span
            .as_ref()
            .expect("expected index")
            .source_text(source),
        "rank"
    );
    assert_eq!(
        tag.key_span
            .as_ref()
            .expect("expected key")
            .source_text(source),
        "item.id"
    );
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[track_caller]
fn assert_has_diagnostic(diagnostics: &[Diagnostic], err_kind: DiagnosticKind) {
    assert!(
        diagnostics.iter().any(|d| d.kind == err_kind),
        "expected diagnostic {err_kind:?}, got: {diagnostics:?}"
    );
}

#[test]
fn unterminated_start_tag() {
    let mut scanner = Scanner::new("<div disabled");
    let (_, diagnostics) = scanner.scan_tokens();
    assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[test]
fn interpolation_escaped_double_quote() {
    let mut scanner = Scanner::new(r#"{ name.replace("\"", "'") }"#);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_escaped_single_quote() {
    let mut scanner = Scanner::new(r"{ 'it\'s a test' }");
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_escaped_backtick() {
    let mut scanner = Scanner::new(r"{ `hello \` world` }");
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn interpolation_escaped_backslash() {
    let mut scanner = Scanner::new(r#"{ "path\\to\\file" }"#);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn style_tag_basic() {
    let source = "<style>.foo { color: red; }</style>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
    if let TokenType::StyleTag(ref st) = tokens[0].token_type {
        assert_eq!(st.content_span.source_text(source), ".foo { color: red; }");
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn style_tag_with_angle_brackets() {
    let source = "<style>a > b { color: red; }</style>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
    if let TokenType::StyleTag(ref st) = tokens[0].token_type {
        assert_eq!(st.content_span.source_text(source), "a > b { color: red; }");
    }
}

#[test]
fn nested_style_tag_scans_as_plain_element_with_text_child() {
    let source = "<div><style>.foo { color: red; }</style></div>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
    assert!(matches!(tokens[1].token_type, TokenType::StartTag(_)));
    assert!(matches!(tokens[2].token_type, TokenType::Text));
    assert!(matches!(tokens[3].token_type, TokenType::EndTag(_)));
    assert!(matches!(tokens[4].token_type, TokenType::EndTag(_)));
    assert!(matches!(tokens[5].token_type, TokenType::EOF));

    let TokenType::StartTag(ref div_tag) = tokens[0].token_type else {
        panic!("expected outer div start tag");
    };
    assert_eq!(div_tag.name_span.source_text(source), "div");

    let TokenType::StartTag(ref style_tag) = tokens[1].token_type else {
        panic!("expected nested style start tag");
    };
    assert_eq!(style_tag.name_span.source_text(source), "style");

    assert_eq!(tokens[2].span.source_text(source), ".foo { color: red; }");

    let TokenType::EndTag(ref style_end) = tokens[3].token_type else {
        panic!("expected nested style end tag");
    };
    assert_eq!(style_end.name_span.source_text(source), "style");
}

fn assert_start_each_tag_with_key(
    source: &str,
    token: &Token,
    expected_collection: &str,
    expected_item: &str,
    expected_key: &str,
) {
    let tag = match &token.token_type {
        TokenType::StartEachTag(t) => t,
        _ => panic!("Expected token.type = StartEachTag"),
    };

    assert_eq!(tag.collection_span.source_text(source), expected_collection);
    assert_eq!(
        tag.context_span
            .as_ref()
            .expect("expected item binding")
            .source_text(source),
        expected_item
    );
    let key = tag.key_span.as_ref().expect("expected key");
    assert_eq!(key.source_text(source), expected_key);
}

#[test]
fn each_block_with_key() {
    let source = "{#each items as item (item.id)}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_start_each_tag_with_key(source, &tokens[0], "items", "item", "item.id");
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn each_block_with_regex_key() {
    let source = r"{#each items as item (/}/.test(item.id) ? item.id : item)}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_start_each_tag_with_key(
        source,
        &tokens[0],
        "items",
        "item",
        "/}/.test(item.id) ? item.id : item",
    );
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn each_block_with_index_and_key() {
    let source = "{#each items as item, i (item.id)}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    let tag = match &tokens[0].token_type {
        TokenType::StartEachTag(t) => t,
        _ => panic!("Expected StartEachTag"),
    };
    assert_eq!(tag.collection_span.source_text(source), "items");
    assert_eq!(
        tag.context_span
            .as_ref()
            .expect("test invariant")
            .source_text(source),
        "item"
    );
    assert_eq!(
        tag.index_span
            .as_ref()
            .expect("test invariant")
            .source_text(source),
        "i"
    );
    assert_eq!(
        tag.key_span
            .as_ref()
            .expect("test invariant")
            .source_text(source),
        "item.id"
    );
}

#[test]
fn each_block_destructured_with_key() {
    let source = "{#each items as {name} (name)}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_start_each_tag_with_key(source, &tokens[0], "items", "{name}", "name");
}

#[test]
fn each_block_context_with_string_containing_closing_brace() {
    let source = r#"{#each items as { label = "}" }, idx}"#;
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert_start_each_tag_with_index(source, &tokens[0], "items", r#"{ label = "}" }"#, "idx");
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn each_block_context_with_template_literal() {
    let source = "{#each items as { label = `}` }, idx}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;

    assert_start_each_tag_with_index(source, &tokens[0], "items", "{ label = `}` }", "idx");
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn class_directive_with_expression() {
    let source = "<div class:active={isActive}>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_start_tag(
        source,
        &tokens[0],
        "div",
        vec![("$classDirective", "isActive")],
        false,
    );
}

#[test]
fn class_directive_shorthand() {
    let source = "<div class:active>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_start_tag(
        source,
        &tokens[0],
        "div",
        vec![("$classDirective", "active")],
        false,
    );
}

#[test]
fn bind_directive_with_expression() {
    let source = "<input bind:value={name}>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_start_tag(
        source,
        &tokens[0],
        "input",
        vec![("$bindDirective", "name")],
        true,
    );
}

#[test]
fn bind_directive_shorthand() {
    let source = "<input bind:value>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert_start_tag(
        source,
        &tokens[0],
        "input",
        vec![("$bindDirective", "value")],
        true,
    );
}

#[test]
fn attribute_concatenation() {
    let source = r#"<div title="hello {name} world">"#;
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
    if let TokenType::StartTag(ref st) = tokens[0].token_type {
        assert_eq!(st.name_span.source_text(source), "div");
        assert_eq!(st.attributes.len(), 1);
        if let Attribute::HTMLAttribute(ref attr) = st.attributes[0] {
            assert_eq!(attr.name_span.source_text(source), "title");
            assert!(matches!(attr.value, AttributeValue::Concatenation(_)));
        } else {
            panic!("Expected HTMLAttribute");
        }
    }
}

#[test]
fn spread_attribute() {
    let source = "<div {...props}>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
    if let TokenType::StartTag(ref st) = tokens[0].token_type {
        assert_eq!(st.attributes.len(), 1);
        if let Attribute::ExpressionTag(ref et) = st.attributes[0] {
            assert_eq!(et.expression_span.source_text(source), "...props");
        } else {
            panic!("Expected ExpressionTag for spread");
        }
    }
}

#[test]
fn shorthand_attribute() {
    let source = "<div {value}>";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartTag(_)));
    if let TokenType::StartTag(ref st) = tokens[0].token_type {
        assert_eq!(st.attributes.len(), 1);
        if let Attribute::ExpressionTag(ref et) = st.attributes[0] {
            assert_eq!(et.expression_span.source_text(source), "value");
        } else {
            panic!("Expected ExpressionTag for shorthand");
        }
    }
}

#[test]
fn snippet_tag_tokens() {
    let source = "{#snippet foo(a, b)}content{/snippet}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(
        tokens[0].token_type,
        TokenType::StartSnippetTag(_)
    ));
    if let TokenType::StartSnippetTag(ref st) = tokens[0].token_type {
        assert_eq!(st.expression_span.source_text(source), "foo(a, b)");
    }
    assert!(tokens[1].token_type == TokenType::Text);
    assert!(tokens[2].token_type == TokenType::EndSnippetTag);
}

#[test]
fn snippet_tag_params_with_string_paren() {
    let source = r#"{#snippet foo(a = ")")}content{/snippet}"#;
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(
        tokens[0].token_type,
        TokenType::StartSnippetTag(_)
    ));
    if let TokenType::StartSnippetTag(ref st) = tokens[0].token_type {
        assert_eq!(st.expression_span.source_text(source), r#"foo(a = ")")"#);
    }
    assert!(tokens[1].token_type == TokenType::Text);
    assert!(tokens[2].token_type == TokenType::EndSnippetTag);
}

#[test]
fn snippet_tag_params_with_template_literal() {
    let source = "{#snippet foo(a = `)`)}content{/snippet}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(
        tokens[0].token_type,
        TokenType::StartSnippetTag(_)
    ));
    if let TokenType::StartSnippetTag(ref st) = tokens[0].token_type {
        assert_eq!(st.expression_span.source_text(source), "foo(a = `)`)");
    }
    assert!(tokens[1].token_type == TokenType::Text);
    assert!(tokens[2].token_type == TokenType::EndSnippetTag);
}

#[test]
fn snippet_tag_name_with_underscore() {
    let source = "{#snippet extra_element()}content{/snippet}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(
        tokens[0].token_type,
        TokenType::StartSnippetTag(_)
    ));
    if let TokenType::StartSnippetTag(ref st) = tokens[0].token_type {
        assert_eq!(st.expression_span.source_text(source), "extra_element()");
    }
    assert!(tokens[1].token_type == TokenType::Text);
    assert!(tokens[2].token_type == TokenType::EndSnippetTag);
}

#[test]
fn snippet_tag_name_with_dollar() {
    let source = "{#snippet $foo()}content{/snippet}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(
        tokens[0].token_type,
        TokenType::StartSnippetTag(_)
    ));
    if let TokenType::StartSnippetTag(ref st) = tokens[0].token_type {
        assert_eq!(st.expression_span.source_text(source), "$foo()");
    }
}

#[test]
fn render_tag_tokens() {
    let source = "{@render foo(x, y)}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::RenderTag(_)));
    if let TokenType::RenderTag(ref rt) = tokens[0].token_type {
        assert_eq!(rt.expression_span.source_text(source), "foo(x, y)");
    }
}

#[test]
fn html_tag_tokens() {
    let source = "{@html content}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::HtmlTag(_)));
    if let TokenType::HtmlTag(ref ht) = tokens[0].token_type {
        assert_eq!(ht.expression_span.source_text(source), "content");
    }
}

#[test]
fn html_tag_with_nested_template_literal() {
    let source = "{@html `a ${foo({ bar: `}` })}`}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::HtmlTag(_)));
    if let TokenType::HtmlTag(ref ht) = tokens[0].token_type {
        assert_eq!(
            ht.expression_span.source_text(source),
            "`a ${foo({ bar: `}` })}`"
        );
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn await_tag_with_regex_literal() {
    let source = r"{#await /}/.test(x) then value}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartAwaitTag(_)));
    if let TokenType::StartAwaitTag(ref tag) = tokens[0].token_type {
        assert_eq!(tag.expression_span.source_text(source), "/}/.test(x)");
        assert_eq!(
            tag.value_span
                .expect("expected value binding")
                .source_text(source),
            "value"
        );
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn await_tag_with_block_comment() {
    let source = "{#await a /* } */ + b then value}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartAwaitTag(_)));
    if let TokenType::StartAwaitTag(ref tag) = tokens[0].token_type {
        assert_eq!(tag.expression_span.source_text(source), "a /* } */ + b");
        assert_eq!(
            tag.value_span
                .expect("expected value binding")
                .source_text(source),
            "value"
        );
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn await_tag_with_nested_template_literal() {
    let source = "{#await `x ${foo({ bar: `}` })}` then value}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartAwaitTag(_)));
    if let TokenType::StartAwaitTag(ref tag) = tokens[0].token_type {
        assert_eq!(
            tag.expression_span.source_text(source),
            "`x ${foo({ bar: `}` })}`"
        );
        assert_eq!(
            tag.value_span
                .expect("expected value binding")
                .source_text(source),
            "value"
        );
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn await_tag_then_inside_string_does_not_terminate_expression() {
    let source = r#"{#await foo(" then ") + bar then value}"#;
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartAwaitTag(_)));
    if let TokenType::StartAwaitTag(ref tag) = tokens[0].token_type {
        assert_eq!(
            tag.expression_span.source_text(source),
            r#"foo(" then ") + bar"#
        );
        assert_eq!(
            tag.value_span
                .expect("expected value binding")
                .source_text(source),
            "value"
        );
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn await_tag_then_binding_with_string_containing_closing_brace() {
    let source = r#"{#await promise then { value = "}" }}"#;
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartAwaitTag(_)));
    if let TokenType::StartAwaitTag(ref tag) = tokens[0].token_type {
        assert_eq!(tag.expression_span.source_text(source), "promise");
        assert_eq!(
            tag.value_span
                .expect("expected value binding")
                .source_text(source),
            r#"{ value = "}" }"#
        );
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn await_tag_catch_binding_with_template_literal() {
    let source = "{#await promise catch { value = `}` }}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::StartAwaitTag(_)));
    if let TokenType::StartAwaitTag(ref tag) = tokens[0].token_type {
        assert_eq!(tag.expression_span.source_text(source), "promise");
        assert_eq!(
            tag.error_span
                .expect("expected error binding")
                .source_text(source),
            "{ value = `}` }"
        );
    }
    assert!(tokens[1].token_type == TokenType::EOF);
}

#[test]
fn const_tag_tokens() {
    let source = "{@const doubled = item * 2}";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().0;
    assert!(matches!(tokens[0].token_type, TokenType::ConstTag(_)));
    if let TokenType::ConstTag(ref ct) = tokens[0].token_type {
        assert_eq!(ct.expression_span.source_text(source), "doubled = item * 2");
    }
}

#[test]
fn recovery_unterminated_start_tag_bare() {
    let source = "<div";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(tokens[0].token_type == TokenType::EOF);
    assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[test]
fn recovery_unterminated_start_tag_with_bool_attr() {
    let source = "<div class";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(tokens[0].token_type == TokenType::EOF);
    assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[test]
fn recovery_unterminated_start_tag_with_partial_attr() {
    let mut scanner = Scanner::new("<div class=");
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(tokens.last().expect("test invariant").token_type == TokenType::EOF);
    assert!(!diagnostics.is_empty());
}

#[test]
fn recovery_unclosed_script_tag() {
    let source = "<script>code";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(matches!(tokens[0].token_type, TokenType::ScriptTag(_)));
    if let TokenType::ScriptTag(ref st) = tokens[0].token_type {
        assert_eq!(st.content_span.source_text(source), "code");
    }
    assert!(tokens.last().expect("test invariant").token_type == TokenType::EOF);
    assert_has_diagnostic(
        &diagnostics,
        DiagnosticKind::ElementUnclosed {
            name: "script".to_string(),
        },
    );
}

#[test]
fn recovery_unclosed_style_tag() {
    let source = "<style>.foo{}";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(matches!(tokens[0].token_type, TokenType::StyleTag(_)));
    if let TokenType::StyleTag(ref st) = tokens[0].token_type {
        assert_eq!(st.content_span.source_text(source), ".foo{}");
    }
    assert!(tokens.last().expect("test invariant").token_type == TokenType::EOF);
    assert_has_diagnostic(
        &diagnostics,
        DiagnosticKind::ExpectedToken {
            token: "</style".into(),
        },
    );
}

#[test]
fn recovery_unclosed_comment() {
    let mut scanner = Scanner::new("<!-- text");
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(matches!(tokens[0].token_type, TokenType::Comment { .. }));
    assert!(tokens.last().expect("test invariant").token_type == TokenType::EOF);
    assert_has_diagnostic(
        &diagnostics,
        DiagnosticKind::ExpectedToken {
            token: "-->".into(),
        },
    );
}

#[test]
fn recovery_unclosed_interpolation() {
    let source = "{name";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(matches!(tokens[0].token_type, TokenType::Interpolation(_)));
    if let TokenType::Interpolation(ref et) = tokens[0].token_type {
        assert_eq!(et.expression_span.source_text(source), "name");
    }
    assert!(tokens.last().expect("test invariant").token_type == TokenType::EOF);
    assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[test]
fn recovery_unclosed_if_tag() {
    let source = "{#if cond";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(matches!(tokens[0].token_type, TokenType::StartIfTag(_)));
    if let TokenType::StartIfTag(ref st) = tokens[0].token_type {
        assert_eq!(st.expression_span.source_text(source), "cond");
    }
    assert!(tokens.last().expect("test invariant").token_type == TokenType::EOF);
    assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[test]
fn recovery_attribute_concatenation_eof() {
    let mut scanner = Scanner::new(r#"<div class="foo"#);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(tokens.last().expect("test invariant").token_type == TokenType::EOF);
    assert!(!diagnostics.is_empty());
}

#[test]
fn recovery_start_tag_then_more_content() {
    let mut scanner = Scanner::new("<div<p>hello</p>");
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(tokens.last().expect("test invariant").token_type == TokenType::EOF);
    assert_has_diagnostic(&diagnostics, DiagnosticKind::InvalidTagName);
}

#[test]
fn attribute_name_with_percent_and_digits() {
    let source = "<Child 0={0} ysc%%gibberish={1} />";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(
        diagnostics.is_empty(),
        "unexpected diagnostics: {diagnostics:?}"
    );

    assert_start_tag(
        source,
        &tokens[0],
        "Child",
        vec![("0", "0"), ("ysc%%gibberish", "1")],
        true,
    );
}

#[test]
fn attribute_name_stops_at_pipe_modifier() {
    let source = "<div on:click|once={h} />";
    let mut scanner = Scanner::new(source);
    let (tokens, diagnostics) = scanner.scan_tokens();
    assert!(
        diagnostics.is_empty(),
        "unexpected diagnostics: {diagnostics:?}"
    );

    let TokenType::StartTag(start_tag) = &tokens[0].token_type else {
        panic!("expected StartTag");
    };
    assert_eq!(start_tag.attributes.len(), 1);
    let Attribute::OnDirectiveLegacy(od) = &start_tag.attributes[0] else {
        panic!("expected OnDirectiveLegacy");
    };
    assert_eq!(od.name_span.source_text(source), "click");
}

#[test]
fn textarea_static_content_is_single_text() {
    let source = "<textarea>just text</textarea>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_rcdata_tokens(
        source,
        &tokens,
        &[
            ("start", "textarea"),
            ("text", "just text"),
            ("end", "textarea"),
            ("eof", ""),
        ],
    );
}

#[test]
fn textarea_interpolation_only() {
    let source = "<textarea>{value}</textarea>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_rcdata_tokens(
        source,
        &tokens,
        &[
            ("start", "textarea"),
            ("interpolation", "value"),
            ("end", "textarea"),
            ("eof", ""),
        ],
    );
}

#[test]
fn textarea_treats_nested_markup_as_text() {
    let source = "<textarea><b>bold</b></textarea>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_rcdata_tokens(
        source,
        &tokens,
        &[
            ("start", "textarea"),
            ("text", "<b>bold</b>"),
            ("end", "textarea"),
            ("eof", ""),
        ],
    );
}

#[test]
fn textarea_mixed_text_markup_and_interpolation() {
    let source = "<textarea>a <p>{foo}</p> b</textarea>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_rcdata_tokens(
        source,
        &tokens,
        &[
            ("start", "textarea"),
            ("text", "a <p>"),
            ("interpolation", "foo"),
            ("text", "</p> b"),
            ("end", "textarea"),
            ("eof", ""),
        ],
    );
}

#[test]
fn textarea_close_tag_allows_whitespace_before_gt() {
    let source = "<textarea>x</textarea\n>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_rcdata_tokens(
        source,
        &tokens,
        &[
            ("start", "textarea"),
            ("text", "x"),
            ("end", "textarea"),
            ("eof", ""),
        ],
    );
}

#[test]
fn textarea_close_tag_is_case_insensitive() {
    let source = "<textarea>x</TEXTAREA>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_rcdata_tokens(
        source,
        &tokens,
        &[
            ("start", "textarea"),
            ("text", "x"),
            ("end", "TEXTAREA"),
            ("eof", ""),
        ],
    );
}

#[test]
fn textarea_close_tag_allows_attributes_before_gt() {
    let source = "<textarea>x</textarea foo=\"y\">";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_rcdata_tokens(
        source,
        &tokens,
        &[
            ("start", "textarea"),
            ("text", "x"),
            ("end", "textarea"),
            ("eof", ""),
        ],
    );
}

#[test]
fn textarea_partial_close_name_is_text() {
    let source = "<textarea>a</textareax>b</textarea>";
    let tokens = Scanner::new(source).scan_tokens().0;

    assert_rcdata_tokens(
        source,
        &tokens,
        &[
            ("start", "textarea"),
            ("text", "a</textareax>b"),
            ("end", "textarea"),
            ("eof", ""),
        ],
    );
}

#[test]
fn textarea_unterminated_reports_eof_diagnostic() {
    let (_, diagnostics) = Scanner::new("<textarea>abc").scan_tokens();

    assert_has_diagnostic(&diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[track_caller]
fn assert_rcdata_tokens(source: &str, tokens: &[Token], expected: &[(&str, &str)]) {
    assert_eq!(
        tokens.len(),
        expected.len(),
        "token count: expected {}, got {} ({:?})",
        expected.len(),
        tokens.len(),
        tokens.iter().map(|t| &t.token_type).collect::<Vec<_>>()
    );

    for (index, (expected_kind, expected_text)) in expected.iter().enumerate() {
        let token = &tokens[index];
        let (kind, text): (&str, String) = match &token.token_type {
            TokenType::StartTag(t) => ("start", t.name_span.source_text(source).to_string()),
            TokenType::EndTag(t) => ("end", t.name_span.source_text(source).to_string()),
            TokenType::Text => ("text", token.span.source_text(source).to_string()),
            TokenType::Interpolation(et) => (
                "interpolation",
                et.expression_span.source_text(source).to_string(),
            ),
            TokenType::EOF => ("eof", String::new()),
            other => ("other", format!("{other:?}")),
        };

        assert_eq!(
            kind, *expected_kind,
            "token {index} kind: expected {expected_kind:?}, got {kind:?}"
        );
        assert_eq!(
            text, *expected_text,
            "token {index} text: expected {expected_text:?}, got {text:?}"
        );
    }
}

#[test]
fn textarea_unclosed_interpolation_recovers() {
    let (tokens, diagnostics) = Scanner::new("<textarea>{foo").scan_tokens();

    assert_recovers(&tokens, &diagnostics, DiagnosticKind::UnexpectedEndOfFile);
}

#[track_caller]
fn assert_recovers(tokens: &[Token], diagnostics: &[Diagnostic], err_kind: DiagnosticKind) {
    assert!(
        diagnostics.iter().any(|d| d.kind == err_kind),
        "expected diagnostic {err_kind:?}, got: {diagnostics:?}"
    );
    assert_eq!(
        tokens.last().map(|t| &t.token_type),
        Some(&TokenType::EOF),
        "recovery must still terminate with EOF, got: {:?}",
        tokens.last().map(|t| &t.token_type)
    );
}

fn scan_js_comment_starts(source: &str) -> Vec<u32> {
    let mut scanner = Scanner::new(source);
    let _ = scanner.scan_tokens();
    let mut starts = scanner.take_js_comment_expr_starts();
    starts.sort_unstable();
    starts
}

#[track_caller]
fn assert_comment_expr_starts(source: &str, expected_prefixes: &[&str]) {
    let starts = scan_js_comment_starts(source);
    assert_eq!(
        starts.len(),
        expected_prefixes.len(),
        "comment-bearing expr count: expected {} ({expected_prefixes:?}), got {} ({starts:?})",
        expected_prefixes.len(),
        starts.len(),
    );
    for (&start, prefix) in starts.iter().zip(expected_prefixes) {
        let slice = &source[start as usize..];
        assert!(
            slice.starts_with(prefix),
            "expr recorded at {start}: expected to start with {prefix:?}, got {:?}",
            &slice[..prefix.len().min(slice.len())],
        );
    }
}

#[test]
fn records_interpolation_line_comment() {
    assert_comment_expr_starts("{ a // }\n + b }", &["a // }"]);
}

#[test]
fn records_interpolation_block_comment() {
    assert_comment_expr_starts("{ a /* } */ + b }", &["a /* } */"]);
}

#[test]
fn records_event_handler_comment() {
    let source = "<button onclick={() => {\n\t// svelte-ignore x\n\tfoo.bar = 1;\n}}></button>";
    assert_comment_expr_starts(source, &["() =>"]);
}

#[test]
fn ignores_expression_without_comment() {
    assert_comment_expr_starts("{ a + b }", &[]);
}

#[test]
fn records_only_comment_bearing_expression() {
    assert_comment_expr_starts("{ x } { y /* c */ }", &["y /* c */"]);
}

#[test]
fn ignores_comment_marker_inside_string() {
    assert_comment_expr_starts(r#"{ "http://x" }"#, &[]);
}
