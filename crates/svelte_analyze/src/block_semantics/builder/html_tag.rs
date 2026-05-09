use svelte_ast::{HtmlTag, Namespace};

use super::super::data::{BlockSemantics, HtmlTagNamespace, HtmlTagSemantics};
use super::walker::Ctx;

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &HtmlTag) {
    let parent_strategy = match ctx.fragment_namespaces.get(ctx.current_fragment_id) {
        Some(Namespace::Svg) => HtmlTagNamespace::Svg,
        Some(Namespace::Mathml) => HtmlTagNamespace::MathMl,
        _ => HtmlTagNamespace::Html,
    };
    let hydration_html_changed_ignored =
        ctx.dev && ctx.ignore_data.is_ignored(tag.id, "hydration_html_changed");
    ctx.store.set(
        tag.id,
        BlockSemantics::HtmlTag(HtmlTagSemantics {
            parent_strategy,
            hydration_html_changed_ignored,
        }),
    );
}
