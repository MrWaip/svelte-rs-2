mod animate_directive;
mod attach_tag;
mod bind;
mod class_directive;
mod concat_attr;
mod custom_element_attr;
mod dispatch;
mod events_common;
mod expression_attr;
mod on_directive_legacy;
mod option_value;
mod regular;
mod select_value;
mod spread_attr;
mod style_directive;
mod transition_directive;
mod use_directive;

pub(super) use dispatch::AttributeOwnerKind;

use oxc_ast::ast::Expression;
use svelte_analyze::{Suspension, Volatility};

use super::data_structures::TemplateMemoState;
use crate::context::Ctx;

pub(super) enum DirectivesSlot<'a> {
    Inline(Expression<'a>),
    Sync(usize),
    Async(usize),
}

fn hoist_directives_slot<'a>(
    memo: &mut TemplateMemoState<'a>,
    volatility: Volatility,
    obj: Expression<'a>,
) -> DirectivesSlot<'a> {
    match volatility {
        Volatility::Heavy => DirectivesSlot::Sync(memo.sync_values_push(obj)),
        Volatility::Asynchronous => {
            DirectivesSlot::Async(memo.async_values_push(obj, Suspension::None))
        }
        Volatility::Static | Volatility::Reactive => DirectivesSlot::Inline(obj),
    }
}

fn hoist_directives_object<'a>(
    ctx: &Ctx<'a>,
    memo: &mut TemplateMemoState<'a>,
    volatility: Volatility,
    obj: Expression<'a>,
) -> Expression<'a> {
    match volatility {
        Volatility::Heavy => {
            let index = memo.sync_values_push(obj);
            memo.sync_param_expr(ctx, index)
        }
        Volatility::Asynchronous => {
            let index = memo.async_values_push(obj, Suspension::None);
            memo.async_param_expr(ctx, index)
        }
        Volatility::Static | Volatility::Reactive => obj,
    }
}
