//! Script info extraction — structural metadata from parsed script Program AST.
//!
//! Pure syntax extraction: declarations, exports, rune detection, prop defaults.
//! No semantic analysis (scoping, store detection, etc.).

use compact_str::CompactString;
use oxc_ast::ast::Expression;
use oxc_span::GetSpan as _;

use rustc_hash::FxHashSet;
use svelte_span::Span;

use crate::types::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};

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

/// Enrich ScriptInfo from OXC's unresolved references.
/// Detects store candidates (`$count` etc.) from unresolved `$`-prefixed references.
pub fn enrich_from_unresolved(scoping: &oxc_semantic::Scoping, info: &mut ScriptInfo) {
    for key in scoping.root_unresolved_references().keys() {
        let name = key.as_str();
        if name.starts_with('$') && name.len() > 1 && !name.starts_with("$$") && !is_rune_name(name) {
            info.store_candidates.push(CompactString::from(&name[1..]));
        }
    }
}
