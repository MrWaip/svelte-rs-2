use oxc_ast::ast::Expression;
use oxc_syntax::scope::ScopeId;
use svelte_analyze::{AnalysisData, ExpressionData, LegacyDependency};
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::SymbolId;

use crate::binding::{LegacyStateSafety, read_binding};
use crate::legacy_wrap;
use crate::runtime::thunk_call;

pub fn each_item_collection_read_legacy<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'_>,
    source_sym: SymbolId,
    hoisted_collection_name: Option<&str>,
) -> Expression<'a> {
    if let Some(name) = hoisted_collection_name {
        return thunk_call(b, name);
    }
    let name = analysis.scoping.symbol_name(source_sym);
    read_binding(b, analysis, source_sym, LegacyStateSafety::FromVarDeclared)
        .unwrap_or_else(|| b.rid_expr(name))
}

pub fn each_item_indexed_member_legacy<'a>(
    b: &Builder<'a>,
    collection: Expression<'a>,
    index_name: &str,
) -> Expression<'a> {
    b.computed_member_expr(collection, b.rid_expr(index_name))
}

pub fn wrap_each_collection_legacy<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'_>,
    collection_expr: Expression<'a>,
    expr_data: &ExpressionData,
    untrack_scope: ScopeId,
) -> Expression<'a> {
    let wrapped = legacy_wrap::maybe(b, collection_expr, Some(expr_data), |sym| {
        legacy_collection_dep(b, analysis, sym)
    });
    seed_untrack_arrow_scope(b, &wrapped, untrack_scope);
    wrapped
}

fn seed_untrack_arrow_scope<'a>(b: &Builder<'a>, expr: &Expression<'a>, scope: ScopeId) {
    let call = match expr {
        Expression::SequenceExpression(seq) => match seq.expressions.last() {
            Some(Expression::CallExpression(call)) => call,
            _ => return,
        },
        Expression::CallExpression(call) => call,
        _ => return,
    };
    let Some(arg) = call.arguments.first() else {
        return;
    };
    if let Some(arrow) = arg.as_expression() {
        b.seed_arrow_scope(arrow, Some(scope));
    }
}

fn legacy_collection_dep<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'_>,
    sym: SymbolId,
) -> Option<Expression<'a>> {
    match analysis.binding_semantics(sym).legacy_dependency() {
        LegacyDependency::SelfTracked => None,
        LegacyDependency::Shallow => {
            read_binding(b, analysis, sym, LegacyStateSafety::FromVarDeclared)
        }
        LegacyDependency::Deep => {
            let base = read_binding(b, analysis, sym, LegacyStateSafety::FromVarDeclared)?;
            Some(b.call_expr("$.deep_read_state", [Arg::Expr(base)]))
        }
    }
}
