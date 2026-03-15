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
        let is_destructured = ctx.is_const_destructured(id);

        if !is_destructured && names.len() == 1 {
            // Simple identifier: const name = $.derived(() => init_expr)
            let thunk = ctx.b.thunk(init_expr);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(thunk)]);
            stmts.push(ctx.b.const_stmt(&names[0], derived));
        } else if is_destructured {
            // Destructuring: const computed_const = $.derived(() => {
            //   const { x, y } = init_expr;
            //   return { x, y };
            // })
            let temp_name = ctx.const_tag_temp_name(id)
                .unwrap_or("computed_const")
                .to_string();
            let pattern_text = ctx.const_tag_pattern_text(id).map(|s| s.to_string());

            // Determine if object or array destructuring from pattern text
            let is_array = pattern_text.as_deref().map_or(false, |t| t.starts_with('['));
            let name_refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();

            // Build: const { x, y } = init_expr;  (or const [a, b] = init_expr;)
            let destruct_stmt = if is_array {
                ctx.b.const_array_destruct_stmt(&name_refs, init_expr)
            } else {
                ctx.b.const_object_destruct_stmt(&name_refs, init_expr)
            };

            // Build: return { x, y };
            let props: Vec<ObjProp<'a>> = names.iter().map(|n| {
                let alloc_name = ctx.b.alloc_str(n);
                ObjProp::Shorthand(alloc_name)
            }).collect();
            let obj = ctx.b.object_expr(props);
            let ret = ctx.b.return_stmt(obj);

            // Build: () => { const { x, y } = init; return { x, y }; }
            let arrow = ctx.b.arrow_block_expr(ctx.b.no_params(), [destruct_stmt, ret]);
            let derived = ctx.b.call_expr("$.derived", [Arg::Expr(arrow)]);

            stmts.push(ctx.b.const_stmt(&temp_name, derived));
        }
    }
}
