use compact_str::CompactString;
use oxc_ast::ast::{
    BindingPattern, CallExpression, Declaration, Expression, Function, IdentifierReference,
    Program, PropertyKey, Statement, VariableDeclaration, VariableDeclarationKind,
};
use oxc_ast_visit::Visit;
use oxc_span::GetSpan as _;

use rustc_hash::FxHashSet;
use svelte_span::Span;

use crate::types::script::{
    DeclarationInfo, DeclarationKind, ExportInfo, PropInfo, PropsDeclaration, RuneKind, ScriptInfo,
};
use crate::utils::binding_pattern::collect_binding_names;
use crate::utils::is_simple_expression;
use crate::utils::property_key_static_name;

pub const STATE_RUNE_NAME: &str = "$state";
pub const DERIVED_RUNE_NAME: &str = "$derived";
pub const EFFECT_RUNE_NAME: &str = "$effect";
pub const PROPS_RUNE_NAME: &str = "$props";
pub const BINDABLE_RUNE_NAME: &str = "$bindable";
pub const INSPECT_RUNE_NAME: &str = "$inspect";
pub const HOST_RUNE_NAME: &str = "$host";

pub fn extract_script_info(
    program: &Program<'_>,
    source: &str,
    runes: bool,
) -> ScriptInfo {
    let mut declarations = Vec::new();
    let mut props_declaration = None;
    let mut exports = Vec::new();

    for stmt in &program.body {
        use Statement;

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
                            Declaration::VariableDeclaration(var_decl)
                                if var_decl.kind == VariableDeclarationKind::Let
                        )
                    {
                        let Declaration::VariableDeclaration(var_decl) = decl else {
                            unreachable!()
                        };
                        props_declaration = collect_legacy_export_props(var_decl, source);
                        collect_declarations_from_declaration(
                            decl,
                            source,
                            runes,
                            &mut declarations,
                            &mut props_declaration,
                        );
                    } else {
                        collect_export_names_from_declaration(decl, &mut exports);
                        collect_declarations_from_declaration(
                            decl,
                            source,
                            runes,
                            &mut declarations,
                            &mut props_declaration,
                        );
                    }
                }
            }
            Statement::VariableDeclaration(decl) => {
                collect_var_declarations(
                    decl,
                    source,
                    runes,
                    &mut declarations,
                    &mut props_declaration,
                );
            }
            Statement::FunctionDeclaration(func) => {
                collect_func_declaration(func, &mut declarations);
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
    decl: &VariableDeclaration<'_>,
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
                    Some(Span::new(sp.start, sp.end)),
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
            declaration_spans: vec![Span::new(decl.span.start, decl.span.end)],
            rest_pattern_span: None,
        })
    }
}

fn detect_rune_in_runes_mode(expr: &Expression<'_>, runes: bool) -> Option<RuneKind> {
    if runes { detect_rune(expr) } else { None }
}

pub fn detect_rune(expr: &Expression<'_>) -> Option<RuneKind> {
    if let Expression::CallExpression(call) = expr.get_inner_expression() {
        return detect_rune_from_call(call);
    }
    None
}

pub fn detect_rune_from_call(call: &CallExpression<'_>) -> Option<RuneKind> {
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
                    (STATE_RUNE_NAME, "snapshot") => Some(RuneKind::StateSnapshot),
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
    decl: &Declaration<'_>,
    exports: &mut Vec<ExportInfo>,
) {
    match decl {
        Declaration::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                if let BindingPattern::BindingIdentifier(ident) = &declarator.id {
                    exports.push(ExportInfo {
                        name: CompactString::from(ident.name.as_str()),
                        alias: None,
                    });
                }
            }
        }
        Declaration::FunctionDeclaration(func) => {
            if let Some(ident) = &func.id {
                exports.push(ExportInfo {
                    name: CompactString::from(ident.name.as_str()),
                    alias: None,
                });
            }
        }
        Declaration::ClassDeclaration(cls) => {
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
    decl: &Declaration<'_>,
    source: &str,
    runes: bool,
    declarations: &mut Vec<DeclarationInfo>,
    props_declaration: &mut Option<PropsDeclaration>,
) {
    match decl {
        Declaration::VariableDeclaration(var_decl) => {
            collect_var_declarations(var_decl, source, runes, declarations, props_declaration);
        }
        Declaration::FunctionDeclaration(func) => {
            collect_func_declaration(func, declarations);
        }
        _ => {}
    }
}

fn collect_func_declaration(
    func: &Function<'_>,
    declarations: &mut Vec<DeclarationInfo>,
) {
    if let Some(ident) = &func.id {
        declarations.push(DeclarationInfo {
            name: CompactString::from(ident.name.as_str()),
            span: Span::new(ident.span.start, ident.span.end),
            kind: DeclarationKind::Function,
            init_span: None,
            is_rune: None,
            rune_init_refs: vec![],
            init_literal: None,
            init_known: true,
        });
    }
}

