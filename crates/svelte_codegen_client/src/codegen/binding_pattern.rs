use std::collections::HashMap;

use oxc_allocator::CloneIn;
use oxc_ast::ast::{BindingPattern, Expression, Statement};
use oxc_semantic::SymbolId;
use svelte_analyze::{
    BindingSemantics, BlockSemantics, ContextualBindingSemantics, DeclaratorSemantics, DerivedEmit,
    EachFlags,
};
use svelte_ast::NodeId;
use svelte_ast_builder::{Arg, ObjProp};
use svelte_component_semantics::{Access, OxcNodeId, walk_bindings};
use svelte_emit_builders::binding_pattern as bp;
use svelte_emit_builders::runes::rune_get;

use super::expr::coarse_wrap;
use super::{Codegen, CodegenError, Result};

const SYNTHETIC_ITEM_NAME: &str = "$$item";

pub(in crate::codegen) enum BindingPatternSource<'a> {
    EachItem { block_id: NodeId },
    AwaitValue,
    ConstTag { id: NodeId, init: Expression<'a> },
    LetCarrier { slot_prop_name: &'a str },
}

pub(in crate::codegen) enum BindingPatternOutput<'a> {
    Statements(Vec<Statement<'a>>),
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
        pattern: &'a BindingPattern<'a>,
        source: BindingPatternSource<'a>,
    ) -> Result<BindingPatternOutput<'a>> {
        use BindingPatternOutput as Out;
        match self.ctx.query.declarator_semantics(decl_node) {
            DeclaratorSemantics::EachItem => {
                let BindingPatternSource::EachItem { block_id } = source else {
                    return CodegenError::unexpected_child(
                        "each-item source",
                        "other binding source",
                    );
                };
                let item_reactive = self.each_item_reactive(block_id)?;
                Ok(Out::Statements(self.emit_each_item(pattern, item_reactive)))
            }
            DeclaratorSemantics::AwaitValue => Ok(Out::Statements(self.emit_await_value(pattern))),
            DeclaratorSemantics::ConstTag { emit } => {
                let BindingPatternSource::ConstTag { id, init } = source else {
                    return CodegenError::unexpected_child(
                        "const tag source",
                        "other binding source",
                    );
                };
                Ok(Out::ConstTagDerived(
                    self.emit_const_tag(id, pattern, init, emit)?,
                ))
            }
            DeclaratorSemantics::LetCarrier { carrier_symbol } => {
                let BindingPatternSource::LetCarrier { slot_prop_name } = source else {
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
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => CodegenError::unexpected_child(
                "template-stage declarator kind",
                "script-stage declarator kind",
            ),
        }
    }

    fn emit_each_item(
        &mut self,
        pattern: &'a BindingPattern<'a>,
        item_reactive: bool,
    ) -> Vec<Statement<'a>> {
        let mut carriers: HashMap<String, String> = HashMap::new();
        let mut carrier_stmts: Vec<Statement<'a>> = Vec::new();
        let mut binding_stmts: Vec<Statement<'a>> = Vec::new();

        walk_bindings(pattern, |v| {
            let needs_derived = v.path.iter().any(|s| s.default.is_some());
            let mut expr = self.item_read_expr(item_reactive);

            for (i, step) in v.path.iter().enumerate() {
                match step.access {
                    Access::Key { key, computed } => {
                        expr = bp::member_access(&self.ctx.b, expr, key, computed);
                    }
                    Access::Index { index, len, has_rest } => {
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
                        let prefix = bp::serialize_prefix(&v.path[..i]);
                        let name =
                            self.ensure_carrier(&mut carriers, &mut carrier_stmts, &prefix, expr, None);
                        let slice_callee =
                            self.ctx.b.static_member_expr(rune_get(&self.ctx.b, &name), "slice");
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
        carrier_stmts
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
        let return_stmt = self.ctx.b.return_stmt(self.ctx.b.shorthand_object_expr(&names));
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
            let Some(tmp_name) = self.ctx.transform_data.const_tag_tmp_names.get(&id).cloned()
            else {
                return CodegenError::unexpected_node(id, "destructured const tag missing tmp_name");
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
        let derived = self.ctx.b.call_expr(helper, [Arg::Expr(self.ctx.b.thunk(prop))]);
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
            if matches!(
                self.ctx.query.view.binding_semantics(v.symbol),
                BindingSemantics::Contextual(
                    ContextualBindingSemantics::LetDirectiveCarrierMember { .. }
                )
            ) {
                names.push(self.ctx.query.symbol_name(v.symbol).to_string());
            }
        });

        let slot_props = self.ctx.b.rid_expr("$$slotProps");
        let source = self.ctx.b.static_member_expr(slot_props, slot_prop_name);
        let destruct_stmt = self
            .ctx
            .b
            .let_destruct_stmt(pattern.clone_in(self.ctx.b.ast.allocator), source);
        let return_stmt = self.ctx.b.return_stmt(self.ctx.b.shorthand_object_expr(&names));
        let derived_body = self.ctx.b.thunk_block(vec![destruct_stmt, return_stmt]);
        let derived = self
            .ctx
            .b
            .call_expr("$.derived", [Arg::Expr(derived_body)]);

        vec![self.ctx.b.const_stmt(&carrier_name, derived)]
    }
}

fn carrier_count(len: u32, has_rest: bool) -> Option<u32> {
    if has_rest { None } else { Some(len) }
}
