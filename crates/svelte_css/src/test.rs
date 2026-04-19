use pretty_assertions::assert_eq;

use crate::ast::*;
use crate::parser::parse;
use crate::printer::Printer;

// ---------------------------------------------------------------------------
// Helper: parse expecting no diagnostics
// ---------------------------------------------------------------------------

fn p(src: &str) -> StyleSheet {
    let (ss, diags) = parse(src);
    if !diags.is_empty() {
        panic!("unexpected diagnostics: {diags:?}");
    }
    ss
}

fn text(span: svelte_span::Span, src: &str) -> &str {
    span.source_text(src)
}

// ---------------------------------------------------------------------------
// Basic rules
// ---------------------------------------------------------------------------

#[test]
fn basic_rule() {
    let src = "p { color: red; }";
    let ss = p(src);

    assert_eq!(ss.children.len(), 1);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };

    // Selector
    assert_eq!(rule.prelude.children.len(), 1);
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 1);
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { name, .. } if name == "p"));

    // Declaration
    assert_eq!(rule.block.children.len(), 1);
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.property, src), "color");
    assert_eq!(text(decl.value, src), "red");
}

#[test]
fn multiple_declarations() {
    let src = "div { color: red; font-size: 16px; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    assert_eq!(rule.block.children.len(), 2);
}

#[test]
fn multiple_selectors() {
    let src = "h1, h2 { font-size: 2em; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    assert_eq!(rule.prelude.children.len(), 2);
}

// ---------------------------------------------------------------------------
// Combinators
// ---------------------------------------------------------------------------

#[test]
fn child_combinator() {
    let src = "div > p { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let complex = &rule.prelude.children[0];
    assert_eq!(complex.children.len(), 2);
    let rel = &complex.children[1];
    assert!(matches!(
        &rel.combinator,
        Some(Combinator {
            kind: CombinatorKind::Child,
            ..
        })
    ));
}

#[test]
fn next_sibling_combinator() {
    let src = "a + b { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let complex = &rule.prelude.children[0];
    assert_eq!(complex.children.len(), 2);
    assert!(matches!(
        &complex.children[1].combinator,
        Some(Combinator {
            kind: CombinatorKind::NextSibling,
            ..
        })
    ));
}

#[test]
fn subsequent_sibling_combinator() {
    let src = "a ~ b { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    assert!(matches!(
        &rule.prelude.children[0].children[1].combinator,
        Some(Combinator {
            kind: CombinatorKind::SubsequentSibling,
            ..
        })
    ));
}

#[test]
fn descendant_combinator() {
    let src = "div p { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let complex = &rule.prelude.children[0];
    assert_eq!(complex.children.len(), 2);
    assert!(matches!(
        &complex.children[1].combinator,
        Some(Combinator {
            kind: CombinatorKind::Descendant,
            ..
        })
    ));
}

// ---------------------------------------------------------------------------
// Selectors: pseudo-classes
// ---------------------------------------------------------------------------

#[test]
fn pseudo_class_hover() {
    let src = "a:hover { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 2);
    let SimpleSelector::PseudoClass(pc) = &rel.selectors[1] else {
        panic!("expected pseudo-class");
    };
    assert_eq!(pc.name.as_str(), "hover");
    assert!(pc.args.is_none());
}

#[test]
fn pseudo_class_not() {
    let src = "div:not(.hidden) { display: block; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::PseudoClass(pc) = &rel.selectors[1] else {
        panic!("expected pseudo-class");
    };
    assert_eq!(pc.name.as_str(), "not");
    assert!(pc.args.is_some());
    let args = pc.args.as_ref().expect("test invariant");
    assert_eq!(args.children.len(), 1);
    assert!(matches!(
        &args.children[0].children[0].selectors[0],
        SimpleSelector::Class { .. }
    ));
}

// ---------------------------------------------------------------------------
// :global
// ---------------------------------------------------------------------------

#[test]
fn global_function() {
    let src = ":global(.foo) { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(
        matches!(
            &rel.selectors[0],
            SimpleSelector::Global { args: Some(_), .. }
        ),
        "expected Global with args"
    );
}

#[test]
fn global_block() {
    let src = ":global { p { color: red; } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(
        matches!(&rel.selectors[0], SimpleSelector::Global { args: None, .. }),
        "expected Global without args"
    );

    assert_eq!(rule.block.children.len(), 1);
    assert!(matches!(
        &rule.block.children[0],
        BlockChild::Rule(Rule::Style(_))
    ));
}

