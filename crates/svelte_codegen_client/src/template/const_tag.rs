use oxc_ast::ast::Statement;

use svelte_analyze::FragmentKey;

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

use super::expression::get_node_expr;

/// Emit `const name = $.derived(() => init_expr)` for each ConstTag in a fragment.
///
/// Called before DOM init code in every fragment codegen path.
pub(crate) fn emit_const_tags<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    stmts: &mut Vec<Statement<'a>>,
) {
    let Some(ids) = ctx.const_tags_for_fragment(&key).cloned() else {
        return;
    };

    for id in ids {
        let names = ctx.const_tag_names(id).cloned().unwrap_or_default();
        let init_expr = get_node_expr(ctx, id);

        if names.len() == 1 {
            // Simple identifier: const name = $.derived(() => init_expr)
            let thunk = ctx.b.thunk(init_expr);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            stmts.push(ctx.b.const_stmt(&names[0], derived));
        } else if names.len() > 1 {
            // Destructured: const tmp = $.derived(() => { const {a, b} = init; return {a, b}; })
            let tmp_name = ctx.analysis.const_tags.tmp_name(id)
                .expect("destructured const tag must have tmp_name from transform");
            let tmp_name: &str = ctx.b.alloc_str(tmp_name);

            let destruct_stmt = ctx.b.const_object_destruct_stmt(&names, init_expr);

            let props: Vec<ObjProp<'a>> = names.iter()
                .map(|n| ObjProp::Shorthand(ctx.b.alloc_str(n)))
                .collect();
            let ret = ctx.b.return_stmt(ctx.b.object_expr(props));

            let thunk = ctx.b.arrow_block_expr(ctx.b.no_params(), [destruct_stmt, ret]);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            stmts.push(ctx.b.const_stmt(tmp_name, derived));
        }
    }
}
