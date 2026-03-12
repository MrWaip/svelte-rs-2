//! Component instantiation codegen.

use oxc_ast::ast::{Expression, Statement};

use svelte_ast::NodeId;

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

/// Generate `ComponentName($$anchor, {})` call.
pub(crate) fn gen_component<'a>(
    ctx: &mut Ctx<'a>,
    id: NodeId,
    anchor: Expression<'a>,
    init: &mut Vec<Statement<'a>>,
) {
    let name = ctx.component_node(id).name.clone();
    let name_str: &str = ctx.b.alloc_str(&name);
    let props = ctx.b.object_expr(std::iter::empty::<ObjProp<'a>>());
    init.push(ctx.b.expr_stmt(
        ctx.b.call_expr(name_str, [Arg::Expr(anchor), Arg::Expr(props)]),
    ));
}