#[test]
fn is_lone_global_block_detection() {
    // Lone :global { } → true
    let ss = p(":global { p { color: red; } }");
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    assert!(rule.is_lone_global_block());

    // Functional :global(.foo) → false
    let ss = p(":global(.foo) { color: red; }");
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    assert!(!rule.is_lone_global_block());

    // Regular rule → false
    let ss = p("p { color: red; }");
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    assert!(!rule.is_lone_global_block());
}

#[test]
fn global_in_compound_selector() {
    // p:global(.active) — type selector followed by :global()
    let src = "p:global(.active) { font-weight: bold; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 2);
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { name, .. } if name == "p"));
    assert!(matches!(
        &rel.selectors[1],
        SimpleSelector::Global { args: Some(_), .. }
    ));

    // Verify the inner selector list contains .active
    let SimpleSelector::Global {
        args: Some(args), ..
    } = &rel.selectors[1]
    else {
        panic!("expected Global with args");
    };
    let inner_rel = &args.children[0].children[0];
    assert!(
        matches!(&inner_rel.selectors[0], SimpleSelector::Class { name, .. } if name == "active")
    );
}

#[test]
fn global_with_complex_inner_selector() {
    // :global(h2.featured) — multiple simple selectors inside :global()
    let src = ":global(h2.featured) { font-style: italic; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 1);
    let SimpleSelector::Global {
        args: Some(args), ..
    } = &rel.selectors[0]
    else {
        panic!("expected Global with args");
    };
    let inner_rel = &args.children[0].children[0];
    assert_eq!(inner_rel.selectors.len(), 2);
    assert!(matches!(&inner_rel.selectors[0], SimpleSelector::Type { name, .. } if name == "h2"));
    assert!(
        matches!(&inner_rel.selectors[1], SimpleSelector::Class { name, .. } if name == "featured")
    );
}

#[test]
fn global_multiple_in_descendant() {
    // :global(.wrapper) :global(.item) — two :global() with descendant combinator
    let src = ":global(.wrapper) :global(.item) { display: flex; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let complex = &rule.prelude.children[0];
    assert_eq!(complex.children.len(), 2);

    let rel0 = &complex.children[0];
    assert!(matches!(
        &rel0.selectors[0],
        SimpleSelector::Global { args: Some(_), .. }
    ));

    let rel1 = &complex.children[1];
    assert!(matches!(
        &rel1.selectors[0],
        SimpleSelector::Global { args: Some(_), .. }
    ));
}

#[test]
fn non_global_pseudo_class_stays_pseudo_class() {
    // :hover should remain PseudoClass, not Global
    let src = "a:hover { color: blue; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 2);
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { name, .. } if name == "a"));
    assert!(
        matches!(&rel.selectors[1], SimpleSelector::PseudoClass(pc) if pc.name == "hover"),
        "expected PseudoClass(:hover), not Global"
    );
}

// ---------------------------------------------------------------------------
// Attribute selectors
// ---------------------------------------------------------------------------

#[test]
fn attribute_presence() {
    let src = "[data-x] { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::Attribute(attr) = &rel.selectors[0] else {
        panic!("expected attribute selector");
    };
    assert_eq!(attr.name.as_str(), "data-x");
    assert!(attr.matcher.is_none());
    assert!(attr.value.is_none());
}

#[test]
fn attribute_with_value() {
    let src = r#"[href^="https"] { color: blue; }"#;
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::Attribute(attr) = &rel.selectors[0] else {
        panic!("expected attribute selector");
    };
    assert_eq!(attr.name.as_str(), "href");
    assert_eq!(text(attr.matcher.expect("test invariant"), src), "^=");
    assert_eq!(text(attr.value.expect("test invariant"), src), "https");
}

#[test]
fn attribute_with_flags() {
    let src = r#"[class~="foo" i] { color: red; }"#;
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::Attribute(attr) = &rel.selectors[0] else {
        panic!("expected attribute selector");
    };
    assert_eq!(attr.name.as_str(), "class");
    assert!(attr.flags.is_some());
    assert_eq!(text(attr.flags.expect("test invariant"), src), "i");
}

// ---------------------------------------------------------------------------
// At-rules
// ---------------------------------------------------------------------------

