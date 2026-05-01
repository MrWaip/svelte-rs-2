use oxc_ast::ast::{
    AssignmentPattern, BindingPattern, ChainElement, Expression, FormalParameter, FormalParameters,
    PropertyKey, Statement,
};
use oxc_span::SPAN;
use svelte_analyze::{SnippetBlockSemantics, SnippetParam};
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::data_structures::{FragmentAnchor, FragmentCtx};
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_snippet_block(
        &mut self,
        state: &mut EmitState<'a>,
        _ctx: &FragmentCtx<'a>,
        id: NodeId,
        sem: SnippetBlockSemantics,
    ) -> Result<()> {
        if state.skip_snippets {
            return Ok(());
        }
        let is_hoistable = sem.hoistable;
        let stmt = self.build_snippet_const(id, &sem)?;
        if let Some(local) = state.local_snippet_decls.as_mut() {
            local.push(stmt);
        } else if is_hoistable {
            self.hoistable_snippets.push(stmt);
        } else {
            self.instance_snippets.push(stmt);
        }
        Ok(())
    }

    pub(in super::super) fn build_snippet_const(
        &mut self,
        block_id: NodeId,
        sem: &SnippetBlockSemantics,
    ) -> Result<Statement<'a>> {
        self.build_snippet_const_with_prefix(block_id, sem, Vec::new())
    }

    pub(in super::super) fn build_snippet_const_with_prefix(
        &mut self,
        block_id: NodeId,
        sem: &SnippetBlockSemantics,
        prepend_stmts: Vec<Statement<'a>>,
    ) -> Result<Statement<'a>> {
        self.enter_snippet_build();
        let res = self.build_snippet_const_inner(block_id, sem, prepend_stmts);
        self.exit_snippet_build();
        res
    }

    fn build_snippet_const_inner(
        &mut self,
        block_id: NodeId,
        sem: &SnippetBlockSemantics,
        prepend_stmts: Vec<Statement<'a>>,
    ) -> Result<Statement<'a>> {
        let name = self.ctx.query.view.symbol_name(sem.name).to_string();

        let block = match self.ctx.query.component.store.get(block_id) {
            svelte_ast::Node::SnippetBlock(b) => b,
            _ => return CodegenError::unexpected_node(block_id, "SnippetBlock"),
        };
        let Some(parsed_stmt) = self.ctx.state.parsed.take_stmt(block.decl.id()) else {
            return CodegenError::missing_expression(block_id);
        };

        let parsed_patterns: Vec<Option<BindingPattern<'a>>> =
            extract_pattern_params(&parsed_stmt, sem.params.len(), self.ctx.b.ast.allocator);

        let (params, binding_decls) =
            self.build_snippet_params_with_patterns(block_id, sem, &parsed_patterns)?;

        let body = match self.ctx.query.component.store.get(block_id) {
            svelte_ast::Node::SnippetBlock(block) => block.body,
            _ => return CodegenError::unexpected_node(block_id, "SnippetBlock"),
        };
        let mut inner_ctx = FragmentCtx::root(self.ctx, body);
        inner_ctx.anchor = FragmentAnchor::CallbackParam {
            name: "$$anchor".to_string(),
            append_inside: false,
        };
        let mut inner_state = EmitState::new();
        self.emit_fragment(&mut inner_state, &inner_ctx, body)?;
        let body_stmts = self.pack_callback_body(inner_state, "$$anchor")?;

        let mut all: Vec<Statement<'a>> = Vec::new();
        if self.ctx.state.dev {
            let args_id = self.ctx.b.rid_expr("arguments");
            all.push(
                self.ctx
                    .b
                    .call_stmt("$.validate_snippet_args", [Arg::Spread(args_id)]),
            );
        }
        all.extend(binding_decls);
        all.extend(prepend_stmts);
        all.extend(body_stmts);

        let snippet_expr = if self.ctx.state.dev {
            let fn_expr = self.ctx.b.function_expr(params, all);
            let component = self.ctx.b.rid_expr(self.ctx.state.name);
            self.ctx
                .b
                .call_expr("$.wrap_snippet", [Arg::Expr(component), Arg::Expr(fn_expr)])
        } else {
            let arrow = self.ctx.b.arrow(params, all);
            Expression::ArrowFunctionExpression(self.ctx.b.alloc(arrow))
        };
        Ok(self.ctx.b.const_stmt(&name, snippet_expr))
    }

    fn build_snippet_params_with_patterns(
        &mut self,
        _block_id: NodeId,
        sem: &SnippetBlockSemantics,
        parsed_patterns: &[Option<BindingPattern<'a>>],
    ) -> Result<(FormalParameters<'a>, Vec<Statement<'a>>)> {
        use oxc_ast::ast::FormalParameterKind;

        let mut params: Vec<FormalParameter<'a>> = Vec::new();
        params.push(self.formal_param_ident("$$anchor", false));

        let mut binding_decls: Vec<Statement<'a>> = Vec::new();

        for (idx, param) in sem.params.iter().enumerate() {
            match param {
                SnippetParam::Identifier { sym } => {
                    let name = self.ctx.query.view.symbol_name(*sym).to_string();
                    params.push(self.formal_param_ident(&name, true));
                }
                SnippetParam::Pattern { .. } => {
                    let arg_name = format!("$$arg{idx}");
                    params.push(self.formal_param_ident(&arg_name, false));
                    if let Some(Some(pattern)) = parsed_patterns.get(idx) {
                        self.emit_snippet_destructure(pattern, &arg_name, &mut binding_decls);
                    }
                }
            }
        }

        let params = self.ctx.b.ast.formal_parameters(
            SPAN,
            FormalParameterKind::ArrowFormalParameters,
            self.ctx.b.ast.vec_from_iter(params),
            oxc_ast::NONE,
        );
        Ok((params, binding_decls))
    }

    fn emit_snippet_destructure(
        &mut self,
        pattern: &BindingPattern<'a>,
        arg_name: &str,
        decls: &mut Vec<Statement<'a>>,
    ) {
        use rustc_hash::FxHashSet;
        let b = &self.ctx.state.b;
        let access = b.maybe_call_expr(b.rid_expr(arg_name), std::iter::empty::<Arg<'_, '_>>());
        let mut inserts: Vec<SnippetInsert<'a>> = Vec::new();
        let mut paths: Vec<SnippetPath<'a>> = Vec::new();
        self.collect_binding_pattern(pattern, access, false, &mut inserts, &mut paths);

        let array_insert_names: FxHashSet<String> =
            inserts.iter().map(|ins| ins.name.clone()).collect();

        for insert in inserts {
            let thunk = self.ctx.b.thunk(insert.value);
            let derived = self.ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            decls.push(self.ctx.b.var_stmt(&insert.name, derived));
        }
        let dev = self.ctx.state.dev;
        for mut path in paths {
            rewrite_array_reads(self, &mut path.expression, &array_insert_names);
            let thunk = self.ctx.b.thunk(path.expression);
            let init = if path.has_default {
                self.ctx
                    .b
                    .call_expr("$.derived_safe_equal", [Arg::Expr(thunk)])
            } else {
                thunk
            };
            decls.push(self.ctx.b.let_init_stmt(&path.name, init));
            if dev {
                let name_alloc = self.ctx.b.alloc_str(&path.name);
                let eager = if path.has_default {
                    self.ctx.b.call_stmt("$.get", [Arg::Ident(name_alloc)])
                } else {
                    self.ctx
                        .b
                        .call_stmt(&path.name, std::iter::empty::<Arg<'_, '_>>())
                };
                decls.push(eager);
            }
        }
    }

    fn collect_binding_pattern(
        &mut self,
        pattern: &BindingPattern<'a>,
        expression: Expression<'a>,
        has_default: bool,
        inserts: &mut Vec<SnippetInsert<'a>>,
        paths: &mut Vec<SnippetPath<'a>>,
    ) {
        match pattern {
            BindingPattern::BindingIdentifier(id) => {
                paths.push(SnippetPath {
                    name: id.name.as_str().to_string(),
                    expression,
                    has_default,
                });
            }
            BindingPattern::ArrayPattern(arr) => {
                let array_name = self.ctx.state.gen_ident("$$array");
                let to_array = if arr.rest.is_some() {
                    self.ctx.b.call_expr("$.to_array", [Arg::Expr(expression)])
                } else {
                    self.ctx.b.call_expr(
                        "$.to_array",
                        [Arg::Expr(expression), Arg::Num(arr.elements.len() as f64)],
                    )
                };
                inserts.push(SnippetInsert {
                    name: array_name.clone(),
                    value: to_array,
                });

                for (index, element) in arr.elements.iter().enumerate() {
                    let Some(inner) = element else { continue };
                    let access = self.ctx.b.computed_member_expr(
                        self.ctx.b.rid_expr(&array_name),
                        self.ctx.b.num_expr(index as f64),
                    );
                    self.collect_binding_pattern(inner, access, has_default, inserts, paths);
                }

                if let Some(rest) = &arr.rest {
                    let slice_callee = self
                        .ctx
                        .b
                        .static_member_expr(self.ctx.b.rid_expr(&array_name), "slice");
                    let slice_call = self
                        .ctx
                        .b
                        .call_expr_callee(slice_callee, [Arg::Num(arr.elements.len() as f64)]);
                    self.collect_binding_pattern(
                        &rest.argument,
                        slice_call,
                        has_default,
                        inserts,
                        paths,
                    );
                }
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    let access = build_object_property_access(self, &expression, prop);
                    self.collect_binding_pattern(&prop.value, access, has_default, inserts, paths);
                }
                if let Some(rest) = &obj.rest {
                    let rest_access = build_object_rest_expr(self, &expression, obj);
                    self.collect_binding_pattern(
                        &rest.argument,
                        rest_access,
                        has_default,
                        inserts,
                        paths,
                    );
                }
            }
            BindingPattern::AssignmentPattern(assign) => {
                let fallback = build_fallback_expr(self, &expression, assign);
                self.collect_binding_pattern(&assign.left, fallback, true, inserts, paths);
            }
        }
    }

    fn formal_param_ident(&self, name: &str, with_noop_default: bool) -> FormalParameter<'a> {
        let inner = self
            .ctx
            .b
            .ast
            .binding_pattern_binding_identifier(SPAN, self.ctx.b.ast.atom(name));
        let pattern = if with_noop_default {
            let default_expr = self
                .ctx
                .b
                .static_member_expr(self.ctx.b.rid_expr("$"), "noop");
            self.ctx
                .b
                .ast
                .binding_pattern_assignment_pattern(SPAN, inner, default_expr)
        } else {
            inner
        };
        self.ctx.b.ast.formal_parameter(
            SPAN,
            self.ctx.b.ast.vec(),
            pattern,
            oxc_ast::NONE,
            oxc_ast::NONE,
            false,
            None,
            false,
            false,
        )
    }
}

