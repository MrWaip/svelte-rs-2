use oxc_ast::ast::{Expression, ObjectPropertyKind, PropertyKey};
use svelte_span::Span;

pub(crate) fn extract_ce_config_from_expr(
    expr: &Expression<'_>,
    offset: u32,
) -> svelte_parser::ParsedCeConfig {
    let mut config = svelte_parser::ParsedCeConfig {
        tag: None,
        shadow: svelte_parser::CeShadowMode::Open,
        delegates_focus: false,
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
                    config.shadow = svelte_parser::CeShadowMode::None;
                }
                Expression::ObjectExpression(obj) => {
                    for shadow_prop in &obj.properties {
                        let ObjectPropertyKind::ObjectProperty(sp) = shadow_prop else {
                            continue;
                        };
                        let PropertyKey::StaticIdentifier(id) = &sp.key else {
                            continue;
                        };
                        if id.name.as_str() == "delegatesFocus"
                            && let Expression::BooleanLiteral(lit) = &sp.value
                        {
                            config.delegates_focus = lit.value;
                        }
                    }
                }
                _ => {}
            },
            "props" => {
                extract_ce_props(&prop.value, &mut config);
            }
            "extend" => {
                use oxc_span::GetSpan as _;
                let ext_span = prop.value.span();
                config.extend_span =
                    Some(Span::new(ext_span.start + offset, ext_span.end + offset));
            }
            _ => {}
        }
    }

    config
}

fn extract_ce_props(value: &Expression<'_>, config: &mut svelte_parser::ParsedCeConfig) {
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
        let mut prop_cfg = svelte_parser::CePropConfig {
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
