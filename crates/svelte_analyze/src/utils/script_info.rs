//! Script info extraction — structural metadata from parsed script Program AST.
//!
//! Pure syntax extraction: declarations, exports, rune detection, prop defaults.
//! No semantic analysis (scoping, store detection, etc.).

use compact_str::CompactString;
use oxc_ast::ast::{CallExpression, Expression};
use oxc_ast_visit::{walk, Visit};
use oxc_span::GetSpan as _;

use rustc_hash::FxHashSet;
use svelte_span::Span;

use crate::types::script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
use crate::utils::binding_pattern::collect_binding_names;

struct SimpleExprChecker(bool);

impl<'a> Visit<'a> for SimpleExprChecker {
    fn visit_expression(&mut self, expr: &Expression<'a>) {
        match expr {
            Expression::NumericLiteral(_)
            | Expression::StringLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::Identifier(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_) => {}
            Expression::ConditionalExpression(_)
            | Expression::BinaryExpression(_)
            | Expression::LogicalExpression(_) => walk::walk_expression(self, expr),
            _ => self.0 = false,
        }
    }
}

fn is_simple_expr(expr: &Expression<'_>) -> bool {
    let mut checker = SimpleExprChecker(true);
    checker.visit_expression(expr);
    checker.0
}

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
            _ => {}
        }
    }

    ScriptInfo {
        declarations,
        props_declaration,
        exports,
        store_candidates: Vec::new(),
    }
}

/// Detect which Svelte rune a call expression invokes.
pub fn detect_rune(expr: &Expression<'_>) -> Option<RuneKind> {
    if let Expression::CallExpression(call) = expr {
        return detect_rune_from_call(call);
    }
    None
}

