use oxc_ast::ast::{AssignmentOperator, AssignmentTarget, Expression};
use oxc_span::SPAN;
use oxc_syntax::reference::ReferenceId;
use svelte_analyze::{
    AnalysisData, BindingSemantics, ReferenceSemantics, SignalReadLocality, SignalReferenceKind,
};
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::SymbolId;

fn reads_via_derived_getter(semantics: BindingSemantics) -> bool {
    match semantics {
        BindingSemantics::Derived(_) | BindingSemantics::OptimizedDerived(_) => true,
        BindingSemantics::NonReactive
        | BindingSemantics::MaybeReactive
        | BindingSemantics::State(_)
        | BindingSemantics::OptimizedRune(_)
        | BindingSemantics::Prop(_)
        | BindingSemantics::LegacyBindableProp(_)
        | BindingSemantics::LegacyApiExport
        | BindingSemantics::LegacyPropsObject
        | BindingSemantics::LegacyState(_)
        | BindingSemantics::Store(_)
        | BindingSemantics::Const(_)
        | BindingSemantics::OptimizedConst(_)
        | BindingSemantics::DeclarationTag
        | BindingSemantics::OptimizedDeclarationTag
        | BindingSemantics::Contextual(_)
        | BindingSemantics::RuntimeRune { .. }
        | BindingSemantics::Unresolved => false,
    }
}

fn reads_runtime_derived(analysis: &AnalysisData<'_>, ref_id: ReferenceId) -> bool {
    let reads_getter = analysis
        .symbol_for_reference(ref_id)
        .is_some_and(|sym| reads_via_derived_getter(analysis.binding_semantics(sym)));
    reads_getter && !reads_element_fragment_local(analysis, ref_id)
}

fn reads_element_fragment_local(analysis: &AnalysisData<'_>, ref_id: ReferenceId) -> bool {
    matches!(
        analysis.reference_semantics(ref_id),
        ReferenceSemantics::SignalRead {
            locality: SignalReadLocality::ElementFragmentLocal,
            ..
        }
    )
}

pub fn store_base_symbol(analysis: &AnalysisData<'_>, store_symbol: SymbolId) -> Option<SymbolId> {
    match analysis.binding_semantics(store_symbol) {
        BindingSemantics::Store(facts) => Some(facts.base_symbol),
        _ => None,
    }
}

pub fn server_store_base_read<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'a>,
    base_symbol: SymbolId,
) -> Expression<'a> {
    let base_name = analysis.scoping.symbol_name(base_symbol);
    let base = b.rid_expr(base_name);
    if reads_via_derived_getter(analysis.binding_semantics(base_symbol)) {
        return b.call_expr_callee(base, []);
    }
    base
}

fn store_subs_assign<'a>(b: &Builder<'a>) -> Expression<'a> {
    let ast = b.ast;
    let target = AssignmentTarget::AssignmentTargetIdentifier(
        ast.alloc(ast.identifier_reference(SPAN, "$$store_subs")),
    );
    let empty_object = ast.expression_object(SPAN, ast.vec());
    Expression::AssignmentExpression(ast.alloc(ast.assignment_expression(
        SPAN,
        AssignmentOperator::LogicalNullish,
        target,
        empty_object,
    )))
}

pub fn server_store_get<'a>(
    b: &Builder<'a>,
    dollar_name: &str,
    base: Expression<'a>,
) -> Expression<'a> {
    let subs = store_subs_assign(b);
    let name: &str = b.alloc_str(dollar_name);
    b.call_expr(
        "$.store_get",
        [Arg::Expr(subs), Arg::StrRef(name), Arg::Expr(base)],
    )
}

pub fn server_store_set<'a>(
    b: &Builder<'a>,
    base: Expression<'a>,
    value: Expression<'a>,
) -> Expression<'a> {
    b.call_expr("$.store_set", [Arg::Expr(base), Arg::Expr(value)])
}

pub fn server_store_update<'a>(
    b: &Builder<'a>,
    dollar_name: &str,
    base: Expression<'a>,
    is_prefix: bool,
    is_decrement: bool,
) -> Expression<'a> {
    let fn_name = if is_prefix {
        "$.update_store_pre"
    } else {
        "$.update_store"
    };
    let subs = store_subs_assign(b);
    let name: &str = b.alloc_str(dollar_name);
    let mut args = vec![Arg::Expr(subs), Arg::StrRef(name), Arg::Expr(base)];
    if is_decrement {
        args.push(Arg::Num(-1.0));
    }
    b.call_expr(fn_name, args)
}

pub fn server_store_mutate<'a>(
    b: &Builder<'a>,
    dollar_name: &str,
    base: Expression<'a>,
    mutation: Expression<'a>,
) -> Expression<'a> {
    let subs = store_subs_assign(b);
    let name: &str = b.alloc_str(dollar_name);
    b.call_expr(
        "$.store_mutate",
        [
            Arg::Expr(subs),
            Arg::StrRef(name),
            Arg::Expr(base),
            Arg::Expr(mutation),
        ],
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
        | ReferenceSemantics::StoreWrite { symbol }
        | ReferenceSemantics::StoreUpdate { symbol } => {
            if let Some(read) = store_read_of_symbol(b, analysis, symbol) {
                *expr = read;
            }
        }
        ReferenceSemantics::SignalRead {
            kind: SignalReferenceKind::Derived(_),
            safe,
            ..
        } if reads_runtime_derived(analysis, ref_id) => {
            let callee = b.rid_expr(id.name.as_str());
            *expr = if safe {
                b.maybe_call_expr(callee, [])
            } else {
                b.call_expr_callee(callee, [])
            };
        }
        _ => {}
    }
}

pub fn force_derived_read<'a>(
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
    if reads_runtime_derived(analysis, ref_id) {
        let callee = b.rid_expr(id.name.as_str());
        *expr = b.call_expr_callee(callee, []);
        return true;
    }
    false
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
