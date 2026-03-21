use super::*;

/// Parse a JS expression and return owned analysis info.
///
/// `source` is the raw expression text (e.g., "count + 1").
/// `offset` is the byte offset in the original .svelte file (for Span adjustment).
///
/// OXC allocator is created and destroyed inside this function.
fn analyze_expression(source: &str, offset: u32) -> Result<ExpressionInfo, Diagnostic> {
    let allocator = Allocator::default();
    let parser = OxcParser::new(&allocator, source, SourceType::default());

    let expr = parser
        .parse_expression()
        .map_err(|_| Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32)))?;

    let info = extract_expression_info(&expr, offset);
    // allocator drops here — all OXC data freed
    Ok(info)
}

#[test]
fn analyze_simple_identifier() {
    let info = analyze_expression("count", 0).unwrap();
    assert_eq!(info.kind, ExpressionKind::Identifier(compact("count")));
    assert_eq!(info.references.len(), 1);
    assert_eq!(info.references[0].name, "count");
}

#[test]
fn analyze_binary_expression() {
    let info = analyze_expression("count + 1", 0).unwrap();
    assert_eq!(info.references.len(), 1);
    assert_eq!(info.references[0].name, "count");
}

#[test]
fn analyze_call_expression() {
    let info = analyze_expression("foo(a, b)", 0).unwrap();
    assert!(matches!(info.kind, ExpressionKind::CallExpression { .. }));
    assert_eq!(info.references.len(), 3); // foo, a, b
    assert!(info.has_side_effects);
}

#[test]
fn analyze_assignment() {
    let info = analyze_expression("count = 10", 0).unwrap();
    assert_eq!(info.kind, ExpressionKind::Assignment);
    assert!(info.references.iter().any(|r| r.name == "count" && matches!(r.flags, ReferenceFlags::Write)));
}

#[test]
fn analyze_script_basic() {
    let (info, _scoping) = analyze_script_with_scoping("let count = $state(0); const name = 'test';", 0, false).unwrap();
    assert_eq!(info.declarations.len(), 2);
    assert_eq!(info.declarations[0].name, "count");
    assert_eq!(info.declarations[0].is_rune, Some(RuneKind::State));
    assert_eq!(info.declarations[1].name, "name");
    assert_eq!(info.declarations[1].is_rune, None);
}

#[test]
fn analyze_with_offset() {
    let info = analyze_expression("x", 100).unwrap();
    assert_eq!(info.references[0].span.start, 100);
    assert_eq!(info.references[0].span.end, 101);
}

#[test]
fn parse_const_declaration_simple() {
    let alloc = Allocator::default();
    let source = alloc.alloc_str("doubled = item * 2");
    let (names, refs, _expr) = parse_const_declaration_with_alloc(&alloc, source, 10, false).unwrap();
    assert_eq!(names, vec![compact("doubled")]);
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].name, "item");
    // "const doubled = item * 2" — "item" starts at byte 16 in wrapped string
    // offset adjustment: 10 - 6 = 4, so span = 16 + 4 = 20
    assert_eq!(refs[0].span.start, 20);
}

#[test]
fn parse_const_declaration_destructuring() {
    let alloc = Allocator::default();
    // offset >= 6 required (compensates "const " prefix in wrapping arithmetic)
    let source = alloc.alloc_str("{a, b} = obj");
    let (names, refs, _expr) = parse_const_declaration_with_alloc(&alloc, source, 10, false).unwrap();
    assert_eq!(names, vec![compact("a"), compact("b")]);
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].name, "obj");
}

#[test]
fn parse_const_declaration_multiple_equals() {
    let alloc = Allocator::default();
    let source = alloc.alloc_str("a = b === c");
    let (names, refs, _expr) = parse_const_declaration_with_alloc(&alloc, source, 10, false).unwrap();
    assert_eq!(names, vec![compact("a")]);
    assert_eq!(refs.len(), 2);
    assert!(refs.iter().any(|r| r.name == "b"));
    assert!(refs.iter().any(|r| r.name == "c"));
}

#[test]
fn analyze_script_exports() {
    let (info, _) = analyze_script_with_scoping(
        "export const PI = 3.14; export function greet(name) { return name; }",
        0, false
    ).unwrap();
    assert_eq!(info.exports.len(), 2);
    assert_eq!(info.exports[0].name, "PI");
    assert_eq!(info.exports[1].name, "greet");
    // Declarations are also extracted from exported statements
    assert!(info.declarations.iter().any(|d| d.name == "PI"));
    assert!(info.declarations.iter().any(|d| d.name == "greet"));
}
