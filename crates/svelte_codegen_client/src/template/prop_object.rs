use oxc_ast::ast::Expression;

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

pub(crate) enum PropOrSpread<'a> {
    Prop(ObjProp<'a>),
    Spread(Expression<'a>),
}

pub(crate) fn build_props_expr<'a>(ctx: &Ctx<'a>, items: Vec<PropOrSpread<'a>>) -> Expression<'a> {
    let has_spread = items.iter().any(|i| matches!(i, PropOrSpread::Spread(_)));

    if !has_spread {
        let props: Vec<ObjProp<'a>> = items
            .into_iter()
            .filter_map(|i| match i {
                PropOrSpread::Prop(p) => Some(p),
                PropOrSpread::Spread(_) => None,
            })
            .collect();
        return ctx.b.object_expr(props);
    }

    let mut args: Vec<Arg<'a, 'a>> = Vec::new();
    let mut current_props: Vec<ObjProp<'a>> = Vec::new();

    for item in items {
        match item {
            PropOrSpread::Prop(p) => current_props.push(p),
            PropOrSpread::Spread(expr) => {
                if !current_props.is_empty() {
                    args.push(Arg::Expr(
                        ctx.b.object_expr(std::mem::take(&mut current_props)),
                    ));
                }
                args.push(Arg::Expr(expr));
            }
        }
    }
    if !current_props.is_empty() {
        args.push(Arg::Expr(ctx.b.object_expr(current_props)));
    }

    ctx.b.call_expr("$.spread_props", args)
}
