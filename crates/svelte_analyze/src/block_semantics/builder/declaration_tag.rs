use super::super::{BlockSemantics, DeclarationTagBlockSemantics};
use super::common::async_kind_from_expression;
use super::walker::Ctx;
use svelte_ast::DeclarationTag;

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &DeclarationTag) {
    let decl_node_id = tag.declaration.id();
    let base = async_kind_from_expression(ctx.expressions.get(tag.id));
    let async_kind = super::declaration_group::resolve(ctx, tag.id, decl_node_id, base);
    ctx.store.set(
        tag.id,
        BlockSemantics::DeclarationTag(DeclarationTagBlockSemantics {
            decl_node_id,
            async_kind,
        }),
    );
}
