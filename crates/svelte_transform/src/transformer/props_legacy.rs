use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{
    BindingPattern, Declaration, Expression, ModuleExportName, Statement, VariableDeclaration,
};
use oxc_span::{GetSpan, GetSpanMut};
use svelte_analyze::{
    AnalysisData, BindingSemantics, LegacyBindablePropSemantics, PropDefaultLowering, PropsFlags,
    is_let_or_var, property_key_static_name,
};
use svelte_ast_builder::Arg;

use super::derived;
use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn process_legacy_export_props(&mut self, stmts: &mut OxcVec<'a, Statement<'a>>) {
        if self.runes {
            return;
        }

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
                if !matches!(
                    analysis.binding_semantics(sym),
                    BindingSemantics::LegacyBindableProp(_)
                ) {
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
        renamed: &rustc_hash::FxHashMap<String, Option<String>>,
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
            let leaves = collect_destructure_leaves(self.b, analysis, &declarator.id);
            if leaves.is_empty() {
                return None;
            }
            for leaf in &leaves {
                let BindingSemantics::LegacyBindableProp(_) =
                    analysis.binding_semantics(leaf.symbol)
                else {
                    return None;
                };
            }

            match &mut declarator.id {
                BindingPattern::BindingIdentifier(id) => {
                    let local = id.name.as_str().to_string();
                    let local_alloc = self.b.alloc_str(&local);
                    let BindingSemantics::LegacyBindableProp(legacy) =
                        analysis.binding_semantics(leaves[0].symbol)
                    else {
                        return None;
                    };
                    let init = declarator.init.as_mut().map(|e| self.b.move_expr(e));
                    let call =
                        self.build_prop_call(local_alloc, /*alias=*/ None, legacy, init);
                    declarators.push((local_alloc, call));
                }
                _ => {
                    let init = declarator.init.as_mut().map(|e| self.b.move_expr(e))?;
                    let extra = self.lower_destructured_legacy_export(leaves, init)?;
                    declarators.extend(extra);
                }
            }
        }
        if declarators.is_empty() {
            return Some(Vec::new());
        }
        Some(vec![self.b.var_decl_multi_stmt(declarators, kind)])
    }

    fn lower_destructured_legacy_export(
        &mut self,
        leaves: Vec<DestructureLeafDescriptor<'a>>,
        init: Expression<'a>,
    ) -> Option<Vec<(&'a str, Expression<'a>)>> {
        let analysis = self.analysis?;
        let tmp_name_owned = self.gen_unique_name("tmp");
        let tmp_name = self.b.alloc_str(&tmp_name_owned);
        let mut declarators: Vec<(&'a str, Expression<'a>)> = vec![(tmp_name, init)];

        let mut array_helpers: Vec<ArrayHelper<'a>> = Vec::new();
        let mut leaf_helpers: Vec<Option<usize>> = Vec::with_capacity(leaves.len());
        for leaf in &leaves {
            let Some(idx) = leaf.array_index else {
                leaf_helpers.push(None);
                continue;
            };
            let pos = match array_helpers
                .iter_mut()
                .position(|h| h.object_path == leaf.object_path)
            {
                Some(p) => {
                    array_helpers[p].len = array_helpers[p].len.max(idx + 1);
                    p
                }
                None => {
                    let helper_owned = self.gen_unique_name("$$array");
                    let helper = self.b.alloc_str(&helper_owned);
                    array_helpers.push(ArrayHelper {
                        object_path: leaf.object_path.clone(),
                        name: helper,
                        len: idx + 1,
                    });
                    self.ident_counter += 1;
                    array_helpers.len() - 1
                }
            };
            leaf_helpers.push(Some(pos));
        }
        self.ident_counter += 1;

        for helper in &array_helpers {
            let mut source = self.b.rid_expr(tmp_name);
            for key in &helper.object_path {
                source = self.b.static_member_expr(source, key);
            }
            let arrow_body = self.b.call_expr(
                "$.to_array",
                [Arg::Expr(source), Arg::Num(helper.len as f64)],
            );
            let derived_call = self
                .b
                .call_expr("$.derived", [Arg::Expr(self.b.thunk(arrow_body))]);
            declarators.push((helper.name, derived_call));
        }

        for (leaf_idx, leaf) in leaves.into_iter().enumerate() {
            let BindingSemantics::LegacyBindableProp(legacy) =
                analysis.binding_semantics(leaf.symbol)
            else {
                continue;
            };
            let source = if let Some(idx) = leaf.array_index {
                let helper_name = array_helpers[leaf_helpers[leaf_idx]
                    .expect("helper slot allocated above for array-index leaf")]
                .name;
                let get_call = self.b.call_expr("$.get", [Arg::Ident(helper_name)]);
                self.b
                    .computed_member_expr(get_call, self.b.num_expr(idx as f64))
            } else {
                let mut expr = self.b.rid_expr(tmp_name);
                for key in &leaf.object_path {
                    expr = self.b.static_member_expr(expr, key);
                }
                expr
            };
            let leaf_init = if let Some(default) = leaf.default_expr {
                self.b
                    .call_expr("$.fallback", [Arg::Expr(source), Arg::Expr(default)])
            } else {
                source
            };
            let local_alloc = self.b.alloc_str(&leaf.local_name);
            let call =
                self.build_prop_call(local_alloc, /*alias=*/ None, legacy, Some(leaf_init));
            declarators.push((local_alloc, call));
        }

        Some(declarators)
    }

    fn try_lower_legacy_specifier_declaration(
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
        let mut runtime_flags = legacy.flags;
        if matches!(legacy.default_lowering, PropDefaultLowering::Lazy) {
            runtime_flags |= PropsFlags::LAZY_INITIAL;
        }
        let flags_bits = runtime_flags.bits();
        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Ident("$$props"), Arg::Str(prop_key)];
        match legacy.default_lowering {
            PropDefaultLowering::None => {
                if !runtime_flags.is_empty() {
                    args.push(Arg::Num(flags_bits as f64));
                }
            }
            PropDefaultLowering::Eager => {
                args.push(Arg::Num(flags_bits as f64));
                let default_expr = default_init
                    .unwrap_or_else(|| panic!("eager default missing for legacy prop {local}"));
                args.push(Arg::Expr(default_expr));
            }
            PropDefaultLowering::Lazy => {
                args.push(Arg::Num(flags_bits as f64));
                let default_expr = default_init
                    .unwrap_or_else(|| panic!("lazy default missing for legacy prop {local}"));
                let lazy = derived::wrap_lazy(self.b, default_expr);
                args.push(Arg::Expr(lazy));
            }
        }
        self.b.call_expr("$.prop", args)
    }
}

