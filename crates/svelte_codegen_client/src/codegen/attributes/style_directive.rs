use oxc_ast::ast::{Expression, Statement};
use svelte_ast::{NodeId, StyleDirectiveValue};
use svelte_ast_builder::{Arg, AssignLeft, ObjProp};

use crate::context::Ctx;

use super::super::data_structures::EmitState;
use super::super::{Codegen, Result};

pub(super) struct StyleProps<'a> {
    pub normal: Vec<ObjProp<'a>>,
    pub important: Vec<ObjProp<'a>>,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn build_style_props(&mut self, owner_id: NodeId) -> Result<StyleProps<'a>> {
        let style_dirs = self.ctx.style_directives(owner_id).to_vec();
        let mut normal: Vec<ObjProp<'a>> = Vec::new();
        let mut important: Vec<ObjProp<'a>> = Vec::new();

        for sd in &style_dirs {
            let name = &sd.name;
            let prop = match &sd.value {
                StyleDirectiveValue::Expression => {
                    let parsed = self.take_attr_expr(sd.id, &sd.expression)?;
                    let same_name = sd.shorthand || self.ctx.is_expression_shorthand(sd.id);
                    self.ctx.b.directive_prop(name, parsed, same_name)
                }
                StyleDirectiveValue::String(s) => {
                    let name_alloc = self.ctx.b.alloc_str(name);
                    ObjProp::KeyValue(name_alloc, self.ctx.b.str_expr(s))
                }
                StyleDirectiveValue::Concatenation(parts) => {
                    let parts = parts.clone();
                    let name_alloc = self.ctx.b.alloc_str(name);
                    let expr = self.build_concat_expr_template(sd.id, &parts)?;
                    ObjProp::KeyValue(name_alloc, expr)
                }
            };
            if sd.important {
                important.push(prop);
            } else {
                normal.push(prop);
            }
        }

        Ok(StyleProps { normal, important })
    }

    pub(in super::super) fn emit_style_directives_aggregate(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_var: &str,
    ) -> Result<()> {
        if !self.ctx.has_style_directives(owner_id) {
            return Ok(());
        }

        let static_style = self.ctx.static_style(owner_id).unwrap_or("").to_string();
        let props = self.build_style_props(owner_id)?;

        let directives_expr = if props.important.is_empty() {
            self.ctx.b.object_expr(props.normal)
        } else {
            let normal_obj = self.ctx.b.object_expr(props.normal);
            let important_obj = self.ctx.b.object_expr(props.important);
            self.ctx
                .b
                .array_from_args([Arg::Expr(normal_obj), Arg::Expr(important_obj)])
        };

        emit_set_style_call(
            self.ctx,
            &mut state.init,
            &mut state.update,
            owner_var,
            static_style,
            directives_expr,
        );

        Ok(())
    }
}

fn emit_set_style_call<'a>(
    ctx: &mut Ctx<'a>,
    init: &mut Vec<Statement<'a>>,
    update: &mut Vec<Statement<'a>>,
    el_name: &str,
    static_style: String,
    directives_expr: Expression<'a>,
) {
    let styles_name = ctx.gen_ident("styles");

    let set_style_call = ctx.b.call_expr(
        "$.set_style",
        [
            Arg::Ident(el_name),
            Arg::Str(static_style),
            Arg::Ident(&styles_name),
            Arg::Expr(directives_expr),
        ],
    );

    let assign = ctx
        .b
        .assign_expr(AssignLeft::Ident(styles_name.clone()), set_style_call);

    init.push(ctx.b.let_stmt(&styles_name));
    update.push(ctx.b.expr_stmt(assign));
}
