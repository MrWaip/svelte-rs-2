mod getter_setter;
mod group;
mod placement;
mod this;

use std::iter;

use svelte_analyze::{
    AttributeSemantics, BindPropertyKind, ElementBindPropertyKind, HtmlBindKind, MediaBindKind,
};
use svelte_ast::{BindDirective, NodeId};
use svelte_ast_builder::Arg;

use super::super::data_structures::EmitState;
use super::super::{Codegen, CodegenError, Result};

use placement::BindPlacement;

fn property_to_bind_kind(p: ElementBindPropertyKind) -> BindPropertyKind {
    match p {
        ElementBindPropertyKind::Value => BindPropertyKind::Value,
        ElementBindPropertyKind::Checked => BindPropertyKind::Checked,
        ElementBindPropertyKind::Group => BindPropertyKind::Group,
        ElementBindPropertyKind::Files => BindPropertyKind::Files,
        ElementBindPropertyKind::Indeterminate => BindPropertyKind::Indeterminate,
        ElementBindPropertyKind::Open => BindPropertyKind::Open,
        ElementBindPropertyKind::This => BindPropertyKind::This,
        ElementBindPropertyKind::ContentEditable(k) => BindPropertyKind::ContentEditable(k),
        ElementBindPropertyKind::ElementSize(k) => BindPropertyKind::ElementSize(k),
        ElementBindPropertyKind::ResizeObserver(k) => BindPropertyKind::ResizeObserver(k),
        ElementBindPropertyKind::Media(k) => BindPropertyKind::Media(k),
        ElementBindPropertyKind::ImageNaturalSize(k) => BindPropertyKind::ImageNaturalSize(k),
        ElementBindPropertyKind::Focused => BindPropertyKind::Focused,
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(in super::super) fn emit_bind_directive(
        &mut self,
        state: &mut EmitState<'a>,
        owner_id: NodeId,
        owner_tag: &str,
        owner_var: &str,
        bind: &BindDirective,
    ) -> Result<()> {
        let AttributeSemantics::ElementBind(payload) =
            self.ctx.query.analysis.attributes.get(bind.id)
        else {
            return CodegenError::semantic_mismatch(
                bind.id,
                "emit_bind_directive requires ElementBind",
            );
        };
        let payload = payload.clone();

        let bind_property = property_to_bind_kind(payload.property);

        let has_use = self.ctx.has_use_directive(owner_id);
        let bind_blockers = payload.blockers.to_vec();

        let placement = if matches!(payload.kind, HtmlBindKind::BindableProp)
            && let Some(p) =
                self.emit_bind_bindable_prop_shorthand(bind, owner_var, has_use, &bind_blockers)?
        {
            Some(p)
        } else {
            self.gen_bind_placement(bind, bind_property, owner_var, owner_tag, has_use)?
        };

        let Some(placement) = placement else {
            return Ok(());
        };

        match placement {
            BindPlacement::AfterUpdate(stmt) => state.after_update.push(stmt),
            BindPlacement::Init(stmt) => state.pending_element_init.push(stmt),
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
        let payload = match self.ctx.query.analysis.attributes.get(bind.id) {
            AttributeSemantics::ElementBind(b) => b.clone(),
            _ => return Ok(None),
        };
        let bind_blockers = payload.blockers.to_vec();
        let is_this = matches!(payload.property, ElementBindPropertyKind::This);

        if !is_this
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

    fn emit_bind_bindable_prop_shorthand(
        &mut self,
        bind: &BindDirective,
        el_name: &str,
        has_use_directive: bool,
        bind_blockers: &[u32],
    ) -> Result<Option<BindPlacement<'a>>> {
        let payload = match self.ctx.query.analysis.attributes.get(bind.id) {
            AttributeSemantics::ElementBind(b) => b.clone(),
            _ => return Ok(None),
        };

        let var_name = if bind.shorthand {
            bind.name.clone()
        } else {
            self.ctx
                .query
                .component
                .source_text(bind.expression.span)
                .to_string()
        };
        let var_alloc = self.ctx.b.alloc_str(&var_name);

        let stmt = match payload.property {
            ElementBindPropertyKind::Value => self
                .ctx
                .b
                .call_stmt("$.bind_value", [Arg::Ident(el_name), Arg::Ident(var_alloc)]),
            ElementBindPropertyKind::Checked => self.ctx.b.call_stmt(
                "$.bind_checked",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Files => self
                .ctx
                .b
                .call_stmt("$.bind_files", [Arg::Ident(el_name), Arg::Ident(var_alloc)]),
            ElementBindPropertyKind::ElementSize(kind) => self.ctx.b.call_stmt(
                "$.bind_element_size",
                [
                    Arg::Ident(el_name),
                    Arg::StrRef(kind.name()),
                    Arg::Ident(var_alloc),
                ],
            ),
            ElementBindPropertyKind::ResizeObserver(kind) => self.ctx.b.call_stmt(
                "$.bind_resize_observer",
                [
                    Arg::Ident(el_name),
                    Arg::StrRef(kind.name()),
                    Arg::Ident(var_alloc),
                ],
            ),
            ElementBindPropertyKind::Focused => self.ctx.b.call_stmt(
                "$.bind_focused",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::ContentEditable(kind) => self.ctx.b.call_stmt(
                "$.bind_content_editable",
                [
                    Arg::StrRef(kind.name()),
                    Arg::Ident(el_name),
                    Arg::Ident(var_alloc),
                ],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::CurrentTime) => self.ctx.b.call_stmt(
                "$.bind_current_time",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::PlaybackRate) => self.ctx.b.call_stmt(
                "$.bind_playback_rate",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::Paused) => self.ctx.b.call_stmt(
                "$.bind_paused",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::Volume) => self.ctx.b.call_stmt(
                "$.bind_volume",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::Muted) => self
                .ctx
                .b
                .call_stmt("$.bind_muted", [Arg::Ident(el_name), Arg::Ident(var_alloc)]),
            ElementBindPropertyKind::Media(MediaBindKind::Buffered) => self.ctx.b.call_stmt(
                "$.bind_buffered",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::Seekable) => self.ctx.b.call_stmt(
                "$.bind_seekable",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::Seeking) => self.ctx.b.call_stmt(
                "$.bind_seeking",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::Ended) => self
                .ctx
                .b
                .call_stmt("$.bind_ended", [Arg::Ident(el_name), Arg::Ident(var_alloc)]),
            ElementBindPropertyKind::Media(MediaBindKind::ReadyState) => self.ctx.b.call_stmt(
                "$.bind_ready_state",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Media(MediaBindKind::Played) => self.ctx.b.call_stmt(
                "$.bind_played",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
            ElementBindPropertyKind::Group => {
                let has_value_attr = matches!(
                    self.ctx.query.analysis.attributes.get(bind.id),
                    AttributeSemantics::ElementBind(b) if b.group_value_attr.is_some()
                );
                let getter = if has_value_attr {
                    let call = self
                        .ctx
                        .b
                        .call_expr_callee(self.ctx.b.rid_expr(var_alloc), iter::empty::<Arg>());
                    self.ctx
                        .b
                        .arrow_expr(self.ctx.b.no_params(), [self.ctx.b.expr_stmt(call)])
                } else {
                    self.ctx.b.rid_expr(var_alloc)
                };
                let setter = self.ctx.b.rid_expr(var_alloc);
                self.emit_bind_group(bind, el_name, getter, setter)?
            }
            _ => return Ok(None),
        };

        let _ = self.take_expr_by_ref(&bind.expression);
        let stmt = self.wrap_use_and_blockers(stmt, has_use_directive, bind_blockers);
        if has_use_directive {
            Ok(Some(BindPlacement::Init(stmt)))
        } else {
            Ok(Some(BindPlacement::AfterUpdate(stmt)))
        }
    }
}
