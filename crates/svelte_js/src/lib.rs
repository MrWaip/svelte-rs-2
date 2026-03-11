//! OXC facade for JavaScript analysis.
//!
//! All OXC allocator lifetimes are contained within function calls.
//! Only owned data is returned.

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan as _, SourceType};

use svelte_span::Span;
use svelte_diagnostics::Diagnostic;

// ---------------------------------------------------------------------------
// Public types (owned, no lifetime)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ExpressionInfo {
    pub kind: ExpressionKind,
    pub references: Vec<Reference>,
    pub has_side_effects: bool,
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub name: String,
    pub span: Span,
    pub flags: ReferenceFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceFlags {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    Identifier(String),
    Literal,
    CallExpression { callee: String },
    MemberExpression,
    ArrowFunction,
    Assignment,
    Other,
}

#[derive(Debug, Clone)]
pub struct PropInfo {
    pub local_name: String,
    pub prop_name: String,
    pub default_span: Option<Span>,
    /// The raw text of the default expression (for codegen to parse).
    pub default_text: Option<String>,
    pub is_bindable: bool,
    pub is_rest: bool,
}

#[derive(Debug, Clone)]
pub struct PropsDeclaration {
    pub props: Vec<PropInfo>,
}

#[derive(Debug, Clone)]
pub struct ScriptInfo {
    pub declarations: Vec<DeclarationInfo>,
    pub props_declaration: Option<PropsDeclaration>,
}

#[derive(Debug, Clone)]
pub struct DeclarationInfo {
    pub name: String,
    pub span: Span,
    pub kind: DeclarationKind,
    pub init_span: Option<Span>,
    pub is_rune: Option<RuneKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    Let,
    Const,
    Var,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuneKind {
    State,
    Derived,
    Effect,
    Props,
    Bindable,
    Inspect,
    Host,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parse a JS expression and return owned analysis info.
///
/// `source` is the raw expression text (e.g., "count + 1").
/// `offset` is the byte offset in the original .svelte file (for Span adjustment).
///
/// OXC allocator is created and destroyed inside this function.
pub fn analyze_expression(source: &str, offset: u32) -> Result<ExpressionInfo, Diagnostic> {
    let allocator = Allocator::default();
    let parser = OxcParser::new(&allocator, source, SourceType::default());

    let expr = parser
        .parse_expression()
        .map_err(|_| Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32)))?;

    let info = extract_expression_info(&expr, offset);
    // allocator drops here — all OXC data freed
    Ok(info)
}

/// Parse a `<script>` block and return owned analysis info.
///
/// `source` is the content between `<script>` tags.
/// `offset` is the byte offset of the content start in the .svelte file.
///
/// If you also need `oxc_semantic::Scoping`, use `analyze_script_with_scoping()`
/// to avoid a second parse.
pub fn analyze_script(source: &str, offset: u32, typescript: bool) -> Result<ScriptInfo, Vec<Diagnostic>> {
    let allocator = Allocator::default();
    let source_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };

    let parser = OxcParser::new(&allocator, source, source_type);
    let result = parser.parse();

    if !result.errors.is_empty() {
        return Err(result.errors.iter().map(|_| {
            Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
        }).collect());
    }

    Ok(extract_script_info(&result.program, offset, source))
}

