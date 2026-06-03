use std::collections::HashMap;
use std::fmt::Write;

use oxc_allocator::CloneIn;
use oxc_ast::ast::{BindingPattern, Expression, PropertyKey, Statement};
use oxc_span::GetSpan;
use svelte_analyze::DeclaratorSemantics;
use svelte_ast_builder::Arg;
use svelte_component_semantics::{Access, OxcNodeId, walk_bindings};
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
            | DeclaratorSemantics::PropsIdentifier { .. }
            | DeclaratorSemantics::PropsObject { .. }
            | DeclaratorSemantics::LegacyStateDestructure { .. }
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
            let mut prefix = String::new();

            for step in v.path {
                match step.access {
                    Access::Key { key, computed } => {
                        prefix.push_str(&self.serialize_key(key, computed));
                        expr = self.member_from_key(expr, key, computed);
                    }
                    Access::Index { index, len, has_rest } => {
                        let name = self.ensure_carrier(
                            &mut carriers,
                            &mut carrier_stmts,
                            &prefix,
                            expr,
                            carrier_count(len, has_rest),
                        );
                        let _ = write!(prefix, "[{index}]");
                        expr = self.ctx.b.computed_member_expr(
                            rune_get(&self.ctx.b, &name),
                            self.ctx.b.num_expr(index as f64),
                        );
                    }
                    Access::Slice { from } => {
                        let name =
                            self.ensure_carrier(&mut carriers, &mut carrier_stmts, &prefix, expr, None);
                        let _ = write!(prefix, "[s{from}]");
                        let slice_callee =
                            self.ctx.b.static_member_expr(rune_get(&self.ctx.b, &name), "slice");
                        expr = self
                            .ctx
                            .b
                            .call_expr_callee(slice_callee, [Arg::Num(from as f64)]);
                    }
                }
                if let Some(default) = step.default {
                    expr = self.build_fallback(expr, default);
                }
            }

            if v.is_rest {
                expr = self.exclude_from_object(expr, v.excluded);
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
        let to_array = match count {
            Some(count) => self
                .ctx
                .b
                .call_expr("$.to_array", [Arg::Expr(array_expr), Arg::Num(count as f64)]),
            None => self.ctx.b.call_expr("$.to_array", [Arg::Expr(array_expr)]),
        };
        let derived = self
            .ctx
            .b
            .call_expr("$.derived", [Arg::Expr(self.ctx.b.thunk(to_array))]);
        carrier_stmts.push(self.ctx.b.var_stmt(&name, derived));
        carriers.insert(prefix.to_string(), name.clone());
        name
    }

    fn member_from_key(
        &self,
        object: Expression<'a>,
        key: &'a PropertyKey<'a>,
        computed: bool,
    ) -> Expression<'a> {
        if !computed {
            match key {
                PropertyKey::StaticIdentifier(id) => {
                    return self.ctx.b.static_member_expr(object, id.name.as_str());
                }
                PropertyKey::StringLiteral(s) => {
                    return self
                        .ctx
                        .b
                        .computed_member_expr(object, self.ctx.b.str_expr(s.value.as_str()));
                }
                PropertyKey::NumericLiteral(n) => {
                    return self
                        .ctx
                        .b
                        .computed_member_expr(object, self.ctx.b.num_expr(n.value));
                }
                _ => {}
            }
        }
        let key_expr = key
            .as_expression()
            .map(|e| e.clone_in(self.ctx.b.ast.allocator))
            .unwrap_or_else(|| self.ctx.b.void_zero_expr());
        self.ctx.b.computed_member_expr(object, key_expr)
    }

    fn exclude_from_object(
        &self,
        object: Expression<'a>,
        excluded: &[&'a PropertyKey<'a>],
    ) -> Expression<'a> {
        let keys: Vec<Expression<'a>> = excluded
            .iter()
            .filter_map(|key| match key {
                PropertyKey::StaticIdentifier(id) => Some(self.ctx.b.str_expr(id.name.as_str())),
                PropertyKey::StringLiteral(s) => Some(self.ctx.b.str_expr(s.value.as_str())),
                PropertyKey::NumericLiteral(n) => {
                    Some(self.ctx.b.str_expr(&format_numeric_key(n.value)))
                }
                _ => key.as_expression().map(|e| {
                    let cloned = e.clone_in(self.ctx.b.ast.allocator);
                    self.ctx.b.call_expr("String", [Arg::Expr(cloned)])
                }),
            })
            .collect();
        let excluded_array = self.ctx.b.array_expr(keys);
        self.ctx
            .b
            .call_expr("$.exclude_from_object", [Arg::Expr(object), Arg::Expr(excluded_array)])
    }

    fn build_fallback(&self, expr: Expression<'a>, default: &'a Expression<'a>) -> Expression<'a> {
        let default = default.clone_in(self.ctx.b.ast.allocator);
        if is_simple_expression(&default) {
            self.ctx
                .b
                .call_expr("$.fallback", [Arg::Expr(expr), Arg::Expr(default)])
        } else {
            let thunk = self.ctx.b.thunk(default);
            self.ctx.b.call_expr(
                "$.fallback",
                [Arg::Expr(expr), Arg::Expr(thunk), Arg::Bool(true)],
            )
        }
    }

    fn serialize_key(&self, key: &'a PropertyKey<'a>, computed: bool) -> String {
        match key {
            PropertyKey::StaticIdentifier(id) if !computed => format!(".{}", id.name),
            PropertyKey::StringLiteral(s) => format!(".{:?}", s.value),
            PropertyKey::NumericLiteral(n) => format!(".#{}", n.value),
            _ => {
                let span = key.span();
                format!(".[{}:{}]", span.start, span.end)
            }
        }
    }
}

fn carrier_count(len: u32, has_rest: bool) -> Option<u32> {
    if has_rest { None } else { Some(len) }
}

fn format_numeric_key(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

fn is_simple_expression(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::Identifier(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_) => true,
        Expression::ConditionalExpression(cond) => {
            is_simple_expression(&cond.test)
                && is_simple_expression(&cond.consequent)
                && is_simple_expression(&cond.alternate)
        }
        Expression::BinaryExpression(bin) => {
            is_simple_expression(&bin.left) && is_simple_expression(&bin.right)
        }
        Expression::LogicalExpression(log) => {
            is_simple_expression(&log.left) && is_simple_expression(&log.right)
        }
        _ => false,
    }
}
