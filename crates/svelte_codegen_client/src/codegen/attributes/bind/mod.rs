mod getter_setter;
mod group;
mod placement;
mod this;

use svelte_analyze::{BindPropertyKind, PropReferenceSemantics, ReferenceSemantics};
use svelte_ast::{BindDirective, NodeId};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};

use placement::BindPlacement;

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_bind_directive(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        bind: &BindDirective,
    ) -> Result<()> {
        let Some(semantics) = self.ctx.bind_target_semantics(bind.id) else {
            return Ok(());
        };

        if semantics.property() == BindPropertyKind::Value
            && owner_tag == "textarea"
            && !self.ctx.needs_textarea_value_lowering(owner_id)
        {
            state.init.push(
                self.ctx
                    .b
                    .call_stmt("$.remove_textarea_child", [Arg::Ident(owner_var)]),
            );
        }

        let has_use = self.ctx.has_use_directive(owner_id);
        let bind_property = semantics.property();
        let bind_blockers = self.ctx.bind_blockers(bind.id).to_vec();

        let placement = if !semantics.is_this()
            && matches!(bind_property, BindPropertyKind::Checked)
        {
            let bind_attr = svelte_ast::Attribute::BindDirective(bind.clone());
            let is_bindable_prop_source = matches!(
                self.ctx.directive_root_reference_semantics(&bind_attr),
                ReferenceSemantics::PropRead(PropReferenceSemantics::Source { bindable: true, .. })
                    | ReferenceSemantics::PropMutation { bindable: true, .. }
            );
            if is_bindable_prop_source {
                self.emit_bind_checked_shorthand(bind, owner_var, has_use, &bind_blockers)?
            } else {
                self.gen_bind_placement(bind, bind_property, owner_var, owner_tag, has_use)?
            }
        } else {
            self.gen_bind_placement(bind, bind_property, owner_var, owner_tag, has_use)?
        };

        let Some(placement) = placement else {
            return Ok(());
        };

        match placement {
            BindPlacement::AfterUpdate(stmt) => state.after_update.push(stmt),
            BindPlacement::Init(stmt) => {
                if semantics.is_this() {
                    state.pending_bind_this.push(stmt);
                } else {
                    state.init.push(stmt);
                }
            }
        }
        Ok(())
    }

    fn gen_bind_placement(
        &mut self,
        bind: &BindDirective,
        bind_property: BindPropertyKind,
        el_name: &str,
        tag_name: &str,
        has_use_directive: bool,
    ) -> Result<Option<BindPlacement<'a>>> {
        let bind_blockers = self.ctx.bind_blockers(bind.id).to_vec();
        let semantics = match self.ctx.bind_target_semantics(bind.id) {
            Some(s) => s,
            None => return Ok(None),
        };

        if !semantics.is_this()
            && let Some(stmt) =
                self.try_build_bind_get_set_stmt(bind, bind_property, el_name, tag_name)?
        {
            let stmt = self.wrap_use_and_blockers(stmt, has_use_directive, &bind_blockers);
            if has_use_directive {
                return Ok(Some(BindPlacement::Init(stmt)));
            }
            return Ok(Some(BindPlacement::AfterUpdate(stmt)));
        }

        if !matches!(bind_property, BindPropertyKind::This) {
            return CodegenError::unexpected_node(
                bind.id,
                "bind without getter/setter must be bind:this",
            );
        }

        self.emit_bind_this(bind, el_name, tag_name)
    }

    fn emit_bind_checked_shorthand(
        &mut self,
        bind: &BindDirective,
        el_name: &str,
        has_use_directive: bool,
        bind_blockers: &[u32],
    ) -> Result<Option<BindPlacement<'a>>> {
        let var_name = if bind.shorthand {
            bind.name.clone()
        } else {
            self.ctx
                .query
                .component
                .source_text(bind.expression.span)
                .to_string()
        };
        let _ = self.take_expr_by_ref(&bind.expression);

        let var_alloc = self.ctx.b.alloc_str(&var_name);
        let mut stmt = self.ctx.b.call_stmt(
            "$.bind_checked",
            [Arg::Ident(el_name), Arg::Ident(var_alloc)],
        );
        stmt = self.wrap_use_and_blockers(stmt, has_use_directive, bind_blockers);
        if has_use_directive {
            Ok(Some(BindPlacement::Init(stmt)))
        } else {
            Ok(Some(BindPlacement::AfterUpdate(stmt)))
        }
    }
}
