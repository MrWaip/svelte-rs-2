mod getter_setter;
mod group;
mod placement;
mod this;

use std::iter;

use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{
    AttributeSemantics, BindPropertyKind, ElementBindPropertyKind, ElementBindSemantics,
    GroupBindValue, HtmlBindKind, ImageNaturalSizeKind, MediaBindKind,
};
use svelte_ast::{BindDirective, NodeId};
use svelte_ast_builder::Arg;
use svelte_component_semantics::SymbolId;

use super::super::data_structures::EmitState;
use super::super::dev::getter_return_member;
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

        self.maybe_emit_element_validate_binding(state, bind, &payload);

        let bind_property = property_to_bind_kind(payload.property);

        let has_use = self.ctx.has_use_directive(owner_id);
        let bind_blockers = payload.blockers.to_vec();

        let placement = if matches!(payload.kind, HtmlBindKind::BindableProp)
            && !payload.property.is_this()
            && let Some(p) = self.emit_bind_bindable_prop_shorthand(
                bind,
                owner_var,
                owner_tag,
                has_use,
                &bind_blockers,
            )? {
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

    fn maybe_emit_element_validate_binding(
        &mut self,
        state: &mut EmitState<'a>,
        bind: &BindDirective,
        payload: &ElementBindSemantics,
    ) {
        if !self.ctx.state.dev || !self.ctx.query.runes() {
            return;
        }
        if !payload.needs_binding_validation {
            return;
        }
        if self.ctx.binding_property_non_reactive_ignored(bind.id) {
            return;
        }
        let stmt = {
            let Some(seq) = self.ctx.state.parsed.expr(bind.expression.id()) else {
                return;
            };
            let Expression::SequenceExpression(seq) = seq.get_inner_expression() else {
                return;
            };
            let Some(member) = seq.expressions.first().and_then(getter_return_member) else {
                return;
            };
            self.build_validate_binding_from_member(bind, member)
        };
        if let Some(stmt) = stmt {
            state.pending_element_init.push(stmt);
        }
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

        if payload.property.is_this() {
            return self.emit_bind_this(bind, el_name, tag_name);
        }

        let stmt = match payload.kind {
            HtmlBindKind::EachItemWriteLegacy { symbol } => self.build_each_item_write_bind_stmt(
                bind,
                bind_property,
                el_name,
                tag_name,
                symbol,
            )?,
            HtmlBindKind::Plain
            | HtmlBindKind::Rune
            | HtmlBindKind::LegacyState
            | HtmlBindKind::BindableProp
            | HtmlBindKind::StoreSubscribed { .. } => {
                self.try_build_bind_get_set_stmt(bind, bind_property, el_name, tag_name)?
            }
        };
        let Some(stmt) = stmt else {
            return CodegenError::unexpected_node(
                bind.id,
                "bind without getter/setter must be bind:this",
            );
        };
        let stmt = self.wrap_use_and_blockers(stmt, has_use_directive, &bind_blockers);
        if has_use_directive {
            Ok(Some(BindPlacement::Init(stmt)))
        } else {
            Ok(Some(BindPlacement::AfterUpdate(stmt)))
        }
    }

    fn build_each_item_write_bind_stmt(
        &mut self,
        bind: &BindDirective,
        bind_property: BindPropertyKind,
        el_name: &str,
        tag_name: &str,
        symbol: SymbolId,
    ) -> Result<Option<Statement<'a>>> {
        let Some(setter_body) = self.build_each_item_writeback_legacy(symbol) else {
            return Ok(None);
        };
        let name = self
            .ctx
            .b
            .alloc_str(self.ctx.query.view.symbol_name(symbol));
        let reads_via_signal = !self
            .ctx
            .query
            .view
            .binding_semantics(symbol)
            .reads_via_each_item_accessor();
        let getter = if reads_via_signal {
            let body = self.ctx.b.call_expr("$.get", [Arg::Ident(name)]);
            if self.ctx.state.dev {
                self.ctx.b.named_function_expr(
                    "get",
                    self.ctx.b.no_params(),
                    vec![self.ctx.b.return_stmt(body)],
                    false,
                )
            } else {
                self.ctx.b.thunk(body)
            }
        } else if self.ctx.state.dev {
            let call = self
                .ctx
                .b
                .call_expr_callee(self.ctx.b.rid_expr(name), iter::empty::<Arg<'_, '_>>());
            self.ctx.b.named_function_expr(
                "get",
                self.ctx.b.no_params(),
                vec![self.ctx.b.return_stmt(call)],
                false,
            )
        } else {
            self.ctx.b.rid_expr(name)
        };
        let setter_stmt = self.ctx.b.expr_stmt(setter_body);
        let setter = if self.ctx.state.dev {
            self.ctx.b.named_function_expr(
                "set",
                self.ctx.b.params(["$$value"]),
                vec![setter_stmt],
                false,
            )
        } else {
            self.ctx
                .b
                .arrow_expr(self.ctx.b.params(["$$value"]), [setter_stmt])
        };
        self.build_bind_call_stmt(bind, bind_property, el_name, tag_name, getter, setter)
            .map(Some)
    }

    fn emit_bind_bindable_prop_shorthand(
        &mut self,
        bind: &BindDirective,
        el_name: &str,
        tag_name: &str,
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

        if self.ctx.state.dev {
            let getter = self.ctx.b.named_function_expr(
                "get",
                self.ctx.b.no_params(),
                vec![self.ctx.b.return_stmt(self.ctx.b.call_expr_callee(
                    self.ctx.b.rid_expr(var_alloc),
                    iter::empty::<Arg<'_, '_>>(),
                ))],
                false,
            );
            let setter =
                self.ctx.b.named_function_expr(
                    "set",
                    self.ctx.b.params(["$$value"]),
                    vec![self.ctx.b.expr_stmt(self.ctx.b.call_expr_callee(
                        self.ctx.b.rid_expr(var_alloc),
                        [Arg::Ident("$$value")],
                    ))],
                    false,
                );
            let bind_property = property_to_bind_kind(payload.property);
            let stmt =
                self.build_bind_call_stmt(bind, bind_property, el_name, tag_name, getter, setter)?;
            let stmt = self.wrap_use_and_blockers(stmt, has_use_directive, bind_blockers);
            return Ok(Some(if has_use_directive {
                BindPlacement::Init(stmt)
            } else {
                BindPlacement::AfterUpdate(stmt)
            }));
        }

        let stmt = match payload.property {
            ElementBindPropertyKind::Value if tag_name == "select" => self.ctx.b.call_stmt(
                "$.bind_select_value",
                [Arg::Ident(el_name), Arg::Ident(var_alloc)],
            ),
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
                let has_value_attr = match self.ctx.query.analysis.attributes.get(bind.id) {
                    AttributeSemantics::ElementBind(b) => match b.group_value {
                        Some(GroupBindValue::Expression { .. }) => true,
                        Some(GroupBindValue::Static { .. }) | None => false,
                    },
                    _ => false,
                };
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
            ElementBindPropertyKind::Open => self.ctx.b.bind_property_call_stmt(
                "open",
                "toggle",
                el_name,
                self.ctx.b.rid_expr(var_alloc),
                Some(self.ctx.b.rid_expr(var_alloc)),
            ),
            ElementBindPropertyKind::Indeterminate => self.ctx.b.bind_property_call_stmt(
                "indeterminate",
                "change",
                el_name,
                self.ctx.b.rid_expr(var_alloc),
                Some(self.ctx.b.rid_expr(var_alloc)),
            ),
            ElementBindPropertyKind::Media(MediaBindKind::Duration) => {
                self.ctx.b.bind_property_call_stmt(
                    "duration",
                    "durationchange",
                    el_name,
                    self.ctx.b.rid_expr(var_alloc),
                    None,
                )
            }
            ElementBindPropertyKind::Media(MediaBindKind::VideoWidth) => {
                self.ctx.b.bind_property_call_stmt(
                    "videoWidth",
                    "resize",
                    el_name,
                    self.ctx.b.rid_expr(var_alloc),
                    None,
                )
            }
            ElementBindPropertyKind::Media(MediaBindKind::VideoHeight) => {
                self.ctx.b.bind_property_call_stmt(
                    "videoHeight",
                    "resize",
                    el_name,
                    self.ctx.b.rid_expr(var_alloc),
                    None,
                )
            }
            ElementBindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalWidth) => {
                self.ctx.b.bind_property_call_stmt(
                    "naturalWidth",
                    "load",
                    el_name,
                    self.ctx.b.rid_expr(var_alloc),
                    None,
                )
            }
            ElementBindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalHeight) => {
                self.ctx.b.bind_property_call_stmt(
                    "naturalHeight",
                    "load",
                    el_name,
                    self.ctx.b.rid_expr(var_alloc),
                    None,
                )
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