struct SnippetInsert<'a> {
    name: String,
    value: Expression<'a>,
}

struct SnippetPath<'a> {
    name: String,
    expression: Expression<'a>,
    has_default: bool,
}

fn extract_pattern_params<'a>(
    stmt: &Statement<'a>,
    expected: usize,
    allocator: &'a oxc_allocator::Allocator,
) -> Vec<Option<BindingPattern<'a>>> {
    use oxc_allocator::CloneIn;
    let mut out: Vec<Option<BindingPattern<'a>>> = Vec::with_capacity(expected);
    let Statement::VariableDeclaration(decl) = stmt else {
        for _ in 0..expected {
            out.push(None);
        }
        return out;
    };
    let Some(declarator) = decl.declarations.first() else {
        for _ in 0..expected {
            out.push(None);
        }
        return out;
    };
    let Some(Expression::ArrowFunctionExpression(arrow)) = &declarator.init else {
        for _ in 0..expected {
            out.push(None);
        }
        return out;
    };
    let items = arrow.params.items.as_slice();
    for param in items.iter().take(expected) {
        let bp = &param.pattern;
        if matches!(bp, BindingPattern::BindingIdentifier(_)) {
            out.push(None);
        } else {
            out.push(Some(bp.clone_in(allocator)));
        }
    }
    while out.len() < expected {
        out.push(None);
    }
    out
}