/// Detect which Svelte rune a `CallExpression` invokes, without requiring the
/// outer `Expression` wrapper. Used by the `ExpressionAnalyzer` visitor.
pub(crate) fn detect_rune_from_call(call: &CallExpression<'_>) -> Option<RuneKind> {
    match &call.callee {
        Expression::Identifier(ident) => match ident.name.as_str() {
            "$state" => Some(RuneKind::State),
            "$derived" => Some(RuneKind::Derived),
            "$effect" => Some(RuneKind::Effect),
            "$props" => Some(RuneKind::Props),
            "$bindable" => Some(RuneKind::Bindable),
            "$inspect" => Some(RuneKind::Inspect),
            "$host" => Some(RuneKind::Host),
            _ => None,
        },
        Expression::StaticMemberExpression(member) => {
            if let Expression::Identifier(obj) = &member.object {
                let prop = member.property.name.as_str();
                match (obj.name.as_str(), prop) {
                    ("$derived", "by") => Some(RuneKind::DerivedBy),
                    ("$state", "raw") => Some(RuneKind::StateRaw),
                    ("$state", "eager") => Some(RuneKind::StateEager),
                    ("$effect", "pre") => Some(RuneKind::EffectPre),
                    ("$effect", "root") => Some(RuneKind::EffectRoot),
                    ("$effect", "tracking") => Some(RuneKind::EffectTracking),
                    ("$effect", "pending") => Some(RuneKind::EffectPending),
                    ("$props", "id") => Some(RuneKind::PropsId),
                    ("$inspect", "trace") => Some(RuneKind::InspectTrace),
                    _ => None,
                }
            } else if member.property.name == "with" {
                // `$inspect(...).with(callback)` — callee is `$inspect(...).with`
                if let Expression::CallExpression(inner) = &member.object {
                    if let Expression::Identifier(id) = &inner.callee {
                        if id.name == "$inspect" {
                            return Some(RuneKind::InspectWith);
                        }
                    }
                }
                None
            } else {
                None
            }
        }
        _ => None,
    }
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
            init_literal: None,
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

                let (init_span, is_rune, rune_init_refs, init_literal) = if let Some(init) =
                    &declarator.init
                {
                    let init_sp = Span::new(init.span().start + offset, init.span().end + offset);
                    let rune = detect_rune(init);
                    let refs = if matches!(rune, Some(RuneKind::Derived | RuneKind::DerivedBy)) {
                        collect_derived_refs(init)
                    } else {
                        vec![]
                    };
                    let literal = if rune.is_some() {
                        extract_call_arg_literal(init)
                    } else {
                        extract_literal(init)
                    };
                    (Some(init_sp), rune, refs, literal)
                } else {
                    (None, None, vec![], None)
                };

                // `const props = $props()` — treat as a single rest prop
                if is_rune == Some(RuneKind::Props) {
                    *props_declaration = Some(PropsDeclaration {
                        props: vec![PropInfo {
                            local_name: name.clone(),
                            prop_name: name.clone(),
                            default_span: None,
                            default_text: None,
                            is_bindable: false,
                            is_rest: true,
                            is_simple_default: true,
                        }],
                        is_identifier_pattern: true,
                    });
                }

                declarations.push(DeclarationInfo {
                    name,
                    span: decl_span,
                    kind,
                    init_span,
                    is_rune,
                    rune_init_refs,
                    init_literal,
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

                        let decl_span = Span::new(prop.span.start + offset, prop.span.end + offset);

                        declarations.push(DeclarationInfo {
                            name: local_name.clone(),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::Props),
                            rune_init_refs: vec![],
                            init_literal: None,
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
                            let decl_span =
                                Span::new(ident.span.start + offset, ident.span.end + offset);
                            declarations.push(DeclarationInfo {
                                name: rest_name.clone(),
                                span: decl_span,
                                kind,
                                init_span: None,
                                is_rune: Some(RuneKind::Props),
                                rune_init_refs: vec![],
                                init_literal: None,
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

                    *props_declaration = Some(PropsDeclaration {
                        props,
                        is_identifier_pattern: false,
                    });
                } else if matches!(
                    rune,
                    Some(
                        RuneKind::State
                            | RuneKind::StateRaw
                            | RuneKind::Derived
                            | RuneKind::DerivedBy
                    )
                ) {
                    let mut names = Vec::new();
                    collect_binding_names(&declarator.id, &mut names);
                    let rune_init_refs =
                        if matches!(rune, Some(RuneKind::Derived | RuneKind::DerivedBy)) {
                            declarator
                                .init
                                .as_ref()
                                .map(collect_derived_refs)
                                .unwrap_or_default()
                        } else {
                            vec![]
                        };
                    for name in names {
                        let decl_span =
                            Span::new(declarator.span.start + offset, declarator.span.end + offset);
                        declarations.push(DeclarationInfo {
                            name: CompactString::from(&name),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: rune,
                            rune_init_refs: rune_init_refs.clone(),
                            init_literal: None,
                        });
                    }
                }
            }
            oxc_ast::ast::BindingPattern::ArrayPattern(_) => {
                let rune = declarator.init.as_ref().and_then(|init| detect_rune(init));
                if let Some(rune_kind) = rune {
                    if matches!(
                        rune_kind,
                        RuneKind::State
                            | RuneKind::StateRaw
                            | RuneKind::Derived
                            | RuneKind::DerivedBy
                    ) {
                        let mut names = Vec::new();
                        collect_binding_names(&declarator.id, &mut names);
                        let rune_init_refs =
                            if matches!(rune_kind, RuneKind::Derived | RuneKind::DerivedBy) {
                                declarator
                                    .init
                                    .as_ref()
                                    .map(collect_derived_refs)
                                    .unwrap_or_default()
                            } else {
                                vec![]
                            };
                        for name in names {
                            let decl_span = Span::new(
                                declarator.span.start + offset,
                                declarator.span.end + offset,
                            );
                            declarations.push(DeclarationInfo {
                                name: CompactString::from(&name),
                                span: decl_span,
                                kind,
                                init_span: None,
                                is_rune: Some(rune_kind),
                                rune_init_refs: rune_init_refs.clone(),
                                init_literal: None,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Extract a literal value from an OXC expression (string, number, boolean).
fn extract_literal(expr: &Expression<'_>) -> Option<CompactString> {
    match expr {
        Expression::StringLiteral(s) => Some(CompactString::from(s.value.as_str())),
        Expression::BooleanLiteral(b) => {
            Some(CompactString::from(if b.value { "true" } else { "false" }))
        }
        Expression::NumericLiteral(n) => n.raw.as_ref().map(|r| CompactString::from(r.as_str())),
        _ => None,
    }
}

/// Extract a literal value from the first argument of a call expression (e.g. `$state(42)` → `"42"`).
fn extract_call_arg_literal(expr: &Expression<'_>) -> Option<CompactString> {
    let Expression::CallExpression(call) = expr else {
        return None;
    };
    let arg = call.arguments.first()?;
    let arg_expr = arg.as_expression()?;
    extract_literal(arg_expr)
}

fn extract_property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<CompactString> {
    match key {
        oxc_ast::ast::PropertyKey::StaticIdentifier(ident) => {
            Some(CompactString::from(ident.name.as_str()))
        }
        oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(CompactString::from(s.value.as_str())),
        _ => None,
    }
}

fn extract_binding_name(pattern: &oxc_ast::ast::BindingPattern<'_>) -> Option<CompactString> {
    pattern
        .get_binding_identifier()
        .map(|id| CompactString::from(id.name.as_str()))
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
                            let expr = arg.as_expression().expect("argument should be expression");
                            (
                                Some(Span::new(sp.start + offset, sp.end + offset)),
                                Some(text.to_string()),
                                is_simple_expr(expr),
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
        let is_simple = is_simple_expr(right);
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

/// Collect unique non-`$` identifier references from a `$derived`/`$derived.by` call's
/// first argument. Uses OXC Visit for complete expression traversal.
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
    let mut collector = IdentCollector { refs: Vec::new() };
    collector.visit_expression(arg_expr);
    let mut seen = FxHashSet::default();
    collector.refs.retain(|r| seen.insert(r.clone()));
    collector.refs
}

/// Visitor that collects all non-`$`-prefixed identifier references.
struct IdentCollector {
    refs: Vec<CompactString>,
}

impl<'a> Visit<'a> for IdentCollector {
    fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
        let name = ident.name.as_str();
        if !name.starts_with('$') {
            self.refs.push(CompactString::from(name));
        }
    }
}

/// Enrich ScriptInfo from OXC's unresolved references.
/// Detects store candidates (`$count` etc.) from unresolved `$`-prefixed references.
pub fn enrich_from_unresolved<'a>(
    unresolved: impl Iterator<Item = &'a str>,
    info: &mut ScriptInfo,
) {
    for name in unresolved {
        if name.starts_with('$') && name.len() > 1 && !name.starts_with("$$") && !is_rune_name(name)
        {
            info.store_candidates.push(CompactString::from(&name[1..]));
        }
    }
}

pub fn enrich_from_component_scoping(
    scoping: &crate::scope::ComponentScoping,
    info: &mut ScriptInfo,
) {
    enrich_from_unresolved(
        scoping
            .root_unresolved_references()
            .keys()
            .map(|name| name.as_str()),
        info,
    );
}
