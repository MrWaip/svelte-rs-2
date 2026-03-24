//! JS parsing: OXC utilities for parsing individual JS constructs.
//!
//! Low-level functions (`parse_expression_with_alloc`, `parse_script_with_alloc`, etc.)
//! wrap `OxcParser` to parse individual JS constructs.
//!
//! Script info extraction lives in `script_info.rs`.
//! The AST walk that fills `JsParseResult` lives in `walk_js.rs`.

use std::cell::Cell;

use compact_str::CompactString;
use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, PropertyKey};
use oxc_parser::Parser as OxcParser;
use oxc_span::SourceType;
use oxc_syntax::node::NodeId;

use svelte_diagnostics::Diagnostic;
use svelte_span::Span;
use crate::types::{
    AwaitBindingInfo, DestructureKind, EachBindingEntry,
    EachContextBinding,
};

// ===========================================================================
// OXC parsing utilities
// ===========================================================================

/// Parse a JS expression into a provided allocator, returning only the AST.
///
/// The `Expression<'a>` lives in the provided allocator (not destroyed after call).
/// Expression metadata (ExpressionInfo) is extracted later by analyze.
pub fn parse_expression_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<Expression<'a>, Diagnostic> {
    let src_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };
    let parser = OxcParser::new(alloc, source, src_type);
    let mut expr = parser.parse_expression().map_err(|_| {
        Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
    })?;
    if typescript {
        strip_ts_expression(&mut expr, alloc);
    }
    Ok(expr)
}

/// Parse a `<script>` block once in a caller-provided allocator.
///
/// Returns only the `Program<'a>` AST. Script metadata (ScriptInfo, Scoping)
/// is extracted later by analyze.
pub fn parse_script_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<oxc_ast::ast::Program<'a>, Vec<Diagnostic>> {
    let source_type = if typescript {
        SourceType::mjs().with_typescript(true)
    } else {
        SourceType::mjs()
    };

    let result = OxcParser::new(alloc, source, source_type).parse();

    if !result.errors.is_empty() {
        return Err(result
            .errors
            .iter()
            .map(|_| {
                Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
            })
            .collect());
    }

    Ok(result.program)
}

/// Parse a `{@const name = expr}` declaration via OXC.
///
/// `source` is the full declaration text including `const` keyword
/// (e.g. `"const doubled = item * 2"`).
/// `offset` is `expression_span.start` in the original .svelte file.
///
/// Returns binding names and the init `Expression` AST (no references — analyze extracts those).
pub fn parse_const_declaration_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<(Vec<CompactString>, Expression<'a>), Diagnostic> {
    let wrapped_owned = format!("{};", source);
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);

    let src_type = if typescript {
        SourceType::default()
            .with_typescript(true)
            .with_module(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, wrapped_str, src_type).parse();

    if !result.errors.is_empty() {
        return Err(Diagnostic::invalid_expression(Span::new(
            offset,
            offset + source.len() as u32,
        )));
    }

    let program = result.program;
    let stmt = program.body.into_iter().next().ok_or_else(|| {
        Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
    })?;

    let oxc_ast::ast::Statement::VariableDeclaration(mut var_decl) = stmt else {
        return Err(Diagnostic::invalid_expression(Span::new(
            offset,
            offset + source.len() as u32,
        )));
    };

    let mut declarator = var_decl.declarations.remove(0);

    let mut names = Vec::new();
    crate::types::extract_all_binding_names(&declarator.id, &mut names);

    let mut init = declarator.init.take().ok_or_else(|| {
        Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
    })?;

    if typescript {
        strip_ts_expression(&mut init, alloc);
    }

    Ok((names, init))
}

/// Parse an each-block destructuring context pattern via OXC into a caller-provided allocator.
///
/// Wraps as `var PATTERN = x;`, parses via OXC, walks `BindingPattern` to extract
/// binding names, property keys, and default expressions.
pub(crate) fn parse_each_context_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    typescript: bool,
) -> Option<EachContextBinding<'a>> {
    let trimmed = source.trim();
    let wrapped_owned = format!("var {} = x;", trimmed);
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);

    let src_type = if typescript {
        SourceType::default().with_typescript(true)
    } else {
        SourceType::default()
    };
    let result = OxcParser::new(alloc, wrapped_str, src_type).parse();

    if !result.errors.is_empty() {
        return None;
    }

    let program = result.program;
    let stmt = program.body.into_iter().next()?;
    let oxc_ast::ast::Statement::VariableDeclaration(mut var_decl) = stmt else {
        return None;
    };
    let declarator = var_decl.declarations.remove(0);

    match declarator.id {
        oxc_ast::ast::BindingPattern::ObjectPattern(obj) => {
            let mut bindings = Vec::new();
            for prop in obj.unbox().properties {
                let key_name = match &prop.key {
                    PropertyKey::StaticIdentifier(id) => Some(CompactString::from(id.name.as_str())),
                    _ => None,
                };
                match prop.value {
                    oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                        let name = CompactString::from(id.name.as_str());
                        let key = if key_name.as_ref() == Some(&name) { None } else { key_name };
                        bindings.push(EachBindingEntry { name, key_name: key, default_expr: None });
                    }
                    oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
                        let assign = assign.unbox();
                        let name = match assign.left {
                            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                                CompactString::from(id.name.as_str())
                            }
                            _ => continue,
                        };
                        let key = if key_name.as_ref() == Some(&name) { None } else { key_name };
                        bindings.push(EachBindingEntry {
                            name,
                            key_name: key,
                            default_expr: Some(assign.right),
                        });
                    }
                    _ => continue,
                }
            }
            Some(EachContextBinding { is_array: false, bindings })
        }
        oxc_ast::ast::BindingPattern::ArrayPattern(arr) => {
            let mut bindings = Vec::new();
            for elem in arr.unbox().elements.into_iter().flatten() {
                match elem {
                    oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                        bindings.push(EachBindingEntry {
                            name: CompactString::from(id.name.as_str()),
                            key_name: None,
                            default_expr: None,
                        });
                    }
                    oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
                        let assign = assign.unbox();
                        let name = match assign.left {
                            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                                CompactString::from(id.name.as_str())
                            }
                            _ => continue,
                        };
                        bindings.push(EachBindingEntry {
                            name,
                            key_name: None,
                            default_expr: Some(assign.right),
                        });
                    }
                    _ => continue,
                }
            }
            Some(EachContextBinding { is_array: true, bindings })
        }
        _ => None,
    }
}

