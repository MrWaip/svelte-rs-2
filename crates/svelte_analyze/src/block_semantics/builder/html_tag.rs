use svelte_ast::{HtmlTag, Namespace};

use super::super::data::{BlockSemantics, HtmlTagAsyncKind, HtmlTagNamespace, HtmlTagSemantics};
use super::walker::Ctx;
use crate::expression_semantics::{ExpressionData, Volatility};

pub(super) fn populate(ctx: &mut Ctx<'_, '_>, tag: &HtmlTag) {
    let parent_strategy = match ctx.fragment_namespaces.get(ctx.current_fragment_id) {
        Some(Namespace::Svg) => HtmlTagNamespace::Svg,
        Some(Namespace::Mathml) => HtmlTagNamespace::MathMl,
        _ => HtmlTagNamespace::Html,
    };
    let hydration_html_changed_ignored = ctx.dev
        && ctx
            .ignore_data
            .is_ignored_warning(tag.id, crate::WarningCode::HydrationHtmlChanged);

    let expression_data = ctx.expressions.get(tag.id).data();
    let async_kind = derive_async_kind(expression_data);

    ctx.store.set(
        tag.id,
        BlockSemantics::HtmlTag(HtmlTagSemantics {
            parent_strategy,
            hydration_html_changed_ignored,
            async_kind,
        }),
    );
}

fn derive_async_kind(data: Option<&ExpressionData>) -> HtmlTagAsyncKind {
    match data {
        Some(d) => match d.volatility {
            Volatility::Asynchronous => HtmlTagAsyncKind::Awaited {
                blockers: d.blockers.clone(),
            },
            Volatility::Static | Volatility::Reactive | Volatility::Heavy => {
                if d.blockers.is_empty() {
                    HtmlTagAsyncKind::Sync
                } else {
                    HtmlTagAsyncKind::Deferred {
                        blockers: d.blockers.clone(),
                    }
                }
            }
        },
        None => HtmlTagAsyncKind::Sync,
    }
}
