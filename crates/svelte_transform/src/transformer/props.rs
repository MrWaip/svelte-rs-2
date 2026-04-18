use oxc_ast::ast::{Argument, BindingPattern, Expression, Statement};
use svelte_analyze::{
    BINDABLE_RUNE_NAME, DeclarationSemantics, PropDeclarationKind, PropDefaultLowering,
    PropLoweringMode, PropsObjectPropertySemantics,
};

use svelte_ast_builder::Arg;

use super::model::ComponentTransformer;
use super::{
    PROPS_IS_BINDABLE, PROPS_IS_IMMUTABLE, PROPS_IS_LAZY_INITIAL, PROPS_IS_RUNES, PROPS_IS_UPDATED,
};

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    pub(crate) fn try_gen_props_declaration_semantic(
        &mut self,
        decl: &mut oxc_ast::ast::VariableDeclaration<'a>,
    ) -> Option<Vec<Statement<'a>>> {
        let analysis = self.analysis?;
        if decl.declarations.len() != 1 {
            return None;
        }

        let declarator = &mut decl.declarations[0];
        let root_node = declarator.node_id();
        match &mut declarator.id {
            BindingPattern::BindingIdentifier(id) => {
                match analysis.declaration_semantics(root_node) {
                    DeclarationSemantics::Prop(prop)
                        if matches!(prop.kind, PropDeclarationKind::Identifier) =>
                    {
                        let arr_expr = self.b.array_from_args(
                            base_rest_excluded(prop.lowering_mode)
                                .into_iter()
                                .map(Arg::Str)
                                .collect::<Vec<_>>(),
                        );
                        let init = self
                            .b
                            .call_expr("$.rest_props", [Arg::Ident("$$props"), Arg::Expr(arr_expr)]);
                        Some(vec![self.b.const_stmt(id.name.as_str(), init)])
                    }
                    _ => None,
                }
            }
            BindingPattern::ObjectPattern(obj) => {
                let DeclarationSemantics::Prop(root_prop) = analysis.declaration_semantics(root_node) else {
                    return None;
                };
                let lowering_mode = root_prop.lowering_mode;
                let PropDeclarationKind::Object { properties, has_rest } = root_prop.kind
                else {
                    return None;
                };

                let mut excluded = base_rest_excluded(lowering_mode);
                if obj.properties.len() != properties.len() || obj.rest.is_some() != has_rest {
                    return None;
                }

                for prop in &obj.properties {
                    let prop_name = static_prop_key_name(&prop.key)?;
                    excluded.push(prop_name.to_string());
                }

                let mut declarators = Vec::new();
                for (prop, property_semantics) in obj.properties.iter_mut().zip(properties.into_iter()) {
                    let prop_name = static_prop_key_name(&prop.key)?;

                    let (local_name, default_expr): (String, Option<Expression<'a>>) = match &mut prop.value {
                        BindingPattern::BindingIdentifier(id) => {
                            (id.name.as_str().to_string(), None)
                        }
                        BindingPattern::AssignmentPattern(assign) => {
                            let BindingPattern::BindingIdentifier(id) = &assign.left else {
                                return None;
                            };
                            (
                                id.name.as_str().to_string(),
                                Some(self.b.move_expr(&mut assign.right)),
                            )
                        }
                        _ => return None,
                    };

                    match property_semantics {
                        PropsObjectPropertySemantics::NonSource => {}
                        PropsObjectPropertySemantics::Source {
                            bindable,
                            updated,
                            default_lowering,
                            default_needs_proxy,
                        } => {
                            let default_expr =
                                default_expr.and_then(|expr| prop_assignment_default_expr(expr, bindable));
                            let mut flags: u32 = 0;
                            if self.immutable || self.runes {
                                flags |= PROPS_IS_IMMUTABLE;
                            }
                            if self.runes {
                                flags |= PROPS_IS_RUNES;
                            }
                            if bindable || !self.runes {
                                flags |= PROPS_IS_BINDABLE;
                            }
                            if self.accessors
                                || updated
                                || matches!(lowering_mode, PropLoweringMode::CustomElement)
                            {
                                flags |= PROPS_IS_UPDATED;
                            }

                            let mut args: Vec<Arg<'a, '_>> = vec![
                                Arg::Ident("$$props"),
                                Arg::Str(prop_name.to_string()),
                            ];
                            match default_lowering {
                                PropDefaultLowering::None => {
                                    if bindable && !updated {
                                        continue;
                                    }
                                    if flags != 0 {
                                        args.push(Arg::Num(flags as f64));
                                    }
                                }
                                PropDefaultLowering::Eager | PropDefaultLowering::Lazy => {
                                    if matches!(default_lowering, PropDefaultLowering::Lazy) {
                                        flags |= PROPS_IS_LAZY_INITIAL;
                                    }
                                    args.push(Arg::Num(flags as f64));

                                    let default_expr = default_expr.unwrap_or_else(|| {
                                        panic!("default expr missing for prop {}", local_name)
                                    });
                                    let default_expr = if default_needs_proxy {
                                        let proxied = self
                                            .b
                                            .call_expr("$.proxy", [Arg::Expr(default_expr)]);
                                        if self.dev {
                                            self.b.call_expr(
                                                "$.tag_proxy",
                                                [Arg::Expr(proxied), Arg::Str(local_name.clone())],
                                            )
                                        } else {
                                            proxied
                                        }
                                    } else {
                                        default_expr
                                    };
                                    let default_expr = if matches!(
                                        default_lowering,
                                        PropDefaultLowering::Eager
                                    ) {
                                        default_expr
                                    } else {
                                        super::derived::wrap_lazy(self.b, default_expr)
                                    };
                                    args.push(Arg::Expr(default_expr));
                                }
                            }

                            declarators.push((
                                self.b.alloc_str(&local_name),
                                self.b.call_expr("$.prop", args),
                            ));
                        }
                    }
                }

                if let Some(rest) = &obj.rest {
                    let BindingPattern::BindingIdentifier(id) = &rest.argument else {
                        return None;
                    };
                    let arr_expr = self.b.array_from_args(
                        excluded.iter().cloned().map(Arg::Str).collect::<Vec<_>>(),
                    );
                    declarators.push((
                        self.b.alloc_str(id.name.as_str()),
                        self.b.call_expr(
                            "$.rest_props",
                            [Arg::Ident("$$props"), Arg::Expr(arr_expr)],
                        ),
                    ));
                }

                Some(if declarators.is_empty() {
                    vec![]
                } else {
                    vec![self.b.let_multi_stmt(declarators)]
                })
            }
            _ => None,
        }
    }

    pub(crate) fn is_props_declaration(decl: &oxc_ast::ast::VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            let is_props_pattern = matches!(
                &d.id,
                oxc_ast::ast::BindingPattern::ObjectPattern(_)
                    | oxc_ast::ast::BindingPattern::BindingIdentifier(_)
            );
            if is_props_pattern {
                if let Some(Expression::CallExpression(call)) = &d.init {
                    if let Expression::Identifier(ident) = &call.callee {
                        return ident.name.as_str() == "$props";
                    }
                }
            }
            false
        })
    }

    pub(crate) fn is_props_id_declaration(decl: &oxc_ast::ast::VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            if let oxc_ast::ast::BindingPattern::BindingIdentifier(_) = &d.id {
                if let Some(Expression::CallExpression(call)) = &d.init {
                    if let Expression::StaticMemberExpression(member) = &call.callee {
                        if let Expression::Identifier(obj) = &member.object {
                            return obj.name.as_str() == "$props"
                                && member.property.name.as_str() == "id";
                        }
                    }
                }
            }
            false
        })
    }

}

