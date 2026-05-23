use oxc_ast::ast::{
    Argument, BindingPattern, Expression, PropertyKey, Statement, VariableDeclaration,
    VariableDeclarationKind,
};
use svelte_analyze::{
    AnalysisData, BINDABLE_RUNE_NAME, BindingSemantics, DeclaratorSemantics, PropBindingKind,
    PropBindingSemantics, PropDefaultEmit, PropEmitMode, PropsDeclKind,
};
use svelte_analyze::scope::SymbolId;

use svelte_ast_builder::Arg;

use super::model::ComponentTransformer;
use super::{
    PROPS_IS_BINDABLE, PROPS_IS_IMMUTABLE, PROPS_IS_LAZY_INITIAL, PROPS_IS_RUNES, PROPS_IS_UPDATED,
};

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    pub(crate) fn try_gen_props_declaration_semantic(
        &mut self,
        decl: &mut VariableDeclaration<'a>,
    ) -> Option<Vec<Statement<'a>>> {
        let analysis = self.analysis?;
        if decl.declarations.len() != 1 {
            return None;
        }

        let declarator = &mut decl.declarations[0];
        let root_node = declarator.node_id();
        let declarator_sem = analysis.declarator_semantics(root_node);
        match &mut declarator.id {
            BindingPattern::BindingIdentifier(id) => {
                let DeclaratorSemantics::PropsIdentifier { sym, kind } = declarator_sem else {
                    return None;
                };
                let BindingSemantics::Prop(prop) = analysis.binding_semantics(sym) else {
                    return None;
                };
                let arr_expr = self.b.array_from_args(
                    base_rest_excluded(prop.lowering_mode)
                        .into_iter()
                        .map(Arg::Str)
                        .collect::<Vec<_>>(),
                );
                let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$props"), Arg::Expr(arr_expr)];
                if self.dev {
                    args.push(Arg::Str(id.name.to_string()));
                }
                let init = self.b.call_expr("$.rest_props", args);
                Some(vec![self.b.var_decl_multi_stmt(
                    vec![(self.b.alloc_str(id.name.as_str()), init)],
                    var_kind_from_props_kind(kind),
                )])
            }
            BindingPattern::ObjectPattern(obj) => {
                let DeclaratorSemantics::PropsObject {
                    leaves,
                    has_rest,
                    kind,
                } = declarator_sem
                else {
                    return None;
                };

                let property_count = if has_rest {
                    leaves.len().saturating_sub(1)
                } else {
                    leaves.len()
                };
                let lowering_mode = self.prop_lowering_mode_from_first(analysis, &leaves)?;

                let mut excluded = base_rest_excluded(lowering_mode);
                if obj.properties.len() != property_count || obj.rest.is_some() != has_rest {
                    return None;
                }

                for prop in &obj.properties {
                    let prop_name = static_prop_key_name(&prop.key)?;
                    excluded.push(prop_name.to_string());
                }

                let mut declarators = Vec::new();
                for (prop, leaf_sym) in obj
                    .properties
                    .iter_mut()
                    .zip(leaves.iter().take(property_count).copied())
                {
                    let BindingSemantics::Prop(leaf_prop) = analysis.binding_semantics(leaf_sym)
                    else {
                        return None;
                    };
                    let prop_name = static_prop_key_name(&prop.key)?;

                    let (local_name, default_expr): (String, Option<Expression<'a>>) =
                        match &mut prop.value {
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

                    match leaf_prop.kind {
                        PropBindingKind::NonSource => {}
                        PropBindingKind::Source {
                            bindable,
                            updated,
                            default_lowering,
                            default_needs_proxy,
                        } => {
                            let default_expr = default_expr
                                .and_then(|expr| prop_assignment_default_expr(expr, bindable));
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
                                || matches!(lowering_mode, PropEmitMode::CustomElement)
                            {
                                flags |= PROPS_IS_UPDATED;
                            }

                            let mut args: Vec<Arg<'a, '_>> =
                                vec![Arg::Ident("$$props"), Arg::Str(prop_name.to_string())];
                            match default_lowering {
                                PropDefaultEmit::None => {
                                    if bindable && !updated {
                                        continue;
                                    }
                                    if flags != 0 {
                                        args.push(Arg::Num(flags as f64));
                                    }
                                }
                                PropDefaultEmit::Eager
                                | PropDefaultEmit::Lazy
                                | PropDefaultEmit::LazyAccessor => {
                                    if matches!(
                                        default_lowering,
                                        PropDefaultEmit::Lazy | PropDefaultEmit::LazyAccessor
                                    ) {
                                        flags |= PROPS_IS_LAZY_INITIAL;
                                    }
                                    args.push(Arg::Num(flags as f64));

                                    let default_expr = default_expr.unwrap_or_else(|| {
                                        panic!("default expr missing for prop {}", local_name)
                                    });
                                    let default_expr = if default_needs_proxy {
                                        let proxied =
                                            self.b.call_expr("$.proxy", [Arg::Expr(default_expr)]);
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
                                    let default_expr =
                                        if matches!(default_lowering, PropDefaultEmit::Eager) {
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
                        PropBindingKind::Identifier | PropBindingKind::Rest => {
                            return None;
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
                    let mut args: Vec<Arg<'a, '_>> =
                        vec![Arg::Ident("$$props"), Arg::Expr(arr_expr)];
                    if self.dev {
                        args.push(Arg::Str(id.name.to_string()));
                    }
                    declarators.push((
                        self.b.alloc_str(id.name.as_str()),
                        self.b.call_expr("$.rest_props", args),
                    ));
                }

                Some(if declarators.is_empty() {
                    vec![]
                } else {
                    vec![self
                        .b
                        .var_decl_multi_stmt(declarators, var_kind_from_props_kind(kind))]
                })
            }
            _ => None,
        }
    }

    fn prop_lowering_mode_from_first(
        &self,
        analysis: &AnalysisData<'a>,
        leaves: &[SymbolId],
    ) -> Option<PropEmitMode> {
        let sym = *leaves.first()?;
        let BindingSemantics::Prop(PropBindingSemantics { lowering_mode, .. }) =
            analysis.binding_semantics(sym)
        else {
            return None;
        };
        Some(lowering_mode)
    }

    pub(crate) fn is_props_declaration(decl: &VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            let is_props_pattern = matches!(
                &d.id,
                BindingPattern::ObjectPattern(_)
                    | BindingPattern::BindingIdentifier(_)
            );
            if is_props_pattern
                && let Some(Expression::CallExpression(call)) = &d.init
                && let Expression::Identifier(ident) = &call.callee
            {
                return ident.name.as_str() == "$props";
            }
            false
        })
    }

    pub(crate) fn is_props_id_declaration(decl: &VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            if let BindingPattern::BindingIdentifier(_) = &d.id
                && let Some(Expression::CallExpression(call)) = &d.init
                && let Expression::StaticMemberExpression(member) = &call.callee
                && let Expression::Identifier(obj) = &member.object
            {
                return obj.name.as_str() == "$props" && member.property.name.as_str() == "id";
            }
            false
        })
    }
}

fn var_kind_from_props_kind(kind: PropsDeclKind) -> VariableDeclarationKind {
    match kind {
        PropsDeclKind::Const => VariableDeclarationKind::Const,
        PropsDeclKind::Let => VariableDeclarationKind::Let,
        PropsDeclKind::Var => VariableDeclarationKind::Var,
    }
}

fn base_rest_excluded(lowering_mode: PropEmitMode) -> Vec<String> {
    let mut excluded = vec![
        "$$slots".to_string(),
        "$$events".to_string(),
        "$$legacy".to_string(),
    ];
    if matches!(lowering_mode, PropEmitMode::CustomElement) {
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

    let is_bindable_call = matches!(
        expr.get_inner_expression(),
        Expression::CallExpression(c)
            if matches!(
                c.callee.get_inner_expression(),
                Expression::Identifier(id) if id.name.as_str() == BINDABLE_RUNE_NAME,
            )
    );
    if !is_bindable_call {
        return Some(expr);
    }
    let Expression::CallExpression(mut call) = expr.into_inner_expression() else {
        return None;
    };

    call.arguments.drain(..).next().and_then(|arg| match arg {
        Argument::SpreadElement(_) => None,
        _ => Some(arg.into_expression()),
    })
}

fn static_prop_key_name<'a>(key: &'a PropertyKey<'a>) -> Option<&'a str> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str()),
        PropertyKey::StringLiteral(str) => Some(str.value.as_str()),
        _ => None,
    }
}
