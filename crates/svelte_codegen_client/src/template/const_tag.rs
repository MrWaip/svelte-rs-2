use oxc_ast::ast::Statement;

use svelte_analyze::FragmentKey;

use crate::builder::Arg;
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
    let Some(ids) = ctx.analysis.const_tags.by_fragment.get(&key).cloned() else {
        return;
    };

    for id in ids {
        let names = ctx.analysis.const_tags.names.get(&id).cloned().unwrap_or_default();
        let init_expr = get_node_expr(ctx, id);

        // Simple identifier: const name = $.derived(() => init_expr)
        if names.len() == 1 {
            let thunk = ctx.b.thunk(init_expr);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            stmts.push(ctx.b.const_stmt(&names[0], derived));
        }
        // Destructuring deferred — only simple identifiers for now
    }
}