#[test]
fn at_media() {
    let src = "@media (min-width: 768px) { p { color: red; } }";
    let ss = p(src);
    assert_eq!(ss.children.len(), 1);
    let StyleSheetChild::Rule(Rule::AtRule(at)) = &ss.children[0] else {
        panic!("expected at-rule");
    };
    assert_eq!(at.name.as_str(), "media");
    assert!(at.block.is_some());
}

#[test]
fn at_import() {
    let src = "@import 'reset.css';";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::AtRule(at)) = &ss.children[0] else {
        panic!("expected at-rule");
    };
    assert_eq!(at.name.as_str(), "import");
    assert!(at.block.is_none());
}

#[test]
fn at_keyframes() {
    let src = "@keyframes fade { from { opacity: 1; } to { opacity: 0; } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::AtRule(at)) = &ss.children[0] else {
        panic!("expected at-rule");
    };
    assert_eq!(at.name.as_str(), "keyframes");
    let block = at.block.as_ref().expect("test invariant");
    assert_eq!(block.children.len(), 2);
}

// ---------------------------------------------------------------------------
// Nested rules
// ---------------------------------------------------------------------------

#[test]
fn nested_rule() {
    let src = ".parent { .child { color: red; } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    assert_eq!(rule.block.children.len(), 1);
    let BlockChild::Rule(Rule::Style(nested)) = &rule.block.children[0] else {
        panic!("expected nested style rule");
    };
    let rel = &nested.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Class { .. }));
}

// ---------------------------------------------------------------------------
// Declarations
// ---------------------------------------------------------------------------

#[test]
fn complex_value() {
    let src = r#"div { background: url("test.png") no-repeat; }"#;
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.property, src), "background");
    assert_eq!(text(decl.value, src), r#"url("test.png") no-repeat"#);
}

#[test]
fn custom_property_empty() {
    let src = "div { --my-var: ; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.property, src), "--my-var");
}

#[test]
fn last_declaration_no_semicolon() {
    let src = "p { color: red }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.value, src), "red");
}

// ---------------------------------------------------------------------------
// Comments
// ---------------------------------------------------------------------------

#[test]
fn top_level_comment() {
    let src = "/* hello */ p { color: red; }";
    let ss = p(src);
    assert_eq!(ss.children.len(), 2);
    assert!(
        matches!(&ss.children[0], StyleSheetChild::Comment(c) if text(c.span, src) == "/* hello */")
    );
}

#[test]
fn comment_in_block() {
    let src = "p { /* a comment */ color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    assert_eq!(rule.block.children.len(), 2);
    assert!(matches!(&rule.block.children[0], BlockChild::Comment(_)));
}

// ---------------------------------------------------------------------------
// Pseudo-elements
// ---------------------------------------------------------------------------

#[test]
fn pseudo_element() {
    let src = "p::before { content: ''; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 2);
    let SimpleSelector::PseudoElement(pe) = &rel.selectors[1] else {
        panic!("expected pseudo-element");
    };
    assert_eq!(pe.name.as_str(), "before");
}

// ---------------------------------------------------------------------------
// Nesting selector
// ---------------------------------------------------------------------------

#[test]
fn nesting_selector() {
    let src = ".parent { & .child { color: red; } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Rule(Rule::Style(nested)) = &rule.block.children[0] else {
        panic!("expected nested rule");
    };
    let rel = &nested.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Nesting(_)));
}

// ---------------------------------------------------------------------------
// Id selector
// ---------------------------------------------------------------------------

#[test]
fn id_selector() {
    let src = "#main { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Id { name, .. } if name == "main"));
}

// ---------------------------------------------------------------------------
// Universal selector
// ---------------------------------------------------------------------------

#[test]
fn universal_selector() {
    let src = "* { margin: 0; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { name, .. } if name == "*"));
}

// ---------------------------------------------------------------------------
// Printer roundtrip
// ---------------------------------------------------------------------------

#[test]
fn printer_basic() {
    let src = "p { color: red; }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, "p {\n  color: red;\n}\n");
}

#[test]
fn printer_multiple_rules() {
    let src = "h1 { color: red; } h2 { color: blue; }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(
        output,
        "h1 {\n  color: red;\n}\n\nh2 {\n  color: blue;\n}\n"
    );
}

