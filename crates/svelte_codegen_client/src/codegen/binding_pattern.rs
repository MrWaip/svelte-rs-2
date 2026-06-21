use std::collections::HashMap;
use std::iter;

use oxc_allocator::CloneIn;
use oxc_ast::ast::{
    BindingPattern, ChainElement, Expression, FormalParameter, PropertyKey, Statement,
};
use oxc_semantic::SymbolId;
use rustc_hash::{FxHashMap, FxHashSet};
use svelte_analyze::{
    BindingSemantics, BlockSemantics, ContextualBindingSemantics, DeclaratorSemantics, DerivedEmit,
    EachFlags, SnippetParam,
};
use svelte_ast::{Node, NodeId};
use svelte_ast_builder::{Arg, ObjProp};
use svelte_component_semantics::{Access, OxcNodeId, walk_bindings};
use svelte_emit_builders::binding_pattern as bp;
use svelte_emit_builders::runes::rune_get;

use super::expr::coarse_wrap;
use super::{Codegen, CodegenError, Result};

const SYNTHETIC_ITEM_NAME: &str = "$$item";

pub(in crate::codegen) enum BindingPatternSource<'a> {
    EachItem {
        block_id: NodeId,
        pattern: &'a BindingPattern<'a>,
    },
    AwaitValue {
        binding_stmt: Option<OxcNodeId>,
    },
    ConstTag {
        id: NodeId,
    },
    LetCarrier {
        slot_prop_name: &'a str,
        pattern: &'a BindingPattern<'a>,
    },
    SnippetParam {
        arg_name: &'a str,
        pattern: &'a BindingPattern<'a>,
    },
}

