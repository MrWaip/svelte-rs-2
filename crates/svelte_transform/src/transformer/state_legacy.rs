use oxc_allocator::Vec as OxcVec;
use oxc_ast::ast::{
    AssignmentTarget, BindingPattern, Expression, PropertyKey, Statement, VariableDeclarationKind,
};
use svelte_analyze::DeclaratorSemantics;
use svelte_ast_builder::Arg;
use svelte_component_semantics::{Access, walk_bindings};

use super::model::ComponentTransformer;

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn expand_legacy_state_destructuring(
        &mut self,
        stmts: &mut OxcVec<'a, Statement<'a>>,
    ) {
        if self.runes {
            return;
        }
        let Some(analysis) = self.analysis else {
            return;
        };
        let mut i = 0;
        while i < stmts.len() {
            let Statement::VariableDeclaration(decl) = &stmts[i] else {
                i += 1;
                continue;
            };
            if decl.declarations.len() != 1 {
                i += 1;
                continue;
            }
            let declarator = &decl.declarations[0];
            let DeclaratorSemantics::LegacyStateDestructure { leaves } =
                analysis.declarator_semantics(declarator.node_id())
            else {
                i += 1;
                continue;
            };
            let Some(_) = declarator.init.as_ref() else {
                i += 1;
                continue;
            };

            let Some(leaf_paths) = collect_leaf_paths(&declarator.id) else {
                i += 1;
                continue;
            };
            let kind = decl.kind;

            let stmt = stmts.remove(i);
            let Statement::VariableDeclaration(decl_box) = stmt else {
                unreachable!();
            };
            let mut decl = decl_box.unbox();
            let mut declarator = decl.declarations.remove(0);
            let init = declarator
                .init
                .take()
                .expect("predicate matched only declarators with an init");

            let tmp_name_owned = self.gen_unique_name("tmp");
            let tmp_name: &'a str = self.b.alloc_str(&tmp_name_owned);
            let mut declarators_out: Vec<(&'a str, Expression<'a>)> = Vec::new();
            declarators_out.push((tmp_name, init));

            for sym in leaves {
                let leaf_name_owned = analysis.scoping.symbol_name(sym).to_string();
                let leaf_name: &'a str = self.b.alloc_str(&leaf_name_owned);
                let path = match leaf_paths.iter().find(|(s, _)| *s == sym) {
                    Some((_, p)) => p.clone(),
                    None => continue,
                };
                let access = build_tmp_access(self, tmp_name, &path);
                let init_expr = self.b.call_expr("$.mutable_source", [Arg::Expr(access)]);
                declarators_out.push((leaf_name, init_expr));
            }

            let replacement = match kind {
                VariableDeclarationKind::Let | VariableDeclarationKind::Var => {
                    self.b.let_multi_stmt(declarators_out)
                }
                _ => self.b.let_multi_stmt(declarators_out),
            };
            stmts.insert(i, replacement);
            self.ident_counter += 1;
            i += 1;
        }
    }
}

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_legacy_state_destructure_assignment_exit(
        &mut self,
        node: &mut Expression<'a>,
    ) -> bool {
        if self.runes {
            return false;
        }
        let Some(analysis) = self.analysis else {
            return false;
        };
        let Expression::AssignmentExpression(assign_box) = node else {
            return false;
        };
        let AssignmentTarget::ArrayAssignmentTarget(_) = &assign_box.left else {
            return false;
        };
        let Some(leaves) = collect_array_assign_legacy_state_leaves(analysis, &assign_box.left)
        else {
            return false;
        };
        let n = leaves.len() as f64;
        let placeholder = self.b.cheap_expr();
        let owned = std::mem::replace(node, placeholder);
        let Expression::AssignmentExpression(assign_box) = owned else {
            unreachable!();
        };
        let assign = assign_box.unbox();
        let rhs = assign.right;

        let value_param = self.b.alloc_str("$$value");
        let array_var = self.b.alloc_str("$$array");
        let mut body: Vec<Statement<'a>> = Vec::new();
        let to_array_call = self
            .b
            .call_expr("$.to_array", [Arg::Ident(value_param), Arg::Num(n)]);
        body.push(self.b.var_stmt(array_var, to_array_call));
        for (idx, leaf_name) in leaves.iter().enumerate() {
            let leaf_alloc: &'a str = self.b.alloc_str(leaf_name);
            let arr_access = self
                .b
                .computed_member_expr(self.b.rid_expr(array_var), self.b.num_expr(idx as f64));
            let set_call = self
                .b
                .call_expr("$.set", [Arg::Ident(leaf_alloc), Arg::Expr(arr_access)]);
            body.push(self.b.expr_stmt(set_call));
        }
        let arrow = self.b.arrow_expr(self.b.params([value_param]), body);
        let iife = self.b.ast.expression_call(
            oxc_span::SPAN,
            arrow,
            oxc_ast::NONE,
            self.b
                .ast
                .vec_from_iter(std::iter::once(oxc_ast::ast::Argument::from(rhs))),
            false,
        );
        *node = iife;
        true
    }
}

