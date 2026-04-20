//! Resolve `{@render}` tag metadata — per-argument prop-source resolution and
//! callee dynamism. Populates `blocks.render_tag_plans[tag_id]`:
//!
//! - `arg_plans[i].prop_source` — `Some(SymbolId)` when argument `i` is an
//!   identifier that resolves to a `$props()` source binding; `None` otherwise.
//! - `callee_mode` — `Direct` / `DynamicRegular` / `Chain` / `DynamicChain`
//!   depending on whether the callee symbol carries reactive meaning and
//!   whether the tag is part of a `.`-chained call (`{@render foo.bar()}`).
//!
//! Both sub-steps read `ReactivitySemantics` — they must run after
//! `BuildReactivitySemantics`.

use oxc_ast::ast::Expression;
use svelte_ast::NodeId;

#[allow(deprecated)]
use crate::types::data::RenderTagCalleeMode;
use crate::types::data::{AnalysisData, DeclarationSemantics};
use crate::ParserResult;

#[deprecated(note = "use BlockSemantics::Render / block_semantics(id) instead")]
#[allow(deprecated)]
pub(crate) fn run(data: &mut AnalysisData, parsed: &ParserResult<'_>) {
    resolve_arg_prop_sources(data, parsed);
    resolve_callee_dynamism(data);
}

#[allow(deprecated)]
fn resolve_arg_prop_sources(data: &mut AnalysisData, parsed: &ParserResult<'_>) {
    let tag_ids: Vec<NodeId> = data.blocks.render_tag_plans.keys().collect();
    for tag_id in tag_ids {
        let handle = match data
            .template
            .template_semantics
            .node_expr_handles
            .get(tag_id)
        {
            Some(&handle) => handle,
            None => continue,
        };
        let resolved: Vec<Option<crate::scope::SymbolId>> = match parsed.expr(handle) {
            Some(Expression::CallExpression(call)) => call
                .arguments
                .iter()
                .map(|arg| {
                    if let Expression::Identifier(ident) = arg.to_expression() {
                        ident
                            .reference_id
                            .get()
                            .and_then(|ref_id| data.scoping.get_reference(ref_id).symbol_id())
                            .filter(|&sym| {
                                matches!(
                                    data.reactivity.declaration_semantics(
                                        data.scoping.symbol_declaration(sym),
                                    ),
                                    crate::DeclarationSemantics::Prop(
                                        crate::PropDeclarationSemantics {
                                            kind: crate::PropDeclarationKind::Source { .. },
                                            ..
                                        },
                                    ),
                                )
                            })
                    } else {
                        None
                    }
                })
                .collect(),
            _ => continue,
        };
        let Some(plan) = data.blocks.render_tag_plans.get_mut(tag_id) else {
            continue;
        };
        for (arg_plan, prop_source) in plan.arg_plans.iter_mut().zip(resolved) {
            arg_plan.prop_source = prop_source;
        }
    }
}

#[allow(deprecated)]
fn resolve_callee_dynamism(data: &mut AnalysisData) {
    let all_ids: Vec<NodeId> = data.blocks.render_tag_plans.keys().collect();

    for node_id in all_ids {
        let is_dynamic = match data.blocks.render_tag_callee_sym.get(node_id) {
            Some(&sym_id) => is_reactive_callee_symbol(data, sym_id),
            None => true,
        };
        let is_chain = data.blocks.render_tag_is_chain.contains(&node_id);

        let mode = match (is_dynamic, is_chain) {
            (true, true) => RenderTagCalleeMode::DynamicChain,
            (true, false) => RenderTagCalleeMode::DynamicRegular,
            (false, true) => RenderTagCalleeMode::Chain,
            (false, false) => RenderTagCalleeMode::Direct,
        };
        if let Some(plan) = data.blocks.render_tag_plans.get_mut(node_id) {
            plan.callee_mode = mode;
        }
    }
}

/// A `{@render}` callee symbol is "dynamic" when it refers to any reactive
/// source (state, derived, prop, store, const-tag alias, contextual binding,
/// or a rune binding that has been optimized to a plain `let` but can still
/// be reassigned from the outside).
fn is_reactive_callee_symbol(data: &AnalysisData<'_>, sym: crate::scope::SymbolId) -> bool {
    !matches!(
        data.declaration_semantics(data.scoping.symbol_declaration(sym)),
        DeclarationSemantics::NonReactive | DeclarationSemantics::Unresolved,
    )
}