pub(in crate::codegen) enum BindingPatternOutput<'a> {
    Statements(Vec<Statement<'a>>),
    EachItem {
        decls: Vec<Statement<'a>>,
        writeback_places: FxHashMap<SymbolId, Expression<'a>>,
    },
    ConstTagDerived(ConstTagDerived<'a>),
}

pub(in crate::codegen) struct ConstTagDerived<'a> {
    pub target: &'a str,
    pub derived: Expression<'a>,
    pub simple: bool,
    pub symbols: Vec<SymbolId>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_binding_pattern(
        &mut self,
        decl_node: OxcNodeId,
        source: BindingPatternSource<'a>,
    ) -> Result<BindingPatternOutput<'a>> {
        use BindingPatternOutput as Out;
        match self.ctx.query.declarator_semantics(decl_node) {
            DeclaratorSemantics::EachItem => {
                let BindingPatternSource::EachItem { block_id, pattern } = source else {
                    return CodegenError::unexpected_child(
                        "each-item source",
                        "other binding source",
                    );
                };
                let item_reactive = self.each_item_reactive(block_id)?;
                let (decls, writeback_places) = self.emit_each_item(pattern, item_reactive);
                Ok(Out::EachItem {
                    decls,
                    writeback_places,
                })
            }
            DeclaratorSemantics::AwaitValue => {
                let BindingPatternSource::AwaitValue { binding_stmt } = source else {
                    return CodegenError::unexpected_child(
                        "await value source",
                        "other binding source",
                    );
                };
                let pattern = self.take_await_pattern(binding_stmt)?;
                let pattern_ref: &'a BindingPattern<'a> = self.ctx.b.ast.allocator.alloc(pattern);
                Ok(Out::Statements(self.emit_await_value(pattern_ref)))
            }
            DeclaratorSemantics::ConstTag { emit } => {
                let BindingPatternSource::ConstTag { id } = source else {
                    return CodegenError::unexpected_child(
                        "const tag source",
                        "other binding source",
                    );
                };
                let (pattern, init) = self.take_const_tag_decl(id)?;
                let pattern_ref: &'a BindingPattern<'a> = self.ctx.b.ast.allocator.alloc(pattern);
                Ok(Out::ConstTagDerived(self.emit_const_tag(
                    id,
                    pattern_ref,
                    init,
                    emit,
                )?))
            }
            DeclaratorSemantics::LetCarrier { carrier_symbol } => {
                let BindingPatternSource::LetCarrier {
                    slot_prop_name,
                    pattern,
                } = source
                else {
                    return CodegenError::unexpected_child(
                        "let carrier source",
                        "other binding source",
                    );
                };
                let stmts = match carrier_symbol {
                    Some(sym) => self.emit_let_carrier(pattern, sym, slot_prop_name),
                    None => self.emit_let_simple(pattern, slot_prop_name),
                };
                Ok(Out::Statements(stmts))
            }
            DeclaratorSemantics::SnippetParam => {
                let BindingPatternSource::SnippetParam { arg_name, pattern } = source else {
                    return CodegenError::unexpected_child(
                        "snippet param source",
                        "other binding source",
                    );
                };
                Ok(Out::Statements(
                    self.emit_snippet_param_bindings(pattern, arg_name),
                ))
            }
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_)
            | DeclaratorSemantics::RuntimeRuneCall { .. } => CodegenError::unexpected_child(
                "template-stage declarator kind",
                "script-stage declarator kind",
            ),
        }
    }

    fn take_await_pattern(
        &mut self,
        binding_stmt: Option<OxcNodeId>,
    ) -> Result<BindingPattern<'a>> {
        let Some(stmt_id) = binding_stmt else {
            return CodegenError::unexpected_child("await destructure statement", "none");
        };
        let Some(Statement::VariableDeclaration(mut var_decl)) =
            self.ctx.state.parsed.take_stmt(stmt_id)
        else {
            return CodegenError::unexpected_child(
                "await destructure VariableDeclaration",
                "other",
            );
        };
        Ok(var_decl.declarations.remove(0).id)
    }

    fn take_const_tag_decl(&mut self, id: NodeId) -> Result<(BindingPattern<'a>, Expression<'a>)> {
        let Node::ConstTag(tag) = self.ctx.query.component.store.get(id) else {
            return CodegenError::missing_expression(id);
        };
        let Some(stmt) = self.ctx.state.parsed.take_stmt(tag.decl.id()) else {
            return CodegenError::missing_expression(id);
        };
        let Statement::VariableDeclaration(mut decl) = stmt else {
            return CodegenError::unexpected_node(id, "const tag stmt must be VariableDeclaration");
        };
        if decl.declarations.is_empty() {
            return CodegenError::unexpected_node(id, "const tag stmt has no declarators");
        }
        let mut declarator = decl.declarations.remove(0);
        let Some(init) = declarator.init.take() else {
            return CodegenError::unexpected_node(id, "const tag declarator must have init");
        };
        Ok((declarator.id, init))
    }

    fn emit_each_item(
        &mut self,
        pattern: &'a BindingPattern<'a>,
        item_reactive: bool,
    ) -> (Vec<Statement<'a>>, FxHashMap<SymbolId, Expression<'a>>) {
        let mut carriers: HashMap<String, String> = HashMap::new();
        let mut carrier_stmts: Vec<Statement<'a>> = Vec::new();
        let mut binding_stmts: Vec<Statement<'a>> = Vec::new();
        let mut writeback_places: FxHashMap<SymbolId, Expression<'a>> = FxHashMap::default();

        walk_bindings(pattern, |v| {
            let needs_derived = v.path.iter().any(|s| s.default.is_some());
            let mut expr = self.item_read_expr(item_reactive);
            let mut update_expr = self.item_read_expr(item_reactive);
            let mut member_chain = !v.is_rest;

            let simple_flags: Option<Vec<bool>> = self
                .ctx
                .transform_data
                .destructure_default_simple
                .get(&v.symbol)
                .cloned();
            let mut default_cursor = 0usize;

            for (i, step) in v.path.iter().enumerate() {
                match step.access {
                    Access::Key { key, computed } => {
                        expr = bp::member_access(&self.ctx.b, expr, key, computed);
                        update_expr = bp::member_access(&self.ctx.b, update_expr, key, computed);
                    }
                    Access::Index {
                        index,
                        len,
                        has_rest,
                    } => {
                        member_chain = false;
                        let prefix = bp::serialize_prefix(&v.path[..i]);
                        let name = self.ensure_carrier(
                            &mut carriers,
                            &mut carrier_stmts,
                            &prefix,
                            expr,
                            carrier_count(len, has_rest),
                        );
                        expr = self.ctx.b.computed_member_expr(
                            rune_get(&self.ctx.b, &name),
                            self.ctx.b.num_expr(index as f64),
                        );
                    }
                    Access::Slice { from } => {
                        member_chain = false;
                        let prefix = bp::serialize_prefix(&v.path[..i]);
                        let name = self.ensure_carrier(
                            &mut carriers,
                            &mut carrier_stmts,
                            &prefix,
                            expr,
                            None,
                        );
                        let slice_callee = self
                            .ctx
                            .b
                            .static_member_expr(rune_get(&self.ctx.b, &name), "slice");
                        expr = self
                            .ctx
                            .b
                            .call_expr_callee(slice_callee, [Arg::Num(from as f64)]);
                    }
                }
                if let Some(default) = step.default {
                    let simple = simple_flags
                        .as_ref()
                        .and_then(|f| f.get(default_cursor).copied())
                        .unwrap_or(false);
                    default_cursor += 1;
                    expr = bp::fallback_with_simple(&self.ctx.b, expr, default, None, simple);
                }
            }

            if v.is_rest {
                expr = bp::exclude_from_object(&self.ctx.b, expr, v.excluded);
            }

            if member_chain {
                writeback_places.insert(v.symbol, update_expr);
            }

            let name = self.ctx.query.symbol_name(v.symbol).to_string();
            let thunk = self.ctx.b.thunk(expr);
            let init = if needs_derived {
                self.ctx
                    .b
                    .call_expr("$.derived_safe_equal", [Arg::Expr(thunk)])
            } else {
                thunk
            };
            binding_stmts.push(self.ctx.b.let_init_stmt(&name, init));
        });

        carrier_stmts.extend(binding_stmts);
        (carrier_stmts, writeback_places)
    }

    fn emit_await_value(&mut self, pattern: &'a BindingPattern<'a>) -> Vec<Statement<'a>> {
        let helper = self.ctx.query.view.derived_helper();

        let mut names: Vec<String> = Vec::new();
        walk_bindings(pattern, |v| {
            names.push(self.ctx.query.symbol_name(v.symbol).to_string());
        });

        let source = rune_get(&self.ctx.b, "$$source");
        let destruct_stmt = self
            .ctx
            .b
            .var_destruct_stmt(pattern.clone_in(self.ctx.b.ast.allocator), source);
        let return_stmt = self
            .ctx
            .b
            .return_stmt(self.ctx.b.shorthand_object_expr(&names));
        let derived_fn = self.ctx.b.thunk_block(vec![destruct_stmt, return_stmt]);
        let derived_call = self.ctx.b.call_expr(helper, [Arg::Expr(derived_fn)]);

        let mut decls = vec![self.ctx.b.var_stmt("$$value", derived_call)];
        for name in &names {
            let member = self
                .ctx
                .b
                .static_member_expr(rune_get(&self.ctx.b, "$$value"), name);
            let getter_fn = self
                .ctx
                .b
                .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(member)]);
            let per_field = self.ctx.b.call_expr(helper, [Arg::Expr(getter_fn)]);
            decls.push(self.ctx.b.var_stmt(name, per_field));
        }
        decls
    }

    fn emit_const_tag(
        &mut self,
        id: NodeId,
        pattern: &'a BindingPattern<'a>,
        init: Expression<'a>,
        emit: DerivedEmit,
    ) -> Result<ConstTagDerived<'a>> {
        let init = coarse_wrap(self.ctx, init, self.ctx.expression_data(id));

        let mut symbols: Vec<SymbolId> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        walk_bindings(pattern, |v| {
            symbols.push(v.symbol);
            names.push(self.ctx.query.view.symbol_name(v.symbol).to_string());
        });

        if matches!(pattern, BindingPattern::BindingIdentifier(_)) {
            let Some(name) = names.first().cloned() else {
                return CodegenError::unexpected_child("const tag binding", "empty bindings");
            };
            let target: &str = self.ctx.b.alloc_str(&name);
            let value_thunk = match emit {
                DerivedEmit::Async => self.ctx.b.async_thunk(init),
                DerivedEmit::Sync => self.ctx.b.thunk(init),
            };
            let derived = self.build_derived(value_thunk, emit);
            let derived = if self.ctx.state.dev {
                self.ctx
                    .b
                    .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef(target)])
            } else {
                derived
            };
            Ok(ConstTagDerived {
                target,
                derived,
                simple: true,
                symbols,
            })
        } else {
            let Some(tmp_name) = self
                .ctx
                .transform_data
                .const_tag_tmp_names
                .get(&id)
                .cloned()
            else {
                return CodegenError::unexpected_node(
                    id,
                    "destructured const tag missing tmp_name",
                );
            };
            let target: &str = self.ctx.b.alloc_str(&tmp_name);
            let destruct_stmt = self
                .ctx
                .b
                .const_destruct_stmt(pattern.clone_in(self.ctx.b.ast.allocator), init);
            let props: Vec<ObjProp<'a>> = names
                .iter()
                .map(|n| ObjProp::Shorthand(self.ctx.b.alloc_str(n)))
                .collect();
            let ret = self.ctx.b.return_stmt(self.ctx.b.object_expr(props));
            let value_thunk = match emit {
                DerivedEmit::Async => self.ctx.b.async_thunk_block(vec![destruct_stmt, ret]),
                DerivedEmit::Sync => self
                    .ctx
                    .b
                    .arrow_block_expr(self.ctx.b.no_params(), [destruct_stmt, ret]),
            };
            let derived = self.build_derived(value_thunk, emit);
            let derived = if self.ctx.state.dev {
                self.ctx
                    .b
                    .call_expr("$.tag", [Arg::Expr(derived), Arg::StrRef("[@const]")])
            } else {
                derived
            };
            Ok(ConstTagDerived {
                target,
                derived,
                simple: false,
                symbols,
            })
        }
    }

    fn build_derived(&mut self, value_thunk: Expression<'a>, emit: DerivedEmit) -> Expression<'a> {
        match emit {
            DerivedEmit::Async => self
                .ctx
                .b
                .call_expr("$.async_derived", [Arg::Expr(value_thunk)]),
            DerivedEmit::Sync => {
                let helper = self.ctx.query.view.derived_helper();
                self.ctx.b.call_expr(helper, [Arg::Expr(value_thunk)])
            }
        }
    }

    fn each_item_reactive(&self, block_id: NodeId) -> Result<bool> {
        match self.ctx.query.analysis.block_semantics(block_id) {
            BlockSemantics::Each(s) => Ok(s.each_flags.contains(EachFlags::ITEM_REACTIVE)),
            _ => CodegenError::unexpected_block_semantics(block_id, "Each expected"),
        }
    }

    fn item_read_expr(&self, item_reactive: bool) -> Expression<'a> {
        if item_reactive {
            rune_get(&self.ctx.b, SYNTHETIC_ITEM_NAME)
        } else {
            self.ctx.b.rid_expr(SYNTHETIC_ITEM_NAME)
        }
    }

    fn ensure_carrier(
        &mut self,
        carriers: &mut HashMap<String, String>,
        carrier_stmts: &mut Vec<Statement<'a>>,
        prefix: &str,
        array_expr: Expression<'a>,
        count: Option<u32>,
    ) -> String {
        if let Some(name) = carriers.get(prefix) {
            return name.clone();
        }
        let name = self.ctx.state.gen_ident("$$array");
        let derived = bp::to_array_derived(&self.ctx.b, array_expr, count, None);
        carrier_stmts.push(self.ctx.b.var_stmt(&name, derived));
        carriers.insert(prefix.to_string(), name.clone());
        name
    }

    fn emit_let_simple(
        &mut self,
        pattern: &'a BindingPattern<'a>,
        slot_prop_name: &'a str,
    ) -> Vec<Statement<'a>> {
        let mut names: Vec<String> = Vec::new();
        walk_bindings(pattern, |v| {
            names.push(self.ctx.query.symbol_name(v.symbol).to_string());
        });
        let Some(name) = names.into_iter().next() else {
            return Vec::new();
        };
        let slot_props = self.ctx.b.rid_expr("$$slotProps");
        let prop = self.ctx.b.static_member_expr(slot_props, slot_prop_name);
        let helper = self.ctx.query.view.derived_helper();
        let derived = self
            .ctx
            .b
            .call_expr(helper, [Arg::Expr(self.ctx.b.thunk(prop))]);
        vec![self.ctx.b.const_stmt(&name, derived)]
    }

    fn emit_let_carrier(
        &mut self,
        pattern: &'a BindingPattern<'a>,
        carrier_symbol: SymbolId,
        slot_prop_name: &'a str,
    ) -> Vec<Statement<'a>> {
        use oxc_allocator::CloneIn;

        let carrier_name = self.ctx.query.symbol_name(carrier_symbol).to_string();

        let mut names: Vec<String> = Vec::new();
        walk_bindings(pattern, |v| {
            let carried = match self.ctx.query.view.binding_semantics(v.symbol) {
                BindingSemantics::Contextual(contextual) => match contextual {
                    ContextualBindingSemantics::LetDirectiveCarrierMember { .. } => true,
                    ContextualBindingSemantics::EachItem(_)
                    | ContextualBindingSemantics::EachIndex(_)
                    | ContextualBindingSemantics::AwaitValue
                    | ContextualBindingSemantics::AwaitError
                    | ContextualBindingSemantics::LetDirective
                    | ContextualBindingSemantics::LetDirectiveDirect
                    | ContextualBindingSemantics::SnippetParam(_) => false,
                },
                BindingSemantics::Prop(_)
                | BindingSemantics::State(_)
                | BindingSemantics::Derived(_)
                | BindingSemantics::OptimizedDerived(_)
                | BindingSemantics::OptimizedRune(_)
                | BindingSemantics::RuntimeRune { .. }
                | BindingSemantics::Store(_)
                | BindingSemantics::LegacyBindableProp(_)
                | BindingSemantics::LegacyState(_)
                | BindingSemantics::Const(_)
                | BindingSemantics::MaybeReactive
                | BindingSemantics::NonReactive
                | BindingSemantics::LegacyApiExport
                | BindingSemantics::Unresolved => false,
            };
            if carried {
                names.push(self.ctx.query.symbol_name(v.symbol).to_string());
            }
        });

        let slot_props = self.ctx.b.rid_expr("$$slotProps");
        let source = self.ctx.b.static_member_expr(slot_props, slot_prop_name);
        let destruct_stmt = self
            .ctx
            .b
            .let_destruct_stmt(pattern.clone_in(self.ctx.b.ast.allocator), source);
        let return_stmt = self
            .ctx
            .b
            .return_stmt(self.ctx.b.shorthand_object_expr(&names));
        let derived_body = self.ctx.b.thunk_block(vec![destruct_stmt, return_stmt]);
        let derived = self.ctx.b.call_expr("$.derived", [Arg::Expr(derived_body)]);

        vec![self.ctx.b.const_stmt(&carrier_name, derived)]
    }

    pub(in crate::codegen) fn emit_snippet_param(
        &mut self,
        param: &SnippetParam,
        idx: usize,
        pattern: Option<BindingPattern<'a>>,
        default: Option<Expression<'a>>,
    ) -> Result<(FormalParameter<'a>, Vec<Statement<'a>>)> {
        match param {
            SnippetParam::Identifier { sym } => {
                let name = self.ctx.query.view.symbol_name(*sym).to_string();
                let Some(default) = default else {
                    return Ok((self.formal_param_ident(&name, true), Vec::new()));
                };
                Ok(self.emit_snippet_identifier_default(&name, idx, &default))
            }
            SnippetParam::Pattern { pattern_id } => {
                let arg_name = format!("$$arg{idx}");
                let formal = self.formal_param_ident(&arg_name, false);
                let Some(pattern) = pattern else {
                    return Ok((formal, Vec::new()));
                };
                let pattern_ref: &'a BindingPattern<'a> = self.ctx.b.ast.allocator.alloc(pattern);
                let arg_name_ref: &'a str = self.ctx.b.alloc_str(&arg_name);
                let out = self.emit_binding_pattern(
                    *pattern_id,
                    BindingPatternSource::SnippetParam {
                        arg_name: arg_name_ref,
                        pattern: pattern_ref,
                    },
                )?;
                let BindingPatternOutput::Statements(stmts) = out else {
                    return CodegenError::unexpected_child(
                        "snippet param statements",
                        "other binding output",
                    );
                };
                Ok((formal, stmts))
            }
        }
    }

    fn emit_snippet_identifier_default(
        &mut self,
        name: &str,
        idx: usize,
        default: &Expression<'a>,
    ) -> (FormalParameter<'a>, Vec<Statement<'a>>) {
        let arg_name = format!("$$arg{idx}");
        let formal = self.formal_param_ident(&arg_name, false);
        let arg_read = self
            .ctx
            .b
            .maybe_call_expr(self.ctx.b.rid_expr(&arg_name), iter::empty::<Arg<'_, '_>>());
        let value = bp::fallback(&self.ctx.b, arg_read, default, None);
        let thunk = self.ctx.b.thunk(value);
        let init = self
            .ctx
            .b
            .call_expr("$.derived_safe_equal", [Arg::Expr(thunk)]);
        let mut stmts = vec![self.ctx.b.let_init_stmt(name, init)];
        if self.ctx.state.dev {
            let name_alloc = self.ctx.b.alloc_str(name);
            stmts.push(self.ctx.b.call_stmt("$.get", [Arg::Ident(name_alloc)]));
        }
        (formal, stmts)
    }

    fn emit_snippet_param_bindings(
        &mut self,
        pattern: &'a BindingPattern<'a>,
        arg_name: &str,
    ) -> Vec<Statement<'a>> {
        let mut carriers: HashMap<String, String> = HashMap::new();
        let mut carrier_stmts: Vec<Statement<'a>> = Vec::new();
        let mut carrier_names: FxHashSet<String> = FxHashSet::default();
        let mut binding_inits: Vec<(String, Expression<'a>, bool)> = Vec::new();

        walk_bindings(pattern, |v| {
            let needs_derived = v.path.iter().any(|s| s.default.is_some());
            let mut expr = self
                .ctx
                .b
                .maybe_call_expr(self.ctx.b.rid_expr(arg_name), iter::empty::<Arg<'_, '_>>());

            for (i, step) in v.path.iter().enumerate() {
                match step.access {
                    Access::Key { key, computed } => {
                        expr = param_member_access(self, expr, key, computed);
                    }
                    Access::Index {
                        index,
                        len,
                        has_rest,
                    } => {
                        let prefix = bp::serialize_prefix(&v.path[..i]);
                        let name = self.ensure_carrier(
                            &mut carriers,
                            &mut carrier_stmts,
                            &prefix,
                            expr,
                            carrier_count(len, has_rest),
                        );
                        carrier_names.insert(name.clone());
                        expr = self.ctx.b.computed_member_expr(
                            self.ctx.b.rid_expr(&name),
                            self.ctx.b.num_expr(index as f64),
                        );
                    }
                    Access::Slice { from } => {
                        let prefix = bp::serialize_prefix(&v.path[..i]);
                        let name = self.ensure_carrier(
                            &mut carriers,
                            &mut carrier_stmts,
                            &prefix,
                            expr,
                            None,
                        );
                        carrier_names.insert(name.clone());
                        let slice_callee = self
                            .ctx
                            .b
                            .static_member_expr(self.ctx.b.rid_expr(&name), "slice");
                        expr = self
                            .ctx
                            .b
                            .call_expr_callee(slice_callee, [Arg::Num(from as f64)]);
                    }
                }
                if let Some(default) = step.default {
                    expr = bp::fallback(&self.ctx.b, expr, default, None);
                }
            }

            if v.is_rest {
                expr = bp::exclude_from_object(&self.ctx.b, expr, v.excluded);
            }

            let name = self.ctx.query.symbol_name(v.symbol).to_string();
            binding_inits.push((name, expr, needs_derived));
        });

        let dev = self.ctx.state.dev;
        for (name, mut expr, needs_derived) in binding_inits {
            rewrite_array_reads(self, &mut expr, &carrier_names);
            let thunk = self.ctx.b.thunk(expr);
            let init = if needs_derived {
                self.ctx
                    .b
                    .call_expr("$.derived_safe_equal", [Arg::Expr(thunk)])
            } else {
                thunk
            };
            carrier_stmts.push(self.ctx.b.let_init_stmt(&name, init));
            if dev {
                let name_alloc = self.ctx.b.alloc_str(&name);
                let eager = if needs_derived {
                    self.ctx.b.call_stmt("$.get", [Arg::Ident(name_alloc)])
                } else {
                    self.ctx.b.call_stmt(&name, iter::empty::<Arg<'_, '_>>())
                };
                carrier_stmts.push(eager);
            }
        }

        carrier_stmts
    }
}

