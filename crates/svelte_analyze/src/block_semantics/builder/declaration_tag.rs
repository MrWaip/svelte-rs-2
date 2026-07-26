use super::super::{BlockSemantics, DeclarationTagBlockSemantics};
use super::common::async_kind_from_expression;
use super::walker::Ctx;
use svelte_ast::DeclarationTag;

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &DeclarationTag) {
    let base = async_kind_from_expression(ctx.expressions.get(tag.id));
    let async_kind = super::declaration_group::resolve(ctx, tag.id, tag.declaration.id(), base);
    ctx.store.set(
        tag.id,
        BlockSemantics::DeclarationTag(DeclarationTagBlockSemantics { async_kind }),
    );
}