#[test]
fn printer_at_rule() {
    let src = "@media (min-width: 768px) { p { color: red; } }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(
        output,
        "@media (min-width: 768px) {\n  p {\n    color: red;\n  }\n}\n"
    );
}

#[test]
fn printer_nested_rule() {
    let src = ".parent { .child { color: red; } }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, ".parent {\n  .child {\n    color: red;\n  }\n}\n");
}

#[test]
fn printer_comment() {
    let src = "/* hello */ p { color: red; }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, "/* hello */\n\np {\n  color: red;\n}\n");
}

#[test]
fn printer_combinators() {
    let src = "div > p { color: red; }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, "div > p {\n  color: red;\n}\n");
}

#[test]
fn printer_global_function() {
    let src = ":global(.foo) { color: red; }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, ":global(.foo) {\n  color: red;\n}\n");
}

#[test]
fn printer_global_block() {
    let src = ":global { p { color: red; } }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, ":global {\n  p {\n    color: red;\n  }\n}\n");
}

#[test]
fn printer_global_in_compound() {
    let src = "p:global(.active) { font-weight: bold; }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, "p:global(.active) {\n  font-weight: bold;\n}\n");
}

#[test]
fn printer_multiple_globals_descendant() {
    let src = ":global(.wrapper) :global(.item) { display: flex; }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(
        output,
        ":global(.wrapper) :global(.item) {\n  display: flex;\n}\n"
    );
}

// ---------------------------------------------------------------------------
// Multiple rules in stylesheet
// ---------------------------------------------------------------------------

#[test]
fn multiple_rules() {
    let src = "p { color: red; } div { margin: 0; } span { padding: 0; }";
    let ss = p(src);
    assert_eq!(ss.children.len(), 3);
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn empty_stylesheet() {
    let src = "";
    let ss = p(src);
    assert!(ss.children.is_empty());
}

#[test]
fn whitespace_only() {
    let src = "   \n\t  ";
    let ss = p(src);
    assert!(ss.children.is_empty());
}

#[test]
fn declaration_with_semicolons_in_value() {
    let src = "div { grid-template: 'a' 1fr / auto; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.property, src), "grid-template");
}

// ---------------------------------------------------------------------------
// Error recovery
// ---------------------------------------------------------------------------

#[test]
fn recovery_bad_selector_skips_rule() {
    let src = "!invalid { color: red; } p { color: blue; }";
    let (ss, diags) = parse(src);
    assert!(!diags.is_empty(), "expected diagnostics for bad selector");

    // Error node should be present for the skipped rule
    let has_error = ss
        .children
        .iter()
        .any(|child| matches!(child, StyleSheetChild::Error(_)));
    assert!(has_error, "expected Error node for bad rule");

    // The valid rule after the bad one should be present
    let has_p_rule = ss.children.iter().any(|child| {
        matches!(child, StyleSheetChild::Rule(Rule::Style(rule))
            if rule.prelude.children.first().is_some_and(|c|
                c.children.first().is_some_and(|r|
                    matches!(&r.selectors[..], [SimpleSelector::Type { name, .. }] if name == "p")
                )
            )
        )
    });
    assert!(has_p_rule, "valid rule after bad rule should be parsed");
}

#[test]
fn recovery_bad_declaration_continues_block() {
    let src = "p { color; font-size: 16px; }";
    let (ss, diags) = parse(src);
    assert!(!diags.is_empty(), "expected diagnostic for bad declaration");
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };

    // Error node for the bad declaration
    let has_error = rule
        .block
        .children
        .iter()
        .any(|child| matches!(child, BlockChild::Error(_)));
    assert!(has_error, "expected Error node for bad declaration");

    // The valid declaration should be present
    let has_font_size = rule.block.children.iter().any(|child| {
        matches!(child, BlockChild::Declaration(d) if d.property.source_text(src) == "font-size")
    });
    assert!(
        has_font_size,
        "valid declaration after bad one should be parsed"
    );
}

#[test]
fn recovery_unclosed_block() {
    // Unclosed block — should produce diagnostic but not panic
    let src = "p { color: red;";
    let (ss, diags) = parse(src);
    assert!(!diags.is_empty(), "expected diagnostic for unclosed block");
    // Should still produce the rule with what it could parse
    assert!(!ss.children.is_empty());
}

