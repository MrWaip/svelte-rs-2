//! JS parsing: OXC utilities + AST walk that fills `JsParseResult`.
//!
//! Low-level functions (`parse_expression_with_alloc`, `parse_script_with_alloc`, etc.)
//! wrap `OxcParser` to parse individual JS constructs.
//! The top-level `parse_js` function walks the component AST, calls these utilities,
//! and populates `JsParseResult` with parsed expressions and metadata.

use std::cell::Cell;

use compact_str::CompactString;
use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, ObjectPropertyKind, PropertyKey};
use oxc_parser::Parser as OxcParser;
use oxc_span::{GetSpan as _, SourceType};
use oxc_syntax::node::NodeId;

use rustc_hash::FxHashSet;
use svelte_ast::{Attribute, Component, ConcatPart, Fragment, Node, ScriptLanguage};
use svelte_diagnostics::Diagnostic;
use svelte_span::Span;
use crate::types::{
    AwaitBindingInfo, CePropConfig, CeShadowMode, DeclarationInfo, DeclarationKind,
    DestructureKind, EachBindingEntry, EachContextBinding, ExportInfo, JsParseResult,
    ParsedCeConfig, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
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

// ---------------------------------------------------------------------------
// Script info helpers
// ---------------------------------------------------------------------------

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

// ===========================================================================
// AST walk — fills JsParseResult by walking Component tree
// ===========================================================================

pub(crate) fn parse_js<'a>(
    alloc: &'a Allocator,
    component: &Component,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let typescript = component.script.as_ref()
        .is_some_and(|s| matches!(s.language, ScriptLanguage::TypeScript));

    if let Some(script) = &component.script {
        let source = component.source_text(script.content_span);
        let arena_source: &'a str = alloc.alloc_str(source);
        match parse_script_with_alloc(
            alloc,
            arena_source,
            script.content_span.start,
            typescript,
        ) {
            Ok(program) => {
                let offset = script.content_span.start;
                let script_info = extract_script_info(&program, offset, source);

                // Parse prop default expressions into the shared allocator
                if let Some(ref props_decl) = script_info.props_declaration {
                    for prop in &props_decl.props {
                        if let Some(span) = prop.default_span {
                            let src = component.source_text(span);
                            let arena_src: &'a str = alloc.alloc_str(src);
                            match parse_expression_with_alloc(
                                alloc, arena_src, span.start, typescript,
                            ) {
                                Ok(expr) => result.parsed.prop_default_exprs.push(Some(expr)),
                                Err(diag) => {
                                    diags.push(diag);
                                    result.parsed.prop_default_exprs.push(None);
                                }
                            }
                        } else {
                            result.parsed.prop_default_exprs.push(None);
                        }
                    }
                }

                result.script_info = Some(script_info);
                result.parsed.script_program = Some(program);
                result.script_content_span = Some(script.content_span);
            }
            Err(errs) => diags.extend(errs),
        }
        result.typescript = typescript;
    }

    walk_fragment(alloc, &component.fragment, component, typescript, result, diags);

    // Parse custom element config expression (if present)
    if let Some(svelte_ast::CustomElementConfig::Expression(span)) =
        component.options.as_ref().and_then(|o| o.custom_element.as_ref())
    {
        let ce_source = component.source_text(*span);
        let config = parse_ce_config(ce_source, span.start);

        if let Some(ext_span) = config.extend_span {
            let ext_src = component.source_text(ext_span);
            let arena_src: &'a str = alloc.alloc_str(ext_src);
            match parse_expression_with_alloc(alloc, arena_src, ext_span.start, typescript) {
                Ok(expr) => { result.parsed.ce_extend_expr = Some(expr); }
                Err(diag) => diags.push(diag),
            }
        }

        result.ce_config = Some(config);
    }
}

/// Parse an expression into the shared allocator, storing AST and offset.
fn parse_expr<'a>(
    alloc: &'a Allocator,
    source: &str,
    offset: u32,
    node_id: svelte_ast::NodeId,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let arena_source: &'a str = alloc.alloc_str(source);
    match parse_expression_with_alloc(alloc, arena_source, offset, typescript) {
        Ok(expr) => {
            result.parsed.exprs.insert(node_id, expr);
            result.parsed.expr_offsets.insert(node_id, offset);
        }
        Err(diag) => diags.push(diag),
    }
}

