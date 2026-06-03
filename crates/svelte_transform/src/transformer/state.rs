use std::collections::HashMap;
use std::iter;
use std::mem;

use rustc_hash::FxHashSet;

use oxc_allocator::{Box as OxcBox, CloneIn, Vec as OxcVec};
use oxc_ast::NONE;
use oxc_ast::ast::{
    Argument, AssignmentOperator, AssignmentTarget, BindingPattern, ClassBody, ClassElement,
    Expression, MethodDefinition, MethodDefinitionKind, PropertyDefinition, PropertyKey,
    Statement, VariableDeclarationKind, VariableDeclarator,
};
use oxc_span::{GetSpan, SPAN};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::{AnalysisData, BindingSemantics, DerivedKind, DerivedEmit, RuneKind, StateKind, property_key_static_name};

use svelte_ast_builder::{Arg, AssignLeft, Builder};
use svelte_component_semantics::{Access, Step, SymbolId, walk_bindings};

use crate::rune_refs;

use super::location::sanitize_location;
use super::model::{AsyncDerivedMode, ClassStateField, ClassStateInfo, ComponentTransformer};

fn serialize_binding_prefix(prefix: &[Step<'_>]) -> String {
    let mut out = String::new();
    for step in prefix {
        match step.access {
            Access::Key { key, computed } => {
                if computed {
                    let span = key.span();
                    out.push('c');
                    out.push_str(&span.start.to_string());
                    out.push('_');
                    out.push_str(&span.end.to_string());
                } else if let Some(name) = property_key_static_name(key) {
                    out.push('k');
                    out.push_str(&name);
                } else {
                    let span = key.span();
                    out.push('k');
                    out.push_str(&span.start.to_string());
                }
            }
            Access::Index { index, .. } => {
                out.push('i');
                out.push_str(&index.to_string());
            }
            Access::Slice { from } => {
                out.push('s');
                out.push_str(&from.to_string());
            }
        }
        out.push('/');
    }
    out
}

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    fn state_destructure_dev_label(
        pattern: &BindingPattern<'a>,
        rune_kind: RuneKind,
    ) -> Option<&'static str> {
        if !matches!(rune_kind, RuneKind::State | RuneKind::StateRaw) {
            return None;
        }

        match pattern {
            BindingPattern::ArrayPattern(_) => Some("[$state iterable]"),
            BindingPattern::ObjectPattern(_) => Some("[$state object]"),
            _ => None,
        }
    }

    fn rewrite_destructured_rune_decls(
        &mut self,
        stmts: &mut OxcVec<'a, Statement<'a>>,
        mut predicate: impl FnMut(&VariableDeclarator<'a>, Option<RuneKind>) -> bool,
        mut rewrite: impl FnMut(
            &mut Self,
            VariableDeclarationKind,
            u32,
            VariableDeclarator<'a>,
            RuneKind,
        ) -> Statement<'a>,
    ) {
        let mut i = 0;
        while i < stmts.len() {
            let Some((should_rewrite, rune_kind)) = (match &stmts[i] {
                Statement::VariableDeclaration(decl) if decl.declarations.len() == 1 => {
                    let declarator = &decl.declarations[0];
                    let rune_kind = self.rune_kind_for_declarator(declarator);
                    Some((predicate(declarator, rune_kind), rune_kind))
                }
                _ => None,
            }) else {
                i += 1;
                continue;
            };

            if !should_rewrite {
                i += 1;
                continue;
            }

            let stmt = stmts.remove(i);
            let Statement::VariableDeclaration(mut decl) = stmt else {
                unreachable!();
            };
            let decl_kind = decl.kind;
            let decl_span_start = decl.span.start;
            let declarator = decl.declarations.remove(0);
            let replacement = rewrite(
                self,
                decl_kind,
                decl_span_start,
                declarator,
                rune_kind.expect("predicate returned true only for known rune kinds"),
            );
            stmts.insert(i, replacement);
            self.ident_counter += 1;
            i += 1;
        }
    }

    fn rune_kind_for_declarator(
        &self,
        declarator: &VariableDeclarator<'a>,
    ) -> Option<RuneKind> {
        Self::first_binding_symbol(&declarator.id).and_then(|sym| self.rune_for_symbol(sym))
    }

    pub(crate) fn class_field_rune_kind(&self, node: OxcNodeId) -> Option<RuneKind> {
        use svelte_analyze::{
            ClassFieldDerivedSemantics, ClassFieldStateSemantics, DeclaratorSemantics,
        };
        let analysis = self.analysis.as_ref()?;
        match analysis.declarator_semantics(node) {
            DeclaratorSemantics::ClassFieldState(ClassFieldStateSemantics { kind, .. }) => {
                Some(match kind {
                    StateKind::State => RuneKind::State,
                    StateKind::StateRaw => RuneKind::StateRaw,
                    StateKind::StateEager => RuneKind::StateEager,
                })
            }
            DeclaratorSemantics::ClassFieldDerived(ClassFieldDerivedSemantics { kind, .. }) => {
                Some(match kind {
                    DerivedKind::Derived => RuneKind::Derived,
                    DerivedKind::DerivedBy => RuneKind::DerivedBy,
                })
            }
            DeclaratorSemantics::None
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem { .. }
            | DeclaratorSemantics::AwaitValue => None,
            DeclaratorSemantics::PropsIdentifier { .. } | DeclaratorSemantics::PropsObject { .. } => None,
            DeclaratorSemantics::LegacyStateDestructure { .. } => None,
        }
    }

    fn first_binding_symbol(
        pattern: &BindingPattern<'a>,
    ) -> Option<SymbolId> {
        let mut first = None;
        walk_bindings(pattern, |v| {
            if first.is_none() {
                first = Some(v.symbol);
            }
        });
        first
    }

    pub(crate) fn process_derived_destructuring(
        &mut self,
        stmts: &mut OxcVec<'a, Statement<'a>>,
    ) {
        let analysis = self.analysis;
        self.rewrite_destructured_rune_decls(
            stmts,
            |declarator, _rune_kind| {
                matches!(
                    derived_destructure_emit(analysis, &declarator.id),
                    Some(
                        DerivedEmit::DestructuredInlineSource
                            | DerivedEmit::DestructuredInlinePropsSource
                            | DerivedEmit::DestructuredBoxedSync,
                    )
                )
            },
            |this, decl_kind, _decl_span_start, mut declarator, rune_kind| {
                let lowering = derived_destructure_emit(this.analysis, &declarator.id)
                    .expect("predicate gated on sync destructure lowering");
                let init = declarator
                    .init
                    .take()
                    .expect("predicate matched only declarators with an initializer");
                this.gen_sync_derived_destructuring(
                    &declarator.id,
                    init,
                    rune_kind,
                    lowering,
                    decl_kind,
                )
            },
        );
        let analysis = self.analysis;
        self.rewrite_destructured_rune_decls(
            stmts,
            |declarator, _rune_kind| {
                matches!(
                    derived_destructure_emit(analysis, &declarator.id),
                    Some(DerivedEmit::DestructuredBoxedAsync),
                )
            },
            |this, decl_kind, decl_span_start, mut declarator, _| {
                let init = declarator
                    .init
                    .take()
                    .expect("predicate matched only declarators with an initializer");
                this.gen_async_derived_destructuring(&declarator.id, init, decl_span_start, decl_kind)
            },
        );
    }

    fn gen_sync_derived_destructuring(
        &mut self,
        pattern: &BindingPattern<'a>,
        init: Expression<'a>,
        rune_kind: RuneKind,
        lowering: DerivedEmit,
        decl_kind: VariableDeclarationKind,
    ) -> Statement<'a> {
        let Expression::CallExpression(mut call) = init else {
            unreachable!("sync derived destructuring should be a call");
        };
        call.callee = self.b.rid_expr("$.derived");

        let mut declarators = Vec::new();

        let access_root = if matches!(lowering, DerivedEmit::DestructuredInlinePropsSource) {
            self.b.rid_expr("$$props")
        } else if matches!(lowering, DerivedEmit::DestructuredInlineSource) {
            call.arguments.remove(0).into_expression()
        } else {
            let arg_expr = call.arguments.remove(0).into_expression();
            let derived_arg = if matches!(rune_kind, RuneKind::DerivedBy) {
                arg_expr
            } else {
                self.b.thunk(arg_expr)
            };
            call.arguments
                .push(Argument::from(derived_arg));
            let tmp_name = self.gen_unique_name("$$d");
            let tmp_name_str = self.b.alloc_str(&tmp_name);
            let derived_call = Expression::CallExpression(call);
            let tmp_declarator = self.b.ast.variable_declarator(
                SPAN,
                decl_kind,
                self.b.ast.binding_pattern_binding_identifier(
                    SPAN,
                    self.b.ast.atom(tmp_name_str),
                ),
                NONE,
                Some(derived_call),
                false,
            );
            declarators.push(tmp_declarator);
            self.b.call_expr("$.get", [Arg::Ident(tmp_name_str)])
        };

        self.gen_destructure_declarators(
            pattern,
            access_root,
            RuneKind::Derived,
            decl_kind,
            None,
            &mut declarators,
        );

        let decl = self.b.ast.variable_declaration(
            SPAN,
            decl_kind,
            self.b.ast.vec_from_iter(declarators),
            false,
        );
        Statement::VariableDeclaration(self.b.alloc(decl))
    }

    pub(crate) fn expand_state_destructuring(
        &mut self,
        stmts: &mut OxcVec<'a, Statement<'a>>,
    ) {
        self.rewrite_destructured_rune_decls(
            stmts,
            |declarator, rune_kind| {
                !matches!(
                    declarator.id,
                    BindingPattern::BindingIdentifier(_)
                ) && matches!(rune_kind, Some(RuneKind::State | RuneKind::StateRaw))
                    && declarator.init.is_some()
            },
            |this, decl_kind, _decl_span_start, mut declarator, rune_kind| {
                let init = declarator
                    .init
                    .take()
                    .expect("predicate matched only declarators with an initializer");
                let value = if let Expression::CallExpression(mut call) = init {
                    if call.arguments.is_empty() {
                        this.b
                            .ast
                            .expression_object(SPAN, this.b.ast.vec())
                    } else {
                        let mut dummy = Argument::from(this.b.cheap_expr());
                        mem::swap(&mut call.arguments[0], &mut dummy);
                        dummy.into_expression()
                    }
                } else {
                    unreachable!()
                };

                this.gen_state_destructuring(&declarator.id, value, rune_kind, decl_kind)
            },
        );
    }

    fn gen_state_destructuring(
        &mut self,
        pattern: &BindingPattern<'a>,
        value: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: VariableDeclarationKind,
    ) -> Statement<'a> {
        let tmp_name = self.gen_unique_name("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut declarators = Vec::new();

        let tmp_declarator = self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(tmp_name_str)),
            NONE,
            Some(value),
            false,
        );
        declarators.push(tmp_declarator);

        let tmp_expr = self.b.rid_expr(tmp_name_str);
        self.gen_destructure_declarators(
            pattern,
            tmp_expr,
            rune_kind,
            decl_kind,
            Self::state_destructure_dev_label(pattern, rune_kind),
            &mut declarators,
        );

        let decl = self.b.ast.variable_declaration(
            SPAN,
            decl_kind,
            self.b.ast.vec_from_iter(declarators),
            false,
        );
        Statement::VariableDeclaration(self.b.alloc(decl))
    }

    fn gen_destructure_declarators(
        &mut self,
        pattern: &BindingPattern<'a>,
        accessor: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: VariableDeclarationKind,
        dev_label: Option<&'static str>,
        declarators: &mut Vec<VariableDeclarator<'a>>,
    ) {
        let mut temps: Vec<(String, &'a str)> = Vec::new();
        walk_bindings(pattern, |v| {
            let access = self.build_decl_access(
                v.path,
                v.is_rest,
                v.excluded,
                &accessor,
                dev_label,
                decl_kind,
                &mut temps,
                declarators,
            );
            self.push_destructure_leaf(v.symbol, access, rune_kind, decl_kind, declarators);
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn build_decl_access<'w>(
        &mut self,
        path: &[Step<'w>],
        is_rest: bool,
        excluded: &[&PropertyKey<'w>],
        root: &Expression<'a>,
        dev_label: Option<&'static str>,
        decl_kind: VariableDeclarationKind,
        temps: &mut Vec<(String, &'a str)>,
        declarators: &mut Vec<VariableDeclarator<'a>>,
    ) -> Expression<'a> {
        let mut current = root.clone_in(self.b.ast.allocator);
        for (i, step) in path.iter().enumerate() {
            match step.access {
                Access::Key { key, computed } => {
                    current = self.build_object_member_access(current, key, computed);
                }
                Access::Index { index, len, has_rest } => {
                    let temp = self.decl_array_temp(
                        &path[..i],
                        current,
                        len,
                        has_rest,
                        dev_label,
                        decl_kind,
                        temps,
                        declarators,
                    );
                    let get = self.b.call_expr("$.get", [Arg::Ident(temp)]);
                    current = self.b.computed_member_expr(get, self.b.num_expr(index as f64));
                }
                Access::Slice { from } => {
                    let temp = self.decl_array_temp(
                        &path[..i],
                        current,
                        from,
                        true,
                        dev_label,
                        decl_kind,
                        temps,
                        declarators,
                    );
                    let get = self.b.call_expr("$.get", [Arg::Ident(temp)]);
                    let slice = self.b.static_member_expr(get, "slice");
                    current = self.b.call_expr_callee(slice, [Arg::Num(from as f64)]);
                }
            }
            if let Some(default) = step.default {
                let default_expr = default.clone_in(self.b.ast.allocator);
                current = self
                    .b
                    .call_expr("$.fallback", [Arg::Expr(current), Arg::Expr(default_expr)]);
            }
        }
        if is_rest {
            let keys = excluded
                .iter()
                .filter_map(|k| Self::property_key_name(k))
                .collect::<Vec<_>>();
            let keys_array = self.b.array_expr(keys.iter().map(|k| self.b.str_expr(k)));
            current = self.b.call_expr(
                "$.exclude_from_object",
                [Arg::Expr(current), Arg::Expr(keys_array)],
            );
        }
        current
    }

    #[allow(clippy::too_many_arguments)]
    fn decl_array_temp<'w>(
        &mut self,
        prefix: &[Step<'w>],
        source: Expression<'a>,
        len: u32,
        has_rest: bool,
        dev_label: Option<&'static str>,
        decl_kind: VariableDeclarationKind,
        temps: &mut Vec<(String, &'a str)>,
        declarators: &mut Vec<VariableDeclarator<'a>>,
    ) -> &'a str {
        let key = serialize_binding_prefix(prefix);
        if let Some((_, name)) = temps.iter().find(|(k, _)| *k == key) {
            return name;
        }
        let name_owned = self.ident_gen.generate("$$array");
        let name: &'a str = self.b.alloc_str(&name_owned);
        let to_array = if has_rest {
            self.b.call_expr("$.to_array", [Arg::Expr(source)])
        } else {
            self.b
                .call_expr("$.to_array", [Arg::Expr(source), Arg::Num(len as f64)])
        };
        let thunk = self
            .b
            .arrow_expr(self.b.no_params(), [self.b.expr_stmt(to_array)]);
        let derived = self.b.call_expr("$.derived", [Arg::Expr(thunk)]);
        let derived = match dev_label.filter(|_| self.dev) {
            Some(label) => self
                .b
                .call_expr("$.tag", [Arg::Expr(derived), Arg::Str(label.to_string())]),
            None => derived,
        };
        let declarator = self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
            NONE,
            Some(derived),
            false,
        );
        declarators.push(declarator);
        temps.push((key, name));
        name
    }

    fn push_destructure_leaf(
        &mut self,
        symbol: SymbolId,
        accessor: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: VariableDeclarationKind,
        declarators: &mut Vec<VariableDeclarator<'a>>,
    ) {
        let name: &'a str = self.b.alloc_str(self.component_scoping.symbol_name(symbol));
        let reassigned = self.component_scoping.is_mutated(symbol)
            || self.component_scoping.is_reexported_specifier_local(symbol);
        let is_signal_source = self
            .analysis
            .as_ref()
            .is_some_and(|a| a.script.is_state_source(reassigned));
        let is_proxy =
            matches!(rune_kind, RuneKind::State) && rune_refs::should_proxy(&accessor);
        let final_value = self.wrap_state_value(accessor, rune_kind, is_signal_source);
        let final_value = if self.dev {
            if is_signal_source {
                self.b
                    .call_expr("$.tag", [Arg::Expr(final_value), Arg::Str(name.to_string())])
            } else if is_proxy {
                self.b.call_expr(
                    "$.tag_proxy",
                    [Arg::Expr(final_value), Arg::Str(name.to_string())],
                )
            } else {
                final_value
            }
        } else {
            final_value
        };
        let declarator = self.b.ast.variable_declarator(
            SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(SPAN, self.b.ast.atom(name)),
            NONE,
            Some(final_value),
            false,
        );
        declarators.push(declarator);
    }

    pub(crate) fn wrap_state_value(
        &self,
        value: Expression<'a>,
        rune_kind: RuneKind,
        is_signal_source: bool,
    ) -> Expression<'a> {
        let value = value.into_inner_expression();
        match rune_kind {
            RuneKind::State => {
                let proxied = if rune_refs::should_proxy(&value) {
                    self.b.call_expr("$.proxy", [Arg::Expr(value)])
                } else {
                    value
                };
                if is_signal_source {
                    self.b.call_expr("$.state", [Arg::Expr(proxied)])
                } else {
                    proxied
                }
            }
            RuneKind::StateRaw => {
                if is_signal_source {
                    self.b.call_expr("$.state", [Arg::Expr(value)])
                } else {
                    value
                }
            }
            RuneKind::Derived | RuneKind::DerivedBy => {
                let thunk = self
                    .b
                    .arrow_expr(self.b.no_params(), [self.b.expr_stmt(value)]);
                self.b.call_expr("$.derived", [Arg::Expr(thunk)])
            }
            _ => value,
        }
    }

    fn gen_async_derived_destructuring(
        &mut self,
        pattern: &BindingPattern<'a>,
        init: Expression<'a>,
        decl_span_start: u32,
        decl_kind: VariableDeclarationKind,
    ) -> Statement<'a> {
        let Expression::CallExpression(mut call) = init else {
            unreachable!("async derived destructuring should be a call");
        };

        let init_span_start = call.span.start;
        let mut dummy = Argument::from(self.b.cheap_expr());
        mem::swap(&mut call.arguments[0], &mut dummy);
        let awaited = dummy.into_expression();

        let thunk = if let Expression::AwaitExpression(await_expr) = awaited {
            let source_expr = await_expr.unbox().argument;
            let await_inner = self.b.await_expr(source_expr);
            self.b.async_thunk(await_inner)
        } else {
            self.b.async_arrow_expr_body(awaited)
        };

        let tmp_name = self.gen_unique_name("$$d");
        let tmp_name_str = self.b.alloc_str(&tmp_name);

        let mut args: Vec<Arg<'a, '_>> = vec![Arg::Expr(thunk)];
        if self.dev {
            let kind = match pattern {
                BindingPattern::ArrayPattern(_) => "iterable",
                _ => "object",
            };
            let label = format!("[$derived {kind}]");
            args.push(Arg::Expr(self.b.str_expr(&label)));

            if !self
                .ignore_query
                .is_ignored_at_span(decl_span_start, "await_waterfall")
            {
                let (line, col) = self.component_line_index.line_col(init_span_start);
                let loc = format!("{}:{}:{}", sanitize_location(self.filename), line, col);
                args.push(Arg::Expr(self.b.str_expr(&loc)));
            }
        }

        let async_derived = self.b.call_expr("$.async_derived", args);
        let tmp_init = match self.async_derived_mode() {
            AsyncDerivedMode::Await => self.b.await_expr(async_derived),
            AsyncDerivedMode::Save => {
                let saved = self.b.call_expr("$.save", [Arg::Expr(async_derived)]);
                self.b
                    .call_expr_callee(self.b.await_expr(saved), iter::empty::<Arg<'a, '_>>())
            }
        };

        let access_root = self.b.call_expr("$.get", [Arg::Ident(tmp_name_str)]);
        if self.function_info_stack.is_empty() {
            let tmp_stmt = self.b.var_stmt(tmp_name_str, tmp_init);
            let mut block_stmts = vec![tmp_stmt];
            self.gen_derived_destructure_assignments(pattern, access_root, &mut block_stmts);
            self.b.block_stmt(block_stmts)
        } else {
            let mut declarators = Vec::new();
            let tmp_declarator = self.b.ast.variable_declarator(
                SPAN,
                decl_kind,
                self.b.ast.binding_pattern_binding_identifier(
                    SPAN,
                    self.b.ast.atom(tmp_name_str),
                ),
                NONE,
                Some(tmp_init),
                false,
            );
            declarators.push(tmp_declarator);
            self.gen_destructure_declarators(
                pattern,
                access_root,
                RuneKind::Derived,
                decl_kind,
                None,
                &mut declarators,
            );
            let decl = self.b.ast.variable_declaration(
                SPAN,
                decl_kind,
                self.b.ast.vec_from_iter(declarators),
                false,
            );
            Statement::VariableDeclaration(self.b.alloc(decl))
        }
    }

    fn gen_derived_destructure_assignments(
        &mut self,
        pattern: &BindingPattern<'a>,
        accessor: Expression<'a>,
        stmts: &mut Vec<Statement<'a>>,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(id) => {
                let value = self.wrap_state_value(accessor, RuneKind::Derived, false);
                let value = if self.dev {
                    self.b
                        .call_expr("$.tag", [Arg::Expr(value), Arg::Str(id.name.to_string())])
                } else {
                    value
                };
                stmts.push(self.b.assign_stmt(
                    AssignLeft::Ident(id.name.to_string()),
                    value,
                ));
            }
            BindingPattern::ObjectPattern(obj) => {
                let mut key_names: Vec<String> = Vec::new();
                for prop in &obj.properties {
                    if let Some(name) = Self::property_key_name(&prop.key) {
                        key_names.push(name);
                    }
                }

                for prop in &obj.properties {
                    let member = self.build_object_member_access(
                        accessor.clone_in(self.b.ast.allocator),
                        &prop.key,
                        prop.computed,
                    );
                    self.gen_derived_destructure_assignments(&prop.value, member, stmts);
                }

                if let Some(rest) = &obj.rest {
                    let keys_array = self
                        .b
                        .array_expr(key_names.iter().map(|k| self.b.str_expr(k)));
                    let exclude_expr = self.b.call_expr(
                        "$.exclude_from_object",
                        [Arg::Expr(accessor), Arg::Expr(keys_array)],
                    );
                    self.gen_derived_destructure_assignments(&rest.argument, exclude_expr, stmts);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                let array_name = self.ident_gen.generate("$$array");
                let array_name_str = self.b.alloc_str(&array_name);

                let len_arg = if arr.rest.is_some() {
                    vec![Arg::Expr(accessor)]
                } else {
                    vec![Arg::Expr(accessor), Arg::Num(arr.elements.len() as f64)]
                };
                let to_array_call = self.b.call_expr("$.to_array", len_arg);
                let thunk = self
                    .b
                    .arrow_expr(self.b.no_params(), [self.b.expr_stmt(to_array_call)]);
                stmts.push(self.b.var_stmt(
                    array_name_str,
                    self.b.call_expr("$.derived", [Arg::Expr(thunk)]),
                ));

                for (idx, elem) in arr.elements.iter().enumerate() {
                    let Some(elem) = elem else { continue };
                    let get_array = self.b.call_expr("$.get", [Arg::Ident(array_name_str)]);
                    let elem_access = self
                        .b
                        .computed_member_expr(get_array, self.b.num_expr(idx as f64));
                    self.gen_derived_destructure_assignments(elem, elem_access, stmts);
                }

                if let Some(rest) = &arr.rest {
                    let get_array = self.b.call_expr("$.get", [Arg::Ident(array_name_str)]);
                    let slice = self.b.static_member_expr(get_array, "slice");
                    let slice_call = self.b.ast.expression_call(
                        SPAN,
                        slice,
                        NONE,
                        self.b.ast.vec_from_array([Argument::from(
                            self.b.num_expr(arr.elements.len() as f64),
                        )]),
                        false,
                    );
                    self.gen_derived_destructure_assignments(&rest.argument, slice_call, stmts);
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                let default_expr = assign.right.clone_in(self.b.ast.allocator);
                let fallback = self
                    .b
                    .call_expr("$.fallback", [Arg::Expr(accessor), Arg::Expr(default_expr)]);
                self.gen_derived_destructure_assignments(&assign.left, fallback, stmts);
            }
        }
    }

    pub(crate) fn gen_unique_name(&mut self, prefix: &str) -> String {
        let n = self.ident_counter;
        if n == 0 {
            prefix.to_string()
        } else {
            let mut s = String::with_capacity(prefix.len() + 4);
            s.push_str(prefix);
            s.push('_');
            s.push_str(&n.to_string());
            s
        }
    }

    pub(crate) fn property_key_name(key: &PropertyKey<'_>) -> Option<String> {
        property_key_static_name(key).map(|s| s.into_owned())
    }

    pub(crate) fn build_object_member_access(
        &self,
        object: Expression<'a>,
        key: &PropertyKey<'_>,
        computed: bool,
    ) -> Expression<'a> {
        if computed {
            if let Some(expr) = Self::property_key_to_expr(self.b, key) {
                self.b.computed_member_expr(object, expr)
            } else {
                object
            }
        } else {
            match key {
                PropertyKey::StaticIdentifier(id) => self
                    .b
                    .static_member_expr(object, self.b.alloc_str(id.name.as_str())),
                PropertyKey::StringLiteral(s) => self
                    .b
                    .static_member_expr(object, self.b.alloc_str(s.value.as_str())),
                _ => object,
            }
        }
    }

    fn property_key_to_expr<'c>(
        b: &'c Builder<'a>,
        key: &PropertyKey<'_>,
    ) -> Option<Expression<'a>> {
        match key {
            PropertyKey::StringLiteral(s) => Some(b.str_expr(s.value.as_str())),
            PropertyKey::NumericLiteral(n) => Some(b.num_expr(n.value)),
            _ => None,
        }
    }

    pub(crate) fn scan_class_state_fields(
        &self,
        body: &ClassBody<'a>,
    ) -> ClassStateInfo {
        let mut fields = Vec::new();

        let mut existing_private: FxHashSet<String> = FxHashSet::default();
        for element in &body.body {
            if let ClassElement::PropertyDefinition(prop) = element
                && let PropertyKey::PrivateIdentifier(id) = &prop.key
            {
                existing_private.insert(id.name.to_string());
            }
        }

        let mut body_public_names: FxHashSet<String> = FxHashSet::default();
        let mut placeholder_public_names: FxHashSet<String> = FxHashSet::default();
        for element in &body.body {
            if let ClassElement::PropertyDefinition(prop) = element {
                if let PropertyKey::StaticIdentifier(id) = &prop.key
                    && !prop.computed
                    && prop.value.is_none()
                {
                    placeholder_public_names.insert(id.name.to_string());
                }
                if prop.value.is_none() {
                    continue;
                }
                let Some(rune_kind) = self.class_field_rune_kind(prop.node_id()) else {
                    continue;
                };

                match &prop.key {
                    PropertyKey::PrivateIdentifier(id) => {
                        fields.push(ClassStateField {
                            public_name: None,
                            private_name: id.name.to_string(),
                            rune_kind,
                        });
                    }
                    PropertyKey::StaticIdentifier(id) if !prop.computed => {
                        let name = id.name.to_string();
                        let mut backing = format!("#{}", name);
                        while existing_private.contains(backing.trim_start_matches('#')) {
                            backing = format!("#_{}", backing.trim_start_matches('#'));
                        }
                        existing_private.insert(backing.trim_start_matches('#').to_string());
                        body_public_names.insert(name.clone());
                        fields.push(ClassStateField {
                            public_name: Some(name),
                            private_name: backing.trim_start_matches('#').to_string(),
                            rune_kind,
                        });
                    }
                    _ => {}
                }
            }
        }

        let mut ctor_synth_names = FxHashSet::default();
        let mut ctor_placeholder_names = FxHashSet::default();
        let mut ctor_private_names: FxHashSet<String> = FxHashSet::default();
        let body_private_field_names: FxHashSet<String> = body
            .body
            .iter()
            .filter_map(|el| {
                if let ClassElement::PropertyDefinition(prop) = el
                    && let PropertyKey::PrivateIdentifier(id) = &prop.key
                {
                    Some(id.name.to_string())
                } else {
                    None
                }
            })
            .collect();
        for element in &body.body {
            if let ClassElement::MethodDefinition(method) = element
                && method.kind == MethodDefinitionKind::Constructor
                && let Some(func_body) = &method.value.body
            {
                for stmt in &func_body.statements {
                    if let Statement::ExpressionStatement(es) = stmt
                        && let Expression::AssignmentExpression(assign) = &es.expression
                        && assign.operator == AssignmentOperator::Assign
                        && let Some(rune_kind) = self.class_field_rune_kind(assign.node_id())
                    {
                        match &assign.left {
                            AssignmentTarget::StaticMemberExpression(member)
                                if matches!(&member.object, Expression::ThisExpression(_)) =>
                            {
                                let name = member.property.name.to_string();
                                if body_public_names.contains(&name)
                                    || !ctor_synth_names.insert(name.clone())
                                {
                                    continue;
                                }
                                let mut backing = format!("#{}", name);
                                while existing_private.contains(backing.trim_start_matches('#')) {
                                    backing = format!("#_{}", backing.trim_start_matches('#'));
                                }
                                existing_private
                                    .insert(backing.trim_start_matches('#').to_string());
                                if placeholder_public_names.contains(&name) {
                                    ctor_placeholder_names.insert(name.clone());
                                }
                                fields.push(ClassStateField {
                                    public_name: Some(name),
                                    private_name: backing
                                        .trim_start_matches('#')
                                        .to_string(),
                                    rune_kind,
                                });
                            }
                            AssignmentTarget::PrivateFieldExpression(member)
                                if matches!(&member.object, Expression::ThisExpression(_)) =>
                            {
                                let name = member.field.name.to_string();
                                if !body_private_field_names.contains(&name) {
                                    continue;
                                }
                                if !ctor_private_names.insert(name.clone()) {
                                    continue;
                                }
                                if fields.iter().any(|f| {
                                    f.public_name.is_none() && f.private_name == name
                                }) {
                                    continue;
                                }
                                fields.push(ClassStateField {
                                    public_name: None,
                                    private_name: name,
                                    rune_kind,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        ClassStateInfo {
            fields,
            ctor_synth_names,
            ctor_placeholder_names,
        }
    }

    pub(crate) fn rewrite_class_body(
        &self,
        body: &mut ClassBody<'a>,
        info: &ClassStateInfo,
    ) {
        use ClassElement;

        let public_fields: HashMap<&str, &ClassStateField> = info
            .fields
            .iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();
        let private_fields: FxHashSet<&str> = info
            .fields
            .iter()
            .filter(|f| f.public_name.is_none())
            .map(|f| f.private_name.as_str())
            .collect();

        let old_elements: Vec<ClassElement<'a>> = {
            let mut temp = self.b.ast.vec();
            mem::swap(&mut body.body, &mut temp);
            temp.into_iter().collect()
        };

        let mut new_body: Vec<ClassElement<'a>> = Vec::new();

        for field_info in info.fields.iter().filter(|f| {
            f.public_name
                .as_deref()
                .is_some_and(|n| info.ctor_synth_names.contains(n))
        }) {
            let name = field_info
                .public_name
                .as_deref()
                .expect("field_info with public_name is required by caller filter");
            new_body.push(self.b.class_private_field(&field_info.private_name, None));
            self.emit_getter_setter(&mut new_body, field_info, name);
        }

        for element in old_elements {
            match element {
                ClassElement::PropertyDefinition(mut prop) => {
                    let is_rune_prop = prop.value.is_some()
                        && self.class_field_rune_kind(prop.node_id()).is_some();
                    if !is_rune_prop {
                        let is_ctor_placeholder = prop.value.is_none()
                            && match &prop.key {
                                PropertyKey::StaticIdentifier(id)
                                    if !prop.computed =>
                                {
                                    info.ctor_placeholder_names.contains(id.name.as_str())
                                }
                                _ => false,
                            };
                        if !is_ctor_placeholder {
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                        continue;
                    }

                    match &prop.key {
                        PropertyKey::PrivateIdentifier(id) => {
                            let name = id.name.to_string();
                            if private_fields.contains(name.as_str()) {
                                self.rewrite_private_field_callee(&mut prop);
                            }
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                        PropertyKey::StaticIdentifier(id) if !prop.computed => {
                            let name = id.name.to_string();
                            if let Some(field_info) = public_fields.get(name.as_str()) {
                                self.emit_public_field_rewrite(
                                    &mut new_body,
                                    &mut prop,
                                    field_info,
                                    &name,
                                );
                            } else {
                                new_body.push(ClassElement::PropertyDefinition(prop));
                            }
                        }
                        _ => {
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                    }
                }
                ClassElement::MethodDefinition(mut method) => {
                    if method.kind == MethodDefinitionKind::Constructor {
                        self.rewrite_constructor(&mut method, info);
                    }
                    new_body.push(ClassElement::MethodDefinition(method));
                }
                other => {
                    new_body.push(other);
                }
            }
        }

        body.body = self.b.ast.vec_from_iter(new_body);
    }

    fn rewrite_private_field_callee(&self, prop: &mut PropertyDefinition<'a>) {
        let rune_kind = self.class_field_rune_kind(prop.node_id());
        if let Some(value) = prop.value.take() {
            prop.value = Some(value.into_inner_expression());
        }
        if let Some(Expression::CallExpression(call)) = &mut prop.value {
            match rune_kind {
                Some(RuneKind::State) => {
                    call.callee = self.b.rid_expr("$.state");
                    if !call.arguments.is_empty() {
                        let mut dummy = Argument::from(self.b.cheap_expr());
                        mem::swap(&mut call.arguments[0], &mut dummy);
                        let arg = dummy.into_expression();
                        let wrapped = if rune_refs::should_proxy(&arg) {
                            self.b.call_expr("$.proxy", [Arg::Expr(arg)])
                        } else {
                            arg
                        };
                        call.arguments[0] = Argument::from(wrapped);
                    }
                }
                Some(RuneKind::StateRaw) => {
                    call.callee = self.b.rid_expr("$.state");
                }
                Some(RuneKind::Derived) => {
                    call.callee = self.b.rid_expr("$.derived");
                    if !call.arguments.is_empty() {
                        let mut dummy = Argument::from(self.b.cheap_expr());
                        mem::swap(&mut call.arguments[0], &mut dummy);
                        let thunked = self.b.thunk(dummy.into_expression());
                        call.arguments[0] = Argument::from(thunked);
                    }
                }
                Some(RuneKind::DerivedBy) => {
                    call.callee = self.b.rid_expr("$.derived");
                }
                _ => {}
            }
            if self.dev && rune_kind.is_some() {
                let field_name = match &prop.key {
                    PropertyKey::PrivateIdentifier(id) => format!("#{}", id.name),
                    _ => String::new(),
                };
                let label = self.class_tag_label(&field_name);
                let value = self.b.move_expr(
                    prop.value
                        .as_mut()
                        .expect("rune property definitions always carry an initializer"),
                );
                prop.value = Some(
                    self.b
                        .call_expr("$.tag", [Arg::Expr(value), Arg::Str(label)]),
                );
            }
        }
    }

    fn emit_public_field_rewrite(
        &self,
        new_body: &mut Vec<ClassElement<'a>>,
        prop: &mut PropertyDefinition<'a>,
        field_info: &ClassStateField,
        name: &str,
    ) {
        let arg = if let Some(Expression::CallExpression(mut call)) =
            prop.value.take().map(|v| v.into_inner_expression())
        {
            if call.arguments.is_empty() {
                None
            } else {
                let mut dummy = Argument::from(self.b.cheap_expr());
                mem::swap(&mut call.arguments[0], &mut dummy);
                Some(dummy.into_expression())
            }
        } else {
            None
        };

        let init_call = match field_info.rune_kind {
            RuneKind::Derived => {
                let thunked = self.b.thunk(arg.unwrap_or_else(|| self.b.cheap_expr()));
                self.b.call_expr("$.derived", [Arg::Expr(thunked)])
            }
            RuneKind::DerivedBy => {
                if let Some(arg) = arg {
                    self.b.call_expr("$.derived", [Arg::Expr(arg)])
                } else {
                    self.b
                        .call_expr("$.derived", iter::empty::<Arg<'a, '_>>())
                }
            }
            RuneKind::State => {
                if let Some(arg) = arg {
                    let wrapped = if rune_refs::should_proxy(&arg) {
                        self.b.call_expr("$.proxy", [Arg::Expr(arg)])
                    } else {
                        arg
                    };
                    self.b.call_expr("$.state", [Arg::Expr(wrapped)])
                } else {
                    self.b
                        .call_expr("$.state", iter::empty::<Arg<'a, '_>>())
                }
            }
            _ => {
                if let Some(arg) = arg {
                    self.b.call_expr("$.state", [Arg::Expr(arg)])
                } else {
                    self.b
                        .call_expr("$.state", iter::empty::<Arg<'a, '_>>())
                }
            }
        };

        let init_call = if self.dev {
            let label = self.class_tag_label(name);
            self.b
                .call_expr("$.tag", [Arg::Expr(init_call), Arg::Str(label)])
        } else {
            init_call
        };

        new_body.push(
            self.b
                .class_private_field(&field_info.private_name, Some(init_call)),
        );
        self.emit_getter_setter(new_body, field_info, name);
    }

    fn emit_getter_setter(
        &self,
        new_body: &mut Vec<ClassElement<'a>>,
        field_info: &ClassStateField,
        name: &str,
    ) {
        let get_call = self.b.call_expr(
            "$.get",
            [Arg::Expr(
                self.b.this_private_member(&field_info.private_name),
            )],
        );
        let return_stmt = self.b.return_stmt(get_call);
        new_body.push(
            self.b
                .class_getter(self.b.public_key(name), vec![return_stmt]),
        );

        let mut set_args: Vec<Arg<'a, '_>> = vec![
            Arg::Expr(self.b.this_private_member(&field_info.private_name)),
            Arg::Ident("value"),
        ];
        if field_info.rune_kind == RuneKind::State {
            set_args.push(Arg::Bool(true));
        }
        let set_call = self.b.call_stmt("$.set", set_args);
        new_body.push(
            self.b
                .class_setter(self.b.public_key(name), "value", vec![set_call]),
        );
    }

    pub(crate) fn rewrite_constructor(
        &self,
        method: &mut OxcBox<'a, MethodDefinition<'a>>,
        info: &ClassStateInfo,
    ) {
        let Some(func_body) = &mut method.value.body else {
            return;
        };

        let ctor_fields: HashMap<&str, &ClassStateField> = info
            .fields
            .iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();
        let ctor_private_fields: HashMap<&str, &ClassStateField> = info
            .fields
            .iter()
            .filter(|f| f.public_name.is_none())
            .map(|f| (f.private_name.as_str(), f))
            .collect();

        for stmt in func_body.statements.iter_mut() {
            if let Statement::ExpressionStatement(es) = stmt
                && let Expression::AssignmentExpression(assign) = &mut es.expression
                && assign.operator == AssignmentOperator::Assign
            {
                let (resolved_field, is_private_target, public_name) = match &assign.left {
                    AssignmentTarget::StaticMemberExpression(member)
                        if matches!(&member.object, Expression::ThisExpression(_)) =>
                    {
                        let name = member.property.name.to_string();
                        let field = ctor_fields.get(name.as_str()).copied();
                        (field, false, Some(name))
                    }
                    AssignmentTarget::PrivateFieldExpression(member)
                        if matches!(&member.object, Expression::ThisExpression(_)) =>
                    {
                        let name = member.field.name.to_string();
                        let field = ctor_private_fields.get(name.as_str()).copied();
                        (field, true, None)
                    }
                    _ => (None, false, None),
                };
                if let Some(field_info) = resolved_field
                    && let Expression::CallExpression(call) = &mut assign.right
                {
                    match field_info.rune_kind {
                        RuneKind::Derived => {
                            call.callee = self.b.rid_expr("$.derived");
                            if !call.arguments.is_empty() {
                                let mut dummy = Argument::from(self.b.cheap_expr());
                                mem::swap(&mut call.arguments[0], &mut dummy);
                                let thunked = self.b.thunk(dummy.into_expression());
                                call.arguments[0] = Argument::from(thunked);
                            }
                        }
                        RuneKind::DerivedBy => {
                            call.callee = self.b.rid_expr("$.derived");
                        }
                        RuneKind::State => {
                            call.callee = self.b.rid_expr("$.state");
                            let needs_proxy = call
                                .arguments
                                .first()
                                .and_then(|a| a.as_expression())
                                .is_some_and(|e| rune_refs::should_proxy(e));
                            if needs_proxy {
                                let mut dummy = Argument::from(self.b.cheap_expr());
                                mem::swap(&mut call.arguments[0], &mut dummy);
                                let inner = dummy.into_expression();
                                let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                                call.arguments[0] = Argument::from(proxied);
                            }
                        }
                        RuneKind::StateRaw => {
                            call.callee = self.b.rid_expr("$.state");
                        }
                        _ => {
                            call.callee = self.b.rid_expr("$.state");
                        }
                    }
                    if self.dev {
                        let label_name = public_name
                            .as_deref()
                            .unwrap_or(field_info.private_name.as_str());
                        let label = self.class_tag_label(label_name);
                        let rhs = self.b.move_expr(&mut assign.right);
                        assign.right = self.b.call_expr("$.tag", [Arg::Expr(rhs), Arg::Str(label)]);
                    }

                    if !is_private_target {
                        let new_left = self.b.this_private_member(&field_info.private_name);
                        if let Expression::PrivateFieldExpression(pfe) = new_left {
                            assign.left = AssignmentTarget::PrivateFieldExpression(pfe);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn is_private_state_field(&self, name: &str) -> bool {
        self.private_state_field_rune_kind(name).is_some()
    }

    pub(crate) fn private_state_field_rune_kind(&self, name: &str) -> Option<RuneKind> {
        self.class_state_stack.last().and_then(|info| {
            info.fields
                .iter()
                .find(|f| f.public_name.is_none() && f.private_name == name)
                .map(|f| f.rune_kind)
        })
    }

    pub(crate) fn in_constructor(&self) -> bool {
        self.function_info_stack
            .last()
            .is_some_and(|f| f.in_constructor)
    }

    pub(crate) fn async_derived_mode(&self) -> AsyncDerivedMode {
        if self.strip_exports && self.function_info_stack.len() > 1 {
            AsyncDerivedMode::Save
        } else {
            AsyncDerivedMode::Await
        }
    }

    fn class_tag_label(&self, field_name: &str) -> String {
        let class_name = self
            .class_name_stack
            .last()
            .and_then(|n| n.as_deref())
            .unwrap_or("[class]");
        format!("{}.{}", class_name, field_name)
    }
}

fn derived_destructure_emit(
    analysis: Option<&AnalysisData<'_>>,
    pattern: &BindingPattern<'_>,
) -> Option<DerivedEmit> {
    let analysis = analysis?;
    let mut first = None;
    walk_bindings(pattern, |v| {
        if first.is_none() {
            first = Some(v.symbol);
        }
    });
    match analysis.binding_semantics(first?) {
        BindingSemantics::Derived(d) => Some(d.lowering),
        BindingSemantics::State(_) | BindingSemantics::OptimizedRune(_) => None,
        BindingSemantics::Prop(_) | BindingSemantics::RuntimeRune { .. } => None,
        BindingSemantics::LegacyState(_) | BindingSemantics::LegacyBindableProp(_) => None,
        BindingSemantics::Store(_) => None,
        BindingSemantics::NonReactive | BindingSemantics::MaybeReactive => None,
        BindingSemantics::Const(_) | BindingSemantics::Contextual(_) => None,
        BindingSemantics::Unresolved | BindingSemantics::LegacyApiExport => None,
    }
}