fn carrier_count(len: u32, has_rest: bool) -> Option<u32> {
    if has_rest { None } else { Some(len) }
}

fn param_member_access<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    object: Expression<'a>,
    key: &PropertyKey<'_>,
    computed: bool,
) -> Expression<'a> {
    let object = break_optional_chain(cg, object);
    if !computed && let PropertyKey::StaticIdentifier(id) = key {
        return cg.ctx.b.static_member_expr(object, id.name.as_str());
    }
    let key_expr = clone_property_key_expr(cg, key);
    cg.ctx.b.computed_member_expr(object, key_expr)
}

fn break_optional_chain<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    object: Expression<'a>,
) -> Expression<'a> {
    use oxc_span::SPAN;
    match object {
        Expression::ChainExpression(_) => cg.ctx.b.ast.expression_parenthesized(SPAN, object),
        other => other,
    }
}

fn clone_property_key_expr<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    key: &PropertyKey<'_>,
) -> Expression<'a> {
    match key {
        PropertyKey::StaticIdentifier(id) => cg.ctx.b.str_expr(id.name.as_str()),
        PropertyKey::StringLiteral(s) => cg.ctx.b.str_expr(s.value.as_str()),
        PropertyKey::NumericLiteral(n) => cg.ctx.b.num_expr(n.value),
        other => match other.as_expression() {
            Some(e) => e.clone_in(cg.ctx.b.ast.allocator),
            None => cg.ctx.b.str_expr(""),
        },
    }
}

