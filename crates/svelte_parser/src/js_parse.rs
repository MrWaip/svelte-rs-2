//! OXC-based JS parsing utilities for the Svelte parser.
//!
//! These functions create `OxcParser` internally to parse JS expressions,
//! destructuring patterns, script blocks, and other constructs that the
//! Svelte template parser encounters. Analysis (ExpressionInfo, ScriptInfo)
//! is extracted later by `svelte_analyze`.

use std::cell::Cell;

use compact_str::CompactString;
use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, ObjectPropertyKind, PropertyKey};
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan as _, SourceType};
use oxc_syntax::node::NodeId;

use svelte_diagnostics::Diagnostic;
use svelte_span::Span;
use svelte_types::{
    AwaitBindingInfo, CePropConfig, CeShadowMode, DestructureKind, EachBindingEntry,
    EachContextBinding, ParsedCeConfig,
};

/// Parse an each-block destructuring context pattern via OXC into a caller-provided allocator.
///
/// Wraps as `var PATTERN = x;`, parses via OXC, walks `BindingPattern` to extract
/// binding names, property keys, and default expressions.
pub fn parse_each_context_with_alloc<'a>(
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
                        svelte_types::extract_all_binding_names(&declarator.id, &mut names);
                        return AwaitBindingInfo::Destructured {
                            kind: DestructureKind::Object,
                            names: names.into_iter().map(|n| n.to_string()).collect(),
                        };
                    }
                    oxc_ast::ast::BindingPattern::ArrayPattern(_) => {
                        let mut names = Vec::new();
                        svelte_types::extract_all_binding_names(&declarator.id, &mut names);
                        return AwaitBindingInfo::Destructured {
                            kind: DestructureKind::Array,
                            names: names.into_iter().map(|n| n.to_string()).collect(),
                        };
                    }
                    oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
                        let mut names = Vec::new();
                        svelte_types::extract_all_binding_names(&assign.left, &mut names);
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

/// Parse a custom element config object expression via OXC.
///
/// `source` is the raw expression text (e.g., `{ tag: "my-el", shadow: "open", props: {...} }`).
/// `offset` is the byte offset of `source` within the original .svelte file,
/// used to adjust `extend` span to absolute coordinates.
pub fn parse_ce_config(source: &str, offset: u32) -> ParsedCeConfig {
    let alloc = Allocator::default();
    let wrapped = format!("var x = {};", source);
    let result = OxcParser::new(&alloc, &wrapped, SourceType::default()).parse();

    let prefix_len: u32 = 8; // "var x = "

    let mut config = ParsedCeConfig {
        tag: None,
        shadow: CeShadowMode::Open,
        props: Vec::new(),
        extend_span: None,
    };

    if !result.errors.is_empty() {
        return config;
    }

    let Some(oxc_ast::ast::Statement::VariableDeclaration(decl)) = result.program.body.first()
    else {
        return config;
    };
    let Some(declarator) = decl.declarations.first() else {
        return config;
    };
    let Some(Expression::ObjectExpression(obj)) = &declarator.init else {
        return config;
    };

    for prop_kind in &obj.properties {
        let ObjectPropertyKind::ObjectProperty(prop) = prop_kind else {
            continue;
        };
        let key_name = match &prop.key {
            PropertyKey::StaticIdentifier(id) => id.name.as_str(),
            _ => continue,
        };

        match key_name {
            "tag" => {
                if let Expression::StringLiteral(lit) = &prop.value {
                    config.tag = Some(lit.value.to_string());
                }
            }
            "shadow" => {
                if let Expression::StringLiteral(lit) = &prop.value {
                    if lit.value.as_str() == "none" {
                        config.shadow = CeShadowMode::None;
                    }
                }
            }
            "props" => {
                if let Expression::ObjectExpression(props_obj) = &prop.value {
                    for prop_entry in &props_obj.properties {
                        let ObjectPropertyKind::ObjectProperty(entry) = prop_entry else {
                            continue;
                        };
                        let prop_name = match &entry.key {
                            PropertyKey::StaticIdentifier(id) => id.name.to_string(),
                            _ => continue,
                        };
                        let mut prop_cfg = CePropConfig {
                            name: prop_name,
                            attribute: None,
                            reflect: false,
                            prop_type: None,
                        };
                        if let Expression::ObjectExpression(def_obj) = &entry.value {
                            for def_prop in &def_obj.properties {
                                let ObjectPropertyKind::ObjectProperty(dp) = def_prop else {
                                    continue;
                                };
                                let dk = match &dp.key {
                                    PropertyKey::StaticIdentifier(id) => id.name.as_str(),
                                    _ => continue,
                                };
                                match dk {
                                    "attribute" => {
                                        if let Expression::StringLiteral(lit) = &dp.value {
                                            prop_cfg.attribute = Some(lit.value.to_string());
                                        }
                                    }
                                    "reflect" => {
                                        if let Expression::BooleanLiteral(lit) = &dp.value {
                                            prop_cfg.reflect = lit.value;
                                        }
                                    }
                                    "type" => {
                                        if let Expression::StringLiteral(lit) = &dp.value {
                                            prop_cfg.prop_type = Some(lit.value.to_string());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        config.props.push(prop_cfg);
                    }
                }
            }
            "extend" => {
                let ext_start = prop.value.span().start;
                let ext_end = prop.value.span().end;
                // Adjust from wrapped-string coordinates to original source coordinates
                config.extend_span = Some(Span::new(
                    ext_start - prefix_len + offset,
                    ext_end - prefix_len + offset,
                ));
            }
            _ => {}
        }
    }

    config
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
                svelte_types::extract_all_binding_names(&param.pattern, &mut names);
            }
            if let Some(rest) = &func.params.rest {
                svelte_types::extract_all_binding_names(&rest.rest.argument, &mut names);
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

/// Parse a `{@const name = expr}` declaration via OXC.
///
/// `source` is the raw declaration text (e.g. `"doubled = item * 2"` or `"{a, b} = obj"`).
/// `offset` is `declaration_span.start` in the original .svelte file.
///
/// Returns binding names and the init `Expression` AST (no references — analyze extracts those).
pub fn parse_const_declaration_with_alloc<'a>(
    alloc: &'a Allocator,
    source: &'a str,
    offset: u32,
    typescript: bool,
) -> Result<(Vec<CompactString>, Expression<'a>), Diagnostic> {
    // Wrap as "const {source};" so OXC can parse it as a full statement
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
    svelte_types::extract_all_binding_names(&declarator.id, &mut names);

    let mut init = declarator.init.take().ok_or_else(|| {
        Diagnostic::invalid_expression(Span::new(offset, offset + source.len() as u32))
    })?;

    if typescript {
        strip_ts_expression(&mut init, alloc);
    }

    Ok((names, init))
}

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

/// Check if an expression text represents a "simple" expression that can be
/// eagerly evaluated (no side effects). Matches Svelte's `is_simple_expression()`.
///
/// Simple expressions: literals, identifiers, functions, and combinations of
/// binary/logical/conditional expressions composed of simples.
pub fn is_simple_expression(text: &str) -> bool {
    let alloc = Allocator::default();
    let Ok(expr) = OxcParser::new(&alloc, text, SourceType::default()).parse_expression() else {
        return false;
    };
    svelte_types::is_simple_expr(&expr)
}
