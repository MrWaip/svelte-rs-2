use oxc_ast::ast::{Expression, Statement};
use svelte_analyze::{BindPropertyKind, ImageNaturalSizeKind, MediaBindKind};
use svelte_ast::{BindDirective, NodeId};
use svelte_ast_builder::Arg;

use super::super::super::{Codegen, CodegenError, Result};

impl<'a, 'ctx> Codegen<'a, 'ctx> {
    pub(super) fn try_build_bind_get_set_stmt(
        &mut self,
        bind: &BindDirective,
        bind_property: BindPropertyKind,
        el_name: &str,
        tag_name: &str,
    ) -> Result<Option<Statement<'a>>> {
        let Some((get_fn, set_fn)) = self.take_bind_getter_setter(bind.id)? else {
            return Ok(None);
        };

        let ctx = &mut *self.ctx;
        let stmt = match bind_property {
            BindPropertyKind::Value if tag_name == "select" => ctx.b.call_stmt(
                "$.bind_select_value",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Value => ctx.b.call_stmt(
                "$.bind_value",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Checked => ctx.b.call_stmt(
                "$.bind_checked",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Files => ctx.b.call_stmt(
                "$.bind_files",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Indeterminate => ctx.b.bind_property_call_stmt(
                "indeterminate",
                "change",
                el_name,
                set_fn,
                Some(get_fn),
            ),
            BindPropertyKind::Open => {
                ctx.b
                    .bind_property_call_stmt("open", "toggle", el_name, set_fn, Some(get_fn))
            }
            BindPropertyKind::ContentEditable(kind) => ctx.b.call_stmt(
                "$.bind_content_editable",
                [
                    Arg::StrRef(kind.name()),
                    Arg::Ident(el_name),
                    Arg::Expr(get_fn),
                    Arg::Expr(set_fn),
                ],
            ),
            BindPropertyKind::ElementSize(kind) => ctx.b.call_stmt(
                "$.bind_element_size",
                [
                    Arg::Ident(el_name),
                    Arg::StrRef(kind.name()),
                    Arg::Expr(set_fn),
                ],
            ),
            BindPropertyKind::ResizeObserver(kind) => ctx.b.call_stmt(
                "$.bind_resize_observer",
                [
                    Arg::Ident(el_name),
                    Arg::StrRef(kind.name()),
                    Arg::Expr(set_fn),
                ],
            ),
            BindPropertyKind::Media(MediaBindKind::CurrentTime) => ctx.b.call_stmt(
                "$.bind_current_time",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Media(MediaBindKind::PlaybackRate) => ctx.b.call_stmt(
                "$.bind_playback_rate",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Media(MediaBindKind::Paused) => ctx.b.call_stmt(
                "$.bind_paused",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Media(MediaBindKind::Volume) => ctx.b.call_stmt(
                "$.bind_volume",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Media(MediaBindKind::Muted) => ctx.b.call_stmt(
                "$.bind_muted",
                [Arg::Ident(el_name), Arg::Expr(get_fn), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Media(MediaBindKind::Buffered) => ctx
                .b
                .call_stmt("$.bind_buffered", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
            BindPropertyKind::Media(MediaBindKind::Seekable) => ctx
                .b
                .call_stmt("$.bind_seekable", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
            BindPropertyKind::Media(MediaBindKind::Seeking) => ctx
                .b
                .call_stmt("$.bind_seeking", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
            BindPropertyKind::Media(MediaBindKind::Ended) => ctx
                .b
                .call_stmt("$.bind_ended", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
            BindPropertyKind::Media(MediaBindKind::ReadyState) => ctx.b.call_stmt(
                "$.bind_ready_state",
                [Arg::Ident(el_name), Arg::Expr(set_fn)],
            ),
            BindPropertyKind::Media(MediaBindKind::Played) => ctx
                .b
                .call_stmt("$.bind_played", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
            BindPropertyKind::Media(MediaBindKind::Duration) => {
                ctx.b
                    .bind_property_call_stmt("duration", "durationchange", el_name, set_fn, None)
            }
            BindPropertyKind::Media(MediaBindKind::VideoWidth) => {
                ctx.b
                    .bind_property_call_stmt("videoWidth", "resize", el_name, set_fn, None)
            }
            BindPropertyKind::Media(MediaBindKind::VideoHeight) => {
                ctx.b
                    .bind_property_call_stmt("videoHeight", "resize", el_name, set_fn, None)
            }
            BindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalWidth) => ctx
                .b
                .bind_property_call_stmt("naturalWidth", "load", el_name, set_fn, None),
            BindPropertyKind::ImageNaturalSize(ImageNaturalSizeKind::NaturalHeight) => ctx
                .b
                .bind_property_call_stmt("naturalHeight", "load", el_name, set_fn, None),
            BindPropertyKind::Focused => ctx
                .b
                .call_stmt("$.bind_focused", [Arg::Ident(el_name), Arg::Expr(set_fn)]),
            BindPropertyKind::Group => {
                return self
                    .emit_bind_group(bind, el_name, get_fn, set_fn)
                    .map(Some);
            }
            BindPropertyKind::This
            | BindPropertyKind::Window(_)
            | BindPropertyKind::Document(_)
            | BindPropertyKind::ComponentProp => {
                return CodegenError::unexpected_node(
                    bind.id,
                    "unexpected bind property routed through getter/setter path",
                );
            }
        };
        Ok(Some(stmt))
    }

    pub(super) fn take_bind_getter_setter(
        &mut self,
        bind_id: NodeId,
    ) -> Result<Option<(Expression<'a>, Expression<'a>)>> {
        let expr = self.take_attr_expr(bind_id)?;
        let Expression::SequenceExpression(seq) = expr else {
            return Ok(None);
        };
        let seq = seq.unbox();
        let mut exprs = seq.expressions.into_iter();
        let Some(get) = exprs.next() else {
            return CodegenError::unexpected_node(
                bind_id,
                "bind SequenceExpression missing getter",
            );
        };
        let Some(set) = exprs.next() else {
            return CodegenError::unexpected_node(
                bind_id,
                "bind SequenceExpression missing setter",
            );
        };
        Ok(Some((get, set)))
    }
}