#[test]
fn recovery_multiple_errors() {
    // Multiple errors — parser should recover from each
    let src = "!bad { x: 1; } p { color: red; } !worse { y: 2; }";
    let (ss, diags) = parse(src);
    assert!(
        diags.len() >= 2,
        "expected at least 2 diagnostics, got {}",
        diags.len()
    );
    // The valid middle rule should survive
    let has_p_rule = ss.children.iter().any(|child| {
        matches!(child, StyleSheetChild::Rule(Rule::Style(rule))
            if rule.prelude.children.first().is_some_and(|c|
                c.children.first().is_some_and(|r|
                    matches!(&r.selectors[..], [SimpleSelector::Type { name, .. }] if name == "p")
                )
            )
        )
    });
    assert!(has_p_rule, "valid rule between bad rules should be parsed");
}

#[test]
fn recovery_empty_declaration_value() {
    let src = "p { color: ; font-size: 16px; }";
    let (ss, diags) = parse(src);
    assert!(!diags.is_empty(), "expected diagnostic for empty value");
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };

    // Error node for the empty-value declaration
    let has_error = rule
        .block
        .children
        .iter()
        .any(|child| matches!(child, BlockChild::Error(_)));
    assert!(has_error, "expected Error node for empty value declaration");

    // font-size declaration should still be parsed
    let has_font_size = rule.block.children.iter().any(|child| {
        matches!(child, BlockChild::Declaration(d) if d.property.source_text(src) == "font-size")
    });
    assert!(has_font_size);
}

#[test]
fn error_node_span_covers_skipped_content() {
    let src = "!bad { x: 1; } p { color: red; }";
    let (ss, _) = parse(src);
    let StyleSheetChild::Error(span) = &ss.children[0] else {
        panic!("expected Error node first");
    };
    let error_text = span.source_text(src);
    // Should cover the entire skipped rule including its block
    assert!(
        error_text.contains("!bad"),
        "error span should cover '!bad', got: {error_text:?}"
    );
}

// ---------------------------------------------------------------------------
// Regression tests for specific bugs
// ---------------------------------------------------------------------------

#[test]
fn nested_pseudo_class_rule() {
    // `a:hover { color: red; }` inside a block must be parsed as a nested rule,
    // not as a declaration with property `a` and value `hover { color: red; }`.
    let src = ".parent { a:hover { color: red; } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Rule(Rule::Style(nested)) = &rule.block.children[0] else {
        panic!(
            "expected nested style rule, got: {:?}",
            rule.block.children[0]
        );
    };
    // The nested rule should have `a:hover` as selector
    let rel = &nested.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { name, .. } if name == "a"));
    assert!(matches!(&rel.selectors[1], SimpleSelector::PseudoClass(pc) if pc.name == "hover"));
}

#[test]
fn nested_declaration_with_colon_value() {
    // Regular declarations inside a block still work
    let src = ".parent { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.property, src), "color");
    assert_eq!(text(decl.value, src), "red");
}

#[test]
fn no_infinite_loop_on_unexpected_char() {
    // `)` in selector position should not cause infinite loop
    let src = ") { color: red; } p { color: blue; }";
    let (ss, diags) = parse(src);
    assert!(!diags.is_empty());
    // Should recover and parse the valid rule
    let has_p_rule = ss.children.iter().any(|child| {
        matches!(child, StyleSheetChild::Rule(Rule::Style(rule))
            if rule.prelude.children.first().is_some_and(|c|
                c.children.first().is_some_and(|r|
                    matches!(&r.selectors[..], [SimpleSelector::Type { name, .. }] if name == "p")
                )
            )
        )
    });
    assert!(
        has_p_rule,
        "valid rule after unexpected char should be parsed"
    );
}

// ---------------------------------------------------------------------------
// CSS escapes in selectors
// ---------------------------------------------------------------------------

#[test]
fn escaped_class_selector() {
    // `\.foo` escapes the dot — treated as class named `foo` with escape
    let src = r".\31 23 { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Class { .. }));
}

#[test]
fn escaped_id_selector() {
    let src = r"#\61 bc { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Id { .. }));
}

// ---------------------------------------------------------------------------
// Namespace selectors
// ---------------------------------------------------------------------------

#[test]
fn namespace_type_selector() {
    let src = "svg|rect { fill: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    // Should parse as Type selector with name "svg" (pipe-separated)
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { .. }));
}

#[test]
fn universal_namespace_selector() {
    let src = "*|div { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { .. }));
}

