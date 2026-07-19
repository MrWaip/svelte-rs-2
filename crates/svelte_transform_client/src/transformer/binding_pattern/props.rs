use std::collections::HashMap;
use std::mem;

use oxc_allocator::Vec as OxcVec;
use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, BindingPattern, Expression, VariableDeclarationKind, VariableDeclarator,
};
use oxc_span::SPAN;

use svelte_analyze::{BindingSemantics, PropBindingKind, PropDefaultKind, PropEmitMode};
use svelte_ast_builder::Arg;
use svelte_component_semantics::{OriginKind, SymbolId, walk_bindings};

use crate::data::{RestExcludeKey, RestExcludes};

use super::super::model::ComponentTransformer;
use super::super::{
    PROPS_IS_BINDABLE, PROPS_IS_IMMUTABLE, PROPS_IS_LAZY_INITIAL, PROPS_IS_RUNES, PROPS_IS_UPDATED,
};

enum ExcludedKey {
    Str(String),
    Num(f64),
}

impl ExcludedKey {
    fn into_rest_key(self) -> RestExcludeKey {
        match self {
            ExcludedKey::Str(s) => RestExcludeKey::Str(s),
            ExcludedKey::Num(n) => RestExcludeKey::Num(n),
        }
    }
}

fn prop_origin_key_arg<'a, 'short>(alias: &str, kind: OriginKind) -> Arg<'a, 'short> {
    match kind {
        OriginKind::Numeric => Arg::Num(alias.parse::<f64>().unwrap_or(0.0)),
        OriginKind::Ident | OriginKind::String => Arg::Str(alias.to_string()),
    }
}

fn prop_excluded_key(alias: &str, kind: OriginKind) -> ExcludedKey {
    match kind {
        OriginKind::Numeric => ExcludedKey::Num(alias.parse::<f64>().unwrap_or(0.0)),
        OriginKind::Ident | OriginKind::String => ExcludedKey::Str(alias.to_string()),
    }
}

