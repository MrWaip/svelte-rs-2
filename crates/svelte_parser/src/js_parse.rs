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

use rustc_hash::FxHashSet;
use svelte_diagnostics::Diagnostic;
use svelte_span::Span;
use crate::types::{
    AwaitBindingInfo, CePropConfig, CeShadowMode, DeclarationInfo, DeclarationKind,
    DestructureKind, EachBindingEntry, EachContextBinding, ExportInfo, ParsedCeConfig, PropInfo,
    PropsDeclaration, RuneKind, ScriptInfo,
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
    crate::types::extract_all_binding_names(&declarator.id, &mut names);

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
    crate::types::is_simple_expr(&expr)
}

// ---------------------------------------------------------------------------
// Script info extraction (moved from svelte_analyze::js_analyze)
// ---------------------------------------------------------------------------

/// Extract structural metadata from a parsed script Program AST.
/// Pure syntax extraction — no semantic analysis (scoping, store detection, etc.).
pub fn extract_script_info(
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    source: &str,
) -> ScriptInfo {
    let mut declarations = Vec::new();
    let mut props_declaration = None;
    let mut exports = Vec::new();
    let mut has_effects = false;
    let mut has_class_state_fields = false;

    for stmt in &program.body {
        use oxc_ast::ast::Statement;

        match stmt {
            Statement::ExportNamedDeclaration(export) => {
                for spec in &export.specifiers {
                    let local = CompactString::from(spec.local.name().as_str());
                    let exported = CompactString::from(spec.exported.name().as_str());
                    let alias = if local != exported {
                        Some(exported)
                    } else {
                        None
                    };
                    exports.push(ExportInfo { name: local, alias });
                }
                if let Some(decl) = &export.declaration {
                    collect_export_names_from_declaration(decl, &mut exports);
                    collect_declarations_from_declaration(
                        decl,
                        offset,
                        source,
                        &mut declarations,
                        &mut props_declaration,
                    );
                }
            }
            Statement::VariableDeclaration(decl) => {
                collect_var_declarations(
                    decl,
                    offset,
                    source,
                    &mut declarations,
                    &mut props_declaration,
                );
            }
            Statement::FunctionDeclaration(func) => {
                collect_func_declaration(func, offset, &mut declarations);
            }
            Statement::ExpressionStatement(es) => {
                if is_effect_call(&es.expression) {
                    has_effects = true;
                }
            }
            Statement::ClassDeclaration(class) => {
                if has_class_state_runes(&class.body) {
                    has_class_state_fields = true;
                }
            }
            _ => {}
        }
    }

    ScriptInfo {
        declarations,
        props_declaration,
        exports,
        has_effects,
        has_class_state_fields,
        store_candidates: Vec::new(),
        has_store_member_mutations: false,
    }
}

fn collect_export_names_from_declaration(
    decl: &oxc_ast::ast::Declaration<'_>,
    exports: &mut Vec<ExportInfo>,
) {
    match decl {
        oxc_ast::ast::Declaration::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                if let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) = &declarator.id {
                    exports.push(ExportInfo {
                        name: CompactString::from(ident.name.as_str()),
                        alias: None,
                    });
                }
            }
        }
        oxc_ast::ast::Declaration::FunctionDeclaration(func) => {
            if let Some(ident) = &func.id {
                exports.push(ExportInfo {
                    name: CompactString::from(ident.name.as_str()),
                    alias: None,
                });
            }
        }
        oxc_ast::ast::Declaration::ClassDeclaration(cls) => {
            if let Some(ident) = &cls.id {
                exports.push(ExportInfo {
                    name: CompactString::from(ident.name.as_str()),
                    alias: None,
                });
            }
        }
        _ => {}
    }
}

fn collect_declarations_from_declaration(
    decl: &oxc_ast::ast::Declaration<'_>,
    offset: u32,
    source: &str,
    declarations: &mut Vec<DeclarationInfo>,
    props_declaration: &mut Option<PropsDeclaration>,
) {
    match decl {
        oxc_ast::ast::Declaration::VariableDeclaration(var_decl) => {
            collect_var_declarations(var_decl, offset, source, declarations, props_declaration);
        }
        oxc_ast::ast::Declaration::FunctionDeclaration(func) => {
            collect_func_declaration(func, offset, declarations);
        }
        _ => {}
    }
}

