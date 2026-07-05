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
use svelte_analyze::Volatility;

use super::data_structures::TemplateMemoState;
use crate::context::Ctx;

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
        Volatility::Static | Volatility::Reactive | Volatility::Asynchronous => obj,
    }
}