// ---------------------------------------------------------------------------
// Multiple pseudo-classes
// ---------------------------------------------------------------------------

#[test]
fn multiple_pseudo_classes() {
    let src = "a:hover:focus { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 3);
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { name, .. } if name == "a"));
    assert!(matches!(&rel.selectors[1], SimpleSelector::PseudoClass(pc) if pc.name == "hover"));
    assert!(matches!(&rel.selectors[2], SimpleSelector::PseudoClass(pc) if pc.name == "focus"));
}

#[test]
fn pseudo_class_chain_with_not() {
    let src = "input:not(:disabled):focus { outline: none; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 3);
    assert!(matches!(&rel.selectors[1], SimpleSelector::PseudoClass(pc) if pc.name == "not"));
    assert!(matches!(&rel.selectors[2], SimpleSelector::PseudoClass(pc) if pc.name == "focus"));
}

// ---------------------------------------------------------------------------
// Deeply nested blocks
// ---------------------------------------------------------------------------

#[test]
fn deeply_nested_rules() {
    let src = ".a { .b { .c { color: red; } } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(a)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Rule(Rule::Style(b)) = &a.block.children[0] else {
        panic!("expected nested rule .b");
    };
    let BlockChild::Rule(Rule::Style(c)) = &b.block.children[0] else {
        panic!("expected nested rule .c");
    };
    let BlockChild::Declaration(decl) = &c.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.property, src), "color");
    assert_eq!(text(decl.value, src), "red");
}

// ---------------------------------------------------------------------------
// At-rule inside nested block
// ---------------------------------------------------------------------------

#[test]
fn at_rule_inside_nested_block() {
    let src = ".parent { @media (max-width: 600px) { color: red; } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Rule(Rule::AtRule(at)) = &rule.block.children[0] else {
        panic!("expected nested at-rule");
    };
    assert_eq!(at.name.as_str(), "media");
    assert!(at.block.is_some());
}

// ---------------------------------------------------------------------------
// Pseudo-element with function (::part, ::slotted)
// ---------------------------------------------------------------------------

#[test]
fn pseudo_element_part() {
    let src = "::part(foo) { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::PseudoElement(pe) = &rel.selectors[0] else {
        panic!("expected pseudo-element");
    };
    assert_eq!(pe.name.as_str(), "part");
    // Args should be preserved
    assert!(pe.args.is_some(), "::part(foo) should have args");
    let args = pe.args.as_ref().expect("test invariant");
    assert_eq!(args.children.len(), 1);
}

#[test]
fn pseudo_element_slotted() {
    let src = "::slotted(.foo) { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::PseudoElement(pe) = &rel.selectors[0] else {
        panic!("expected pseudo-element");
    };
    assert_eq!(pe.name.as_str(), "slotted");
    assert!(pe.args.is_some());
}

#[test]
fn pseudo_element_no_args() {
    let src = "p::before { content: ''; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::PseudoElement(pe) = &rel.selectors[1] else {
        panic!("expected pseudo-element");
    };
    assert_eq!(pe.name.as_str(), "before");
    assert!(pe.args.is_none());
}

#[test]
fn printer_pseudo_element_part() {
    let src = "::part(foo) { color: red; }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, "::part(foo) {\n  color: red;\n}\n");
}

// ---------------------------------------------------------------------------
// Data attributes
// ---------------------------------------------------------------------------

#[test]
fn data_attribute_presence() {
    let src = "[data-testid] { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::Attribute(attr) = &rel.selectors[0] else {
        panic!("expected attribute selector");
    };
    assert_eq!(attr.name.as_str(), "data-testid");
    assert!(attr.matcher.is_none());
}

#[test]
fn data_attribute_with_value() {
    let src = r#"[data-foo-bar="baz"] { color: red; }"#;
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::Attribute(attr) = &rel.selectors[0] else {
        panic!("expected attribute selector");
    };
    assert_eq!(attr.name.as_str(), "data-foo-bar");
    assert_eq!(text(attr.value.expect("test invariant"), src), "baz");
}

// ---------------------------------------------------------------------------
// Values with parentheses (var(), calc())
// ---------------------------------------------------------------------------

#[test]
fn value_with_var() {
    let src = "div { color: var(--my-color); }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.value, src), "var(--my-color)");
}

