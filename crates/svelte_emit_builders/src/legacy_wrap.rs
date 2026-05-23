use oxc_ast::ast::Expression;
use std::iter::once;
use svelte_analyze::{ExpressionData, LegacyWrap, SyntheticPropsCarrier};
use svelte_ast_builder::{Arg, Builder};
use svelte_component_semantics::SymbolId;

pub fn is_untrack_call(expr: &Expression<'_>) -> bool {
    let Expression::CallExpression(call) = expr.get_inner_expression() else {
        return false;
    };
    matches!(call.callee.get_inner_expression(), Expression::Identifier(id) if id.name.as_str() == "$.untrack")
}

pub fn apply<'a>(
    b: &Builder<'a>,
    expr: Expression<'a>,
    wrap: LegacyWrap,
    refs: &[SymbolId],
    mut dep_resolver: impl FnMut(SymbolId) -> Option<Expression<'a>>,
) -> Expression<'a> {
    if matches!(wrap, LegacyWrap::None) {
        return expr;
    }
    if is_untrack_call(&expr) {
        return expr;
    }
    let mut seq_parts: Vec<Expression<'a>> = Vec::new();
    let carrier = match wrap {
        LegacyWrap::Synthetic(c) | LegacyWrap::CoarseAndSynthetic(c) => Some(c),
        LegacyWrap::None | LegacyWrap::CoarseWrap => None,
    };
    if let Some(c) = carrier {
        if matches!(
            c,
            SyntheticPropsCarrier::RestProps | SyntheticPropsCarrier::Both
        ) {
            seq_parts.push(b.call_expr("$.deep_read_state", [Arg::Ident("$$restProps")]));
        }
        if matches!(
            c,
            SyntheticPropsCarrier::SanitizedProps | SyntheticPropsCarrier::Both
        ) {
            seq_parts.push(b.call_expr("$.deep_read_state", [Arg::Ident("$$sanitized_props")]));
        }
    }
    for &sym in refs {
        if let Some(getter) = dep_resolver(sym) {
            seq_parts.push(getter);
        }
    }
    if seq_parts.is_empty() {
        return b.call_expr("$.untrack", [Arg::Expr(b.thunk(expr))]);
    }
    let mut iter = seq_parts.into_iter();
    let Some(first) = iter.next() else {
        return expr;
    };
    let mut sequence = first;
    for next in iter.chain(once(
        b.call_expr("$.untrack", [Arg::Expr(b.thunk(expr))]),
    )) {
        sequence = b.seq_expr([sequence, next]);
    }
    sequence
}

pub fn maybe<'a>(
    b: &Builder<'a>,
    expr: Expression<'a>,
    data: Option<&ExpressionData>,
    dep_resolver: impl FnMut(SymbolId) -> Option<Expression<'a>>,
) -> Expression<'a> {
    let Some(data) = data else { return expr };
    if matches!(data.legacy_wrap, LegacyWrap::None) {
        return expr;
    }
    apply(b, expr, data.legacy_wrap, &data.references, dep_resolver)
}
