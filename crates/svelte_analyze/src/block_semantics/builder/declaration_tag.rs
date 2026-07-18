use super::super::{BlockSemantics, DeclarationTagBlockSemantics};
use super::common::async_kind_from_expression;
use super::walker::Ctx;
use svelte_ast::DeclarationTag;

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &DeclarationTag) {
    let async_kind = async_kind_from_expression(ctx.expressions.get(tag.id));
    ctx.store.set(
        tag.id,
        BlockSemantics::DeclarationTag(DeclarationTagBlockSemantics { async_kind }),
    );
}
