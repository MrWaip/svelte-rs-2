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

fn text<'a>(span: svelte_span::Span, src: &'a str) -> &'a str {
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
    let args = pc.args.as_ref().unwrap();
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
    let SimpleSelector::PseudoClass(pc) = &rel.selectors[0] else {
        panic!("expected pseudo-class");
    };
    assert_eq!(pc.name.as_str(), "global");
    assert!(pc.args.is_some());
}

#[test]
fn global_block() {
    let src = ":global { p { color: red; } }";
    let ss = p(src);
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    let rel = &rule.prelude.children[0].children[0];
    let SimpleSelector::PseudoClass(pc) = &rel.selectors[0] else {
        panic!("expected pseudo-class");
    };
    assert_eq!(pc.name.as_str(), "global");
    assert!(pc.args.is_none());

    assert_eq!(rule.block.children.len(), 1);
    assert!(matches!(&rule.block.children[0], BlockChild::Rule(Rule::Style(_))));
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
    assert_eq!(text(attr.matcher.unwrap(), src), "^=");
    assert_eq!(text(attr.value.unwrap(), src), "https");
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
    assert_eq!(text(attr.flags.unwrap(), src), "i");
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
    let block = at.block.as_ref().unwrap();
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
    assert!(matches!(&ss.children[0], StyleSheetChild::Comment(c) if text(c.span, src) == "/* hello */"));
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
    assert_eq!(output, "h1 {\n  color: red;\n}\n\nh2 {\n  color: blue;\n}\n");
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
    assert_eq!(
        output,
        ".parent {\n  .child {\n    color: red;\n  }\n}\n"
    );
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
    // Invalid selector: `{` after `>` with no following selector.
    // The bad rule should be skipped, the next rule should parse fine.
    let src = "!invalid { color: red; } p { color: blue; }";
    let (ss, diags) = parse(src);
    assert!(!diags.is_empty(), "expected diagnostics for bad selector");
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
    // Missing colon in first declaration — should skip it and parse the next one
    let src = "p { color; font-size: 16px; }";
    let (ss, diags) = parse(src);
    assert!(!diags.is_empty(), "expected diagnostic for bad declaration");
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    // The valid declaration should be present
    let has_font_size = rule.block.children.iter().any(|child| {
        matches!(child, BlockChild::Declaration(d) if d.property.source_text(src) == "font-size")
    });
    assert!(has_font_size, "valid declaration after bad one should be parsed");
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
    assert!(diags.len() >= 2, "expected at least 2 diagnostics, got {}", diags.len());
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
    // Non-custom-property with empty value
    let src = "p { color: ; font-size: 16px; }";
    let (ss, diags) = parse(src);
    assert!(!diags.is_empty(), "expected diagnostic for empty value");
    let StyleSheetChild::Rule(Rule::Style(rule)) = &ss.children[0] else {
        panic!("expected style rule");
    };
    // font-size declaration should still be parsed
    let has_font_size = rule.block.children.iter().any(|child| {
        matches!(child, BlockChild::Declaration(d) if d.property.source_text(src) == "font-size")
    });
    assert!(has_font_size);
}