fn rewrite_array_reads<'a, 'ctx>(
    cg: &Codegen<'a, 'ctx>,
    expr: &mut Expression<'a>,
    carrier_names: &FxHashSet<String>,
) {
    let expr = expr.get_inner_expression_mut();
    match expr {
        Expression::StaticMemberExpression(member) => {
            rewrite_array_reads(cg, &mut member.object, carrier_names);
        }
        Expression::ComputedMemberExpression(member) => {
            rewrite_array_reads(cg, &mut member.object, carrier_names);
            rewrite_array_reads(cg, &mut member.expression, carrier_names);
        }
        Expression::ChainExpression(chain) => match &mut chain.expression {
            ChainElement::StaticMemberExpression(member) => {
                rewrite_array_reads(cg, &mut member.object, carrier_names);
            }
            ChainElement::ComputedMemberExpression(member) => {
                rewrite_array_reads(cg, &mut member.object, carrier_names);
                rewrite_array_reads(cg, &mut member.expression, carrier_names);
            }
            ChainElement::CallExpression(call) => {
                rewrite_array_reads(cg, &mut call.callee, carrier_names);
                for arg in call.arguments.iter_mut() {
                    if let Some(arg_expr) = arg.as_expression_mut() {
                        rewrite_array_reads(cg, arg_expr, carrier_names);
                    }
                }
            }
            _ => {}
        },
        Expression::CallExpression(call) => {
            rewrite_array_reads(cg, &mut call.callee, carrier_names);
            for arg in call.arguments.iter_mut() {
                if let Some(arg_expr) = arg.as_expression_mut() {
                    rewrite_array_reads(cg, arg_expr, carrier_names);
                }
            }
        }
        Expression::ArrayExpression(arr) => {
            for element in arr.elements.iter_mut() {
                if let Some(el_expr) = element.as_expression_mut() {
                    rewrite_array_reads(cg, el_expr, carrier_names);
                }
            }
        }
        _ => {}
    }

    let Expression::Identifier(ident) = expr else {
        return;
    };
    if !carrier_names.contains(ident.name.as_str()) {
        return;
    }
    let name_alloc = cg.ctx.b.alloc_str(ident.name.as_str());
    *expr = rune_get(&cg.ctx.b, name_alloc);
}
