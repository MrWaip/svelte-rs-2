use oxc_ast::ast::{Expression, Statement};

use crate::builder::Arg;

use super::{
    PROPS_IS_BINDABLE, PROPS_IS_IMMUTABLE, PROPS_IS_LAZY_INITIAL, PROPS_IS_RUNES,
    PROPS_IS_UPDATED, ScriptTransformer,
};

impl<'b, 'a> ScriptTransformer<'b, 'a> {
    pub(super) fn is_props_declaration(decl: &oxc_ast::ast::VariableDeclaration<'a>) -> bool {
        decl.declarations.iter().any(|d| {
            if let oxc_ast::ast::BindingPattern::ObjectPattern(_) = &d.id {
                if let Some(init) = &d.init {
                    if let Expression::CallExpression(call) = init {
                        if let Expression::Identifier(ident) = &call.callee {
                            return ident.name.as_str() == "$props";
                        }
                    }
                }
            }
            false
        })
    }

    pub(super) fn is_props_id_declaration(decl: &oxc_ast::ast::VariableDeclaration<'a>) -> bool {
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

    pub(super) fn gen_props_statements(&mut self) -> Vec<Statement<'a>> {
        let Some(props_gen) = &self.props_gen else {
            return vec![];
        };

        let mut declarators: Vec<(&str, Expression<'a>)> = Vec::new();
        let mut seen_names: Vec<String> = vec![
            "$$slots".to_string(),
            "$$events".to_string(),
            "$$legacy".to_string(),
        ];

        for (i, prop) in props_gen.props.iter().enumerate() {
            seen_names.push(prop.prop_name.clone());

            if prop.is_rest {
                let excluded: Vec<Arg<'a, '_>> = seen_names.iter()
                    .filter(|n| *n != &prop.local_name)
                    .map(|n| Arg::Str(n.clone()))
                    .collect();
                let arr_expr = self.b.array_from_args(excluded);
                let init = self.b.call_expr("$.rest_props", [
                    Arg::Ident("$$props"),
                    Arg::Expr(arr_expr),
                ]);
                declarators.push((self.b.alloc_str(&prop.local_name), init));
                continue;
            }

            if !prop.is_prop_source {
                continue;
            }

            let mut flags: u32 = PROPS_IS_IMMUTABLE | PROPS_IS_RUNES;
            if prop.is_bindable {
                flags |= PROPS_IS_BINDABLE;
            }
            if prop.is_mutated {
                flags |= PROPS_IS_UPDATED;
            }

            let mut args: Vec<Arg<'a, '_>> = vec![
                Arg::Ident("$$props"),
                Arg::Str(prop.prop_name.clone()),
            ];

            if prop.default_text.is_some() {
                if prop.is_lazy_default {
                    flags |= PROPS_IS_LAZY_INITIAL;
                }

                args.push(Arg::Num(flags as f64));

                let default_expr = self.prop_default_exprs.get_mut(i)
                    .and_then(|e| e.take())
                    .unwrap_or_else(|| panic!("prop_default_exprs missing for prop {}", prop.local_name));
                // Wrap $bindable() defaults in $.proxy() when needed
                let default_expr = if prop.is_bindable && svelte_transform::rune_refs::should_proxy(&default_expr) {
                    self.b.call_expr("$.proxy", [Arg::Expr(default_expr)])
                } else {
                    default_expr
                };
                if !prop.is_lazy_default {
                    args.push(Arg::Expr(default_expr));
                } else {
                    args.push(Arg::Expr(super::traverse::wrap_lazy(self.b, default_expr)));
                }
            } else {
                if flags != 0 {
                    args.push(Arg::Num(flags as f64));
                }
            }

            let name: &'a str = self.b.alloc_str(&prop.local_name);
            declarators.push((name, self.b.call_expr("$.prop", args)));
        }

        if declarators.is_empty() {
            return vec![];
        }

        vec![self.b.let_multi_stmt(declarators)]
    }
}