fn base_rest_excluded(emit_mode: PropEmitMode) -> Vec<String> {
    let mut excluded = vec![
        "$$slots".to_string(),
        "$$events".to_string(),
        "$$legacy".to_string(),
    ];
    if matches!(emit_mode, PropEmitMode::CustomElement) {
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
                Expression::Identifier(id) if id.name.as_str() == "$bindable",
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

impl<'a> ComponentTransformer<'_, 'a> {
    pub(super) fn rewrite_props(
        &mut self,
        decl_kind: VariableDeclarationKind,
        mut declarator: VariableDeclarator<'a>,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        match &declarator.id {
            BindingPattern::BindingIdentifier(_) => {
                self.rewrite_props_identifier(decl_kind, &declarator, out)
            }
            BindingPattern::ObjectPattern(_) => {
                self.rewrite_props_object(decl_kind, &mut declarator, out)
            }
            _ => {}
        }
    }

    fn record_rest_excludes(&mut self, keys: Vec<RestExcludeKey>) -> &'a str {
        let name = self.ident_gen.generate("rest_excludes");
        let name_ref: &'a str = self.b.alloc_str(&name);
        self.transform_data
            .rest_excludes
            .push(RestExcludes { name, keys });
        name_ref
    }

    fn rewrite_props_identifier(
        &mut self,
        decl_kind: VariableDeclarationKind,
        declarator: &VariableDeclarator<'a>,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        let Some(analysis) = self.analysis else {
            return;
        };
        let BindingPattern::BindingIdentifier(id) = &declarator.id else {
            return;
        };
        let Some(sym) = id.symbol_id.get() else {
            return;
        };
        let BindingSemantics::Prop(prop) = analysis.binding_semantics(sym) else {
            return;
        };
        let keys: Vec<RestExcludeKey> = base_rest_excluded(prop.emit_mode)
            .into_iter()
            .map(RestExcludeKey::Str)
            .collect();
        let excludes_ref = self.record_rest_excludes(keys);
        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$props"), Arg::Ident(excludes_ref)];
        if self.dev {
            args.push(Arg::Str(id.name.to_string()));
        }
        let init = self.b.call_expr("$.rest_props", args);
        let name: &'a str = self.b.alloc_str(id.name.as_str());
        out.push(self.props_declarator(decl_kind, name, init));
    }

    fn rewrite_props_object(
        &mut self,
        decl_kind: VariableDeclarationKind,
        declarator: &mut VariableDeclarator<'a>,
        out: &mut OxcVec<'a, VariableDeclarator<'a>>,
    ) {
        let Some(analysis) = self.analysis else {
            return;
        };
        let Some(emit_mode) = self.first_prop_emit_mode(&declarator.id) else {
            return;
        };

        let mut defaults = self.take_prop_defaults(&mut declarator.id);
        let mut excluded: Vec<ExcludedKey> = base_rest_excluded(emit_mode)
            .into_iter()
            .map(ExcludedKey::Str)
            .collect();
        let mut declarators: Vec<VariableDeclarator<'a>> = Vec::new();

        walk_bindings(&declarator.id, |v| {
            if v.is_rest {
                let keys: Vec<RestExcludeKey> = mem::take(&mut excluded)
                    .into_iter()
                    .map(ExcludedKey::into_rest_key)
                    .collect();
                let excludes_ref = self.record_rest_excludes(keys);
                let name: &'a str = self
                    .b
                    .alloc_str(self.component_scoping.symbol_name(v.symbol));
                let mut args: Vec<Arg<'a, '_>> =
                    vec![Arg::Ident("$$props"), Arg::Ident(excludes_ref)];
                if self.dev {
                    args.push(Arg::Str(name.to_string()));
                }
                let init = self.b.call_expr("$.rest_props", args);
                declarators.push(self.props_declarator(decl_kind, name, init));
                return;
            }

            let Some((alias_cow, origin_kind)) = analysis.binding_origin_key(v.symbol) else {
                return;
            };
            let alias = alias_cow.into_owned();
            excluded.push(prop_excluded_key(&alias, origin_kind));

            let BindingSemantics::Prop(leaf_prop) = analysis.binding_semantics(v.symbol) else {
                return;
            };
            let local_name: &'a str = self
                .b
                .alloc_str(self.component_scoping.symbol_name(v.symbol));
            let default_expr = defaults.remove(&v.symbol);

            let bindable = leaf_prop.bindable;
            match leaf_prop.kind {
                PropBindingKind::NonSource
                | PropBindingKind::Identifier
                | PropBindingKind::Rest => {}
                PropBindingKind::Source {
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
                    if self.accessors || updated || matches!(emit_mode, PropEmitMode::CustomElement)
                    {
                        flags |= PROPS_IS_UPDATED;
                    }

                    let mut args: Vec<Arg<'a, '_>> = vec![
                        Arg::Ident("$$props"),
                        prop_origin_key_arg(&alias, origin_kind),
                    ];
                    match default_lowering {
                        PropDefaultKind::None => {
                            if bindable && !updated {
                                return;
                            }
                            if flags != 0 {
                                args.push(Arg::Num(flags as f64));
                            }
                        }
                        PropDefaultKind::Eager
                        | PropDefaultKind::Lazy
                        | PropDefaultKind::LazyAccessor => {
                            if matches!(
                                default_lowering,
                                PropDefaultKind::Lazy | PropDefaultKind::LazyAccessor
                            ) {
                                flags |= PROPS_IS_LAZY_INITIAL;
                            }
                            args.push(Arg::Num(flags as f64));

                            let default_expr = default_expr.unwrap_or_else(|| {
                                panic!("default expr missing for prop {local_name}")
                            });
                            let default_expr = if default_needs_proxy {
                                let proxied =
                                    self.b.call_expr("$.proxy", [Arg::Expr(default_expr)]);
                                if self.dev {
                                    self.b.call_expr(
                                        "$.tag_proxy",
                                        [Arg::Expr(proxied), Arg::Str(local_name.to_string())],
                                    )
                                } else {
                                    proxied
                                }
                            } else {
                                default_expr
                            };
                            let default_expr = if matches!(default_lowering, PropDefaultKind::Eager)
                            {
                                default_expr
                            } else {
                                let lazy = super::derived::wrap_lazy(self.b, default_expr);
                                self.b.seed_arrow_scope(&lazy, self.gen_arrow_scope);
                                lazy
                            };
                            args.push(Arg::Expr(default_expr));
                        }
                    }

                    let init = self.b.call_expr("$.prop", args);
                    declarators.push(self.props_declarator(decl_kind, local_name, init));
                }
            }
        });

        out.extend(declarators);
    }

    fn take_prop_defaults(
        &self,
        pattern: &mut BindingPattern<'a>,
    ) -> HashMap<SymbolId, Expression<'a>> {
        let mut map = HashMap::new();
        let BindingPattern::ObjectPattern(obj) = pattern else {
            return map;
        };
        for prop in obj.properties.iter_mut() {
            if let BindingPattern::AssignmentPattern(assign) = &mut prop.value
                && let BindingPattern::BindingIdentifier(id) = &assign.left
                && let Some(sym) = id.symbol_id.get()
            {
                map.insert(sym, self.b.move_expr(&mut assign.right));
            }
        }
        map
    }

    fn first_prop_emit_mode(&self, pattern: &BindingPattern<'a>) -> Option<PropEmitMode> {
        let analysis = self.analysis?;
        let mut first = None;
        walk_bindings(pattern, |v| {
            if first.is_none() {
                first = Some(v.symbol);
            }
        });
        match analysis.binding_semantics(first?) {
            BindingSemantics::Prop(prop) => Some(prop.emit_mode),
            _ => None,
        }
    }

    fn props_declarator(
        &self,
        decl_kind: VariableDeclarationKind,
        name: &'a str,
        init: Expression<'a>,
    ) -> VariableDeclarator<'a> {
        self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b.ast.binding_pattern_binding_identifier(SPAN, name),
            NONE,
            Some(init),
            false,
        )
    }
}
