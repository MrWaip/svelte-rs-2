use oxc_ast::NONE;
use oxc_ast::ast::{Argument, Expression, NumberBase};
use oxc_span::SPAN;
use svelte_analyze::reactivity_semantics::legacy_reactive::legacy_reactive_import_wrapper_name;
use svelte_analyze::{
    AnalysisData, BindingSemantics, PropBindingKind, PropBindingSemantics, PropEmitMode,
};
use svelte_ast_builder::Builder;
use svelte_component_semantics::SymbolId;

use crate::runtime::dollar_member;

pub fn build_store_base_read<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'_>,
    base_sym: SymbolId,
) -> Expression<'a> {
    let ast = b.ast;
    let base_name = analysis.scoping.symbol_name(base_sym);
    if analysis
        .reactivity
        .legacy_reactive()
        .is_mutated_import(base_sym)
    {
        let wrapper: &str = b.alloc_str(&legacy_reactive_import_wrapper_name(base_name));
        return b.call_expr_callee(b.rid_expr(wrapper), []);
    }
    match analysis.binding_semantics(base_sym) {
        BindingSemantics::LegacyState(_)
        | BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_) => {
            let ident = ast.expression_identifier(SPAN, ast.atom(base_name));
            let callee = dollar_member(b, "get");
            ast.expression_call(SPAN, callee, NONE, ast.vec1(Argument::from(ident)), false)
        }
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::NonSource,
            emit_mode: PropEmitMode::Standard,
        }) => {
            let object = ast.expression_identifier(SPAN, ast.atom("$$props"));
            let property = ast.identifier_name(SPAN, ast.atom(base_name));
            Expression::StaticMemberExpression(
                ast.alloc(ast.static_member_expression(SPAN, object, property, false)),
            )
        }
        BindingSemantics::LegacyBindableProp(_) => {
            let callee = ast.expression_identifier(SPAN, ast.atom(base_name));
            ast.expression_call(SPAN, callee, NONE, ast.vec(), false)
        }
        _ => ast.expression_identifier(SPAN, ast.atom(base_name)),
    }
}

pub fn make_store_set<'a>(
    b: &Builder<'a>,
    base: Expression<'a>,
    value: Expression<'a>,
) -> Expression<'a> {
    let ast = b.ast;
    let callee = dollar_member(b, "store_set");
    ast.expression_call(
        SPAN,
        callee,
        NONE,
        ast.vec_from_array([Argument::from(base), Argument::from(value)]),
        false,
    )
}

pub fn make_store_mutate<'a>(
    b: &Builder<'a>,
    base: Expression<'a>,
    mutation: Expression<'a>,
    untracked: Expression<'a>,
) -> Expression<'a> {
    let ast = b.ast;
    let callee = dollar_member(b, "store_mutate");
    ast.expression_call(
        SPAN,
        callee,
        NONE,
        ast.vec_from_array([
            Argument::from(base),
            Argument::from(mutation),
            Argument::from(untracked),
        ]),
        false,
    )
}

pub fn make_store_update<'a>(
    b: &Builder<'a>,
    base: Expression<'a>,
    dollar_name: &str,
    is_prefix: bool,
    is_increment: bool,
) -> Expression<'a> {
    let ast = b.ast;
    let fn_name = if is_prefix {
        "update_pre_store"
    } else {
        "update_store"
    };
    let callee = dollar_member(b, fn_name);
    let name_arg = Argument::from(base);
    let thunk_callee = ast.expression_identifier(SPAN, ast.atom(dollar_name));
    let thunk_call = ast.expression_call(SPAN, thunk_callee, NONE, ast.vec(), false);
    let thunk_arg = Argument::from(thunk_call);

    let args = if is_increment {
        ast.vec_from_array([name_arg, thunk_arg])
    } else {
        let delta =
            Argument::from(ast.expression_numeric_literal(SPAN, -1.0, None, NumberBase::Decimal));
        ast.vec_from_array([name_arg, thunk_arg, delta])
    };

    ast.expression_call(SPAN, callee, NONE, args, false)
}
