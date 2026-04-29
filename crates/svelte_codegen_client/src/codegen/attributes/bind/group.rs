use oxc_ast::ast::{BinaryOperator, Expression, Statement};
use svelte_ast::BindDirective;
use svelte_ast_builder::{Arg, AssignLeft};

use super::super::super::data_structures::EmitState;
use super::super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn emit_bind_group(
        &mut self,
        bind: &BindDirective,
        el_name: &str,
        get_fn: Expression<'a>,
        set_fn: Expression<'a>,
    ) -> Result<Statement<'a>> {
        self.ctx.state.needs_binding_group = true;

        let parent_eaches = self.ctx.parent_each_blocks(bind.id);
        let index_array = if parent_eaches.is_empty() {
            self.ctx.b.empty_array_expr()
        } else {
            let mut indexes: Vec<Expression<'a>> = Vec::with_capacity(parent_eaches.len());
            for each_id in &parent_eaches {
                let Some(idx_name) = self
                    .ctx
                    .each_index_name(*each_id)
                    .or_else(|| self.ctx.state.group_index_names.get(each_id).cloned())
                else {
                    return CodegenError::unexpected_node(
                        *each_id,
                        "missing index name for bind:group parent each block",
                    );
                };
                indexes.push(self.ctx.b.rid_expr(&idx_name));
            }
            self.ctx.b.array_expr(indexes)
        };

        let getter = if let Some(val_attr_id) = self.ctx.bind_group_value_attr(bind.id) {
            let val_expr = {
                let store = &self.ctx.query.component.store;
                let mut found_id: Option<oxc_syntax::node::NodeId> = None;
                for n in store.iter_nodes() {
                    let attrs: &[svelte_ast::Attribute] = match n {
                        svelte_ast::Node::Element(el) => &el.attributes,
                        svelte_ast::Node::SvelteElement(el) => &el.attributes,
                        svelte_ast::Node::ComponentNode(cn) => &cn.attributes,
                        _ => continue,
                    };
                    for a in attrs {
                        if a.id() == val_attr_id {
                            if let svelte_ast::Attribute::ExpressionAttribute(ea) = a {
                                found_id = Some(ea.expression.id());
                            }
                            break;
                        }
                    }
                    if found_id.is_some() {
                        break;
                    }
                }
                found_id
                    .and_then(|id| self.ctx.state.parsed.expr(id))
                    .map(|expr| self.ctx.b.clone_expr(expr))
                    .unwrap_or_else(|| self.ctx.b.str_expr(""))
            };
            let val_stmt = self.ctx.b.expr_stmt(val_expr);

            let Some(body_expr) =
                svelte_ast_builder::Builder::try_extract_expression_stmt_expr(get_fn)
            else {
                return CodegenError::unexpected_node(
                    bind.id,
                    "bind:group getter is not ArrowFunctionExpression with ExpressionStatement body",
                );
            };
            let return_stmt = self.ctx.b.return_stmt(body_expr);
            self.ctx
                .b
                .arrow_block_expr(self.ctx.b.no_params(), vec![val_stmt, return_stmt])
        } else {
            get_fn
        };

        Ok(self.ctx.b.call_stmt(
            "$.bind_group",
            [
                Arg::Ident("binding_group"),
                Arg::Expr(index_array),
                Arg::Ident(el_name),
                Arg::Expr(getter),
                Arg::Expr(set_fn),
            ],
        ))
    }

    pub(in super::super) fn emit_bind_group_static_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val: &str,
    ) {
        let val_expr = self.ctx.b.str_expr(val);
        let dunder_value_assign = self.ctx.b.assign_expr(
            AssignLeft::StaticMember(
                self.ctx
                    .b
                    .static_member(self.ctx.b.rid_expr(el_name), "__value"),
            ),
            val_expr,
        );
        let value_assign = self.ctx.b.assign_stmt(
            AssignLeft::StaticMember(
                self.ctx
                    .b
                    .static_member(self.ctx.b.rid_expr(el_name), "value"),
            ),
            dunder_value_assign,
        );
        state.init.push(value_assign);
    }

    pub(in super::super) fn emit_bind_group_value(
        &mut self,
        state: &mut EmitState<'a>,
        el_name: &str,
        val_expr: Expression<'a>,
    ) {
        let mut prefix = String::with_capacity(el_name.len() + 6);
        prefix.push_str(el_name);
        prefix.push_str("_value");
        let cache_name = self.ctx.state.gen_ident(&prefix);

        state.init.push(self.ctx.b.var_uninit_stmt(&cache_name));

        let val_expr2 = self.ctx.b.clone_expr(&val_expr);

        let cache_assign = self
            .ctx
            .b
            .assign_expr(AssignLeft::Ident(cache_name.clone()), val_expr);

        let test = self.ctx.b.ast.expression_binary(
            oxc_span::SPAN,
            self.ctx.b.rid_expr(&cache_name),
            BinaryOperator::StrictInequality,
            cache_assign,
        );

        let dunder_value_assign = self.ctx.b.assign_expr(
            AssignLeft::StaticMember(
                self.ctx
                    .b
                    .static_member(self.ctx.b.rid_expr(el_name), "__value"),
            ),
            val_expr2,
        );

        let coalesced = self
            .ctx
            .b
            .logical_coalesce(dunder_value_assign, self.ctx.b.str_expr(""));

        let value_assign = self.ctx.b.assign_stmt(
            AssignLeft::StaticMember(
                self.ctx
                    .b
                    .static_member(self.ctx.b.rid_expr(el_name), "value"),
            ),
            coalesced,
        );

        let if_body = self.ctx.b.block_stmt(vec![value_assign]);
        let if_stmt = self.ctx.b.if_stmt(test, if_body, None);

        let effect_fn = self
            .ctx
            .b
            .arrow_block_expr(self.ctx.b.no_params(), vec![if_stmt]);
        let effect_call = self
            .ctx
            .b
            .call_stmt("$.template_effect", [Arg::Expr(effect_fn)]);
        state.init.push(effect_call);
    }
}
