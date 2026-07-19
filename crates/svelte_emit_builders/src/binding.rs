use oxc_ast::ast::Expression;
use svelte_analyze::reactivity_semantics::legacy_reactive::legacy_reactive_import_wrapper_name;
use svelte_analyze::{
    AnalysisData, BindingSemantics, ConstTagSemantics, ContextualBindingSemantics,
    EachIndexStrategy, EachItemStrategy, PropBindingKind, PropBindingSemantics,
    SnippetParamStrategy,
};
use svelte_ast_builder::Builder;
use svelte_component_semantics::SymbolId;

use crate::props::props_member;
use crate::runes::{member_get_via_get, rune_get, rune_safe_get};
use crate::runtime::thunk_call;

#[derive(Clone, Copy, Debug)]
pub enum LegacyStateSafety {
    Static(bool),
    FromVarDeclared,
}

pub fn read_binding<'a>(
    b: &Builder<'a>,
    analysis: &AnalysisData<'_>,
    sym: SymbolId,
    safety: LegacyStateSafety,
) -> Option<Expression<'a>> {
    let name: &'a str = b.alloc_str(analysis.scoping.symbol_name(sym));
    if analysis.reactivity.legacy_reactive().is_mutated_import(sym) {
        let wrapper: &str = b.alloc_str(&legacy_reactive_import_wrapper_name(name));
        return Some(b.call_expr_callee(b.rid_expr(wrapper), []));
    }
    match analysis.binding_semantics(sym) {
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::NonSource,
            ..
        }) => {
            let (prop_name, _origin_kind) = analysis.binding_origin_key(sym)?;
            Some(props_member(b, b.alloc_str(prop_name.as_ref())))
        }
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Source { .. },
            ..
        }) => Some(thunk_call(b, name)),
        BindingSemantics::Prop(PropBindingSemantics {
            kind: PropBindingKind::Rest,
            ..
        }) => Some(b.rid_expr(name)),
        BindingSemantics::LegacyBindableProp(_) => Some(thunk_call(b, name)),
        BindingSemantics::Store(store) => {
            let dollar_name: &'a str =
                b.alloc_str(analysis.scoping.symbol_name(store.store_symbol));
            Some(thunk_call(b, dollar_name))
        }
        BindingSemantics::LegacyState(state) => {
            let safe = match safety {
                LegacyStateSafety::Static(s) => s,
                LegacyStateSafety::FromVarDeclared => state.var_declared,
            };
            Some(if safe {
                rune_safe_get(b, name)
            } else {
                rune_get(b, name)
            })
        }
        BindingSemantics::State(_)
        | BindingSemantics::Derived(_)
        | BindingSemantics::OptimizedDerived(_)
        | BindingSemantics::OptimizedRune(_) => Some(rune_get(b, name)),
        BindingSemantics::Const(ConstTagSemantics {
            destructured: false,
            ..
        })
        | BindingSemantics::OptimizedConst(ConstTagSemantics {
            destructured: false,
            ..
        }) => Some(rune_get(b, name)),
        BindingSemantics::Const(ConstTagSemantics {
            destructured: true, ..
        })
        | BindingSemantics::OptimizedConst(ConstTagSemantics {
            destructured: true, ..
        }) => None,
        BindingSemantics::Contextual(ck) => match ck {
            ContextualBindingSemantics::EachItem(EachItemStrategy::IndexedLegacy) => None,
            ContextualBindingSemantics::EachItem(EachItemStrategy::Accessor)
            | ContextualBindingSemantics::SnippetParam(SnippetParamStrategy::Accessor) => {
                Some(thunk_call(b, name))
            }
            ContextualBindingSemantics::EachItem(EachItemStrategy::Direct)
            | ContextualBindingSemantics::EachIndex(EachIndexStrategy::Direct) => {
                Some(b.rid_expr(name))
            }
            ContextualBindingSemantics::EachItem(EachItemStrategy::Signal)
            | ContextualBindingSemantics::EachIndex(EachIndexStrategy::Signal)
            | ContextualBindingSemantics::SnippetParam(SnippetParamStrategy::Signal)
            | ContextualBindingSemantics::AwaitValue
            | ContextualBindingSemantics::AwaitError
            | ContextualBindingSemantics::LetDirective => Some(rune_get(b, name)),
            ContextualBindingSemantics::LetDirectiveCarrierMember { carrier_symbol } => {
                let carrier_name: &'a str =
                    b.alloc_str(analysis.scoping.symbol_name(carrier_symbol));
                Some(member_get_via_get(b, carrier_name, name))
            }
            ContextualBindingSemantics::LetDirectiveDirect => Some(b.rid_expr(name)),
        },
        BindingSemantics::NonReactive | BindingSemantics::MaybeReactive
            if analysis.scoping.is_import(sym) =>
        {
            Some(b.rid_expr(name))
        }
        _ => None,
    }
}
