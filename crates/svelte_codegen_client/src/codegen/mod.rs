mod anchor;
mod async_plan;
mod attributes;
mod blocks;
mod component_props;
mod containers;
pub(in crate::codegen) mod data_structures;
mod dev;
mod effect;
mod expr;
mod fragment;
mod hoisted;
mod let_directive_legacy;
mod namespace;

use oxc_ast::ast::Statement;
use svelte_ast::NodeId;
use svelte_ast_builder::Arg;

use crate::context::Ctx;

pub(crate) use data_structures::{CodegenError, CodegenResult, Result};
pub(crate) use fragment::FragmentEmitKind;

use data_structures::{EmitState, FragmentCtx};

pub(crate) struct Codegen<'a, 'ctx> {
    ctx: &'ctx mut Ctx<'a>,
    hoisted: Vec<Statement<'a>>,
    instance_snippets: Vec<Statement<'a>>,
    hoistable_snippets: Vec<Statement<'a>>,
    snippet_depth: u32,
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(crate) fn new(ctx: &'ctx mut Ctx<'a>) -> Self {
        Self {
            ctx,
            hoisted: Vec::new(),
            instance_snippets: Vec::new(),
            hoistable_snippets: Vec::new(),
            snippet_depth: 0,
        }
    }

    pub(in crate::codegen) fn hoist(&mut self, stmt: Statement<'a>) {
        self.hoisted.push(stmt);
    }

    pub(in crate::codegen) fn enter_snippet_build(&mut self) {
        self.snippet_depth += 1;
    }

    pub(in crate::codegen) fn exit_snippet_build(&mut self) {
        self.snippet_depth -= 1;
    }

    pub(in crate::codegen) fn try_resolve_known_from_expr(
        &self,
        expr: &oxc_ast::ast::Expression<'_>,
    ) -> Option<String> {
        if let oxc_ast::ast::Expression::Identifier(ident) = expr {
            return self
                .ctx
                .query
                .view
                .known_value(ident.name.as_str())
                .map(|s| s.to_string());
        }
        None
    }

    pub(in crate::codegen) fn is_node_expr_definitely_defined(
        &self,
        nid: NodeId,
        expr: &oxc_ast::ast::Expression<'_>,
    ) -> bool {
        if !matches!(expr, oxc_ast::ast::Expression::Identifier(_)) {
            return false;
        }
        let Some(info) = self.ctx.expression(nid) else {
            return false;
        };
        if !info.is_identifier() {
            return false;
        }
        if info.ref_symbols().len() != 1 {
            return false;
        }
        self.ctx.is_each_index_sym(info.ref_symbols()[0])
    }

    pub(in crate::codegen) fn pack_body(
        &mut self,
        state: EmitState<'a>,
        anchor_ident: &str,
    ) -> Result<Vec<Statement<'a>>> {
        let EmitState {
            template: _,
            init,
            update,
            after_update,
            root_var,
            special_elements,
            memo_attrs,
            script_blockers,
            extra_blockers,
            ..
        } = state;

        let mut body =
            Vec::with_capacity(init.len() + after_update.len() + special_elements.len() + 2);
        body.extend(init);

        effect::emit_template_effect_with_memo(
            self.ctx,
            &mut body,
            update,
            memo_attrs,
            script_blockers,
            extra_blockers,
        )?;

        body.extend(after_update);
        body.extend(special_elements);

        if let Some(name) = root_var {
            body.push(
                self.ctx
                    .state
                    .b
                    .call_stmt("$.append", [Arg::Ident(anchor_ident), Arg::Ident(&name)]),
            );
        }

        Ok(body)
    }

    pub(in crate::codegen) fn pack_callback_body(
        &mut self,
        state: EmitState<'a>,
        anchor_ident: &str,
    ) -> Result<Vec<Statement<'a>>> {
        self.pack_body(state, anchor_ident)
    }

    fn finalize(mut self, state: EmitState<'a>) -> Result<CodegenResult<'a>> {
        let body = self.pack_body(state, "$$anchor")?;
        Ok(CodegenResult {
            hoisted: self.hoisted,
            body,
            instance_snippets: self.instance_snippets,
            hoistable_snippets: self.hoistable_snippets,
        })
    }
}

pub(crate) fn codegen_root_fragment<'a>(ctx: &mut Ctx<'a>) -> Result<CodegenResult<'a>> {
    let root_fragment = &ctx.query.component.fragment;
    let root_ctx = FragmentCtx::root(ctx, root_fragment);
    let mut cg = Codegen::new(ctx);
    let mut state = EmitState::new();

    cg.emit_fragment(&mut state, &root_ctx, root_fragment)?;

    cg.finalize(state)
}
