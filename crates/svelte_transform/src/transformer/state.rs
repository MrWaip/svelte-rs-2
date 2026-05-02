use rustc_hash::FxHashSet;

use oxc_allocator::CloneIn;
use oxc_ast::NONE;
use oxc_ast::ast::{Expression, Statement};
use oxc_syntax::node::NodeId as OxcNodeId;
use svelte_analyze::RuneKind;

use svelte_ast_builder::Arg;

use super::location::sanitize_location;
use super::model::{AsyncDerivedMode, ClassStateField, ClassStateInfo, ComponentTransformer};

impl<'b, 'a> ComponentTransformer<'b, 'a> {
    fn state_destructure_dev_label(
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        rune_kind: RuneKind,
    ) -> Option<&'static str> {
        if !matches!(rune_kind, RuneKind::State | RuneKind::StateRaw) {
            return None;
        }

        match pattern {
            oxc_ast::ast::BindingPattern::ArrayPattern(_) => Some("[$state iterable]"),
            oxc_ast::ast::BindingPattern::ObjectPattern(_) => Some("[$state object]"),
            _ => None,
        }
    }

    fn rewrite_destructured_rune_decls(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
        mut predicate: impl FnMut(&oxc_ast::ast::VariableDeclarator<'a>, Option<RuneKind>) -> bool,
        mut rewrite: impl FnMut(
            &mut Self,
            oxc_ast::ast::VariableDeclarationKind,
            u32,
            oxc_ast::ast::VariableDeclarator<'a>,
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
        declarator: &oxc_ast::ast::VariableDeclarator<'a>,
    ) -> Option<RuneKind> {
        Self::first_binding_symbol(&declarator.id)
            .and_then(|sym| self.rune_for_symbol(sym))
            .or_else(|| {
                declarator
                    .init
                    .as_ref()
                    .and_then(|init| self.rune_kind_from_expr(init))
            })
    }

    pub(crate) fn rune_kind_from_expr(&self, expr: &Expression<'_>) -> Option<RuneKind> {
        if let Some(kind) = Self::detect_class_field_rune_kind(expr) {
            return Some(kind);
        }
        if let Some(index) = self.script_rune_calls
            && let Some(kind) = script_rune_call_node_id(expr, self.script_node_id_offset)
                .and_then(|node| index.kind(node))
        {
            return Some(kind);
        }
        None
    }

    fn first_binding_symbol(
        pattern: &oxc_ast::ast::BindingPattern<'a>,
    ) -> Option<svelte_component_semantics::SymbolId> {
        let mut first = None;
        svelte_component_semantics::walk_bindings(pattern, |v| {
            if first.is_none() {
                first = Some(v.symbol);
            }
        });
        first
    }

    pub(crate) fn process_derived_destructuring(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
    ) {
        let dev = self.dev;
        self.rewrite_destructured_rune_decls(
            stmts,
            |declarator, rune_kind| {
                !matches!(declarator.id, oxc_ast::ast::BindingPattern::BindingIdentifier(_))
                    && matches!(rune_kind, Some(RuneKind::Derived | RuneKind::DerivedBy))
                    && declarator.init.as_ref().is_some_and(|init| {
                        if let Expression::CallExpression(call) = init {
                            call.arguments.first()
                                .and_then(|arg| arg.as_expression())
                                .is_some_and(|expr| {
                                    !(matches!(expr, Expression::AwaitExpression(_))
                                        || (dev
                                            && matches!(expr, Expression::CallExpression(c)
                                                if c.arguments.is_empty() && matches!(&c.callee, Expression::AwaitExpression(_)))))
                                })
                        } else {
                            false
                        }
                    })
            },
            |this, decl_kind, _decl_span_start, mut declarator, rune_kind| {
                let init = declarator
                    .init
                    .take()
                    .expect("predicate matched only declarators with an initializer");
                this.gen_sync_derived_destructuring(&declarator.id, init, rune_kind, decl_kind)
            },
        );
        self.rewrite_destructured_rune_decls(
            stmts,
            |declarator, rune_kind| {
                !matches!(declarator.id, oxc_ast::ast::BindingPattern::BindingIdentifier(_))
                    && matches!(rune_kind, Some(RuneKind::Derived))
                    && declarator.init.as_ref().is_some_and(|init| {
                        if let Expression::CallExpression(call) = init {
                            call.arguments.first()
                                .and_then(|arg| arg.as_expression())
                                .is_some_and(|expr| {
                                    matches!(expr, Expression::AwaitExpression(_))
                                    || (dev && matches!(expr, Expression::CallExpression(c)
                                        if c.arguments.is_empty() && matches!(&c.callee, Expression::AwaitExpression(_))))
                                })
                        } else {
                            false
                        }
                    })
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
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        init: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: oxc_ast::ast::VariableDeclarationKind,
    ) -> Statement<'a> {
        let Expression::CallExpression(mut call) = init else {
            unreachable!("sync derived destructuring should be a call");
        };
        call.callee = self.b.rid_expr("$.derived");

        let mut declarators = Vec::new();

        let arg_expr = call.arguments.remove(0).into_expression();

        let use_direct_access =
            matches!(rune_kind, RuneKind::Derived) && matches!(arg_expr, Expression::Identifier(_));
        let access_root = if use_direct_access {
            arg_expr
        } else {
            let derived_arg = if matches!(rune_kind, RuneKind::DerivedBy) {
                arg_expr
            } else {
                self.b.thunk(arg_expr)
            };
            call.arguments
                .push(oxc_ast::ast::Argument::from(derived_arg));
            let tmp_name = self.gen_unique_name("$$d");
            let tmp_name_str = self.b.alloc_str(&tmp_name);
            let derived_call = Expression::CallExpression(call);
            let tmp_declarator = self.b.ast.variable_declarator(
                oxc_span::SPAN,
                decl_kind,
                self.b.ast.binding_pattern_binding_identifier(
                    oxc_span::SPAN,
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
            oxc_span::SPAN,
            decl_kind,
            self.b.ast.vec_from_iter(declarators),
            false,
        );
        Statement::VariableDeclaration(self.b.alloc(decl))
    }

    pub(crate) fn expand_state_destructuring(
        &mut self,
        stmts: &mut oxc_allocator::Vec<'a, Statement<'a>>,
    ) {
        self.rewrite_destructured_rune_decls(
            stmts,
            |declarator, rune_kind| {
                !matches!(
                    declarator.id,
                    oxc_ast::ast::BindingPattern::BindingIdentifier(_)
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
                            .expression_object(oxc_span::SPAN, this.b.ast.vec())
                    } else {
                        let mut dummy = oxc_ast::ast::Argument::from(this.b.cheap_expr());
                        std::mem::swap(&mut call.arguments[0], &mut dummy);
                        dummy.into_expression()
                    }
                } else {
                    unreachable!()
                };

                this.gen_state_destructuring(&declarator.id, value, rune_kind, decl_kind)
            },
        );
    }

    pub(crate) fn detect_class_field_rune_kind(expr: &Expression<'_>) -> Option<RuneKind> {
        if let Expression::CallExpression(call) = expr {
            match &call.callee {
                Expression::Identifier(id) => match id.name.as_str() {
                    "$state" => return Some(RuneKind::State),
                    "$derived" => return Some(RuneKind::Derived),
                    _ => {}
                },
                Expression::StaticMemberExpression(member) => {
                    if let Expression::Identifier(obj) = &member.object {
                        match (obj.name.as_str(), member.property.name.as_str()) {
                            ("$state", "raw") => return Some(RuneKind::StateRaw),
                            ("$derived", "by") => return Some(RuneKind::DerivedBy),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn gen_state_destructuring(
        &mut self,
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        value: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: oxc_ast::ast::VariableDeclarationKind,
    ) -> Statement<'a> {
        let tmp_name = self.gen_unique_name("tmp");
        let tmp_name_str: &str = self.b.alloc_str(&tmp_name);

        let mut declarators = Vec::new();

        let tmp_declarator = self.b.ast.variable_declarator(
            oxc_span::SPAN,
            decl_kind,
            self.b
                .ast
                .binding_pattern_binding_identifier(oxc_span::SPAN, self.b.ast.atom(tmp_name_str)),
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
            oxc_span::SPAN,
            decl_kind,
            self.b.ast.vec_from_iter(declarators),
            false,
        );
        Statement::VariableDeclaration(self.b.alloc(decl))
    }

    fn gen_destructure_declarators(
        &mut self,
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        accessor: Expression<'a>,
        rune_kind: RuneKind,
        decl_kind: oxc_ast::ast::VariableDeclarationKind,
        dev_label: Option<&'static str>,
        declarators: &mut Vec<oxc_ast::ast::VariableDeclarator<'a>>,
    ) {
        match pattern {
            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                let name = id.name.as_str();
                let sym_id = id.symbol_id.get();
                let is_mutated = sym_id.is_some_and(|s| self.component_scoping.is_mutated(s));

                let is_proxy = matches!(rune_kind, RuneKind::State)
                    && crate::rune_refs::should_proxy(&accessor);

                let final_value = self.wrap_state_value(accessor, rune_kind, is_mutated);

                let final_value = if self.dev {
                    if is_mutated {
                        self.b.call_expr(
                            "$.tag",
                            [Arg::Expr(final_value), Arg::Str(name.to_string())],
                        )
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
                    oxc_span::SPAN,
                    decl_kind,
                    self.b
                        .ast
                        .binding_pattern_binding_identifier(oxc_span::SPAN, self.b.ast.atom(name)),
                    NONE,
                    Some(final_value),
                    false,
                );
                declarators.push(declarator);
            }
            oxc_ast::ast::BindingPattern::ObjectPattern(obj) => {
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
                    self.gen_destructure_declarators(
                        &prop.value,
                        member,
                        rune_kind,
                        decl_kind,
                        dev_label,
                        declarators,
                    );
                }

                if let Some(rest) = &obj.rest {
                    let keys_array = self
                        .b
                        .array_expr(key_names.iter().map(|k| self.b.str_expr(k)));
                    let exclude_expr = self.b.call_expr(
                        "$.exclude_from_object",
                        [Arg::Expr(accessor), Arg::Expr(keys_array)],
                    );
                    self.gen_destructure_declarators(
                        &rest.argument,
                        exclude_expr,
                        rune_kind,
                        decl_kind,
                        dev_label,
                        declarators,
                    );
                }
            }
            oxc_ast::ast::BindingPattern::ArrayPattern(arr) => {
                let array_name = self.gen_unique_name("$$array");
                let array_name_str: &str = self.b.alloc_str(&array_name);

                let len_arg = if arr.rest.is_some() {
                    vec![Arg::Expr(accessor)]
                } else {
                    vec![Arg::Expr(accessor), Arg::Num(arr.elements.len() as f64)]
                };

                let to_array_call = self.b.call_expr("$.to_array", len_arg);
                let thunk = self
                    .b
                    .arrow_expr(self.b.no_params(), [self.b.expr_stmt(to_array_call)]);
                let derived_call = self.b.call_expr("$.derived", [Arg::Expr(thunk)]);
                let derived_call = match dev_label.filter(|_| self.dev) {
                    Some(label) => self.b.call_expr(
                        "$.tag",
                        [Arg::Expr(derived_call), Arg::Str(label.to_string())],
                    ),
                    None => derived_call,
                };

                let array_declarator = self.b.ast.variable_declarator(
                    oxc_span::SPAN,
                    decl_kind,
                    self.b.ast.binding_pattern_binding_identifier(
                        oxc_span::SPAN,
                        self.b.ast.atom(array_name_str),
                    ),
                    NONE,
                    Some(derived_call),
                    false,
                );
                declarators.push(array_declarator);

                for (idx, elem) in arr.elements.iter().enumerate() {
                    let Some(elem) = elem else { continue };

                    let get_array = self.b.call_expr("$.get", [Arg::Ident(array_name_str)]);
                    let elem_access = self
                        .b
                        .computed_member_expr(get_array, self.b.num_expr(idx as f64));
                    self.gen_destructure_declarators(
                        elem,
                        elem_access,
                        rune_kind,
                        decl_kind,
                        dev_label,
                        declarators,
                    );
                }

                if let Some(rest) = &arr.rest {
                    let get_array = self.b.call_expr("$.get", [Arg::Ident(array_name_str)]);
                    let slice = self.b.static_member_expr(get_array, "slice");
                    let slice_call = self.b.ast.expression_call(
                        oxc_span::SPAN,
                        slice,
                        NONE,
                        self.b.ast.vec_from_array([oxc_ast::ast::Argument::from(
                            self.b.num_expr(arr.elements.len() as f64),
                        )]),
                        false,
                    );
                    self.gen_destructure_declarators(
                        &rest.argument,
                        slice_call,
                        rune_kind,
                        decl_kind,
                        dev_label,
                        declarators,
                    );
                }
            }
            oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
                let default_expr = assign.right.clone_in(self.b.ast.allocator);
                let fallback = self
                    .b
                    .call_expr("$.fallback", [Arg::Expr(accessor), Arg::Expr(default_expr)]);
                self.gen_destructure_declarators(
                    &assign.left,
                    fallback,
                    rune_kind,
                    decl_kind,
                    dev_label,
                    declarators,
                );
            }
        }
    }

    pub(crate) fn wrap_state_value(
        &self,
        value: Expression<'a>,
        rune_kind: RuneKind,
        is_mutated: bool,
    ) -> Expression<'a> {
        match rune_kind {
            RuneKind::State => {
                let proxied = if crate::rune_refs::should_proxy(&value) {
                    self.b.call_expr("$.proxy", [Arg::Expr(value)])
                } else {
                    value
                };
                if is_mutated {
                    self.b.call_expr("$.state", [Arg::Expr(proxied)])
                } else {
                    proxied
                }
            }
            RuneKind::StateRaw => {
                if is_mutated {
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
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        init: Expression<'a>,
        decl_span_start: u32,
        decl_kind: oxc_ast::ast::VariableDeclarationKind,
    ) -> Statement<'a> {
        let Expression::CallExpression(mut call) = init else {
            unreachable!("async derived destructuring should be a call");
        };

        let init_span_start = call.span.start;
        let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
        std::mem::swap(&mut call.arguments[0], &mut dummy);
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
                oxc_ast::ast::BindingPattern::ArrayPattern(_) => "iterable",
                _ => "object",
            };
            let label = format!("[$derived {kind}]");
            args.push(Arg::Expr(self.b.str_expr(&label)));

            if !self
                .ignore_query
                .is_ignored_at_span(decl_span_start, "await_waterfall")
            {
                let full_offset = self.script_content_start + init_span_start;
                let (line, col) = self.component_line_index.line_col(full_offset);
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
                    .call_expr_callee(self.b.await_expr(saved), std::iter::empty::<Arg<'a, '_>>())
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
                oxc_span::SPAN,
                decl_kind,
                self.b.ast.binding_pattern_binding_identifier(
                    oxc_span::SPAN,
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
                oxc_span::SPAN,
                decl_kind,
                self.b.ast.vec_from_iter(declarators),
                false,
            );
            Statement::VariableDeclaration(self.b.alloc(decl))
        }
    }

    fn gen_derived_destructure_assignments(
        &mut self,
        pattern: &oxc_ast::ast::BindingPattern<'a>,
        accessor: Expression<'a>,
        stmts: &mut Vec<Statement<'a>>,
    ) {
        match pattern {
            oxc_ast::ast::BindingPattern::BindingIdentifier(id) => {
                let value = self.wrap_state_value(accessor, RuneKind::Derived, false);
                let value = if self.dev {
                    self.b
                        .call_expr("$.tag", [Arg::Expr(value), Arg::Str(id.name.to_string())])
                } else {
                    value
                };
                stmts.push(self.b.assign_stmt(
                    svelte_ast_builder::AssignLeft::Ident(id.name.to_string()),
                    value,
                ));
            }
            oxc_ast::ast::BindingPattern::ObjectPattern(obj) => {
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
            oxc_ast::ast::BindingPattern::ArrayPattern(arr) => {
                let array_name = self.gen_unique_name("$$array");
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
                        oxc_span::SPAN,
                        slice,
                        NONE,
                        self.b.ast.vec_from_array([oxc_ast::ast::Argument::from(
                            self.b.num_expr(arr.elements.len() as f64),
                        )]),
                        false,
                    );
                    self.gen_derived_destructure_assignments(&rest.argument, slice_call, stmts);
                }
            }
            oxc_ast::ast::BindingPattern::AssignmentPattern(assign) => {
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

    pub(crate) fn property_key_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<String> {
        svelte_analyze::property_key_static_name(key).map(str::to_string)
    }

    pub(crate) fn build_object_member_access(
        &self,
        object: Expression<'a>,
        key: &oxc_ast::ast::PropertyKey<'a>,
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
                oxc_ast::ast::PropertyKey::StaticIdentifier(id) => self
                    .b
                    .static_member_expr(object, self.b.alloc_str(id.name.as_str())),
                oxc_ast::ast::PropertyKey::StringLiteral(s) => self
                    .b
                    .static_member_expr(object, self.b.alloc_str(s.value.as_str())),
                _ => object,
            }
        }
    }

    fn property_key_to_expr<'c>(
        b: &'c svelte_ast_builder::Builder<'a>,
        key: &oxc_ast::ast::PropertyKey<'a>,
    ) -> Option<Expression<'a>> {
        match key {
            oxc_ast::ast::PropertyKey::StringLiteral(s) => Some(b.str_expr(s.value.as_str())),
            oxc_ast::ast::PropertyKey::NumericLiteral(n) => Some(b.num_expr(n.value)),
            _ => None,
        }
    }

    pub(crate) fn scan_class_state_fields(
        &self,
        body: &oxc_ast::ast::ClassBody<'a>,
    ) -> ClassStateInfo {
        let mut fields = Vec::new();

        let mut existing_private: FxHashSet<String> = FxHashSet::default();
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::PropertyDefinition(prop) = element
                && let oxc_ast::ast::PropertyKey::PrivateIdentifier(id) = &prop.key
            {
                existing_private.insert(id.name.to_string());
            }
        }

        let mut body_public_names: FxHashSet<String> = FxHashSet::default();
        let mut placeholder_public_names: FxHashSet<String> = FxHashSet::default();
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::PropertyDefinition(prop) = element {
                if let oxc_ast::ast::PropertyKey::StaticIdentifier(id) = &prop.key
                    && !prop.computed
                    && prop.value.is_none()
                {
                    placeholder_public_names.insert(id.name.to_string());
                }
                let Some(value) = &prop.value else { continue };
                let Some(rune_kind) = self.rune_kind_from_expr(value) else {
                    continue;
                };

                match &prop.key {
                    oxc_ast::ast::PropertyKey::PrivateIdentifier(id) => {
                        fields.push(ClassStateField {
                            public_name: None,
                            private_name: id.name.to_string(),
                            rune_kind,
                        });
                    }
                    oxc_ast::ast::PropertyKey::StaticIdentifier(id) if !prop.computed => {
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
        for element in &body.body {
            if let oxc_ast::ast::ClassElement::MethodDefinition(method) = element
                && method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor
                && let Some(func_body) = &method.value.body
            {
                for stmt in &func_body.statements {
                    if let Statement::ExpressionStatement(es) = stmt
                        && let Expression::AssignmentExpression(assign) = &es.expression
                        && assign.operator == oxc_ast::ast::AssignmentOperator::Assign
                        && let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member) =
                            &assign.left
                        && let Expression::ThisExpression(_) = &member.object
                        && let Some(rune_kind) = self.rune_kind_from_expr(&assign.right)
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
                        existing_private.insert(backing.trim_start_matches('#').to_string());
                        if placeholder_public_names.contains(&name) {
                            ctor_placeholder_names.insert(name.clone());
                        }
                        fields.push(ClassStateField {
                            public_name: Some(name),
                            private_name: backing.trim_start_matches('#').to_string(),
                            rune_kind,
                        });
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
        body: &mut oxc_ast::ast::ClassBody<'a>,
        info: &ClassStateInfo,
    ) {
        use oxc_ast::ast::ClassElement;

        let public_fields: std::collections::HashMap<&str, &ClassStateField> = info
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
            std::mem::swap(&mut body.body, &mut temp);
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
                    let is_rune_prop = prop
                        .value
                        .as_ref()
                        .is_some_and(|v| self.rune_kind_from_expr(v).is_some());
                    if !is_rune_prop {
                        let is_ctor_placeholder = prop.value.is_none()
                            && match &prop.key {
                                oxc_ast::ast::PropertyKey::StaticIdentifier(id)
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
                        oxc_ast::ast::PropertyKey::PrivateIdentifier(id) => {
                            let name = id.name.to_string();
                            if private_fields.contains(name.as_str()) {
                                self.rewrite_private_field_callee(&mut prop);
                            }
                            new_body.push(ClassElement::PropertyDefinition(prop));
                        }
                        oxc_ast::ast::PropertyKey::StaticIdentifier(id) if !prop.computed => {
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
                    if method.kind == oxc_ast::ast::MethodDefinitionKind::Constructor {
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

    fn rewrite_private_field_callee(&self, prop: &mut oxc_ast::ast::PropertyDefinition<'a>) {
        let rune_kind = prop
            .value
            .as_ref()
            .and_then(|v| self.rune_kind_from_expr(v));
        if let Some(Expression::CallExpression(call)) = &mut prop.value {
            match rune_kind {
                Some(RuneKind::State | RuneKind::StateRaw) => {
                    call.callee = self.b.rid_expr("$.state");
                }
                Some(RuneKind::Derived) => {
                    call.callee = self.b.rid_expr("$.derived");
                    if !call.arguments.is_empty() {
                        let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                        std::mem::swap(&mut call.arguments[0], &mut dummy);
                        let thunked = self.b.thunk(dummy.into_expression());
                        call.arguments[0] = oxc_ast::ast::Argument::from(thunked);
                    }
                }
                Some(RuneKind::DerivedBy) => {
                    call.callee = self.b.rid_expr("$.derived");
                }
                _ => {}
            }
            if self.dev && rune_kind.is_some() {
                let field_name = match &prop.key {
                    oxc_ast::ast::PropertyKey::PrivateIdentifier(id) => format!("#{}", id.name),
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
        new_body: &mut Vec<oxc_ast::ast::ClassElement<'a>>,
        prop: &mut oxc_ast::ast::PropertyDefinition<'a>,
        field_info: &ClassStateField,
        name: &str,
    ) {
        let arg = if let Some(Expression::CallExpression(mut call)) = prop.value.take() {
            if call.arguments.is_empty() {
                None
            } else {
                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                std::mem::swap(&mut call.arguments[0], &mut dummy);
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
                        .call_expr("$.derived", std::iter::empty::<Arg<'a, '_>>())
                }
            }
            _ => {
                if let Some(arg) = arg {
                    self.b.call_expr("$.state", [Arg::Expr(arg)])
                } else {
                    self.b
                        .call_expr("$.state", std::iter::empty::<Arg<'a, '_>>())
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
        new_body: &mut Vec<oxc_ast::ast::ClassElement<'a>>,
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
        method: &mut oxc_allocator::Box<'a, oxc_ast::ast::MethodDefinition<'a>>,
        info: &ClassStateInfo,
    ) {
        let Some(func_body) = &mut method.value.body else {
            return;
        };

        let ctor_fields: std::collections::HashMap<&str, &ClassStateField> = info
            .fields
            .iter()
            .filter_map(|f| f.public_name.as_deref().map(|n| (n, f)))
            .collect();

        for stmt in func_body.statements.iter_mut() {
            if let Statement::ExpressionStatement(es) = stmt
                && let Expression::AssignmentExpression(assign) = &mut es.expression
                && assign.operator == oxc_ast::ast::AssignmentOperator::Assign
                && let oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member) = &assign.left
                && let Expression::ThisExpression(_) = &member.object
            {
                let name = member.property.name.to_string();
                if let Some(field_info) = ctor_fields.get(name.as_str())
                    && let Expression::CallExpression(call) = &mut assign.right
                {
                    match field_info.rune_kind {
                        RuneKind::Derived => {
                            call.callee = self.b.rid_expr("$.derived");
                            if !call.arguments.is_empty() {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut call.arguments[0], &mut dummy);
                                let thunked = self.b.thunk(dummy.into_expression());
                                call.arguments[0] = oxc_ast::ast::Argument::from(thunked);
                            }
                        }
                        RuneKind::DerivedBy => {
                            call.callee = self.b.rid_expr("$.derived");
                        }
                        _ => {
                            call.callee = self.b.rid_expr("$.state");
                            let needs_proxy = call
                                .arguments
                                .first()
                                .and_then(|a| a.as_expression())
                                .is_some_and(|e| crate::rune_refs::should_proxy(e));
                            if needs_proxy {
                                let mut dummy = oxc_ast::ast::Argument::from(self.b.cheap_expr());
                                std::mem::swap(&mut call.arguments[0], &mut dummy);
                                let inner = dummy.into_expression();
                                let proxied = self.b.call_expr("$.proxy", [Arg::Expr(inner)]);
                                call.arguments[0] = oxc_ast::ast::Argument::from(proxied);
                            }
                        }
                    }
                    if self.dev {
                        let label = self.class_tag_label(&name);
                        let rhs = self.b.move_expr(&mut assign.right);
                        assign.right = self.b.call_expr("$.tag", [Arg::Expr(rhs), Arg::Str(label)]);
                    }

                    let new_left = self.b.this_private_member(&field_info.private_name);
                    if let Expression::PrivateFieldExpression(pfe) = new_left {
                        assign.left = oxc_ast::ast::AssignmentTarget::PrivateFieldExpression(pfe);
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

fn script_rune_call_node_id(expr: &Expression<'_>, node_id_offset: u32) -> Option<OxcNodeId> {
    match expr {
        Expression::CallExpression(call) => Some(OxcNodeId::from_usize(
            call.node_id().index() + node_id_offset as usize,
        )),
        Expression::TSAsExpression(expr) => {
            script_rune_call_node_id(&expr.expression, node_id_offset)
        }
        Expression::TSSatisfiesExpression(expr) => {
            script_rune_call_node_id(&expr.expression, node_id_offset)
        }
        Expression::TSNonNullExpression(expr) => {
            script_rune_call_node_id(&expr.expression, node_id_offset)
        }
        Expression::TSTypeAssertion(expr) => {
            script_rune_call_node_id(&expr.expression, node_id_offset)
        }
        Expression::TSInstantiationExpression(expr) => {
            script_rune_call_node_id(&expr.expression, node_id_offset)
        }
        _ => None,
    }
}
