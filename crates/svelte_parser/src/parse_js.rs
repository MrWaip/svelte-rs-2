//! JS parsing: OXC utilities for parsing individual JS constructs.
//!
//! Low-level functions (`parse_expression_with_alloc`, `parse_script_with_alloc`, etc.)
//! wrap `OxcParser` to parse individual JS constructs.
//!
//! Script info extraction lives in `script_info.rs`.
//! The AST walk that fills `JsParseResult` lives in `walk_js.rs`.

use std::cell::Cell;

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_parser::Parser as OxcParser;
use oxc_span::SourceType;
use oxc_syntax::node::NodeId;

use svelte_diagnostics::Diagnostic;
use svelte_span::Span;

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
/// `source` is the assignment text without `const` keyword
/// (e.g. `"doubled = item * 2"` or `"{a, b}: T = obj"`).
/// Wraps as `const SOURCE;` for OXC and returns the full Statement.
/// Scope building and codegen extract binding names / init expression from it directly.
pub fn parse_const_declaration_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<oxc_ast::ast::Statement<'a>, Diagnostic> {
    let wrapped_owned = format!("const {};", source);
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
    let mut stmt = program.body.into_iter().next().ok_or_else(|| {
        Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
    })?;

    // Strip TS type annotations from the init expression so codegen gets plain JS.
    if typescript {
        if let oxc_ast::ast::Statement::VariableDeclaration(ref mut var_decl) = stmt {
            if let Some(declarator) = var_decl.declarations.first_mut() {
                if let Some(ref mut init) = declarator.init {
                    strip_ts_expression(init, alloc);
                }
            }
        }
    }

    Ok(stmt)
}

/// Parse an each-block destructuring context pattern via OXC into a caller-provided allocator.
///
/// Wraps as `var PATTERN = x;` and returns the full Statement.
/// Codegen extracts binding names, property keys, and default expressions via shallow destructure.
/// Returns `None` for parse errors (pattern is invalid).
pub(crate) fn parse_each_context_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    typescript: bool,
) -> Option<oxc_ast::ast::Statement<'a>> {
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

    result.program.body.into_iter().next()
}


/// Parse snippet parameters from the raw params text into a caller-provided allocator.
///
/// Wraps as `function f(PARAMS) {}` and returns the full Statement. Scope building and
/// codegen extract parameter names / patterns from it directly.
/// Returns `None` on parse failure (params text is malformed).
pub fn parse_snippet_params_with_alloc<'a>(
    alloc: &'a Allocator,
    params_text: &'a str,
) -> Option<oxc_ast::ast::Statement<'a>> {
    let wrapped_owned = format!("function f({}) {{}}", params_text);
    let wrapped_str: &'a str = alloc.alloc_str(&wrapped_owned);
    let result = OxcParser::new(alloc, wrapped_str, SourceType::default()).parse();
    if !result.errors.is_empty() {
        return None;
    }
    result.program.body.into_iter().next()
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