fn collect_var_declarations(
    decl: &VariableDeclaration<'_>,
    source: &str,
    runes: bool,
    declarations: &mut Vec<DeclarationInfo>,
    props_declaration: &mut Option<PropsDeclaration>,
) {
    let kind = match decl.kind {
        VariableDeclarationKind::Let => DeclarationKind::Let,
        VariableDeclarationKind::Const => DeclarationKind::Const,
        VariableDeclarationKind::Var => DeclarationKind::Var,
        _ => DeclarationKind::Var,
    };

    for declarator in &decl.declarations {
        match &declarator.id {
            BindingPattern::BindingIdentifier(ident) => {
                let name = CompactString::from(ident.name.as_str());
                let decl_span = Span::new(ident.span.start, ident.span.end);

                let (init_span, is_rune, rune_init_refs, init_literal, init_known) = if let Some(
                    init,
                ) =
                    &declarator.init
                {
                    let init_sp = Span::new(init.span().start, init.span().end);
                    let rune = detect_rune_in_runes_mode(init, runes);
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
                    let known = rune.is_none() && extract_init_known(init);
                    (Some(init_sp), rune, refs, literal, known)
                } else {
                    (None, None, vec![], None, false)
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
                            decl.span.start,
                            decl.span.end,
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
                    init_known,
                });
            }
            BindingPattern::ObjectPattern(obj_pat) => {
                let rune = declarator
                    .init
                    .as_ref()
                    .and_then(|init| detect_rune_in_runes_mode(init, runes));

                if rune == Some(RuneKind::Props) {
                    let mut props = Vec::new();

                    for prop in &obj_pat.properties {
                        let key_name = extract_property_key_name(&prop.key);
                        let Some(key_name) = key_name else { continue };

                        let local_name = extract_binding_name(&prop.value);
                        let local_name = local_name.unwrap_or_else(|| key_name.clone());

                        let (default_span, default_text, is_bindable, is_simple_default) =
                            extract_prop_default(&prop.value, source);

                        let decl_span = Span::new(prop.span.start, prop.span.end);

                        declarations.push(DeclarationInfo {
                            name: local_name.clone(),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::Props),
                            rune_init_refs: vec![],
                            init_literal: None,
                            init_known: false,
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
                        && let BindingPattern::BindingIdentifier(ident) =
                            &rest.argument
                    {
                        let rest_name = CompactString::from(ident.name.as_str());
                        let decl_span =
                            Span::new(ident.span.start, ident.span.end);
                        declarations.push(DeclarationInfo {
                            name: rest_name.clone(),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(RuneKind::Props),
                            rune_init_refs: vec![],
                            init_literal: None,
                            init_known: false,
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
                            Some(Span::new(rest.span.start, rest.span.end));
                    }

                    *props_declaration = Some(PropsDeclaration {
                        props,
                        is_identifier_pattern: false,
                        declaration_spans: vec![Span::new(
                            decl.span.start,
                            decl.span.end,
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
                            Span::new(declarator.span.start, declarator.span.end);
                        declarations.push(DeclarationInfo {
                            name: CompactString::from(&name),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: rune,
                            rune_init_refs: rune_init_refs.clone(),
                            init_literal: None,
                            init_known: false,
                        });
                    }
                }
            }
            BindingPattern::ArrayPattern(_) => {
                let rune = declarator
                    .init
                    .as_ref()
                    .and_then(|init| detect_rune_in_runes_mode(init, runes));
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
                            Span::new(declarator.span.start, declarator.span.end);
                        declarations.push(DeclarationInfo {
                            name: CompactString::from(&name),
                            span: decl_span,
                            kind,
                            init_span: None,
                            is_rune: Some(rune_kind),
                            rune_init_refs: rune_init_refs.clone(),
                            init_literal: None,
                            init_known: false,
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

fn extract_init_known(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_) => true,
        Expression::TemplateLiteral(t) => t.expressions.is_empty(),
        Expression::UnaryExpression(u) => extract_init_known(&u.argument),
        Expression::BinaryExpression(b) => {
            extract_init_known(&b.left) && extract_init_known(&b.right)
        }
        Expression::LogicalExpression(l) => {
            extract_init_known(&l.left) && extract_init_known(&l.right)
        }
        Expression::ParenthesizedExpression(p) => extract_init_known(&p.expression),
        _ => false,
    }
}

fn extract_property_key_name(key: &PropertyKey<'_>) -> Option<CompactString> {
    property_key_static_name(key).map(CompactString::from)
}

fn extract_binding_name(pattern: &BindingPattern<'_>) -> Option<CompactString> {
    pattern
        .get_binding_identifier()
        .map(|id| CompactString::from(id.name.as_str()))
}

fn extract_prop_default(
    pattern: &BindingPattern<'_>,
    source: &str,
) -> (Option<Span>, Option<String>, bool, bool) {
    if let BindingPattern::AssignmentPattern(assign) = pattern {
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
                    Some(Span::new(sp.start, sp.end)),
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
            Some(Span::new(sp.start, sp.end)),
            Some(text.to_string()),
            false,
            is_simple,
        )
    } else {
        (None, None, false, true)
    }
}

fn collect_derived_refs(expr: &Expression<'_>) -> Vec<CompactString> {
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
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
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
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