struct ArrayHelper<'a> {
    object_path: Vec<String>,
    name: &'a str,
    len: u32,
}

struct DestructureLeafDescriptor<'a> {
    local_name: String,
    symbol: svelte_analyze::scope::SymbolId,
    object_path: Vec<String>,
    array_index: Option<u32>,
    default_expr: Option<Expression<'a>>,
}

fn collect_destructure_leaves<'a>(
    builder: &svelte_ast_builder::Builder<'a>,
    analysis: &AnalysisData<'a>,
    pattern: &BindingPattern<'_>,
) -> Vec<DestructureLeafDescriptor<'a>> {
    use oxc_allocator::CloneIn;
    use svelte_component_semantics::{Access, walk_bindings};
    let allocator = builder.ast.allocator;
    let mut out: Vec<DestructureLeafDescriptor<'a>> = Vec::new();
    walk_bindings(pattern, |visit| {
        let mut object_path: Vec<String> = Vec::new();
        let mut array_index: Option<u32> = None;
        let mut default_expr: Option<Expression<'a>> = None;
        for step in visit.path {
            if array_index.is_some() {
                continue;
            }
            match step.access {
                Access::Key { key, .. } => {
                    if let Some(name) = property_key_static_name(key) {
                        object_path.push(name.to_string());
                    }
                    if let Some(d) = step.default {
                        default_expr = Some(d.clone_in(allocator));
                    }
                }
                Access::Index(i) => {
                    array_index = Some(i);
                }
            }
        }
        out.push(DestructureLeafDescriptor {
            local_name: analysis.scoping.symbol_name(visit.symbol).to_string(),
            symbol: visit.symbol,
            object_path,
            array_index,
            default_expr,
        });
    });
    out
}
