use std::{iter, mem};

use oxc_allocator::{CloneIn, Vec as OxcVec};
use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, AssignmentTarget, AssignmentTargetMaybeDefault, AssignmentTargetProperty,
    BindingPattern, Expression, IdentifierReference, PropertyKey, Statement,
};
use oxc_span::SPAN;
use svelte_analyze::{AnalysisData, DeclaratorSemantics, ReferenceSemantics};
use svelte_ast_builder::{Arg, AssignLeft};
use svelte_component_semantics::SymbolId;

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

            if !is_supported_pattern(&declarator.id) {
                i += 1;
                continue;
            }
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

            let root_access = self.b.rid_expr(tmp_name);
            self.emit_legacy_destructure_decls(
                &declarator.id,
                root_access,
                &leaves,
                &mut declarators_out,
            );

            let replacement = self.b.var_decl_multi_stmt(declarators_out, kind);
            stmts.insert(i, replacement);
            self.ident_counter += 1;
            i += 1;
        }
    }

    fn emit_legacy_destructure_decls(
        &mut self,
        pat: &BindingPattern<'a>,
        access: Expression<'a>,
        leaves: &[SymbolId],
        out: &mut Vec<(&'a str, Expression<'a>)>,
    ) {
        match pat {
            BindingPattern::BindingIdentifier(id) => {
                let Some(sym) = id.symbol_id.get() else {
                    return;
                };
                let name_alloc = self.b.alloc_str(id.name.as_str());
                let init = if leaves.contains(&sym) {
                    self.b.call_expr("$.mutable_source", [Arg::Expr(access)])
                } else {
                    access
                };
                out.push((name_alloc, init));
            }
            BindingPattern::AssignmentPattern(_) => {}
            BindingPattern::ObjectPattern(obj) => {
                let allocator = self.b.ast.allocator;
                for prop in obj.properties.iter() {
                    let Some(key_name) = property_key_static_name(&prop.key) else {
                        return;
                    };
                    let key_alloc = self.b.alloc_str(key_name);
                    let parent = access.clone_in(allocator);
                    let child_access = self.b.static_member_expr(parent, key_alloc);
                    self.emit_legacy_destructure_decls(&prop.value, child_access, leaves, out);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                let var_name_owned = self.ident_gen.generate("$$array");
                let var_name: &'a str = self.b.alloc_str(&var_name_owned);
                let len = arr.elements.len() as f64;
                let to_array =
                    self.b
                        .call_expr("$.to_array", [Arg::Expr(access), Arg::Num(len)]);
                let thunk = self.b.thunk(to_array);
                let derived = self.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                out.push((var_name, derived));
                for (idx, elem) in arr.elements.iter().enumerate() {
                    let Some(elem) = elem else { continue };
                    let get_call = self.b.call_expr("$.get", [Arg::Ident(var_name)]);
                    let elem_access = self
                        .b
                        .computed_member_expr(get_call, self.b.num_expr(idx as f64));
                    self.emit_legacy_destructure_decls(elem, elem_access, leaves, out);
                }
            }
        }
    }
}