/// Parse an attribute expression into the shared allocator, storing AST and offset.
fn parse_attr_expr<'a>(
    alloc: &'a Allocator,
    source: &str,
    offset: u32,
    attr_id: svelte_ast::NodeId,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let arena_source: &'a str = alloc.alloc_str(source);
    match parse_expression_with_alloc(alloc, arena_source, offset, typescript) {
        Ok(expr) => {
            result.parsed.attr_exprs.insert(attr_id, expr);
            result.parsed.attr_expr_offsets.insert(attr_id, offset);
        }
        Err(diag) => diags.push(diag),
    }
}

/// Parse concatenation parts (shared by ConcatenationAttribute and StyleDirective::Concatenation).
fn parse_concat_parts<'a>(
    alloc: &'a Allocator,
    parts: &[ConcatPart],
    attr_id: svelte_ast::NodeId,
    component: &Component,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    let mut dyn_idx = 0usize;
    for part in parts {
        if let ConcatPart::Dynamic(span) = part {
            let source = component.source_text(*span);
            let arena_source: &'a str = alloc.alloc_str(source);
            match parse_expression_with_alloc(alloc, arena_source, span.start, typescript) {
                Ok(expr) => {
                    result.parsed.concat_part_exprs.insert((attr_id, dyn_idx), expr);
                    result.parsed.concat_part_offsets.insert((attr_id, dyn_idx), span.start);
                }
                Err(diag) => diags.push(diag),
            }
            dyn_idx += 1;
        }
    }
}

fn walk_fragment<'a>(
    alloc: &'a Allocator,
    fragment: &Fragment,
    component: &Component,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for node in &fragment.nodes {
        walk_node(alloc, node, component, typescript, result, diags);
    }
}