/// Parse an await binding pattern via OXC.
///
/// Wraps the text as `var PATTERN = x;` and inspects the parsed `BindingPattern`
/// to determine if it's a simple identifier, object destructuring, or array destructuring.
pub fn parse_await_binding(text: &str) -> AwaitBindingInfo {
    let trimmed = text.trim();

    let alloc = Allocator::default();
    let source = format!("var {} = x;", trimmed);
    let result = OxcParser::new(&alloc, &source, SourceType::default()).parse();

    if result.errors.is_empty() {
        if let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) =
            result.program.body.first()
        {
            if let Some(declarator) = decl.declarations.first() {
                match &declarator.id {
                    oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                        return AwaitBindingInfo::Simple(id.name.to_string());
                    }
                    oxc_ast::ast::BindingPattern::ObjectPattern(_) => {
                        let mut names = Vec::new();
                        crate::types::extract_all_binding_names(&declarator.id, &mut names);
                        return AwaitBindingInfo::Destructured {
                            kind: DestructureKind::Object,
                            names: names.into_iter().map(|n| n.to_string()).collect(),
                        };
                    }
                    oxc_ast::ast::BindingPattern::ArrayPattern(_) => {
                        let mut names = Vec::new();
                        crate::types::extract_all_binding_names(&declarator.id, &mut names);
                        return AwaitBindingInfo::Destructured {
                            kind: DestructureKind::Array,
                            names: names.into_iter().map(|n| n.to_string()).collect(),
                        };
                    }
                    oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
                        let mut names = Vec::new();
                        crate::types::extract_all_binding_names(&assign.left, &mut names);
                        if names.len() == 1 {
                            return AwaitBindingInfo::Simple(names[0].to_string());
                        }
                        return AwaitBindingInfo::Destructured {
                            kind: DestructureKind::Object,
                            names: names.into_iter().map(|n| n.to_string()).collect(),
                        };
                    }
                }
            }
        }
    }

    // Fallback: treat as simple identifier
    AwaitBindingInfo::Simple(trimmed.to_string())
}


/// Parse snippet parameter names from the raw params text (e.g. `"a, b"` or `"{ name }, count"`).
///
/// Uses OXC to parse `function f(PARAMS) {}` so destructuring patterns and default
/// values with commas are handled correctly. Falls back to a simple comma split on
/// parse failure.
pub fn parse_snippet_params(params_text: &str) -> Vec<String> {
    let alloc = Allocator::default();
    let source = format!("function f({}) {{}}", params_text);
    let result = OxcParser::new(&alloc, &source, SourceType::default()).parse();

    if result.errors.is_empty() {
        if let Some(oxc_ast::ast::Statement::FunctionDeclaration(func)) =
            result.program.body.first()
        {
            let mut names: Vec<CompactString> = Vec::new();
            for param in &func.params.items {
                crate::types::extract_all_binding_names(&param.pattern, &mut names);
            }
            if let Some(rest) = &func.params.rest {
                crate::types::extract_all_binding_names(&rest.rest.argument, &mut names);
            }
            return names.iter().map(|n| n.to_string()).collect();
        }
    }

    // Fallback: simple comma split for trivial cases (should rarely trigger)
    params_text
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Unwrap TypeScript expression wrappers in-place, extracting the inner JS expression.
/// Handles: TSAsExpression, TSSatisfiesExpression, TSNonNullExpression,
///          TSTypeAssertion, TSInstantiationExpression.
fn strip_ts_expression<'a>(expr: &mut Expression<'a>, alloc: &'a Allocator) {
    let dummy = || {
        Expression::NullLiteral(oxc_allocator::Box::new_in(
            oxc_ast::ast::NullLiteral {
                span: oxc_span::SPAN,
                node_id: Cell::new(NodeId::DUMMY),
            },
            alloc,
        ))
    };
    // Unwrap top-level TS wrappers (may be nested, e.g. `x as T satisfies U`)
    loop {
        let inner = match std::mem::replace(expr, dummy()) {
            Expression::TSAsExpression(ts) => ts.unbox().expression,
            Expression::TSSatisfiesExpression(ts) => ts.unbox().expression,
            Expression::TSNonNullExpression(ts) => ts.unbox().expression,
            Expression::TSTypeAssertion(ts) => ts.unbox().expression,
            Expression::TSInstantiationExpression(ts) => ts.unbox().expression,
            other => {
                *expr = other;
                break;
            }
        };
        *expr = inner;
    }
}