fn extract_script_info(program: &oxc_ast::ast::Program<'_>, offset: u32, source: &str) -> ScriptInfo {
    let mut declarations = Vec::new();
    let mut props_declaration = None;

    for stmt in &program.body {
        use oxc_ast::ast::Statement;

        match stmt {
            Statement::VariableDeclaration(decl) => {
                let kind = match decl.kind {
                    oxc_ast::ast::VariableDeclarationKind::Let => DeclarationKind::Let,
                    oxc_ast::ast::VariableDeclarationKind::Const => DeclarationKind::Const,
                    oxc_ast::ast::VariableDeclarationKind::Var => DeclarationKind::Var,
                    _ => DeclarationKind::Var,
                };

                for declarator in &decl.declarations {
                    match &declarator.id {
                        oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => {
                            let name = ident.name.to_string();
                            let decl_span = Span::new(
                                ident.span.start + offset,
                                ident.span.end + offset,
                            );

                            let (init_span, is_rune) = if let Some(init) = &declarator.init {
                                let init_sp = Span::new(
                                    init.span().start + offset,
                                    init.span().end + offset,
                                );
                                let rune = detect_rune(init);
                                (Some(init_sp), rune)
                            } else {
                                (None, None)
                            };

                            declarations.push(DeclarationInfo {
                                name,
                                span: decl_span,
                                kind,
                                init_span,
                                is_rune,
                            });
                        }
                        oxc_ast::ast::BindingPattern::ObjectPattern(obj_pat) => {
                            let is_props = declarator.init.as_ref()
                                .and_then(|init| detect_rune(init))
                                .map(|r| r == RuneKind::Props)
                                .unwrap_or(false);

                            if is_props {
                                let mut props = Vec::new();

                                for prop in &obj_pat.properties {
                                    let key_name = extract_property_key_name(&prop.key);
                                    let Some(key_name) = key_name else { continue };

                                    let local_name = extract_binding_name(&prop.value);
                                    let local_name = local_name.unwrap_or_else(|| key_name.clone());

                                    let (default_span, default_text, is_bindable) = extract_prop_default(&prop.value, offset, source);

                                    let decl_span = Span::new(
                                        prop.span.start + offset,
                                        prop.span.end + offset,
                                    );

                                    declarations.push(DeclarationInfo {
                                        name: local_name.clone(),
                                        span: decl_span,
                                        kind,
                                        init_span: None,
                                        is_rune: Some(RuneKind::Props),
                                    });

                                    props.push(PropInfo {
                                        local_name,
                                        prop_name: key_name,
                                        default_span,
                                        default_text,
                                        is_bindable,
                                        is_rest: false,
                                    });
                                }

                                if let Some(rest) = &obj_pat.rest {
                                    if let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &rest.argument {
                                        let rest_name = ident.name.to_string();
                                        let decl_span = Span::new(
                                            ident.span.start + offset,
                                            ident.span.end + offset,
                                        );
                                        declarations.push(DeclarationInfo {
                                            name: rest_name.clone(),
                                            span: decl_span,
                                            kind,
                                            init_span: None,
                                            is_rune: Some(RuneKind::Props),
                                        });
                                        props.push(PropInfo {
                                            local_name: rest_name.clone(),
                                            prop_name: rest_name,
                                            default_span: None,
                                            default_text: None,
                                            is_bindable: false,
                                            is_rest: true,
                                        });
                                    }
                                }

                                props_declaration = Some(PropsDeclaration { props });
                            }
                        }
                        _ => {}
                    }
                }
            }
            Statement::FunctionDeclaration(func) => {
                if let Some(ident) = &func.id {
                    declarations.push(DeclarationInfo {
                        name: ident.name.to_string(),
                        span: Span::new(
                            ident.span.start + offset,
                            ident.span.end + offset,
                        ),
                        kind: DeclarationKind::Function,
                        init_span: None,
                        is_rune: None,
                    });
                }
            }
            _ => {}
        }
    }

    ScriptInfo { declarations, props_declaration }
}

/// Parse a `<script>` block once and return both owned analysis info and OXC Scoping.
///
/// This avoids double-parsing: the single OXC parse produces both the `ScriptInfo`
/// (declarations, props) and the `Scoping` (symbol table + scope tree for semantic analysis).
pub fn analyze_script_with_scoping(
    source: &str,
    offset: u32,
    typescript: bool,
) -> Result<(ScriptInfo, oxc_semantic::Scoping), Vec<Diagnostic>> {
    let allocator = Allocator::default();
    let source_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };

    let parser = OxcParser::new(&allocator, source, source_type);
    let result = parser.parse();

    if !result.errors.is_empty() {
        return Err(result
            .errors
            .iter()
            .map(|_| {
                Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
            })
            .collect());
    }

    let program = &result.program;

    // Extract ScriptInfo by walking the AST
    let script_info = extract_script_info(program, offset, source);

    // Build semantic analysis and extract Scoping
    let sem = oxc_semantic::SemanticBuilder::new().build(program);
    let scoping = sem.semantic.into_scoping();

    Ok((script_info, scoping))
}

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

fn extract_expression_info(expr: &Expression<'_>, offset: u32) -> ExpressionInfo {
    let kind = match expr {
        Expression::Identifier(ident) => ExpressionKind::Identifier(ident.name.to_string()),
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_) => ExpressionKind::Literal,
        Expression::CallExpression(call) => {
            let callee = match &call.callee {
                Expression::Identifier(ident) => ident.name.to_string(),
                _ => String::new(),
            };
            ExpressionKind::CallExpression { callee }
        }
        Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_) => {
            ExpressionKind::MemberExpression
        }
        Expression::ArrowFunctionExpression(_) => ExpressionKind::ArrowFunction,
        Expression::AssignmentExpression(_) => ExpressionKind::Assignment,
        _ => ExpressionKind::Other,
    };

    let mut references = Vec::new();
    collect_references(expr, offset, &mut references);

    let has_side_effects = matches!(
        expr,
        Expression::CallExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::UpdateExpression(_)
    );

    ExpressionInfo {
        kind,
        references,
        has_side_effects,
    }
}

