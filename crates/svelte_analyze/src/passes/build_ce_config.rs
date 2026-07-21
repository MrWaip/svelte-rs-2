use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, ObjectPropertyKind, PropertyKey};
use oxc_span::GetSpan as _;
use svelte_ast::{Component, CustomElementConfig};
use svelte_component_semantics::{ComponentSemantics, SymbolId};
use svelte_parser::{CeDomMode, CePropConfig, ParsedCeConfig};
use svelte_span::Span;

use crate::JsAst;

pub(crate) fn build(
    component: &Component,
    parsed: &JsAst<'_>,
    semantics: &ComponentSemantics<'_>,
) -> Option<ParsedCeConfig> {
    let CustomElementConfig::Expression(span) =
        component.options.as_ref()?.custom_element.as_ref()?
    else {
        return None;
    };
    let expr = parsed.pending_expr(span.start)?;

    let mut config = extract(expr, span.start);
    infer_prop_types(&mut config, semantics);
    Some(config)
}

fn infer_prop_types(config: &mut ParsedCeConfig, semantics: &ComponentSemantics<'_>) {
    let Some(instance_scope) = semantics.instance_scope_id() else {
        return;
    };
    for prop in &mut config.props {
        if prop.prop_type.is_some() {
            continue;
        }
        if let Some(sym) = semantics.find_binding(instance_scope, &prop.name)
            && declared_default_is_boolean(semantics, sym)
        {
            prop.prop_type = Some("Boolean".to_string());
        }
    }
}

fn declared_default_is_boolean(semantics: &ComponentSemantics<'_>, sym: SymbolId) -> bool {
    let default = match semantics
        .js_parent_id(semantics.symbol_declaration(sym))
        .and_then(|parent| semantics.js_kind(parent))
    {
        Some(AstKind::VariableDeclarator(decl)) => decl.init.as_ref(),
        Some(AstKind::AssignmentPattern(pat)) => Some(&pat.right),
        _ => None,
    };
    matches!(default, Some(Expression::BooleanLiteral(_)))
}

fn extract(expr: &Expression<'_>, offset: u32) -> ParsedCeConfig {
    let mut config = ParsedCeConfig {
        tag: None,
        shadow: CeDomMode::Open,
        props: Vec::new(),
        extend_span: None,
    };

    let Expression::ObjectExpression(obj) = expr else {
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
            "shadow" => match &prop.value {
                Expression::StringLiteral(lit) if lit.value.as_str() == "none" => {
                    config.shadow = CeDomMode::None;
                }
                Expression::ObjectExpression(_) => {
                    config.shadow = CeDomMode::Custom;
                }
                _ => {}
            },
            "props" => {
                extract_props(&prop.value, &mut config);
            }
            "extend" => {
                let ext_span = prop.value.span();
                config.extend_span =
                    Some(Span::new(ext_span.start + offset, ext_span.end + offset));
            }
            _ => {}
        }
    }

    config
}

fn extract_props(value: &Expression<'_>, config: &mut ParsedCeConfig) {
    let Expression::ObjectExpression(props_obj) = value else {
        return;
    };
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