fn collect_func_declaration(
    func: &oxc_ast::ast::Function<'_>,
    offset: u32,
    declarations: &mut Vec<DeclarationInfo>,
) {
    if let Some(ident) = &func.id {
        declarations.push(DeclarationInfo {
            name: CompactString::from(ident.name.as_str()),
            span: Span::new(ident.span.start + offset, ident.span.end + offset),
            kind: DeclarationKind::Function,
            init_span: None,
            is_rune: None,
            rune_init_refs: vec![],
        });
    }
}

fn collect_var_declarations(
    decl: &oxc_ast::ast::VariableDeclaration<'_>,
    offset: u32,
    source: &str,
    declarations: &mut Vec<DeclarationInfo>,
    props_declaration: &mut Option<PropsDeclaration>,
) {
    let kind = match decl.kind {
        oxc_ast::ast::VariableDeclarationKind::Let => DeclarationKind::Let,
        oxc_ast::ast::VariableDeclarationKind::Const => DeclarationKind::Const,
        oxc_ast::ast::VariableDeclarationKind::Var => DeclarationKind::Var,
        _ => DeclarationKind::Var,
    };

    for declarator in &decl.declarations {
        match &declarator.id {
            oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => {
                let name = CompactString::from(ident.name.as_str());
                let decl_span = Span::new(ident.span.start + offset, ident.span.end + offset);

                let (init_span, is_rune, rune_init_refs) = if let Some(init) = &declarator.init {
                    let init_sp = Span::new(
                        init.span().start + offset,
                        init.span().end + offset,
                    );
                    let rune = detect_rune(init);
                    let refs = if matches!(rune, Some(RuneKind::Derived | RuneKind::DerivedBy)) {
                        collect_derived_refs(init)
                    } else {
                        vec![]
                    };
                    (Some(init_sp), rune, refs)
                } else {
                    (None, None, vec![])
                };

                declarations.push(DeclarationInfo {
                    name,
                    span: decl_span,
                    kind,
                    init_span,
                    is_rune,
                    rune_init_refs,
                });
            }
            oxc_ast::ast::BindingPattern::ObjectPattern(obj_pat) => {
                let rune = declarator.init.as_ref().and_then(|init| detect_rune(init));

                if rune == Some(RuneKind::Props) {
                    let mut props = Vec::new();

                    for prop in &obj_pat.properties {
                        let key_name = extract_property_key_name(&prop.key);
                        let Some(key_name) = key_name else { continue };

                        let local_name = extract_binding_name(&prop.value);
                        let local_name = local_name.unwrap_or_else(|| key_name.clone());

                        let (default_span, default_text, is_bindable, is_simple_default) =
                            extract_prop_default(&prop.value, offset, source);

                        let decl_span =
                            Span::new(prop.span.start + offset, prop.span.end + offset);

                        declarations.push(DeclarationInfo {
                            name: local_name.clone(),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::Props),
                            rune_init_refs: vec![],
                        });

                        props.push(PropInfo {
                            local_name,
                            prop_name: key_name,
                            default_span,
                            default_text,
                            is_bindable,
                            is_rest: false,
                            is_simple_default,
                        });
                    }

                    if let Some(rest) = &obj_pat.rest {
                        if let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) =
                            &rest.argument
                        {
                            let rest_name = CompactString::from(ident.name.as_str());
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
                                rune_init_refs: vec![],
                            });
                            props.push(PropInfo {
                                local_name: rest_name.clone(),
                                prop_name: rest_name,
                                default_span: None,
                                default_text: None,
                                is_bindable: false,
                                is_rest: true,
                                is_simple_default: true,
                            });
                        }
                    }

                    *props_declaration = Some(PropsDeclaration { props });
                } else if matches!(rune, Some(RuneKind::State | RuneKind::StateRaw)) {
                    let mut names = Vec::new();
                    crate::types::extract_all_binding_names(&declarator.id, &mut names);
                    for name in names {
                        let decl_span = Span::new(
                            declarator.span.start + offset,
                            declarator.span.end + offset,
                        );
                        declarations.push(DeclarationInfo {
                            name,
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::StateRaw),
                            rune_init_refs: vec![],
                        });
                    }
                }
            }
            oxc_ast::ast::BindingPattern::ArrayPattern(_) => {
                let rune = declarator.init.as_ref().and_then(|init| detect_rune(init));
                if let Some(rune_kind) = rune {
                    if matches!(rune_kind, RuneKind::State | RuneKind::StateRaw) {
                        let mut names = Vec::new();
                        crate::types::extract_all_binding_names(&declarator.id, &mut names);
                        for name in names {
                            let decl_span = Span::new(
                                declarator.span.start + offset,
                                declarator.span.end + offset,
                            );
                            declarations.push(DeclarationInfo {
                                name,
                                span: decl_span,
                                kind,
                                init_span: None,
                                is_rune: Some(RuneKind::StateRaw),
                                rune_init_refs: vec![],
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn extract_property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<CompactString> {
    match key {
        oxc_ast::ast::PropertyKey::StaticIdentifier(ident) => {
            Some(CompactString::from(ident.name.as_str()))
        }
        oxc_ast::ast::PropertyKey::StringLiteral(s) => {
            Some(CompactString::from(s.value.as_str()))
        }
        _ => None,
    }
}

fn extract_binding_name(pattern: &oxc_ast::ast::BindingPattern<'_>) -> Option<CompactString> {
    match pattern {
        oxc_ast::ast::BindingPattern::BindingIdentifier(ident) => {
            Some(CompactString::from(ident.name.as_str()))
        }
        oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
            extract_binding_name(&assign.left)
        }
        _ => None,
    }
}

fn extract_prop_default(
    pattern: &oxc_ast::ast::BindingPattern<'_>,
    offset: u32,
    source: &str,
) -> (Option<Span>, Option<String>, bool, bool) {
    if let oxc_ast::ast::BindingPattern::AssignmentPattern(assign) = pattern {
        let right = &assign.right;
        if let Expression::CallExpression(call) = right {
            if let Expression::Identifier(ident) = &call.callee {
                if ident.name.as_str() == "$bindable" {
                    let (default_span, default_text, is_simple) =
                        if let Some(arg) = call.arguments.first() {
                            let sp = arg.span();
                            let text = &source[sp.start as usize..sp.end as usize];
                            let expr =
                                arg.as_expression().expect("argument should be expression");
                            (
                                Some(Span::new(sp.start + offset, sp.end + offset)),
                                Some(text.to_string()),
                                crate::types::is_simple_expr(expr),
                            )
                        } else {
                            (None, None, true)
                        };
                    return (default_span, default_text, true, is_simple);
                }
            }
        }
        let sp = right.span();
        let text = &source[sp.start as usize..sp.end as usize];
        let is_simple = crate::types::is_simple_expr(right);
        (
            Some(Span::new(sp.start + offset, sp.end + offset)),
            Some(text.to_string()),
            false,
            is_simple,
        )
    } else {
        (None, None, false, true)
    }
}

fn is_effect_call(expr: &Expression<'_>) -> bool {
    if let Expression::CallExpression(call) = expr {
        match &call.callee {
            Expression::Identifier(id) if id.name.as_str() == "$effect" => return true,
            Expression::StaticMemberExpression(member) => {
                if let Expression::Identifier(obj) = &member.object {
                    if obj.name.as_str() == "$effect" && member.property.name.as_str() == "pre" {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    false
}

fn has_class_state_runes(body: &oxc_ast::ast::ClassBody<'_>) -> bool {
    for element in &body.body {
        match element {
            oxc_ast::ast::ClassElement::PropertyDefinition(prop) => {
                if let Some(value) = &prop.value {
                    if let Some(kind) = detect_rune(value) {
                        if matches!(kind, RuneKind::State | RuneKind::StateRaw) {
                            return true;
                        }
                    }
                }
            }
            oxc_ast::ast::ClassElement::MethodDefinition(method) => {
                if method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor {
                    if let Some(body) = &method.value.body {
                        for stmt in &body.statements {
                            if let oxc_ast::ast::Statement::ExpressionStatement(es) = stmt {
                                if let Expression::AssignmentExpression(assign) = &es.expression {
                                    if let Some(kind) = detect_rune(&assign.right) {
                                        if matches!(kind, RuneKind::State | RuneKind::StateRaw) {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    false
}

/// Detect which Svelte rune a call expression invokes.
pub fn detect_rune(expr: &Expression<'_>) -> Option<RuneKind> {
    if let Expression::CallExpression(call) = expr {
        match &call.callee {
            Expression::Identifier(ident) => {
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
            Expression::StaticMemberExpression(member) => {
                if let Expression::Identifier(obj) = &member.object {
                    let prop = member.property.name.as_str();
                    return match (obj.name.as_str(), prop) {
                        ("$derived", "by") => Some(RuneKind::DerivedBy),
                        ("$state", "raw") => Some(RuneKind::StateRaw),
                        ("$state", "eager") => Some(RuneKind::StateEager),
                        ("$effect", "tracking") => Some(RuneKind::EffectTracking),
                        ("$effect", "pending") => Some(RuneKind::EffectPending),
                        ("$props", "id") => Some(RuneKind::PropsId),
                        _ => None,
                    };
                }
            }
            _ => {}
        }
    }
    None
}

/// Check if a `$`-prefixed name is a known rune (not a store candidate).
pub fn is_rune_name(name: &str) -> bool {
    matches!(
        name,
        "$state" | "$derived" | "$effect" | "$props" | "$bindable" | "$inspect" | "$host"
    )
}

fn collect_derived_refs(expr: &Expression<'_>) -> Vec<CompactString> {
    let Expression::CallExpression(call) = expr else {
        return vec![];
    };
    if call.arguments.is_empty() {
        return vec![];
    }
    let Some(arg_expr) = call.arguments[0].as_expression() else {
        return vec![];
    };
    let mut refs = Vec::new();
    collect_idents_recursive(arg_expr, &mut refs);
    let mut seen = FxHashSet::default();
    refs.retain(|r| seen.insert(r.clone()));
    refs
}

fn collect_idents_recursive(expr: &Expression<'_>, refs: &mut Vec<CompactString>) {
    use oxc_ast::ast::Expression::*;
    match expr {
        Identifier(id) => {
            let name = id.name.as_str();
            if !name.starts_with('$') {
                refs.push(CompactString::from(name));
            }
        }
        BinaryExpression(bin) => {
            collect_idents_recursive(&bin.left, refs);
            collect_idents_recursive(&bin.right, refs);
        }
        CallExpression(call) => {
            collect_idents_recursive(&call.callee, refs);
            for arg in &call.arguments {
                if let Some(e) = arg.as_expression() {
                    collect_idents_recursive(e, refs);
                }
            }
        }
        ArrowFunctionExpression(arrow) => {
            for stmt in &arrow.body.statements {
                match stmt {
                    oxc_ast::ast::Statement::ExpressionStatement(es) => {
                        collect_idents_recursive(&es.expression, refs);
                    }
                    oxc_ast::ast::Statement::ReturnStatement(ret) => {
                        if let Some(arg) = &ret.argument {
                            collect_idents_recursive(arg, refs);
                        }
                    }
                    _ => {}
                }
            }
        }
        UnaryExpression(unary) => {
            collect_idents_recursive(&unary.argument, refs);
        }
        ConditionalExpression(cond) => {
            collect_idents_recursive(&cond.test, refs);
            collect_idents_recursive(&cond.consequent, refs);
            collect_idents_recursive(&cond.alternate, refs);
        }
        LogicalExpression(log) => {
            collect_idents_recursive(&log.left, refs);
            collect_idents_recursive(&log.right, refs);
        }
        StaticMemberExpression(m) => {
            collect_idents_recursive(&m.object, refs);
        }
        ComputedMemberExpression(m) => {
            collect_idents_recursive(&m.object, refs);
            collect_idents_recursive(&m.expression, refs);
        }
        _ => {}
    }
}
