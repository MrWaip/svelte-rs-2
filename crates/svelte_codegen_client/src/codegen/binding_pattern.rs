use std::collections::HashMap;

use oxc_allocator::CloneIn;
use oxc_ast::ast::{BindingPattern, Expression, Statement};
use svelte_analyze::DeclaratorSemantics;
use svelte_ast_builder::Arg;
use svelte_component_semantics::{Access, OxcNodeId, walk_bindings};
use svelte_emit_builders::binding_pattern as bp;
use svelte_emit_builders::runes::rune_get;

use super::Codegen;

const SYNTHETIC_ITEM_NAME: &str = "$$item";

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in crate::codegen) fn emit_binding_pattern(
        &mut self,
        decl_node: OxcNodeId,
        pattern: &'a BindingPattern<'a>,
    ) -> Vec<Statement<'a>> {
        match self.ctx.query.declarator_semantics(decl_node) {
            DeclaratorSemantics::EachItem { item_reactive } => {
                self.emit_each_item(pattern, item_reactive)
            }
            DeclaratorSemantics::AwaitValue => self.emit_await_value(pattern),
            DeclaratorSemantics::LetCarrier { .. } => {
                unimplemented!("let: carrier unfold not yet routed through emit_binding_pattern")
            }
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => {
                unreachable!("script-stage declarator kind reached the codegen unfold door")
            }
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
}

fn carrier_count(len: u32, has_rest: bool) -> Option<u32> {
    if has_rest { None } else { Some(len) }
}
