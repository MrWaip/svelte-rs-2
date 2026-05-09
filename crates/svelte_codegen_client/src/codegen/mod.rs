mod anchor;
mod async_emit;
mod attributes;
mod blocks;
mod component_props;
mod concatenation;
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
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(crate) fn new(ctx: &'ctx mut Ctx<'a>) -> Self {
        Self {
            ctx,
            hoisted: Vec::new(),
            instance_snippets: Vec::new(),
            hoistable_snippets: Vec::new(),
        }
    }

    pub(in crate::codegen) fn hoist(&mut self, stmt: Statement<'a>) {
        self.hoisted.push(stmt);
    }


    pub(in crate::codegen) fn pack_body(
        &mut self,
        state: EmitState<'a>,
        anchor_ident: &str,
    ) -> Result<Vec<Statement<'a>>> {
        debug_assert!(
            state.pending_element_init.is_empty(),
            "pending_element_init not flushed before pack_body"
        );
        let EmitState {
            template: _,
            init,
            update,
            after_update,
            root_var,
            special_elements,
            memo_attrs,
            shared_memo,
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
            shared_memo,
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
    let root_fragment = ctx.query.component.root;
    let root_ctx = FragmentCtx::root(ctx, root_fragment);
    let mut cg = Codegen::new(ctx);
    let mut state = EmitState::new();

    cg.emit_fragment(&mut state, &root_ctx, root_fragment)?;

    cg.finalize(state)
}