fn walk_node<'a>(
    alloc: &'a Allocator,
    node: &Node,
    component: &Component,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    match node {
        Node::ExpressionTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, result, diags);
        }
        Node::Element(el) => {
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &el.fragment, component, typescript, result, diags);
        }
        Node::ComponentNode(cn) => {
            walk_attrs(alloc, &cn.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &cn.fragment, component, typescript, result, diags);
        }
        Node::IfBlock(block) => {
            let source = component.source_text(block.test_span);
            parse_expr(alloc, source, block.test_span.start, block.id, typescript, result, diags);
            walk_fragment(alloc, &block.consequent, component, typescript, result, diags);
            if let Some(alt) = &block.alternate {
                walk_fragment(alloc, alt, component, typescript, result, diags);
            }
        }
        Node::EachBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, result, diags);
            if let Some(key_span) = block.key_span {
                let key_source = component.source_text(key_span);
                let arena_source: &'a str = alloc.alloc_str(key_source);
                match parse_expression_with_alloc(alloc, arena_source, key_span.start, typescript) {
                    Ok(expr) => {
                        result.parsed.key_exprs.insert(block.id, expr);
                        result.parsed.key_expr_offsets.insert(block.id, key_span.start);
                    }
                    Err(diag) => diags.push(diag),
                }
            }

            // Pre-parse destructuring context via OXC so codegen doesn't re-parse
            let ctx_source = component.source_text(block.context_span);
            let ctx_trimmed = ctx_source.trim();
            if ctx_trimmed.starts_with('{') || ctx_trimmed.starts_with('[') {
                let arena_ctx: &'a str = alloc.alloc_str(ctx_source);
                if let Some(binding) = parse_each_context_with_alloc(alloc, arena_ctx, typescript) {
                    result.parsed.each_context_bindings.insert(block.id, binding);
                }
            }

            walk_fragment(alloc, &block.body, component, typescript, result, diags);

            if let Some(fb) = &block.fallback {
                walk_fragment(alloc, fb, component, typescript, result, diags);
            }
        }
        Node::SnippetBlock(block) => {
            // Pre-compute snippet param names for scope building
            if let Some(span) = block.params_span {
                let params = parse_snippet_params(component.source_text(span));
                result.snippet_param_names.insert(block.id, params);
            }
            walk_fragment(alloc, &block.body, component, typescript, result, diags);
        }
        Node::RenderTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, result, diags);

            // Unwrap ChainExpression → CallExpression, recording the chain flag.
            if matches!(result.parsed.exprs.get(&tag.id), Some(Expression::ChainExpression(_))) {
                result.render_tag_is_chain.insert(tag.id);
                if let Some(Expression::ChainExpression(chain)) = result.parsed.exprs.remove(&tag.id) {
                    if let oxc_ast::ast::ChainElement::CallExpression(call) = chain.unbox().expression {
                        result.parsed.exprs.insert(tag.id, Expression::CallExpression(call));
                    }
                }
            }

            // Store callee name (arg metadata is computed in analyze)
            if let Some(Expression::CallExpression(call)) = result.parsed.exprs.get(&tag.id) {
                if let Expression::Identifier(ident) = &call.callee {
                    result.render_tag_callee_name.insert(tag.id, ident.name.to_string());
                }
            }
        }
        Node::HtmlTag(tag) => {
            let source = component.source_text(tag.expression_span);
            parse_expr(alloc, source, tag.expression_span.start, tag.id, typescript, result, diags);
        }
        Node::KeyBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, result, diags);
            walk_fragment(alloc, &block.fragment, component, typescript, result, diags);
        }
        Node::AwaitBlock(block) => {
            let source = component.source_text(block.expression_span);
            parse_expr(alloc, source, block.expression_span.start, block.id, typescript, result, diags);

            if let Some(val_span) = block.value_span {
                let binding_text = component.source_text(val_span);
                let info = parse_await_binding(binding_text);
                result.await_values.insert(block.id, info);
            }
            if let Some(err_span) = block.error_span {
                let binding_text = component.source_text(err_span);
                let info = parse_await_binding(binding_text);
                result.await_errors.insert(block.id, info);
            }

            if let Some(ref p) = block.pending {
                walk_fragment(alloc, p, component, typescript, result, diags);
            }
            if let Some(ref t) = block.then {
                walk_fragment(alloc, t, component, typescript, result, diags);
            }
            if let Some(ref c) = block.catch {
                walk_fragment(alloc, c, component, typescript, result, diags);
            }
        }
        Node::ConstTag(tag) => {
            let decl_text = component.source_text(tag.declaration_span);
            let arena_source: &'a str = alloc.alloc_str(decl_text);
            match parse_const_declaration_with_alloc(alloc, arena_source, tag.declaration_span.start, typescript) {
                Ok((names, init_expr)) => {
                    // Store the offset adjusted for the "const " prefix that was wrapped around the source
                    let ref_offset = tag.declaration_span.start.wrapping_sub(6);
                    result.parsed.exprs.insert(tag.id, init_expr);
                    result.parsed.expr_offsets.insert(tag.id, ref_offset);
                    result.const_tag_names.insert(tag.id, names.iter().map(|n| n.to_string()).collect());
                }
                Err(diag) => diags.push(diag),
            }
        }
        Node::SvelteHead(head) => {
            walk_fragment(alloc, &head.fragment, component, typescript, result, diags);
        }
        Node::SvelteElement(el) => {
            if !el.static_tag {
                let tag_source = component.source_text(el.tag_span);
                parse_expr(alloc, tag_source, el.tag_span.start, el.id, typescript, result, diags);
            }
            walk_attrs(alloc, &el.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &el.fragment, component, typescript, result, diags);
        }
        Node::SvelteWindow(w) => {
            walk_attrs(alloc, &w.attributes, component, typescript, result, diags);
        }
        Node::SvelteDocument(d) => {
            walk_attrs(alloc, &d.attributes, component, typescript, result, diags);
        }
        Node::SvelteBody(b) => {
            walk_attrs(alloc, &b.attributes, component, typescript, result, diags);
        }
        Node::SvelteBoundary(b) => {
            walk_attrs(alloc, &b.attributes, component, typescript, result, diags);
            walk_fragment(alloc, &b.fragment, component, typescript, result, diags);
        }
        Node::DebugTag(tag) => {
            for (i, span) in tag.identifiers.iter().enumerate() {
                let name = component.source_text(*span);
                let arena_name: &'a str = alloc.alloc_str(name);
                match parse_expression_with_alloc(alloc, arena_name, span.start, typescript) {
                    Ok(expr) => {
                        result.parsed.debug_tag_exprs.insert((tag.id, i), expr);
                    }
                    Err(_) => {}
                }
            }
        }
        Node::Text(_) | Node::Comment(_) | Node::Error(_) => {}
    }
}

