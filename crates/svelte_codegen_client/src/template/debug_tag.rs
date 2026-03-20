use oxc_ast::ast::Statement;

use svelte_analyze::FragmentKey;

use crate::builder::{Arg, ObjProp};
use crate::context::Ctx;

/// Emit `$.template_effect(() => { console.log({...}); debugger; })` for each DebugTag.
///
/// Only emits when `ctx.dev` is true.
pub(crate) fn emit_debug_tags<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    stmts: &mut Vec<Statement<'a>>,
) {
    if !ctx.dev {
        return;
    }

    let Some(ids) = ctx.debug_tags_for_fragment(&key).cloned() else {
        return;
    };

    for id in ids {
        let tag = ctx.debug_tag(id);
        let identifiers = tag.identifiers.clone();

        let props: Vec<ObjProp<'a>> = identifiers.iter().enumerate().map(|(i, span)| {
            let name = ctx.component.source_text(*span);
            let name_alloc: &str = ctx.b.alloc_str(name);
            // Use pre-transformed expression (with $.get() wrapping for each-block vars)
            let ident_expr = ctx.parsed.debug_tag_exprs.remove(&(id, i))
                .unwrap_or_else(|| ctx.b.rid_expr(name_alloc));
            let snapshot = ctx.b.call_expr("$.snapshot", [Arg::Expr(ident_expr)]);
            ObjProp::KeyValue(name_alloc, snapshot)
        }).collect();

        let obj = ctx.b.object_expr(props);
        let log_call = ctx.b.call_stmt("console.log", [Arg::Expr(obj)]);
        let debugger = ctx.b.debugger_stmt();
        let thunk = ctx.b.thunk_block(vec![log_call, debugger]);
        stmts.push(ctx.b.call_stmt("$.template_effect", [Arg::Expr(thunk)]));
    }
}