fn collect_leaf_paths<'a>(
    pat: &BindingPattern<'a>,
) -> Option<Vec<(svelte_component_semantics::SymbolId, Vec<AccessPathStep>)>> {
    let mut out: Vec<(svelte_component_semantics::SymbolId, Vec<AccessPathStep>)> = Vec::new();
    let mut bail = false;
    walk_bindings(pat, |visit| {
        if bail {
            return;
        }
        if visit.is_rest {
            bail = true;
            return;
        }
        let mut path = Vec::with_capacity(visit.path.len());
        for step in visit.path {
            if step.default.is_some() {
                bail = true;
                return;
            }
            match step.access {
                Access::Key { key, computed } => {
                    if computed {
                        bail = true;
                        return;
                    }
                    let Some(name) = property_key_static_name(key) else {
                        bail = true;
                        return;
                    };
                    path.push(AccessPathStep::Key(name.to_string()));
                }
                Access::Index(idx) => path.push(AccessPathStep::Index(idx)),
            }
        }
        out.push((visit.symbol, path));
    });
    if bail { None } else { Some(out) }
}

#[derive(Clone)]
enum AccessPathStep {
    Key(String),
    Index(u32),
}

fn property_key_static_name<'a>(key: &'a PropertyKey<'a>) -> Option<&'a str> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str()),
        PropertyKey::StringLiteral(lit) => Some(lit.value.as_str()),
        _ => None,
    }
}

fn build_tmp_access<'a>(
    cx: &ComponentTransformer<'_, 'a>,
    tmp_name: &'a str,
    path: &[AccessPathStep],
) -> Expression<'a> {
    let mut current = cx.b.rid_expr(tmp_name);
    for step in path {
        match step {
            AccessPathStep::Key(name) => {
                let key_alloc = cx.b.alloc_str(name);
                current = cx.b.static_member_expr(current, key_alloc);
            }
            AccessPathStep::Index(idx) => {
                current =
                    cx.b.computed_member_expr(current, cx.b.num_expr(*idx as f64));
            }
        }
    }
    current
}

fn collect_array_assign_legacy_state_leaves<'a>(
    analysis: &svelte_analyze::AnalysisData<'a>,
    target: &AssignmentTarget<'a>,
) -> Option<Vec<String>> {
    let AssignmentTarget::ArrayAssignmentTarget(arr) = target else {
        return None;
    };
    let mut out: Vec<String> = Vec::with_capacity(arr.elements.len());
    if arr.rest.is_some() {
        return None;
    }
    for elem in arr.elements.iter() {
        let Some(elem) = elem else {
            return None;
        };
        let oxc_ast::ast::AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(id) = elem
        else {
            return None;
        };
        let ref_id = id.reference_id.get()?;
        if !matches!(
            analysis.reference_semantics(ref_id),
            svelte_analyze::ReferenceSemantics::LegacyStateWrite
                | svelte_analyze::ReferenceSemantics::LegacyStateUpdate { .. }
        ) {
            return None;
        }
        out.push(id.name.as_str().to_string());
    }
    Some(out)
}