fn base_rest_excluded(lowering_mode: PropLoweringMode) -> Vec<String> {
    let mut excluded = vec![
        "$$slots".to_string(),
        "$$events".to_string(),
        "$$legacy".to_string(),
    ];
    if matches!(lowering_mode, PropLoweringMode::CustomElement) {
        excluded.push("$$host".to_string());
    }
    excluded
}

fn prop_assignment_default_expr<'a>(
    expr: Expression<'a>,
    bindable: bool,
) -> Option<Expression<'a>> {
    if !bindable {
        return Some(expr);
    }

    let Expression::CallExpression(mut call) = expr else {
        return Some(expr);
    };
    let Expression::Identifier(ident) = &call.callee else {
        return Some(Expression::CallExpression(call));
    };
    if ident.name.as_str() != BINDABLE_RUNE_NAME {
        return Some(Expression::CallExpression(call));
    }

    let default_expr = call.arguments.drain(..).next().and_then(|arg| match arg {
        Argument::SpreadElement(_) => None,
        _ => Some(arg.into_expression()),
    });
    default_expr
}

fn static_prop_key_name<'a>(key: &'a oxc_ast::ast::PropertyKey<'a>) -> Option<&'a str> {
    match key {
        oxc_ast::ast::PropertyKey::StaticIdentifier(id) => Some(id.name.as_str()),
        oxc_ast::ast::PropertyKey::StringLiteral(str) => Some(str.value.as_str()),
        _ => None,
    }
}