#[test]
fn value_with_nested_parens() {
    let src = "div { width: calc(100% - var(--gap)); }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.value, src), "calc(100% - var(--gap))");
}

// ---------------------------------------------------------------------------
// :nth-child variants
// ---------------------------------------------------------------------------

#[test]
fn nth_child_2n_plus_1() {
    let src = "li:nth-child(2n+1) { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[1], SimpleSelector::PseudoClass(pc) if pc.name == "nth-child"));
}

#[test]
fn nth_child_even() {
    let src = "tr:nth-child(even) { background: gray; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[1], SimpleSelector::PseudoClass(pc) if pc.name == "nth-child"));
}

// ---------------------------------------------------------------------------
// Multiple at-rules
// ---------------------------------------------------------------------------

#[test]
fn multiple_at_rules() {
    let src = "@import 'a.css'; @import 'b.css'; p { color: red; }";
    let ss = p(src);
    assert_eq!(ss.children.len(), 3);
    assert!(matches!(
        &ss.children[0],
        StyleSheetChild::Rule(Rule::AtRule(_))
    ));
    assert!(matches!(
        &ss.children[1],
        StyleSheetChild::Rule(Rule::AtRule(_))
    ));
    assert!(matches!(
        &ss.children[2],
        StyleSheetChild::Rule(Rule::Style(_))
    ));
}

// ---------------------------------------------------------------------------
// Unicode identifiers
// ---------------------------------------------------------------------------

#[test]
fn unicode_class_name() {
    let src = ".café { color: brown; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Class { name, .. } if name == "café"));
}

#[test]
fn unicode_type_selector() {
    let src = "日本語 { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { name, .. } if name == "日本語"));
}

// ---------------------------------------------------------------------------
// Complex selectors
// ---------------------------------------------------------------------------

#[test]
fn compound_selector_type_class_pseudo() {
    let src = "div.active:hover { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert_eq!(rel.selectors.len(), 3);
    assert!(matches!(&rel.selectors[0], SimpleSelector::Type { name, .. } if name == "div"));
    assert!(matches!(&rel.selectors[1], SimpleSelector::Class { name, .. } if name == "active"));
    assert!(matches!(&rel.selectors[2], SimpleSelector::PseudoClass(pc) if pc.name == "hover"));
}

#[test]
fn complex_multi_combinator() {
    let src = "main > article p > span { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let complex = &rule.prelude.children[0];
    assert_eq!(complex.children.len(), 4);
    assert!(matches!(
        complex.children[1].combinator,
        Some(Combinator {
            kind: CombinatorKind::Child,
            ..
        })
    ));
    assert!(matches!(
        complex.children[2].combinator,
        Some(Combinator {
            kind: CombinatorKind::Descendant,
            ..
        })
    ));
    assert!(matches!(
        complex.children[3].combinator,
        Some(Combinator {
            kind: CombinatorKind::Child,
            ..
        })
    ));
}

// ---------------------------------------------------------------------------
// Printer edge cases
// ---------------------------------------------------------------------------

#[test]
fn printer_import() {
    let src = "@import 'reset.css';";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(output, "@import 'reset.css';\n");
}

#[test]
fn printer_nested_at_rule() {
    let src = ".parent { @media (max-width: 600px) { color: red; } }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(
        output,
        ".parent {\n  @media (max-width: 600px) {\n    color: red;\n  }\n}\n"
    );
}

#[test]
fn printer_deeply_nested() {
    let src = ".a { .b { .c { color: red; } } }";
    let ss = p(src);
    let output = Printer::print(&ss, src);
    assert_eq!(
        output,
        ".a {\n  .b {\n    .c {\n      color: red;\n    }\n  }\n}\n"
    );
}

// ---------------------------------------------------------------------------
// Keyframes with percentage selectors
// ---------------------------------------------------------------------------

#[test]
fn keyframes_percentages() {
    let src = "@keyframes slide { 0% { left: 0; } 100% { left: 100px; } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::AtRule(at)) = &ss.children[0] else {
        panic!("expected at-rule");
    };
    assert_eq!(at.name.as_str(), "keyframes");
    let block = at.block.as_ref().expect("test invariant");
    assert_eq!(block.children.len(), 2);
}

// ---------------------------------------------------------------------------
// Custom properties
// ---------------------------------------------------------------------------

