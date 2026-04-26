//! LEGACY(svelte4): lower legacy `export let` / `export var` / specifier props
//! to `let foo = $.prop($$props, "foo", flags, default)` declarations.
//!
//! Reads only `data.declaration_semantics(...)` plus the AST. No detours.
//! Deprecated in Svelte 5, remove in Svelte 6.

use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{
    BindingPattern, Declaration, Expression, ModuleExportName, Statement, VariableDeclaration,
    VariableDeclarationKind,
};
use oxc_span::{GetSpan, GetSpanMut};
use svelte_analyze::{DeclarationSemantics, LegacyBindablePropSemantics, PropDefaultLowering};
use svelte_ast_builder::Arg;

use super::derived;
use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    /// LEGACY(svelte4): replace every `ExportNamedDeclaration` containing a `let`/`var`
    /// declaration of legacy bindable props with the lowered `let foo = $.prop(...)` form.
    /// Specifier-form exports (`export { foo }`, `export { foo as bar }`) are rewritten
    /// against the original `let foo = …` declaration; the export specifier statement
    /// is removed.
    pub(crate) fn process_legacy_export_props(&mut self, stmts: &mut OxcVec<'a, Statement<'a>>) {
        if self.runes {
            return;
        }
        // Snapshot the original sequence of statements; mutate in place.
        let mut i = 0;
        let mut renamed: rustc_hash::FxHashMap<String, (Vec<u8>, /*alias*/ Option<String>)> =
            rustc_hash::FxHashMap::default();
        // First pass: collect specifier exports whose local binding the analyzer classified
        // as `LegacyBindableProp` (so the matching `let foo = …` declaration further up
        // can be lowered with the right prop key). Specifier exports of regular module
        // values (`export { foo, bar }` in `<script module>`) skip this map.
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
                let node = analysis.scoping.symbol_declaration(sym);
                if !matches!(
                    analysis.declaration_semantics(node),
                    DeclarationSemantics::LegacyBindableProp(_)
                ) {
                    continue;
                }
                let alias = if local != exported {
                    Some(exported)
                } else {
                    None
                };
                renamed.insert(local, (Vec::new(), alias));
            }
        }
        // Second pass: rewrite inline-declaration exports + specifier-targeted let/var declarations.
        while i < stmts.len() {
            let replacement = self.try_rewrite_legacy_stmt(&mut stmts[i], &renamed);
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

    fn try_rewrite_legacy_stmt(
        &mut self,
        stmt: &mut Statement<'a>,
        renamed: &rustc_hash::FxHashMap<String, (Vec<u8>, Option<String>)>,
    ) -> Option<Vec<Statement<'a>>> {
        match stmt {
            Statement::ExportNamedDeclaration(export) => {
                if let Some(decl) = export.declaration.as_mut() {
                    let Declaration::VariableDeclaration(var_decl) = decl else {
                        return None;
                    };
                    if !is_let_or_var(var_decl.kind) {
                        return None;
                    }
                    self.try_lower_legacy_inline_declaration(var_decl)
                } else if export.specifiers.iter().all(|s| {
                    let local = match &s.local {
                        ModuleExportName::IdentifierReference(id) => id.name.as_str(),
                        ModuleExportName::IdentifierName(id) => id.name.as_str(),
                        _ => return false,
                    };
                    renamed.contains_key(local)
                }) {
                    // Pure specifier-only export — drop entirely; the matching
                    // `let foo = …` declaration is rewritten on its own pass.
                    Some(Vec::new())
                } else {
                    None
                }
            }
            Statement::VariableDeclaration(var_decl) => {
                if !is_let_or_var(var_decl.kind) {
                    return None;
                }
                self.try_lower_legacy_specifier_declaration(var_decl, renamed)
            }
            _ => None,
        }
    }

    fn try_lower_legacy_inline_declaration(
        &mut self,
        decl: &mut VariableDeclaration<'a>,
    ) -> Option<Vec<Statement<'a>>> {
        let analysis = self.analysis?;
        let kind = decl.kind;
        let mut declarators: Vec<(&'a str, Expression<'a>)> = Vec::new();
        for declarator in &mut decl.declarations {
            let leaf_node_ids = collect_leaf_symbol_node_ids(analysis, &declarator.id);
            if leaf_node_ids.is_empty() {
                return None;
            }
            // Confirm every leaf is a LegacyBindableProp; bail if even one isn't.
            for (_local, _path, node_id) in &leaf_node_ids {
                let DeclarationSemantics::LegacyBindableProp(_) =
                    analysis.declaration_semantics(*node_id)
                else {
                    return None;
                };
            }

            match &mut declarator.id {
                BindingPattern::BindingIdentifier(id) => {
                    let local = id.name.as_str().to_string();
                    let local_alloc = self.b.alloc_str(&local);
                    let DeclarationSemantics::LegacyBindableProp(legacy) =
                        analysis.declaration_semantics(leaf_node_ids[0].2)
                    else {
                        return None;
                    };
                    let init = declarator.init.as_mut().map(|e| self.b.move_expr(e));
                    let call =
                        self.build_prop_call(local_alloc, /*alias=*/ None, legacy, init);
                    declarators.push((local_alloc, call));
                }
                _ => {
                    // TODO(legacy destructure): emit per-leaf $.prop(...) declarations using
                    // the AST path; for now bail and let consumer use case handle it.
                    return None;
                }
            }
        }
        if declarators.is_empty() {
            return Some(Vec::new());
        }
        Some(vec![self.b.var_decl_multi_stmt(declarators, kind)])
    }

    fn try_lower_legacy_specifier_declaration(
        &mut self,
        decl: &mut VariableDeclaration<'a>,
        renamed: &rustc_hash::FxHashMap<String, (Vec<u8>, Option<String>)>,
    ) -> Option<Vec<Statement<'a>>> {
        let analysis = self.analysis?;
        // Only handle a single-declarator simple identifier pattern; that covers
        // every form `let foo = …; export { foo as bar };` produces.
        if decl.declarations.len() != 1 {
            return None;
        }
        let declarator = &mut decl.declarations[0];
        let BindingPattern::BindingIdentifier(id) = &declarator.id else {
            return None;
        };
        let local_name = id.name.as_str().to_string();
        let (_, alias) = renamed.get(&local_name)?;
        let sym = id.symbol_id.get()?;
        let node_id = analysis.scoping.symbol_declaration(sym);
        let DeclarationSemantics::LegacyBindableProp(legacy) =
            analysis.declaration_semantics(node_id)
        else {
            return None;
        };
        let local_alloc = self.b.alloc_str(&local_name);
        let init = declarator.init.as_mut().map(|e| self.b.move_expr(e));
        let alias_str = alias.as_deref();
        let call = self.build_prop_call(local_alloc, alias_str, legacy, init);
        Some(vec![self.b.let_multi_stmt(vec![(local_alloc, call)])])
    }

    fn build_prop_call(
        &mut self,
        local: &'a str,
        alias: Option<&str>,
        legacy: LegacyBindablePropSemantics,
        default_init: Option<Expression<'a>>,
    ) -> Expression<'a> {
        let prop_key = alias.unwrap_or(local).to_string();
        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$props"), Arg::Str(prop_key)];
        match legacy.default_lowering {
            PropDefaultLowering::None => {
                if legacy.flags != 0 {
                    args.push(Arg::Num(legacy.flags as f64));
                }
            }
            PropDefaultLowering::Eager => {
                args.push(Arg::Num(legacy.flags as f64));
                let default_expr = default_init
                    .unwrap_or_else(|| panic!("eager default missing for legacy prop {local}"));
                args.push(Arg::Expr(default_expr));
            }
            PropDefaultLowering::Lazy => {
                args.push(Arg::Num(legacy.flags as f64));
                let default_expr = default_init
                    .unwrap_or_else(|| panic!("lazy default missing for legacy prop {local}"));
                let lazy = derived::wrap_lazy(self.b, default_expr);
                args.push(Arg::Expr(lazy));
            }
        }
        self.b.call_expr("$.prop", args)
    }
}

fn is_let_or_var(kind: VariableDeclarationKind) -> bool {
    matches!(
        kind,
        VariableDeclarationKind::Let | VariableDeclarationKind::Var
    )
}

/// Walks BindingPattern leaves; for each leaf identifier, yields (local_name, path_string, node_id).
/// node_id == BindingIdentifier OxcNodeId via symbol declaration lookup. Path string unused for
/// inline declarations.
fn collect_leaf_symbol_node_ids<'a>(
    analysis: &svelte_analyze::AnalysisData<'a>,
    pat: &BindingPattern<'a>,
) -> Vec<(String, String, svelte_component_semantics::OxcNodeId)> {
    use svelte_component_semantics::walk_bindings;
    let mut out = Vec::new();
    walk_bindings(pat, |visit| {
        let name = analysis.scoping.symbol_name(visit.symbol).to_string();
        let node_id = analysis.scoping.symbol_declaration(visit.symbol);
        out.push((name, String::new(), node_id));
    });
    out
}