fn is_supported_pattern<'a>(pat: &BindingPattern<'a>) -> bool {
    match pat {
        BindingPattern::BindingIdentifier(id) => id.symbol_id.get().is_some(),
        BindingPattern::AssignmentPattern(_) => false,
        BindingPattern::ObjectPattern(obj) => {
            if obj.rest.is_some() {
                return false;
            }
            obj.properties.iter().all(|p| {
                !p.computed
                    && property_key_static_name(&p.key).is_some()
                    && is_supported_pattern(&p.value)
            })
        }
        BindingPattern::ArrayPattern(arr) => {
            if arr.rest.is_some() {
                return false;
            }
            arr.elements.iter().all(|e| match e {
                Some(elem) => is_supported_pattern(elem),
                None => true,
            })
        }
    }
}

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn rewrite_legacy_state_destructure_assignment_exit(
        &mut self,
        node: &mut Expression<'a>,
    ) -> bool {
        let Expression::AssignmentExpression(assign_box) = node else {
            return false;
        };
        if !matches!(
            &assign_box.left,
            AssignmentTarget::ArrayAssignmentTarget(_)
                | AssignmentTarget::ObjectAssignmentTarget(_)
        ) {
            return false;
        }
        if self.runes {
            self.reserve_assignment_target_array_names(&assign_box.left);
            return false;
        }
        let Some(analysis) = self.analysis else {
            self.reserve_assignment_target_array_names(&assign_box.left);
            return false;
        };
        if !any_ident_is_legacy_state(analysis, &assign_box.left) {
            self.reserve_assignment_target_array_names(&assign_box.left);
            return false;
        }

        let has_array = target_has_array_pattern(&assign_box.left);
        let should_cache = !matches!(&assign_box.right, Expression::Identifier(_));
        let use_iife = has_array || should_cache;

        let placeholder = self.b.cheap_expr();
        let owned = mem::replace(node, placeholder);
        let Expression::AssignmentExpression(assign_box) = owned else {
            unreachable!();
        };
        let assign = assign_box.unbox();
        let rhs = assign.right;

        if use_iife {
            let value_param = match &rhs {
                Expression::Identifier(id) if !should_cache => self.b.alloc_str(id.name.as_str()),
                _ => self.b.alloc_str("$$value"),
            };
            let mut decls: Vec<Statement<'a>> = Vec::new();
            let mut setters: Vec<Statement<'a>> = Vec::new();
            let root_access = self.b.rid_expr(value_param);
            self.emit_destructure_assignment(
                &assign.left,
                root_access,
                &mut decls,
                &mut setters,
                analysis,
            );
            let mut body: Vec<Statement<'a>> = Vec::with_capacity(decls.len() + setters.len());
            body.extend(decls);
            body.extend(setters);
            let arrow = self.b.arrow_block_expr(self.b.params([value_param]), body);
            let iife = self.b.ast.expression_call(
                SPAN,
                arrow,
                NONE,
                self.b.ast.vec_from_iter(iter::once(Argument::from(rhs))),
                false,
            );
            *node = iife;
        } else {
            let mut decls: Vec<Statement<'a>> = Vec::new();
            let mut setters: Vec<Statement<'a>> = Vec::new();
            self.emit_destructure_assignment(
                &assign.left,
                rhs,
                &mut decls,
                &mut setters,
                analysis,
            );
            debug_assert!(decls.is_empty());
            let allocator = self.b.ast.allocator;
            let mut seq: OxcVec<'a, Expression<'a>> =
                OxcVec::with_capacity_in(setters.len(), allocator);
            for stmt in setters {
                if let Statement::ExpressionStatement(es) = stmt {
                    let es = es.unbox();
                    seq.push(es.expression);
                }
            }
            *node = self.b.ast.expression_sequence(SPAN, seq);
        }
        true
    }

    fn emit_destructure_assignment(
        &mut self,
        target: &AssignmentTarget<'a>,
        access_expr: Expression<'a>,
        decls: &mut Vec<Statement<'a>>,
        setters: &mut Vec<Statement<'a>>,
        analysis: &AnalysisData<'a>,
    ) {
        match target {
            AssignmentTarget::AssignmentTargetIdentifier(id) => {
                self.emit_ident_setter(id, access_expr, setters, analysis);
            }
            AssignmentTarget::ObjectAssignmentTarget(obj) => {
                let allocator = self.b.ast.allocator;
                for prop in obj.properties.iter() {
                    let parent_access = access_expr.clone_in(allocator);
                    match prop {
                        AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(sh) => {
                            let name_alloc = self.b.alloc_str(sh.binding.name.as_str());
                            let child_access = self.b.static_member_expr(parent_access, name_alloc);
                            self.emit_ident_setter(&sh.binding, child_access, setters, analysis);
                        }
                        AssignmentTargetProperty::AssignmentTargetPropertyProperty(kv) => {
                            let Some(key_name) = property_key_static_name(&kv.name) else {
                                return;
                            };
                            let key_alloc = self.b.alloc_str(key_name);
                            let child_access = self.b.static_member_expr(parent_access, key_alloc);
                            let Some(child_target) = kv.binding.as_assignment_target() else {
                                return;
                            };
                            self.emit_destructure_assignment(
                                child_target,
                                child_access,
                                decls,
                                setters,
                                analysis,
                            );
                        }
                    }
                }
            }
            AssignmentTarget::ArrayAssignmentTarget(arr) => {
                let var_name_owned = self.ident_gen.generate("$$array");
                let var_name: &'a str = self.b.alloc_str(&var_name_owned);
                let n = arr.elements.len() as f64;
                let to_array_call =
                    self.b
                        .call_expr("$.to_array", [Arg::Expr(access_expr), Arg::Num(n)]);
                decls.push(self.b.var_stmt(var_name, to_array_call));
                for (idx, elem) in arr.elements.iter().enumerate() {
                    let Some(elem) = elem else {
                        return;
                    };
                    let AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(_) = elem else {
                        let Some(child) = elem.as_assignment_target() else {
                            return;
                        };
                        let child_access = self
                            .b
                            .computed_member_expr(self.b.rid_expr(var_name), self.b.num_expr(idx as f64));
                        self.emit_destructure_assignment(
                            child,
                            child_access,
                            decls,
                            setters,
                            analysis,
                        );
                        continue;
                    };
                    return;
                }
            }
            _ => {}
        }
    }

    fn emit_ident_setter(
        &mut self,
        id: &IdentifierReference<'a>,
        access_expr: Expression<'a>,
        setters: &mut Vec<Statement<'a>>,
        analysis: &AnalysisData<'a>,
    ) {
        let leaf_alloc: &'a str = self.b.alloc_str(id.name.as_str());
        let semantics = id
            .reference_id
            .get()
            .map(|ref_id| analysis.reference_semantics(ref_id));
        let is_legacy_state = matches!(
            semantics,
            Some(
                ReferenceSemantics::LegacyStateWrite
                    | ReferenceSemantics::LegacyStateUpdate { .. }
                    | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
                    | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            )
        );
        if !is_legacy_state {
            let assign = self
                .b
                .assign_expr(AssignLeft::Ident(leaf_alloc.into()), access_expr);
            setters.push(self.b.expr_stmt(assign));
            return;
        }
        let mut set_call = self
            .b
            .call_expr("$.set", [Arg::Ident(leaf_alloc), Arg::Expr(access_expr)]);
        let store_symbol = match semantics {
            Some(ReferenceSemantics::LegacyStateSubscribedWrite { store_symbol })
            | Some(ReferenceSemantics::LegacyStateSubscribedUpdate { store_symbol, .. }) => {
                Some(store_symbol)
            }
            _ => None,
        };
        if let Some(store_sym) = store_symbol {
            let dollar_name = analysis.scoping.symbol_name(store_sym).to_string();
            set_call = self.make_store_unsub(set_call, &dollar_name);
        }
        setters.push(self.b.expr_stmt(set_call));
    }
}