fn build_object_property_access<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    access: &Expression<'a>,
    prop: &oxc_ast::ast::BindingProperty<'a>,
) -> Expression<'a> {
    if !prop.computed
        && let PropertyKey::StaticIdentifier(id) = &prop.key
    {
        return build_chain_static_member(cg, access, id.name.as_str());
    }
    let key_expr = clone_property_key_expr(cg, &prop.key);
    build_chain_computed_member(cg, access, key_expr)
}

fn build_chain_static_member<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    object: &Expression<'a>,
    prop: &str,
) -> Expression<'a> {
    use oxc_span::SPAN;
    if let Expression::ChainExpression(chain) = object {
        let property = cg.ctx.b.ast.identifier_name(SPAN, cg.ctx.b.ast.atom(prop));
        let member = cg.ctx.b.ast.alloc_static_member_expression(
            SPAN,
            clone_chain_element_expr(cg, &chain.expression),
            property,
            false,
        );
        return Expression::ChainExpression(
            cg.ctx.b.alloc(
                cg.ctx
                    .b
                    .ast
                    .chain_expression(SPAN, ChainElement::StaticMemberExpression(member)),
            ),
        );
    }
    cg.ctx
        .b
        .static_member_expr(cg.ctx.b.clone_expr(object), prop)
}

