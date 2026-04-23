use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{Attribute, NodeId};
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

use crate::context::Ctx;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_class_attribute_and_directives(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_var: &str,
    ) -> Result<()> {
        let has_class_attr = self.ctx.has_class_attribute(owner_id);
        let has_class_dirs = self.ctx.has_class_directives(owner_id);

        if !has_class_attr && !has_class_dirs {
            return Ok(());
        }

        let class_value = if has_class_attr {
            let Some(class_attr_id) = self.ctx.class_attr_id(owner_id) else {
                return CodegenError::unexpected_node(
                    owner_id,
                    "element with class attribute should have class_attr_id",
                );
            };
            self.build_class_attr_value(owner_id, class_attr_id)?
        } else {
            let static_class = self.ctx.static_class(owner_id).unwrap_or("").to_string();
            let hash = self.ctx.css_hash().to_string();
            if self.ctx.is_css_scoped(owner_id) && !hash.is_empty() {
                let combined = if static_class.is_empty() {
                    hash
                } else {
                    format!("{static_class} {hash}")
                };
                self.ctx.b.str_expr(&combined)
            } else {
                self.ctx.b.str_expr(&static_class)
            }
        };

        let directives_obj = if has_class_dirs {
            self.build_class_directives_object(owner_id)?
        } else {
            None
        };

        let has_state = self.ctx.class_needs_state(owner_id);

        let hash = self.ctx.css_hash().to_string();
        let scope_arg = if has_class_attr && self.ctx.is_css_scoped(owner_id) && !hash.is_empty() {
            self.ctx.b.str_expr(&hash)
        } else {
            self.ctx.b.null_expr()
        };

        emit_set_class_call(
            self.ctx,
            &mut state.init,
            &mut state.update,
            owner_var,
            class_value,
            scope_arg,
            directives_obj,
            has_state,
        );

        Ok(())
    }

    fn build_class_attr_value(
        &mut self,
        owner_id: NodeId,
        class_attr_id: NodeId,
    ) -> Result<Expression<'a>> {
        let el = self.ctx.element(owner_id);
        let attributes = el.attributes.clone();

        let Some(attr) = self
            .ctx
            .attr_index(owner_id)
            .and_then(|index| index.find_by_id(&attributes, class_attr_id))
        else {
            return CodegenError::unexpected_node(
                class_attr_id,
                "class attr id not found on element",
            );
        };

        match attr {
            Attribute::ExpressionAttribute(ea) => {
                let mut expr = self.take_attr_expr(class_attr_id, &ea.expression)?;
                let _ = ea;
                if self.ctx.needs_clsx(class_attr_id) {
                    expr = self.ctx.b.call_expr("$.clsx", [Arg::Expr(expr)]);
                }
                Ok(expr)
            }
            Attribute::ConcatenationAttribute(a) => {
                let parts = a.parts.clone();
                self.build_concat_expr_collapse_single(class_attr_id, &parts)
            }
            _ => CodegenError::unexpected_node(
                class_attr_id,
                "class_attr_id must reference ExpressionAttribute or ConcatenationAttribute",
            ),
        }
    }

    pub(in super::super) fn emit_svelte_element_class_directives(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_var: &str,
    ) -> Result<()> {
        let Some(dir_obj) = self.build_class_directives_object(owner_id)? else {
            return Ok(());
        };
        let call = self.ctx.b.call_stmt(
            "$.set_class",
            [
                Arg::Ident(owner_var),
                Arg::Num(0.0),
                Arg::StrRef(""),
                Arg::Expr(self.ctx.b.null_expr()),
                Arg::Expr(self.ctx.b.object_expr(Vec::<ObjProp<'a>>::new())),
                Arg::Expr(dir_obj),
            ],
        );
        state.init.push(call);
        Ok(())
    }

    pub(in super::super) fn build_class_directives_object(
        &mut self,
        owner_id: NodeId,
    ) -> Result<Option<Expression<'a>>> {
        let dir_snapshot: Vec<(NodeId, String, bool, oxc_syntax::node::NodeId)> =
            match self.ctx.query.view.class_directive_info(owner_id) {
                Some(dirs) => dirs
                    .iter()
                    .map(|cd| (cd.id, cd.name.clone(), cd.has_expression, cd.expr_id))
                    .collect(),
                None => return Ok(None),
            };

        let mut props: Vec<ObjProp<'a>> = Vec::new();
        for (id, name, has_expression, expr_id) in &dir_snapshot {
            let (expr, same_name) = if *has_expression {
                let Some(parsed) = self.ctx.state.parsed.take_expr(*expr_id) else {
                    return crate::codegen::CodegenError::missing_expression(*id);
                };
                let parsed = self.maybe_wrap_legacy_slots_read(parsed);
                (parsed, self.ctx.is_expression_shorthand(*id))
            } else {
                (self.ctx.b.rid_expr(name), true)
            };
            props.push(self.ctx.b.directive_prop(name, expr, same_name));
        }

        Ok(Some(self.ctx.b.object_expr(props)))
    }
}

fn emit_set_class_call<'a>(
    ctx: &mut Ctx<'a>,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    el_name: &str,
    class_value: Expression<'a>,
    scope_arg: Expression<'a>,
    directives_obj: Option<Expression<'a>>,
    has_state: bool,
) {
    if let Some(dir_obj) = directives_obj {
        if has_state {
            let classes_name = ctx.gen_ident("classes");
            let set_class_call = ctx.b.call_expr(
                "$.set_class",
                [
                    Arg::Ident(el_name),
                    Arg::Num(1.0),
                    Arg::Expr(class_value),
                    Arg::Expr(scope_arg),
                    Arg::Ident(&classes_name),
                    Arg::Expr(dir_obj),
                ],
            );
            let assign = ctx
                .b
                .assign_expr(AssignLeft::Ident(classes_name.clone()), set_class_call);
            init.push(ctx.b.let_stmt(&classes_name));
            update.push(ctx.b.expr_stmt(assign));
        } else {
            let set_class_call = ctx.b.call_expr(
                "$.set_class",
                [
                    Arg::Ident(el_name),
                    Arg::Num(1.0),
                    Arg::Expr(class_value),
                    Arg::Expr(scope_arg),
                    Arg::Expr(ctx.b.object_expr(vec![])),
                    Arg::Expr(dir_obj),
                ],
            );
            init.push(ctx.b.expr_stmt(set_class_call));
        }
    } else {
        let set_class_call = ctx.b.call_expr(
            "$.set_class",
            [Arg::Ident(el_name), Arg::Num(1.0), Arg::Expr(class_value)],
        );
        let target = if has_state { &mut *update } else { &mut *init };
        target.push(ctx.b.expr_stmt(set_class_call));
    }
}
