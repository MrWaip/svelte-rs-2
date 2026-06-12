use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{BindingPattern, Expression, ModuleExportName, Statement, VariableDeclaration};
use oxc_span::{GetSpan, GetSpanMut};
use svelte_analyze::{
    BindingSemantics, LegacyBindablePropSemantics, PropDefaultKind, PropsFlags, is_let_or_var,
};
use svelte_ast_builder::Arg;

use super::derived;
use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_split_export_props_legacy(
        &mut self,
        stmts: &mut OxcVec<'a, Statement<'a>>,
    ) {
        let mut i = 0;
        let mut renamed: rustc_hash::FxHashMap<String, Option<String>> =
            rustc_hash::FxHashMap::default();

        let analysis = self.analysis;
        let instance_scope = analysis.and_then(|a| a.scoping.instance_scope_id());
        for stmt in stmts.iter() {
            let Statement::ExportNamedDeclaration(export) = stmt else {
                continue;
            };
            if export.declaration.is_some() {
                continue;
            }
            for spec in &export.specifiers {
                let local = match &spec.local {
                    ModuleExportName::IdentifierReference(id) => id.name.as_str().to_string(),
                    ModuleExportName::IdentifierName(id) => id.name.as_str().to_string(),
                    _ => continue,
                };
                let exported = match &spec.exported {
                    ModuleExportName::IdentifierReference(id) => id.name.as_str().to_string(),
                    ModuleExportName::IdentifierName(id) => id.name.as_str().to_string(),
                    _ => continue,
                };
                let Some(analysis) = analysis else { continue };
                let Some(scope) = instance_scope else {
                    continue;
                };
                let Some(sym) = analysis.scoping.find_binding(scope, local.as_str()) else {
                    continue;
                };
                if !analysis.binding_semantics(sym).is_legacy_prop() {
                    continue;
                }
                let alias = if local != exported {
                    Some(exported)
                } else {
                    None
                };
                renamed.insert(local, alias);
            }
        }

        while i < stmts.len() {
            let replacement = self.try_rewrite_split_export_stmt_legacy(&mut stmts[i], &renamed);
            if let Some(new_stmts) = replacement {
                let span = stmts[i].span();
                stmts.remove(i);
                if new_stmts.is_empty() {
                    continue;
                }
                let mut k = 0;
                for mut stmt in new_stmts {
                    if k == 0 {
                        *stmt.span_mut() = span;
                    }
                    stmts.insert(i + k, stmt);
                    k += 1;
                }
                i += k;
            } else {
                i += 1;
            }
        }
    }

    fn try_rewrite_split_export_stmt_legacy(
        &mut self,
        stmt: &mut Statement<'a>,
        renamed: &rustc_hash::FxHashMap<String, Option<String>>,
    ) -> Option<Vec<Statement<'a>>> {
        match stmt {
            Statement::ExportNamedDeclaration(export) => {
                if export.declaration.is_some() {
                    None
                } else if export.specifiers.iter().all(|s| {
                    let local = match &s.local {
                        ModuleExportName::IdentifierReference(id) => id.name.as_str(),
                        ModuleExportName::IdentifierName(id) => id.name.as_str(),
                        _ => return false,
                    };
                    renamed.contains_key(local)
                }) {
                    Some(Vec::new())
                } else {
                    None
                }
            }
            Statement::VariableDeclaration(var_decl) => {
                if !is_let_or_var(var_decl.kind) {
                    return None;
                }
                self.rewrite_split_export_prop_declaration_legacy(var_decl, renamed)
            }
            _ => None,
        }
    }

    fn rewrite_split_export_prop_declaration_legacy(
        &mut self,
        decl: &mut VariableDeclaration<'a>,
        renamed: &rustc_hash::FxHashMap<String, Option<String>>,
    ) -> Option<Vec<Statement<'a>>> {
        let analysis = self.analysis?;

        if decl.declarations.len() != 1 {
            return None;
        }
        let declarator = &mut decl.declarations[0];
        let BindingPattern::BindingIdentifier(id) = &declarator.id else {
            return None;
        };
        let local_name = id.name.as_str().to_string();
        let alias = renamed.get(&local_name)?;
        let sym = id.symbol_id.get()?;
        let BindingSemantics::LegacyBindableProp(legacy) = analysis.binding_semantics(sym) else {
            return None;
        };
        let local_alloc = self.b.alloc_str(&local_name);
        let init = declarator.init.as_mut().map(|e| self.b.move_expr(e));
        let alias_str = alias.as_deref();
        let call = self.build_legacy_prop_call(local_alloc, alias_str, legacy, init);
        Some(vec![
            self.b
                .var_decl_multi_stmt(vec![(local_alloc, call)], decl.kind),
        ])
    }

    fn build_legacy_prop_call(
        &mut self,
        local: &'a str,
        alias: Option<&str>,
        legacy: LegacyBindablePropSemantics,
        default_init: Option<Expression<'a>>,
    ) -> Expression<'a> {
        let prop_key = alias.unwrap_or(local).to_string();
        let mut runtime_flags = legacy.flags;
        if matches!(
            legacy.default_kind,
            PropDefaultKind::Lazy | PropDefaultKind::LazyAccessor
        ) {
            runtime_flags |= PropsFlags::LAZY_INITIAL;
        }
        let flags_bits = runtime_flags.bits();
        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$props"), Arg::Str(prop_key)];
        let default_init = default_init.map(unwrap_paren_and_ts);
        match legacy.default_kind {
            PropDefaultKind::None => {
                if !runtime_flags.is_empty() {
                    args.push(Arg::Num(flags_bits as f64));
                }
            }
            PropDefaultKind::Eager => {
                args.push(Arg::Num(flags_bits as f64));
                let default_expr = default_init
                    .unwrap_or_else(|| panic!("eager default missing for legacy prop {local}"));
                args.push(Arg::Expr(default_expr));
            }
            PropDefaultKind::Lazy => {
                args.push(Arg::Num(flags_bits as f64));
                let default_expr = default_init
                    .unwrap_or_else(|| panic!("lazy default missing for legacy prop {local}"));
                let lazy = derived::wrap_lazy(self.b, default_expr);
                self.b.seed_arrow_scope(&lazy, self.gen_arrow_scope);
                args.push(Arg::Expr(lazy));
            }
            PropDefaultKind::LazyAccessor => {
                args.push(Arg::Num(flags_bits as f64));
                let default_expr = default_init.unwrap_or_else(|| {
                    panic!("lazy accessor default missing for legacy prop {local}")
                });
                let accessor_name = match &default_expr {
                    Expression::Identifier(id) => id.name.as_str().to_string(),
                    Expression::CallExpression(call) if call.arguments.is_empty() => {
                        match &call.callee {
                            Expression::Identifier(callee) => callee.name.as_str().to_string(),
                            _ => panic!(
                                "lazy accessor default must be a bare-identifier call for legacy prop {local}"
                            ),
                        }
                    }
                    _ => panic!(
                        "lazy accessor default must be an identifier or bare-call for legacy prop {local}"
                    ),
                };
                args.push(Arg::Expr(self.b.rid_expr(&accessor_name)));
            }
        }
        self.b.call_expr("$.prop", args)
    }
}

fn unwrap_paren_and_ts<'a>(expr: Expression<'a>) -> Expression<'a> {
    let mut inner = expr.into_inner_expression();
    match &mut inner {
        Expression::ArrowFunctionExpression(arrow) => arrow.pife = false,
        Expression::FunctionExpression(func) => func.pife = false,
        _ => {}
    }
    inner
}