impl<'a> ComponentTransformer<'_, 'a> {
    pub(crate) fn reserve_assignment_target_array_names(&mut self, target: &AssignmentTarget<'a>) {
        match target {
            AssignmentTarget::ArrayAssignmentTarget(arr) => {
                let _ = self.ident_gen.generate("$$array");
                for elem in arr.elements.iter().flatten() {
                    let inner = match elem {
                        AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(d) => {
                            Some(&d.binding)
                        }
                        other => other.as_assignment_target(),
                    };
                    if let Some(inner) = inner {
                        self.reserve_assignment_target_array_names(inner);
                    }
                }
            }
            AssignmentTarget::ObjectAssignmentTarget(obj) => {
                for prop in &obj.properties {
                    if let AssignmentTargetProperty::AssignmentTargetPropertyProperty(kv) = prop
                        && let Some(child) = kv.binding.as_assignment_target()
                    {
                        self.reserve_assignment_target_array_names(child);
                    }
                }
            }
            _ => {}
        }
    }
}

fn target_has_array_pattern<'a>(target: &AssignmentTarget<'a>) -> bool {
    let mut has = false;
    let _ = svelte_component_semantics::walk_assignment_target_idents(target, |_| {});
    fn scan<'a>(t: &AssignmentTarget<'a>, has: &mut bool) {
        match t {
            AssignmentTarget::ArrayAssignmentTarget(_) => *has = true,
            AssignmentTarget::ObjectAssignmentTarget(obj) => {
                for prop in &obj.properties {
                    if let AssignmentTargetProperty::AssignmentTargetPropertyProperty(kv) = prop
                        && let Some(child) = kv.binding.as_assignment_target()
                    {
                        scan(child, has);
                    }
                }
            }
            _ => {}
        }
    }
    scan(target, &mut has);
    has
}

fn any_ident_is_legacy_state<'a>(
    analysis: &AnalysisData<'a>,
    target: &AssignmentTarget<'a>,
) -> bool {
    let mut any = false;
    let walked = svelte_component_semantics::walk_assignment_target_idents(target, |id| {
        let Some(ref_id) = id.reference_id.get() else {
            return;
        };
        if matches!(
            analysis.reference_semantics(ref_id),
            ReferenceSemantics::LegacyStateWrite
                | ReferenceSemantics::LegacyStateUpdate { .. }
                | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
                | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
        ) {
            any = true;
        }
    });
    walked && any
}

fn property_key_static_name<'a>(key: &'a PropertyKey<'a>) -> Option<&'a str> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str()),
        PropertyKey::StringLiteral(lit) => Some(lit.value.as_str()),
        _ => None,
    }
}

