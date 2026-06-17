use oxc_allocator::CloneIn;
use oxc_ast::ast::{Expression, ObjectPropertyKind, PropertyKind, Statement};
use oxc_span::SPAN;
use svelte_analyze::scope::SymbolId;
use svelte_analyze::{ComponentBindSemantics, ComponentBindTarget};
use svelte_ast::{BindDirective, NodeId};
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};
use svelte_emit_builders::runes::rune_get;
use svelte_emit_builders::runtime::thunk_call;
use svelte_emit_builders::store::build_store_base_read;

use super::super::Codegen;
use super::dispatch::{OwnershipBinding, PropOrSpread};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_bind_member_expr(
        &mut self,
        directive: &BindDirective,
        bind: &ComponentBindSemantics,
        expr: Expression<'a>,
        items: &mut Vec<PropOrSpread<'a>>,
        validate_binding_stmts: &mut Vec<Statement<'a>>,
    ) -> super::super::Result<()> {
        let Expression::ObjectExpression(obj) = expr else {
            return super::super::CodegenError::unexpected_node(
                directive.id,
                "ComponentBind Expression: get/set object expected",
            );
        };
        let obj = obj.unbox();
        if self.ctx.state.dev
            && let Some(stmt) = self.build_validate_binding_stmt(directive, bind, &obj.properties)
        {
            validate_binding_stmts.push(stmt);
        }
        for prop in obj.properties {
            items.push(PropOrSpread::Prop(ObjProp::Raw(prop)));
        }
        Ok(())
    }

    fn build_validate_binding_stmt(
        &self,
        directive: &BindDirective,
        bind: &ComponentBindSemantics,
        properties: &oxc_allocator::Vec<'a, ObjectPropertyKind<'a>>,
    ) -> Option<Statement<'a>> {
        let getter_prop = properties.iter().find_map(|p| match p {
            ObjectPropertyKind::ObjectProperty(op) if op.kind == PropertyKind::Get => Some(op),
            _ => None,
        })?;
        let Expression::FunctionExpression(func) = &getter_prop.value else {
            return None;
        };
        let body = func.body.as_ref()?;
        let return_stmt = body.statements.iter().find_map(|s| match s {
            Statement::ReturnStatement(r) => Some(r),
            _ => None,
        })?;
        let member_expr = return_stmt.argument.as_ref()?;
        let alloc = self.ctx.b.ast.allocator;
        let (object_clone, property_thunk) = match member_expr {
            Expression::StaticMemberExpression(m) => {
                let object = m.object.clone_in(alloc);
                let name = m.property.name.as_str().to_string();
                let leaf = self.ctx.b.thunk(self.ctx.b.str_expr(&name));
                (object, leaf)
            }
            Expression::ComputedMemberExpression(m) => {
                let object = m.object.clone_in(alloc);
                let leaf = self.ctx.b.thunk(m.expression.clone_in(alloc));
                (object, leaf)
            }
            _ => return None,
        };
        let object_thunk = self.ctx.b.thunk(object_clone);
        let source_text = self
            .ctx
            .query
            .component
            .source_text(directive.span)
            .to_string();
        let each_ids: Vec<Expression<'a>> = bind
            .each_context_vars
            .iter()
            .map(|sym| {
                let name = self.ctx.query.view.symbol_name(*sym);
                let alloc_name = self.ctx.b.alloc_str(name);
                self.ctx.b.rid_expr(alloc_name)
            })
            .collect();
        let each_array = self.ctx.b.array_expr(each_ids);
        let (line, col) = self.ctx.state.line_index.line_col(directive.span.start);
        Some(self.ctx.b.call_stmt(
            "$.validate_binding",
            [
                Arg::Str(source_text),
                Arg::Expr(each_array),
                Arg::Expr(object_thunk),
                Arg::Expr(property_thunk),
                Arg::Num(line as f64),
                Arg::Num(col as f64),
            ],
        ))
    }

    pub(super) fn emit_bind_function_pair(
        &mut self,
        bind_id: NodeId,
        name: &str,
        expr: Expression<'a>,
        items: &mut Vec<PropOrSpread<'a>>,
        init_stmts: &mut Vec<Statement<'a>>,
    ) -> super::super::Result<()> {
        let Expression::SequenceExpression(seq) = expr else {
            return super::super::CodegenError::unexpected_node(
                bind_id,
                "ComponentBind FunctionPair: SequenceExpression expected",
            );
        };
        let seq = seq.unbox();
        let mut iter = seq.expressions.into_iter();
        let Some(get_expr) = iter.next() else {
            return super::super::CodegenError::unexpected_node(
                bind_id,
                "ComponentBind FunctionPair: missing getter",
            );
        };
        let Some(set_expr) = iter.next() else {
            return super::super::CodegenError::unexpected_node(
                bind_id,
                "ComponentBind FunctionPair: missing setter",
            );
        };

        let bind_get_name = self.ctx.state.gen_ident("bind_get");
        let bind_set_name = self.ctx.state.gen_ident("bind_set");
        let bind_get_ref: &'a str = self.ctx.b.alloc_str(&bind_get_name);
        let bind_set_ref: &'a str = self.ctx.b.alloc_str(&bind_set_name);

        init_stmts.push(self.ctx.b.var_stmt(bind_get_ref, get_expr));
        init_stmts.push(self.ctx.b.var_stmt(bind_set_ref, set_expr));

        let key = self.ctx.b.alloc_str(name);
        let getter_call = self.ctx.b.call_expr(bind_get_ref, []);
        items.push(PropOrSpread::Prop(ObjProp::Getter(key, getter_call)));
        let setter_call = self.ctx.b.call_expr(bind_set_ref, [Arg::Ident("$$value")]);
        items.push(PropOrSpread::Prop(ObjProp::Setter(
            key,
            "$$value",
            None,
            vec![self.ctx.b.expr_stmt(setter_call)],
        )));
        Ok(())
    }

    pub(super) fn emit_bind_plain(
        &mut self,
        name: &str,
        source_text: &str,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        let source_ref = self.ctx.b.alloc_str(source_text);
        let get_body = self.ctx.b.rid_expr(source_ref);
        items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
        let set_body = self.ctx.b.assign_expr(
            AssignLeft::Ident(source_text.to_string()),
            self.ctx.b.rid_expr("$$value"),
        );
        items.push(PropOrSpread::Prop(ObjProp::Setter(
            key,
            "$$value",
            None,
            vec![self.ctx.b.expr_stmt(set_body)],
        )));
    }

    pub(super) fn emit_bind_identifier(
        &mut self,
        _el_id: NodeId,
        name: &str,
        source_text: &str,
        target: ComponentBindTarget,
        items: &mut Vec<PropOrSpread<'a>>,
        ownership_bindings: &mut Vec<OwnershipBinding<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        let source_ref = self.ctx.b.alloc_str(source_text);
        match target {
            ComponentBindTarget::PropSource | ComponentBindTarget::PropSourceOwned => {
                let get_body = self.ctx.b.call_expr(source_ref, []);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self.ctx.b.call_expr(source_ref, [Arg::Ident("$$value")]);
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
                if matches!(target, ComponentBindTarget::PropSourceOwned) {
                    ownership_bindings.push(OwnershipBinding {
                        name: name.to_string(),
                        source_ident: source_ref,
                    });
                }
            }
            ComponentBindTarget::Rune { proxy } => {
                let get_body = rune_get(&self.ctx.b, source_ref);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = if proxy {
                    self.ctx.b.call_expr(
                        "$.set",
                        [
                            Arg::Ident(source_ref),
                            Arg::Ident("$$value"),
                            Arg::Bool(true),
                        ],
                    )
                } else {
                    self.ctx
                        .b
                        .call_expr("$.set", [Arg::Ident(source_ref), Arg::Ident("$$value")])
                };
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindTarget::RuneDerived => {
                let get_body = rune_get(&self.ctx.b, source_ref);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self
                    .ctx
                    .b
                    .call_expr("$.set", [Arg::Ident(source_ref), Arg::Ident("$$value")]);
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindTarget::LegacyState => {
                let get_body = rune_get(&self.ctx.b, source_ref);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_body = self
                    .ctx
                    .b
                    .call_expr("$.set", [Arg::Ident(source_ref), Arg::Ident("$$value")]);
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindTarget::LegacyStateSubscribed => {
                let get_body = rune_get(&self.ctx.b, source_ref);
                items.push(PropOrSpread::Prop(ObjProp::Getter(key, get_body)));
                let set_call = self
                    .ctx
                    .b
                    .call_expr("$.set", [Arg::Ident(source_ref), Arg::Ident("$$value")]);
                let store_literal = format!("${source_text}");
                let set_body = self.ctx.b.call_expr(
                    "$.store_unsub",
                    [
                        Arg::Expr(set_call),
                        Arg::Str(store_literal),
                        Arg::Ident("$$stores"),
                    ],
                );
                items.push(PropOrSpread::Prop(ObjProp::Setter(
                    key,
                    "$$value",
                    None,
                    vec![self.ctx.b.expr_stmt(set_body)],
                )));
            }
            ComponentBindTarget::EachItemDestructureLegacy { symbol } => {
                match self.build_each_item_destructure_writeback_legacy(symbol) {
                    Some(setter_body) => {
                        let getter = self.ctx.b.call_expr(source_ref, []);
                        items.push(PropOrSpread::Prop(ObjProp::Getter(key, getter)));
                        items.push(PropOrSpread::Prop(ObjProp::Setter(
                            key,
                            "$$value",
                            None,
                            vec![self.ctx.b.expr_stmt(setter_body)],
                        )));
                    }
                    None => self.emit_bind_plain(name, source_text, items),
                }
            }
            ComponentBindTarget::Plain => {
                self.emit_bind_plain(name, source_text, items);
            }
        }
    }

    fn read_each_item_source_legacy(&self, sym: SymbolId) -> Expression<'a> {
        let name = self.ctx.query.view.symbol_name(sym);
        let semantics = self.ctx.query.analysis.binding_semantics(sym);
        if semantics.reads_via_each_item_accessor() || semantics.reads_via_thunk() {
            return thunk_call(&self.ctx.b, name);
        }
        if semantics.is_non_reactive() {
            return self.ctx.b.rid_expr(name);
        }
        rune_get(&self.ctx.b, name)
    }

    fn read_each_item_collection_legacy(
        &self,
        item_sym: SymbolId,
        source_sym: SymbolId,
    ) -> Expression<'a> {
        if let Some(block_id) = self
            .ctx
            .state
            .transform_data
            .each_collection_block_by_item_legacy
            .get(&item_sym)
            && let Some(name) = self
                .ctx
                .state
                .transform_data
                .each_collection_internal_names_legacy
                .get(block_id)
        {
            return thunk_call(&self.ctx.b, name.as_str());
        }
        self.read_each_item_source_legacy(source_sym)
    }

    pub(crate) fn build_each_item_destructure_writeback_legacy(
        &self,
        symbol: SymbolId,
    ) -> Option<Expression<'a>> {
        let place = self
            .ctx
            .state
            .each_item_writeback_places
            .as_ref()?
            .get(&symbol)?
            .clone_in(self.ctx.b.ast.allocator);
        let assignment = self.ctx.b.assign_expr_raw(
            self.ctx.b.expr_to_assignment_target(place),
            self.ctx.b.rid_expr("$$value"),
        );

        let analysis = self.ctx.query.analysis;
        let sources = analysis.each_item_indirect_sources(symbol).unwrap_or(&[]);
        let inner = match sources {
            [single] => self.read_each_item_collection_legacy(symbol, *single),
            many => {
                let reads: Vec<Expression<'a>> = many
                    .iter()
                    .map(|&s| self.read_each_item_collection_legacy(symbol, s))
                    .collect();
                self.ctx
                    .b
                    .ast
                    .expression_sequence(SPAN, self.ctx.b.ast.vec_from_iter(reads))
            }
        };
        let thunk = self
            .ctx
            .b
            .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(inner)]);
        let invalidate_inner = self
            .ctx
            .b
            .call_expr("$.invalidate_inner_signals", [Arg::Expr(thunk)]);

        let mut seq: Vec<Expression<'a>> = vec![assignment, invalidate_inner];
        if let Some(&store_sym) = sources
            .iter()
            .find(|&&s| analysis.binding_semantics(s).is_store())
        {
            let store_name = self.ctx.query.view.symbol_name(store_sym).to_string();
            seq.push(self.ctx.b.call_expr(
                "$.invalidate_store",
                [Arg::Ident("$$stores"), Arg::Str(store_name)],
            ));
        }
        Some(
            self.ctx
                .b
                .ast
                .expression_sequence(SPAN, self.ctx.b.ast.vec_from_iter(seq)),
        )
    }

    pub(super) fn emit_bind_store(
        &mut self,
        name: &str,
        base_symbol: SymbolId,
        items: &mut Vec<PropOrSpread<'a>>,
    ) {
        let key = self.ctx.b.alloc_str(name);
        let base_name = self.ctx.query.view.symbol_name(base_symbol);
        let store_ref = format!("${base_name}");
        let store_id = self.ctx.b.alloc_str(&store_ref);

        let mark_stmt = self
            .ctx
            .b
            .expr_stmt(self.ctx.b.call_expr("$.mark_store_binding", []));
        let return_expr = self.ctx.b.call_expr(store_id, []);
        let return_stmt = self.ctx.b.return_stmt(return_expr);
        items.push(PropOrSpread::Prop(ObjProp::GetterBody(
            key,
            vec![mark_stmt, return_stmt],
        )));

        let base_expr = build_store_base_read(&self.ctx.b, self.ctx.query.analysis, base_symbol);
        let set_body = self
            .ctx
            .b
            .call_expr("$.store_set", [Arg::Expr(base_expr), Arg::Ident("$$value")]);
        items.push(PropOrSpread::Prop(ObjProp::Setter(
            key,
            "$$value",
            None,
            vec![self.ctx.b.expr_stmt(set_body)],
        )));
    }
}