fn collect_references(expr: &Expression<'_>, offset: u32, refs: &mut Vec<Reference>) {
    match expr {
        Expression::Identifier(ident) => {
            refs.push(Reference {
                name: ident.name.to_string(),
                span: Span::new(
                    ident.span.start + offset,
                    ident.span.end + offset,
                ),
                flags: ReferenceFlags::Read,
            });
        }
        Expression::AssignmentExpression(assign) => {
            // LHS is write, RHS is read
            if let oxc_ast::ast::AssignmentTarget::AssignmentTargetIdentifier(ident) = &assign.left {
                refs.push(Reference {
                    name: ident.name.to_string(),
                    span: Span::new(
                        ident.span.start + offset,
                        ident.span.end + offset,
                    ),
                    flags: ReferenceFlags::Write,
                });
            }
            collect_references(&assign.right, offset, refs);
        }
        Expression::BinaryExpression(bin) => {
            collect_references(&bin.left, offset, refs);
            collect_references(&bin.right, offset, refs);
        }
        Expression::LogicalExpression(log) => {
            collect_references(&log.left, offset, refs);
            collect_references(&log.right, offset, refs);
        }
        Expression::UnaryExpression(un) => {
            collect_references(&un.argument, offset, refs);
        }
        Expression::UpdateExpression(upd) => {
            if let oxc_ast::ast::SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) =
                &upd.argument
            {
                refs.push(Reference {
                    name: ident.name.to_string(),
                    span: Span::new(ident.span.start + offset, ident.span.end + offset),
                    flags: ReferenceFlags::Write,
                });
            }
        }
        Expression::CallExpression(call) => {
            collect_references(&call.callee, offset, refs);
            for arg in &call.arguments {
                if let oxc_ast::ast::Argument::SpreadElement(spread) = arg {
                    collect_references(&spread.argument, offset, refs);
                } else if let Some(expr) = arg.as_expression() {
                    collect_references(expr, offset, refs);
                }
            }
        }
        Expression::ConditionalExpression(cond) => {
            collect_references(&cond.test, offset, refs);
            collect_references(&cond.consequent, offset, refs);
            collect_references(&cond.alternate, offset, refs);
        }
        Expression::StaticMemberExpression(mem) => {
            collect_references(&mem.object, offset, refs);
        }
        Expression::ComputedMemberExpression(mem) => {
            collect_references(&mem.object, offset, refs);
            collect_references(&mem.expression, offset, refs);
        }
        Expression::TemplateLiteral(tl) => {
            for expr in &tl.expressions {
                collect_references(expr, offset, refs);
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            collect_references(&paren.expression, offset, refs);
        }
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                match elem {
                    oxc_ast::ast::ArrayExpressionElement::SpreadElement(spread) => {
                        collect_references(&spread.argument, offset, refs);
                    }
                    _ => {
                        if let Some(expr) = elem.as_expression() {
                            collect_references(expr, offset, refs);
                        }
                    }
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    oxc_ast::ast::ObjectPropertyKind::ObjectProperty(p) => {
                        collect_references(&p.value, offset, refs);
                    }
                    oxc_ast::ast::ObjectPropertyKind::SpreadProperty(spread) => {
                        collect_references(&spread.argument, offset, refs);
                    }
                }
            }
        }
        _ => {}
    }
}

fn extract_property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<String> {
    match key {
        oxc_ast::ast::PropertyKey::StaticIdentifier(ident) => Some(ident.name.to_string()),
        oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(s.value.to_string()),
        _ => None,
    }
}

fn extract_binding_name(pattern: &oxc_ast::ast::BindingPattern<'_>) -> Option<String> {
    match pattern {
        oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => Some(ident.name.to_string()),
        oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
            extract_binding_name(&assign.left)
        }
        _ => None,
    }
}

/// Extract default span, default text, and bindable flag from a prop's binding pattern.
fn extract_prop_default(pattern: &oxc_ast::ast::BindingPattern<'_>, offset: u32, source: &str) -> (Option<Span>, Option<String>, bool) {
    if let oxc_ast::ast::BindingPattern::AssignmentPattern(assign) = pattern {
        let right = &assign.right;
        // Check if default is $bindable(expr) or $bindable()
        if let Expression::CallExpression(call) = right {
            if let Expression::Identifier(ident) = &call.callee {
                if ident.name.as_str() == "$bindable" {
                    let (default_span, default_text) = if let Some(arg) = call.arguments.first() {
                        let sp = arg.span();
                        let text = &source[sp.start as usize..sp.end as usize];
                        (Some(Span::new(sp.start + offset, sp.end + offset)), Some(text.to_string()))
                    } else {
                        (None, None)
                    };
                    return (default_span, default_text, true);
                }
            }
        }
        let sp = right.span();
        let text = &source[sp.start as usize..sp.end as usize];
        (Some(Span::new(sp.start + offset, sp.end + offset)), Some(text.to_string()), false)
    } else {
        (None, None, false)
    }
}

fn detect_rune(expr: &Expression<'_>) -> Option<RuneKind> {
    if let Expression::CallExpression(call) = expr {
        if let Expression::Identifier(ident) = &call.callee {
            return match ident.name.as_str() {
                "$state" => Some(RuneKind::State),
                "$derived" => Some(RuneKind::Derived),
                "$effect" => Some(RuneKind::Effect),
                "$props" => Some(RuneKind::Props),
                "$bindable" => Some(RuneKind::Bindable),
                "$inspect" => Some(RuneKind::Inspect),
                "$host" => Some(RuneKind::Host),
                _ => None,
            };
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_simple_identifier() {
        let info = analyze_expression("count", 0).unwrap();
        assert_eq!(info.kind, ExpressionKind::Identifier("count".to_string()));
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
        let info = analyze_script("let count = $state(0); const name = 'test';", 0, false).unwrap();
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
}