fn build_chain_computed_member<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    object: &Expression<'a>,
    property: Expression<'a>,
) -> Expression<'a> {
    use oxc_span::SPAN;
    if let Expression::ChainExpression(chain) = object {
        let member = cg.ctx.b.ast.alloc_computed_member_expression(
            SPAN,
            clone_chain_element_expr(cg, &chain.expression),
            property,
            false,
        );
        return Expression::ChainExpression(
            cg.ctx.b.alloc(
                cg.ctx
                    .b
                    .ast
                    .chain_expression(SPAN, ChainElement::ComputedMemberExpression(member)),
            ),
        );
    }
    cg.ctx
        .b
        .computed_member_expr(cg.ctx.b.clone_expr(object), property)
}

fn rewrite_array_reads<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    expr: &mut Expression<'a>,
    array_insert_names: &rustc_hash::FxHashSet<String>,
) {
    match expr {
        Expression::StaticMemberExpression(member) => {
            rewrite_array_reads(cg, &mut member.object, array_insert_names);
        }
        Expression::ComputedMemberExpression(member) => {
            rewrite_array_reads(cg, &mut member.object, array_insert_names);
            rewrite_array_reads(cg, &mut member.expression, array_insert_names);
        }
        Expression::ChainExpression(chain) => match &mut chain.expression {
            ChainElement::StaticMemberExpression(member) => {
                rewrite_array_reads(cg, &mut member.object, array_insert_names);
            }
            ChainElement::ComputedMemberExpression(member) => {
                rewrite_array_reads(cg, &mut member.object, array_insert_names);
                rewrite_array_reads(cg, &mut member.expression, array_insert_names);
            }
            ChainElement::CallExpression(call) => {
                rewrite_array_reads(cg, &mut call.callee, array_insert_names);
                for arg in call.arguments.iter_mut() {
                    if let Some(arg_expr) = arg.as_expression_mut() {
                        rewrite_array_reads(cg, arg_expr, array_insert_names);
                    }
                }
            }
            _ => {}
        },
        Expression::CallExpression(call) => {
            rewrite_array_reads(cg, &mut call.callee, array_insert_names);
            for arg in call.arguments.iter_mut() {
                if let Some(arg_expr) = arg.as_expression_mut() {
                    rewrite_array_reads(cg, arg_expr, array_insert_names);
                }
            }
        }
        Expression::ArrayExpression(arr) => {
            for element in arr.elements.iter_mut() {
                if let Some(el_expr) = element.as_expression_mut() {
                    rewrite_array_reads(cg, el_expr, array_insert_names);
                }
            }
        }
        _ => {}
    }

    let Expression::Identifier(ident) = expr else {
        return;
    };
    if !array_insert_names.contains(ident.name.as_str()) {
        return;
    }
    let name_alloc = cg.ctx.b.alloc_str(ident.name.as_str());
    *expr = cg.ctx.b.call_expr("$.get", [Arg::Ident(name_alloc)]);
}