/// Parse and store attribute expressions, keyed by attribute NodeId.
fn walk_attrs<'a>(
    alloc: &'a Allocator,
    attrs: &[Attribute],
    component: &Component,
    typescript: bool,
    result: &mut JsParseResult<'a>,
    diags: &mut Vec<Diagnostic>,
) {
    for attr in attrs {
        let attr_id = attr.id();
        match attr {
            Attribute::ExpressionAttribute(a) => {
                let source = component.source_text(a.expression_span);
                parse_attr_expr(alloc, source, a.expression_span.start, attr_id, typescript, result, diags);
                // Detect semantic shorthand: expression is a simple identifier matching attr name
                if let Some(Expression::Identifier(ident)) = result.parsed.attr_exprs.get(&attr_id) {
                    if ident.name.as_str() == a.name {
                        result.expression_shorthand.insert(attr_id);
                    }
                }
                // class={[...]} or class={{...}} or class={x} need clsx to resolve
                if a.name == "class" {
                    if let Some(expr) = result.parsed.attr_exprs.get(&attr_id) {
                        let needs = !matches!(
                            expr,
                            oxc_ast::ast::Expression::StringLiteral(_)
                                | oxc_ast::ast::Expression::TemplateLiteral(_)
                                | oxc_ast::ast::Expression::BinaryExpression(_)
                        );
                        if needs {
                            result.needs_clsx.insert(attr_id);
                        }
                    }
                }
            }
            Attribute::ConcatenationAttribute(a) => {
                parse_concat_parts(alloc, &a.parts, attr_id, component, typescript, result, diags);
            }
            Attribute::ClassDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                    if let Some(Expression::Identifier(ident)) = result.parsed.attr_exprs.get(&attr_id) {
                        if ident.name.as_str() == a.name {
                            result.expression_shorthand.insert(attr_id);
                        }
                    }
                }
            }
            Attribute::StyleDirective(a) => {
                use svelte_ast::StyleDirectiveValue;
                match &a.value {
                    StyleDirectiveValue::Expression(span) => {
                        let source = component.source_text(*span);
                        parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                        if let Some(Expression::Identifier(ident)) = result.parsed.attr_exprs.get(&attr_id) {
                            if ident.name.as_str() == a.name {
                                result.expression_shorthand.insert(attr_id);
                            }
                        }
                    }
                    StyleDirectiveValue::Concatenation(parts) => {
                        parse_concat_parts(alloc, parts, attr_id, component, typescript, result, diags);
                    }
                    StyleDirectiveValue::Shorthand | StyleDirectiveValue::String(_) => {}
                }
            }
            Attribute::BindDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
            }
            Attribute::SpreadAttribute(a) => {
                debug_assert!(
                    a.expression_span.end >= a.expression_span.start + 3,
                    "spread expression span too short to contain '...'"
                );
                let span = svelte_span::Span::new(a.expression_span.start + 3, a.expression_span.end);
                let source = component.source_text(span);
                parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
            }
            Attribute::Shorthand(a) => {
                let source = component.source_text(a.expression_span);
                parse_attr_expr(alloc, source, a.expression_span.start, attr_id, typescript, result, diags);
            }
            Attribute::UseDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
                let name_src = component.source_text(a.name);
                let arena_src: &'a str = alloc.alloc_str(name_src);
                if let Ok(expr) = parse_expression_with_alloc(alloc, arena_src, a.name.start, typescript) {
                    result.parsed.directive_name_exprs.insert(a.id, expr);
                }
            }
            Attribute::StringAttribute(_) | Attribute::BooleanAttribute(_) => {}
            // LEGACY(svelte4): on:directive — parse expression if present
            Attribute::OnDirectiveLegacy(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
            }
            Attribute::TransitionDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
                let name_src = component.source_text(a.name);
                let arena_src: &'a str = alloc.alloc_str(name_src);
                if let Ok(expr) = parse_expression_with_alloc(alloc, arena_src, a.name.start, typescript) {
                    result.parsed.directive_name_exprs.insert(a.id, expr);
                }
            }
            Attribute::AnimateDirective(a) => {
                if let Some(span) = a.expression_span {
                    let source = component.source_text(span);
                    parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
                }
                let name_src = component.source_text(a.name);
                let arena_src: &'a str = alloc.alloc_str(name_src);
                if let Ok(expr) = parse_expression_with_alloc(alloc, arena_src, a.name.start, typescript) {
                    result.parsed.directive_name_exprs.insert(a.id, expr);
                }
            }
            Attribute::AttachTag(a) => {
                let span = a.expression_span;
                let source = component.source_text(span);
                parse_attr_expr(alloc, source, span.start, attr_id, typescript, result, diags);
            }
        }
    }
}
