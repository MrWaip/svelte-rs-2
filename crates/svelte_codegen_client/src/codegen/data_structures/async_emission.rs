use svelte_analyze::Volatility;
use svelte_ast::NodeId;

use crate::context::Ctx;

pub(crate) enum AsyncEmission {
    Sync,

    Awaited { blockers: Vec<u32> },

    Deferred { blockers: Vec<u32> },
}

impl AsyncEmission {
    pub(crate) fn for_node(ctx: &Ctx<'_>, id: NodeId) -> Self {
        let Some(data) = ctx.expression_data(id) else {
            return Self::Sync;
        };
        let blockers = data.blockers.to_vec();
        match data.volatility {
            Volatility::Asynchronous => Self::Awaited { blockers },
            Volatility::Static | Volatility::Reactive | Volatility::Heavy => {
                if blockers.is_empty() {
                    Self::Sync
                } else {
                    Self::Deferred { blockers }
                }
            }
        }
    }
}
