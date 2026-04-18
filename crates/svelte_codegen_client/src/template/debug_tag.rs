use oxc_ast::ast::Statement;

use svelte_analyze::FragmentKey;

use svelte_ast_builder::{Arg, ObjProp};
use crate::context::Ctx;

/// Emit `$.template_effect(() => { console.log({...}); debugger; })` for each DebugTag.
///
/// Emitted in both dev and non-dev modes (Svelte always generates debug output).
pub(crate) fn emit_debug_tags<'a>(
    ctx: &mut Ctx<'a>,
    key: FragmentKey,
    stmts: &mut Vec<Statement<'a>>,
) {
    let Some(ids) = ctx.debug_tags_for_fragment(&key).cloned() else {
        return;
    };

    for id in ids {
        let tag = ctx.debug_tag(id);
        let identifiers = tag.identifiers.clone();

        let props: Vec<ObjProp<'a>> = identifiers
            .iter()
            .enumerate()
            .map(|(_i, span)| {
                let name = ctx.query.component.source_text(*span);
                let name_alloc: &str = ctx.b.alloc_str(name);
                // Use pre-transformed expression (with $.get() wrapping for each-block vars)
                let ident_expr = ctx
                    .state
                    .parsed
                    .expr_handle(span.start)
                    .and_then(|handle| ctx.state.parsed.take_expr(handle))
                    .unwrap_or_else(|| ctx.b.rid_expr(name_alloc));
                let snapshot = ctx.b.call_expr("$.snapshot", [Arg::Expr(ident_expr)]);
                // Non-runes mode: snapshot runs inside an effect but stores are tracked;
                // $.untrack prevents reactive reads from registering as effect dependencies.
                let value = if ctx.query.runes() {
                    snapshot
                } else {
                    ctx.b
                        .call_expr("$.untrack", [Arg::Expr(ctx.b.thunk(snapshot))])
                };
                ObjProp::KeyValue(name_alloc, value)
            })
            .collect();

        let obj = ctx.b.object_expr(props);
        let log_call = ctx.b.call_stmt("console.log", [Arg::Expr(obj)]);
        let debugger = ctx.b.debugger_stmt();
        let thunk = ctx.b.thunk_block(vec![log_call, debugger]);
        stmts.push(ctx.b.call_stmt("$.template_effect", [Arg::Expr(thunk)]));
    }
}