fn clone_chain_element_expr<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    element: &ChainElement<'a>,
) -> Expression<'a> {
    use oxc_allocator::CloneIn;
    match element {
        ChainElement::CallExpression(call) => {
            Expression::CallExpression(call.clone_in(cg.ctx.b.ast.allocator))
        }
        ChainElement::StaticMemberExpression(member) => {
            Expression::StaticMemberExpression(member.clone_in(cg.ctx.b.ast.allocator))
        }
        ChainElement::ComputedMemberExpression(member) => {
            Expression::ComputedMemberExpression(member.clone_in(cg.ctx.b.ast.allocator))
        }
        ChainElement::PrivateFieldExpression(member) => {
            Expression::PrivateFieldExpression(member.clone_in(cg.ctx.b.ast.allocator))
        }
        ChainElement::TSNonNullExpression(expr) => {
            Expression::TSNonNullExpression(expr.clone_in(cg.ctx.b.ast.allocator))
        }
    }
}

fn build_object_rest_expr<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    access: &Expression<'a>,
    obj: &oxc_ast::ast::ObjectPattern<'a>,
) -> Expression<'a> {
    use oxc_allocator::CloneIn;
    let excluded = cg.ctx.b.array_expr(
        obj.properties
            .iter()
            .filter_map(|prop| {
                if !prop.computed {
                    return match &prop.key {
                        PropertyKey::StaticIdentifier(id) => {
                            Some(cg.ctx.b.str_expr(id.name.as_str()))
                        }
                        PropertyKey::StringLiteral(s) => Some(cg.ctx.b.str_expr(s.value.as_str())),
                        PropertyKey::NumericLiteral(n) => Some(cg.ctx.b.num_expr(n.value)),
                        _ => None,
                    };
                }
                let expr_ref: &Expression<'a> = prop.key.as_expression()?;
                let key_expr = expr_ref.clone_in(cg.ctx.b.ast.allocator);
                Some(cg.ctx.b.call_expr("String", [Arg::Expr(key_expr)]))
            })
            .collect::<Vec<_>>(),
    );
    cg.ctx.b.call_expr(
        "$.exclude_from_object",
        [Arg::Expr(cg.ctx.b.clone_expr(access)), Arg::Expr(excluded)],
    )
}

fn build_fallback_expr<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    access: &Expression<'a>,
    assign: &AssignmentPattern<'a>,
) -> Expression<'a> {
    use oxc_allocator::CloneIn;
    let default_val = assign.right.clone_in(cg.ctx.b.ast.allocator);
    let is_computed = !svelte_analyze::is_simple_expression(&assign.right);
    if is_computed {
        let thunk = cg.ctx.b.thunk(default_val);
        cg.ctx.b.call_expr(
            "$.fallback",
            [
                Arg::Expr(cg.ctx.b.clone_expr(access)),
                Arg::Expr(thunk),
                Arg::Bool(true),
            ],
        )
    } else {
        cg.ctx.b.call_expr(
            "$.fallback",
            [
                Arg::Expr(cg.ctx.b.clone_expr(access)),
                Arg::Expr(default_val),
            ],
        )
    }
}

fn clone_property_key_expr<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    key: &PropertyKey<'a>,
) -> Expression<'a> {
    use oxc_allocator::CloneIn;
    match key {
        PropertyKey::StaticIdentifier(id) => cg.ctx.b.str_expr(id.name.as_str()),
        PropertyKey::StringLiteral(s) => cg.ctx.b.str_expr(s.value.as_str()),
        PropertyKey::NumericLiteral(n) => cg.ctx.b.num_expr(n.value),
        other => {
            let expr_ref: &Expression<'a> = match other.as_expression() {
                Some(e) => e,
                None => return cg.ctx.b.str_expr(""),
            };
            expr_ref.clone_in(cg.ctx.b.ast.allocator)
        }
    }
}
