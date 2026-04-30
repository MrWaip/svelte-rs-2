use compact_str::CompactString;
use oxc_ast::ast::{CallExpression, Expression};
use oxc_ast_visit::Visit;
use oxc_span::GetSpan as _;

use rustc_hash::FxHashSet;
use svelte_span::Span;

use crate::types::script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
use crate::utils::binding_pattern::collect_binding_names;
use crate::utils::is_simple_expression;

pub const STATE_RUNE_NAME: &str = "$state";
pub const DERIVED_RUNE_NAME: &str = "$derived";
pub const EFFECT_RUNE_NAME: &str = "$effect";
pub const PROPS_RUNE_NAME: &str = "$props";
pub const BINDABLE_RUNE_NAME: &str = "$bindable";
pub const INSPECT_RUNE_NAME: &str = "$inspect";
pub const HOST_RUNE_NAME: &str = "$host";

pub fn extract_script_info(
    program: &oxc_ast::ast::Program<'_>,
    offset: u32,
    source: &str,
    runes: bool,
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
                    if !runes
                        && props_declaration.is_none()
                        && matches!(
                            decl,
                            oxc_ast::ast::Declaration::VariableDeclaration(var_decl)
                                if var_decl.kind == oxc_ast::ast::VariableDeclarationKind::Let
                        )
                    {
                        let oxc_ast::ast::Declaration::VariableDeclaration(var_decl) = decl else {
                            unreachable!()
                        };
                        props_declaration = collect_legacy_export_props(var_decl, offset, source);
                        collect_declarations_from_declaration(
                            decl,
                            offset,
                            source,
                            &mut declarations,
                            &mut props_declaration,
                        );
                    } else {
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

fn collect_legacy_export_props(
    decl: &oxc_ast::ast::VariableDeclaration<'_>,
    offset: u32,
    source: &str,
) -> Option<PropsDeclaration> {
    let mut props = Vec::new();

    for declarator in &decl.declarations {
        let Some(local_name) = extract_binding_name(&declarator.id) else {
            continue;
        };
        let prop_name = local_name.clone();
        let (default_span, default_text, is_bindable, is_simple_default) =
            if let Some(init) = &declarator.init {
                let sp = init.span();
                (
                    Some(Span::new(sp.start + offset, sp.end + offset)),
                    Some(source[sp.start as usize..sp.end as usize].to_string()),
                    false,
                    is_simple_expression(init),
                )
            } else {
                (None, None, false, true)
            };

        props.push(PropInfo {
            local_name,
            prop_name,
            default_span,
            default_text,
            is_bindable,
            is_rest: false,
            is_simple_default,
        });
    }

    if props.is_empty() {
        None
    } else {
        Some(PropsDeclaration {
            props,
            is_identifier_pattern: false,
            declaration_spans: vec![Span::new(decl.span.start + offset, decl.span.end + offset)],
            rest_pattern_span: None,
        })
    }
}

pub fn detect_rune(expr: &Expression<'_>) -> Option<RuneKind> {
    if let Expression::CallExpression(call) = expr {
        return detect_rune_from_call(call);
    }
    None
}

pub(crate) fn detect_rune_from_call(call: &CallExpression<'_>) -> Option<RuneKind> {
    match &call.callee {
        Expression::Identifier(ident) => match ident.name.as_str() {
            STATE_RUNE_NAME => Some(RuneKind::State),
            DERIVED_RUNE_NAME => Some(RuneKind::Derived),
            EFFECT_RUNE_NAME => Some(RuneKind::Effect),
            PROPS_RUNE_NAME => Some(RuneKind::Props),
            BINDABLE_RUNE_NAME => Some(RuneKind::Bindable),
            INSPECT_RUNE_NAME => Some(RuneKind::Inspect),
            HOST_RUNE_NAME => Some(RuneKind::Host),
            _ => None,
        },
        Expression::StaticMemberExpression(member) => {
            if let Expression::Identifier(obj) = &member.object {
                let prop = member.property.name.as_str();
                match (obj.name.as_str(), prop) {
                    (DERIVED_RUNE_NAME, "by") => Some(RuneKind::DerivedBy),
                    (STATE_RUNE_NAME, "raw") => Some(RuneKind::StateRaw),
                    (STATE_RUNE_NAME, "eager") => Some(RuneKind::StateEager),
                    (EFFECT_RUNE_NAME, "pre") => Some(RuneKind::EffectPre),
                    (EFFECT_RUNE_NAME, "root") => Some(RuneKind::EffectRoot),
                    (EFFECT_RUNE_NAME, "tracking") => Some(RuneKind::EffectTracking),
                    (EFFECT_RUNE_NAME, "pending") => Some(RuneKind::EffectPending),
                    (PROPS_RUNE_NAME, "id") => Some(RuneKind::PropsId),
                    (INSPECT_RUNE_NAME, "trace") => Some(RuneKind::InspectTrace),
                    _ => None,
                }
            } else if member.property.name == "with" {
                if let Expression::CallExpression(inner) = &member.object
                    && let Expression::Identifier(id) = &inner.callee
                    && id.name == INSPECT_RUNE_NAME
                {
                    return Some(RuneKind::InspectWith);
                }
                None
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn is_rune_name(name: &str) -> bool {
    svelte_ast::is_rune_name(name)
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
                        declaration_spans: vec![Span::new(
                            decl.span.start + offset,
                            decl.span.end + offset,
                        )],
                        rest_pattern_span: None,
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

                    let mut rest_pattern_span = None;
                    if let Some(rest) = &obj_pat.rest
                        && let oxc_ast::ast::BindingPattern::BindingIdentifier(ident) =
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
                        rest_pattern_span =
                            Some(Span::new(rest.span.start + offset, rest.span.end + offset));
                    }

                    *props_declaration = Some(PropsDeclaration {
                        props,
                        is_identifier_pattern: false,
                        declaration_spans: vec![Span::new(
                            decl.span.start + offset,
                            decl.span.end + offset,
                        )],
                        rest_pattern_span,
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
                if let Some(rune_kind) = rune
                    && matches!(
                        rune_kind,
                        RuneKind::State
                            | RuneKind::StateRaw
                            | RuneKind::Derived
                            | RuneKind::DerivedBy
                    )
                {
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
                        let decl_span =
                            Span::new(declarator.span.start + offset, declarator.span.end + offset);
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
            _ => {}
        }
    }
}

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

fn extract_call_arg_literal(expr: &Expression<'_>) -> Option<CompactString> {
    let Expression::CallExpression(call) = expr else {
        return None;
    };
    let arg = call.arguments.first()?;
    let arg_expr = arg.as_expression()?;
    extract_literal(arg_expr)
}

fn extract_property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<CompactString> {
    crate::utils::property_key_static_name(key).map(CompactString::from)
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
        if let Expression::CallExpression(call) = right
            && let Expression::Identifier(ident) = &call.callee
            && ident.name.as_str() == "$bindable"
        {
            let (default_span, default_text, is_simple) = if let Some(arg) = call.arguments.first()
            {
                let sp = arg.span();
                let text = &source[sp.start as usize..sp.end as usize];
                let expr = arg.as_expression().expect("argument should be expression");
                (
                    Some(Span::new(sp.start + offset, sp.end + offset)),
                    Some(text.to_string()),
                    is_simple_expression(expr),
                )
            } else {
                (None, None, true)
            };
            return (default_span, default_text, true, is_simple);
        }
        let sp = right.span();
        let text = &source[sp.start as usize..sp.end as usize];
        let is_simple = is_simple_expression(right);
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
