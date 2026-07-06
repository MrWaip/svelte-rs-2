use oxc_ast::ast::{AssignmentOperator, AssignmentTarget, Expression};
use oxc_span::SPAN;
use oxc_syntax::reference::ReferenceId;
use svelte_analyze::reactivity_semantics::legacy_reactive::legacy_reactive_import_wrapper_name;
use svelte_analyze::{AnalysisData, BindingSemantics, ReferenceSemantics, SignalReferenceKind};
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::SymbolId;

fn reads_runtime_derived(analysis: &AnalysisData<'_>, ref_id: ReferenceId) -> bool {
    analysis.symbol_for_reference(ref_id).is_some_and(|sym| {
        matches!(
            analysis.binding_semantics(sym),
            BindingSemantics::Derived(_)
        )
    })
}

fn store_base_symbol(analysis: &AnalysisData<'_>, store_symbol: SymbolId) -> Option<SymbolId> {
    match analysis.binding_semantics(store_symbol) {
        BindingSemantics::Store(facts) => Some(facts.base_symbol),
        _ => None,
    }
}

fn server_store_base_read<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'a>,
    base_symbol: SymbolId,
) -> Expression<'a> {
    let base_name = analysis.scoping.symbol_name(base_symbol);
    if analysis
        .reactivity
        .legacy_reactive()
        .is_mutated_import(base_symbol)
    {
        let wrapper: &str = b.alloc_str(&legacy_reactive_import_wrapper_name(base_name));
        return b.call_expr_callee(b.rid_expr(wrapper), []);
    }
    b.rid_expr(base_name)
}

pub fn server_store_get<'a>(
    b: &Builder<'a>,
    dollar_name: &str,
    base: Expression<'a>,
) -> Expression<'a> {
    let ast = b.ast;
    let target = AssignmentTarget::AssignmentTargetIdentifier(
        ast.alloc(ast.identifier_reference(SPAN, ast.atom("$$store_subs"))),
    );
    let empty_object = ast.expression_object(SPAN, ast.vec());
    let subs = Expression::AssignmentExpression(ast.alloc(ast.assignment_expression(
        SPAN,
        AssignmentOperator::LogicalNullish,
        target,
        empty_object,
    )));
    let name: &str = b.alloc_str(dollar_name);
    b.call_expr(
        "$.store_get",
        [Arg::Expr(subs), Arg::StrRef(name), Arg::Expr(base)],
    )
}

fn store_read_of_symbol<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'a>,
    store_symbol: SymbolId,
) -> Option<Expression<'a>> {
    let base_symbol = store_base_symbol(analysis, store_symbol)?;
    let dollar_name = analysis.scoping.symbol_name(store_symbol).to_string();
    let base = server_store_base_read(b, analysis, base_symbol);
    Some(server_store_get(b, &dollar_name, base))
}

pub fn rewrite_identifier_read<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'a>,
    expr: &mut Expression<'a>,
) {
    let Expression::Identifier(id) = &*expr else {
        return;
    };
    let Some(ref_id) = id.reference_id.get() else {
        return;
    };
    match analysis.reference_semantics(ref_id) {
        ReferenceSemantics::LegacyPropsIdentifierRead => {
            *expr = b.rid_expr("$$sanitized_props");
        }
        ReferenceSemantics::StoreRead { symbol }
        | ReferenceSemantics::ImportSubscribedRead {
            store_symbol: symbol,
        } => {
            if let Some(read) = store_read_of_symbol(b, analysis, symbol) {
                *expr = read;
            }
        }
        ReferenceSemantics::SignalRead {
            kind: SignalReferenceKind::Derived(_),
            ..
        } => {
            if reads_runtime_derived(analysis, ref_id) {
                let callee = b.rid_expr(id.name.as_str());
                *expr = b.call_expr_callee(callee, []);
            }
        }
        _ => {}
    }
}

pub fn force_store_read<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'a>,
    expr: &mut Expression<'a>,
) -> bool {
    let Expression::Identifier(id) = &*expr else {
        return false;
    };
    let Some(ref_id) = id.reference_id.get() else {
        return false;
    };
    let store_symbol = match analysis.reference_semantics(ref_id) {
        ReferenceSemantics::StoreRead { symbol }
        | ReferenceSemantics::StoreWrite { symbol }
        | ReferenceSemantics::StoreUpdate { symbol }
        | ReferenceSemantics::ImportSubscribedRead {
            store_symbol: symbol,
        } => symbol,
        _ => return false,
    };
    if let Some(read) = store_read_of_symbol(b, analysis, store_symbol) {
        *expr = read;
        return true;
    }
    false
}