#[test]
fn custom_property_with_complex_value() {
    let src = "div { --gradient: linear-gradient(90deg, red, blue); }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.property, src), "--gradient");
    assert_eq!(text(decl.value, src), "linear-gradient(90deg, red, blue)");
}

// ---------------------------------------------------------------------------
// :is() and :where()
// ---------------------------------------------------------------------------

#[test]
fn pseudo_class_is() {
    let src = ":is(h1, h2, h3) { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::PseudoClass(pc) = &rel.selectors[0] else {
        panic!("expected pseudo-class");
    };
    assert_eq!(pc.name.as_str(), "is");
    let args = pc.args.as_ref().expect("test invariant");
    assert_eq!(args.children.len(), 3);
}

#[test]
fn pseudo_class_where() {
    let src = ":where(.a, .b) { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::PseudoClass(pc) = &rel.selectors[0] else {
        panic!("expected pseudo-class");
    };
    assert_eq!(pc.name.as_str(), "where");
    assert!(pc.args.is_some());
}

// ---------------------------------------------------------------------------
// Comments edge cases
// ---------------------------------------------------------------------------

#[test]
fn unclosed_comment_does_not_panic() {
    let src = "/* unterminated comment";
    let (ss, _diags) = parse(src);
    // Should produce a comment node (best effort), not crash
    assert!(!ss.children.is_empty() || ss.children.is_empty()); // just assert no panic
}

#[test]
fn html_style_comment() {
    // HTML-style comments (<!-- -->) are consumed as whitespace, not preserved
    let src = "<!-- comment --> p { color: red; }";
    let ss = p(src);
    // The HTML comment is skipped; only the rule remains
    assert_eq!(ss.children.len(), 1);
    assert!(matches!(
        &ss.children[0],
        StyleSheetChild::Rule(Rule::Style(_))
    ));
}

#[test]
fn comment_between_rules() {
    let src = "p { color: red; } /* separator */ div { margin: 0; }";
    let ss = p(src);
    assert_eq!(ss.children.len(), 3);
    assert!(matches!(
        &ss.children[0],
        StyleSheetChild::Rule(Rule::Style(_))
    ));
    assert!(matches!(&ss.children[1], StyleSheetChild::Comment(_)));
    assert!(matches!(
        &ss.children[2],
        StyleSheetChild::Rule(Rule::Style(_))
    ));
}

// ---------------------------------------------------------------------------
// Values with parens (var/calc edge cases)
// ---------------------------------------------------------------------------

#[test]
fn value_with_var_fallback_containing_braces() {
    // Parens protect braces inside var() from being treated as block delimiters
    let src = "div { content: var(--x, \"{\"); }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let BlockChild::Declaration(decl) = &rule.block.children[0] else {
        panic!("expected declaration");
    };
    assert_eq!(text(decl.property, src), "content");
    assert!(text(decl.value, src).contains("var("));
}

// ---------------------------------------------------------------------------
// :nth-child with "of" syntax
// ---------------------------------------------------------------------------

#[test]
fn nth_child_of_syntax() {
    let src = "li:nth-child(2n+1 of .important) { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    assert!(matches!(&rel.selectors[1], SimpleSelector::PseudoClass(pc) if pc.name == "nth-child"));
}

// ---------------------------------------------------------------------------
// Attribute selector edge cases
// ---------------------------------------------------------------------------

#[test]
fn attribute_unquoted_value() {
    let src = "[type=text] { color: red; }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::Attribute(attr) = &rel.selectors[0] else {
        panic!("expected attribute selector");
    };
    assert_eq!(attr.name.as_str(), "type");
    assert!(attr.matcher.is_some());
    assert_eq!(text(attr.value.expect("test invariant"), src), "text");
}

#[test]
fn attribute_all_matchers() {
    for (op, css) in [
        ("=", "[a=b]"),
        ("~=", "[a~=b]"),
        ("|=", "[a|=b]"),
        ("^=", "[a^=b]"),
        ("$=", "[a$=b]"),
        ("*=", "[a*=b]"),
    ] {
        let full = format!("{css} {{ color: red; }}");
        let ss = p(&full);
        let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
            panic!("expected style rule for {op}");
        };
        let rel = &rule.prelude.children[0].children[0];
        let SimpleSelector::Attribute(attr) = &rel.selectors[0] else {
            panic!("expected attribute selector for {op}");
        };
        assert_eq!(text(attr.matcher.expect("test invariant"), &full), op);
    }
}
