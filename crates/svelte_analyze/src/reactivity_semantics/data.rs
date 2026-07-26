use std::mem;

use crate::scope::SymbolId;
use oxc_index::IndexVec;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::{FragmentId, NodeId, RunesMode};
use svelte_component_semantics::{OxcNodeId, ReferenceId};
use svelte_span::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindingSemantics {
    NonReactive,

    MaybeReactive,

    State(StateDeclarationSemantics),

    Derived(DerivedDeclarationSemantics),

    OptimizedDerived(DerivedDeclarationSemantics),

    OptimizedRune(OptimizedRuneSemantics),

    Prop(PropBindingSemantics),

    LegacyBindableProp(LegacyBindablePropSemantics),

    LegacyApiExport,

    LegacyPropsObject,

    LegacyState(LegacyStateSemantics),

    Store(StoreBindingSemantics),

    Const(ConstTagSemantics),

    OptimizedConst(ConstTagSemantics),

    DeclarationTag,

    OptimizedDeclarationTag,

    Contextual(ContextualBindingSemantics),

    RuntimeRune { kind: RuntimeRuneKind },

    Unresolved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyDependency {
    SelfTracked,
    Shallow,
    Deep,
}

impl BindingSemantics {
    pub fn legacy_dependency(&self) -> LegacyDependency {
        match self {
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::Contextual(ContextualBindingSemantics::LetDirectiveDirect) => {
                LegacyDependency::SelfTracked
            }
            BindingSemantics::Prop(PropBindingSemantics {
                kind: PropBindingKind::NonSource | PropBindingKind::Rest,
                ..
            })
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::MaybeReactive
            | BindingSemantics::Contextual(
                ContextualBindingSemantics::LetDirective
                | ContextualBindingSemantics::LetDirectiveCarrierMember { .. }
                | ContextualBindingSemantics::AwaitValue
                | ContextualBindingSemantics::AwaitError
                | ContextualBindingSemantics::EachIndex(EachIndexStrategy::Signal),
            ) => LegacyDependency::Deep,
            BindingSemantics::Prop(PropBindingSemantics {
                kind: PropBindingKind::Identifier | PropBindingKind::Source { .. },
                ..
            })
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved
            | BindingSemantics::Contextual(
                ContextualBindingSemantics::EachItem(_)
                | ContextualBindingSemantics::EachIndex(EachIndexStrategy::Direct)
                | ContextualBindingSemantics::SnippetParam(_),
            ) => LegacyDependency::Shallow,
        }
    }

    pub fn is_reactive(&self) -> bool {
        match self {
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::Prop(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Store(_)
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive => true,
            BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_rune_backed(&self) -> bool {
        match self {
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. } => true,
            BindingSemantics::Prop(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Store(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_store(&self) -> bool {
        match self {
            BindingSemantics::Store(_) => true,
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Prop(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_props(&self) -> bool {
        match self {
            BindingSemantics::Prop(_) | BindingSemantics::LegacyBindableProp(_) => true,
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_bindable(&self) -> bool {
        match self {
            BindingSemantics::Prop(prop) => prop.bindable,
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_runes_prop(&self) -> bool {
        match self {
            BindingSemantics::Prop(_) => true,
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_legacy_prop(&self) -> bool {
        match self {
            BindingSemantics::LegacyBindableProp(_) => true,
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn reads_via_thunk(&self) -> bool {
        match self {
            BindingSemantics::Store(_) | BindingSemantics::LegacyBindableProp(_) => true,
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_rest_props(&self) -> bool {
        match self {
            BindingSemantics::Prop(prop) => match &prop.kind {
                PropBindingKind::Rest => true,
                PropBindingKind::Identifier
                | PropBindingKind::Source { .. }
                | PropBindingKind::NonSource => false,
            },
            BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_maybe_reactive(&self) -> bool {
        match self {
            BindingSemantics::MaybeReactive => true,
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_legacy_state(&self) -> bool {
        match self {
            BindingSemantics::LegacyState(_) => true,
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn legacy_state_immutable(&self) -> Option<bool> {
        match self {
            BindingSemantics::LegacyState(state) => Some(state.immutable),
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => None,
        }
    }

    pub(crate) fn is_legacy_api_export(&self) -> bool {
        match self {
            BindingSemantics::LegacyApiExport => true,
            BindingSemantics::LegacyPropsObject
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_legacy_props_object(&self) -> bool {
        match self {
            BindingSemantics::LegacyPropsObject => true,
            BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_each_item_indexed_legacy(&self) -> bool {
        let BindingSemantics::Contextual(ContextualBindingSemantics::EachItem(strategy)) = self
        else {
            return false;
        };
        match strategy {
            EachItemStrategy::IndexedLegacy => true,
            EachItemStrategy::Accessor | EachItemStrategy::Signal | EachItemStrategy::Direct => {
                false
            }
        }
    }

    pub fn reads_via_each_item_accessor(&self) -> bool {
        let BindingSemantics::Contextual(ContextualBindingSemantics::EachItem(strategy)) = self
        else {
            return false;
        };
        match strategy {
            EachItemStrategy::Accessor => true,
            EachItemStrategy::IndexedLegacy
            | EachItemStrategy::Signal
            | EachItemStrategy::Direct => false,
        }
    }

    pub fn is_derived(&self) -> bool {
        match self {
            BindingSemantics::Derived(_) => true,
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_non_reactive(&self) -> bool {
        match self {
            BindingSemantics::NonReactive | BindingSemantics::LegacyPropsObject => true,
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn is_optimized_rune(&self) -> bool {
        match self {
            BindingSemantics::OptimizedRune(_) => true,
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => false,
        }
    }

    pub fn state(&self) -> Option<StateDeclarationSemantics> {
        match self {
            BindingSemantics::State(state) => Some(*state),
            BindingSemantics::Prop(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => None,
        }
    }

    pub fn runtime_rune(&self) -> Option<RuntimeRuneKind> {
        match self {
            BindingSemantics::RuntimeRune { kind } => Some(*kind),
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::LegacyState(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => None,
        }
    }

    pub fn legacy_state(&self) -> Option<LegacyStateSemantics> {
        match self {
            BindingSemantics::LegacyState(state) => Some(*state),
            BindingSemantics::Prop(_)
            | BindingSemantics::State(_)
            | BindingSemantics::Derived(_)
            | BindingSemantics::OptimizedDerived(_)
            | BindingSemantics::OptimizedRune(_)
            | BindingSemantics::RuntimeRune { .. }
            | BindingSemantics::Store(_)
            | BindingSemantics::LegacyBindableProp(_)
            | BindingSemantics::Const(_)
            | BindingSemantics::OptimizedConst(_)
            | BindingSemantics::DeclarationTag
            | BindingSemantics::OptimizedDeclarationTag
            | BindingSemantics::Contextual(_)
            | BindingSemantics::MaybeReactive
            | BindingSemantics::NonReactive
            | BindingSemantics::LegacyApiExport
            | BindingSemantics::LegacyPropsObject
            | BindingSemantics::Unresolved => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateDeclarationSemantics {
    pub kind: StateKind,
    pub proxied: bool,
    pub var_declared: bool,
    pub is_signal_source: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeRuneKind {
    PropsId,

    EffectTracking,

    EffectPending,

    Host,

    InspectTrace,

    Effect,

    EffectPre,

    EffectRoot,

    Inspect,

    InspectWith,

    StateSnapshot,

    StateEager,

    Bindable,
}

impl RuntimeRuneKind {
    pub fn display_name(self) -> &'static str {
        match self {
            RuntimeRuneKind::PropsId => "$props.id",
            RuntimeRuneKind::EffectTracking => "$effect.tracking",
            RuntimeRuneKind::EffectPending => "$effect.pending",
            RuntimeRuneKind::Host => "$host",
            RuntimeRuneKind::InspectTrace => "$inspect.trace",
            RuntimeRuneKind::Effect => "$effect",
            RuntimeRuneKind::EffectPre => "$effect.pre",
            RuntimeRuneKind::EffectRoot => "$effect.root",
            RuntimeRuneKind::Inspect => "$inspect",
            RuntimeRuneKind::InspectWith => "$inspect().with",
            RuntimeRuneKind::StateSnapshot => "$state.snapshot",
            RuntimeRuneKind::StateEager => "$state.eager",
            RuntimeRuneKind::Bindable => "$bindable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateKind {
    State,

    StateRaw,

    StateEager,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptimizedRuneSemantics {
    pub kind: StateKind,

    pub proxy_init: bool,

    pub var_declared: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DerivedDeclarationSemantics {
    pub kind: DerivedKind,
    pub async_kind: DerivedAsyncKind,
    pub var_declared: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivedKind {
    Derived,

    DerivedBy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivedAsyncKind {
    Sync,

    Async,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DerivedSource {
    #[default]
    Computed,

    Passthrough,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropBindingSemantics {
    pub emit_mode: PropEmitMode,
    pub kind: PropBindingKind,
    pub bindable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropEmitMode {
    Standard,

    CustomElement,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PropsSummary {
    pub has_props: bool,
    pub has_bindable: bool,
    pub has_custom_element: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReactivitySummary {
    pub props: PropsSummary,
    pub has_store_bindings: bool,
    pub has_runes_bindable: bool,
    pub has_legacy_reactive_statements: bool,
    pub has_named_runes_prop: bool,
    pub has_named_legacy_prop: bool,
    pub has_inspect_trace: bool,
    pub legacy: LegacySummary,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LegacySummary {
    pub has_bindable_prop: bool,
    pub reads_props_object: bool,
    pub reads_rest_props_object: bool,
    pub has_member_mutated: bool,
    pub reads_slots_object: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropBindingKind {
    Identifier,

    Source {
        updated: bool,
        default_lowering: PropDefaultKind,

        default_needs_proxy: bool,
    },

    Rest,

    NonSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegacyBindablePropSemantics {
    pub default_kind: PropDefaultKind,
    pub flags: crate::PropsFlags,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegacyStateSemantics {
    pub var_declared: bool,
    pub immutable: bool,
    pub is_signal_source: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropDefaultKind {
    None,

    Eager,

    Lazy,

    LazyAccessor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoreBindingSemantics {
    pub base_symbol: SymbolId,
    pub store_symbol: SymbolId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstTagSemantics {
    pub destructured: bool,
    pub initial_is_function: bool,
    pub owner_node: NodeId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualBindingSemantics {
    EachItem(EachItemStrategy),

    EachIndex(EachIndexStrategy),

    AwaitValue,

    AwaitError,

    LetDirective,

    LetDirectiveCarrierMember { carrier_symbol: SymbolId },

    LetDirectiveDirect,

    SnippetParam(SnippetParamStrategy),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachItemStrategy {
    Accessor,

    Signal,

    Direct,

    IndexedLegacy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachIndexStrategy {
    Signal,

    Direct,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetParamStrategy {
    Accessor,

    Signal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeclaratorSemantics {
    None,

    RuntimeRuneCall {
        kind: RuntimeRuneKind,
    },

    RuneProps,

    LegacyProps,

    LegacyState,

    RuneState {
        kind: StateKind,
    },

    RuneDerived {
        kind: DerivedKind,
        async_kind: DerivedAsyncKind,
        source: DerivedSource,
    },

    ConstTag {
        async_kind: DerivedAsyncKind,
    },

    LetCarrier {
        carrier_symbol: Option<SymbolId>,
    },

    EachItem,

    AwaitValue,

    SnippetParam,

    ClassFieldState(ClassFieldStateSemantics),

    ClassFieldDerived(ClassFieldDerivedSemantics),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclaratorGroup {
    Rune,
    Legacy,
    Contextual,
    Plain,
}

impl DeclaratorSemantics {
    pub fn group(&self) -> DeclaratorGroup {
        match self {
            DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => DeclaratorGroup::Rune,
            DeclaratorSemantics::LegacyProps | DeclaratorSemantics::LegacyState => {
                DeclaratorGroup::Legacy
            }
            DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam => DeclaratorGroup::Contextual,
            DeclaratorSemantics::None => DeclaratorGroup::Plain,
        }
    }

    pub fn is_rune_props(&self) -> bool {
        match self {
            DeclaratorSemantics::RuneProps => true,
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => false,
        }
    }

    pub fn is_legacy_props(&self) -> bool {
        match self {
            DeclaratorSemantics::LegacyProps => true,
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => false,
        }
    }

    pub fn is_props_id_call(&self) -> bool {
        match self {
            DeclaratorSemantics::RuntimeRuneCall {
                kind: RuntimeRuneKind::PropsId,
            } => true,
            DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::None
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => false,
        }
    }

    pub fn is_rune_derived(&self) -> bool {
        match self {
            DeclaratorSemantics::RuneDerived { .. } => true,
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => false,
        }
    }

    pub fn is_bindable_call(&self) -> bool {
        match self {
            DeclaratorSemantics::RuntimeRuneCall { kind } => match kind {
                RuntimeRuneKind::Bindable => true,
                RuntimeRuneKind::PropsId
                | RuntimeRuneKind::EffectTracking
                | RuntimeRuneKind::EffectPending
                | RuntimeRuneKind::Host
                | RuntimeRuneKind::InspectTrace
                | RuntimeRuneKind::Effect
                | RuntimeRuneKind::EffectPre
                | RuntimeRuneKind::EffectRoot
                | RuntimeRuneKind::Inspect
                | RuntimeRuneKind::InspectWith
                | RuntimeRuneKind::StateSnapshot
                | RuntimeRuneKind::StateEager => false,
            },
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam
            | DeclaratorSemantics::ClassFieldState(_)
            | DeclaratorSemantics::ClassFieldDerived(_) => false,
        }
    }

    pub fn class_field_state(&self) -> Option<ClassFieldStateSemantics> {
        match self {
            DeclaratorSemantics::ClassFieldState(state) => Some(*state),
            DeclaratorSemantics::None
            | DeclaratorSemantics::RuntimeRuneCall { .. }
            | DeclaratorSemantics::RuneProps
            | DeclaratorSemantics::LegacyProps
            | DeclaratorSemantics::LegacyState
            | DeclaratorSemantics::RuneState { .. }
            | DeclaratorSemantics::RuneDerived { .. }
            | DeclaratorSemantics::ConstTag { .. }
            | DeclaratorSemantics::LetCarrier { .. }
            | DeclaratorSemantics::EachItem
            | DeclaratorSemantics::AwaitValue
            | DeclaratorSemantics::SnippetParam
            | DeclaratorSemantics::ClassFieldDerived(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassFieldStateSemantics {
    pub kind: StateKind,
    pub proxied: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassFieldSemantics {
    None,
    State {
        kind: StateKind,
        proxy: bool,
        tracked: bool,
    },
    Derived {
        kind: DerivedKind,
    },
}

impl ClassFieldSemantics {
    pub fn is_field(&self) -> bool {
        match self {
            ClassFieldSemantics::State { .. } | ClassFieldSemantics::Derived { .. } => true,
            ClassFieldSemantics::None => false,
        }
    }

    pub fn is_derived(&self) -> bool {
        match self {
            ClassFieldSemantics::Derived { .. } => true,
            ClassFieldSemantics::None | ClassFieldSemantics::State { .. } => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassFieldDerivedSemantics {
    pub kind: DerivedKind,
    pub async_kind: DerivedAsyncKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceSemantics {
    NonReactive,

    Proxy,

    SignalRead {
        kind: SignalReferenceKind,
        safe: bool,
        locality: SignalReadLocality,
    },

    SignalWrite {
        kind: StateKind,
        proxy: bool,
        store_unsub: Option<SymbolId>,
    },

    SignalUpdate {
        kind: StateKind,
        safe: bool,
        proxy: bool,
        store_unsub: Option<SymbolId>,
    },

    DerivedWrite,

    DerivedUpdate,

    StoreRead {
        symbol: SymbolId,
    },

    StoreWrite {
        symbol: SymbolId,
    },

    StoreUpdate {
        symbol: SymbolId,
    },

    PropRead(PropReferenceSemantics),

    PropMutation {
        bindable: bool,
        symbol: SymbolId,
    },

    PropSourceMemberMutationRoot {
        bindable: bool,
        symbol: SymbolId,
    },

    PropNonSourceMemberMutationRoot {
        symbol: SymbolId,
    },

    ConstAliasRead {
        owner_node: NodeId,
    },

    ContextualRead(ContextualReadSemantics),

    CarrierMemberRead(CarrierMemberReadSemantics),

    RestPropMemberRewrite,

    LegacyPropsIdentifierRead,

    LegacyRestPropsIdentifierRead,

    LegacySlotsIdentifierRead,

    LegacyStateRead {
        safe: bool,
    },

    LegacyStateWrite,

    LegacyStateUpdate {
        safe: bool,
    },

    LegacyStateSubscribedRead {
        safe: bool,
        store_symbol: SymbolId,
    },

    LegacyStateSubscribedWrite {
        store_symbol: SymbolId,
    },

    LegacyStateSubscribedUpdate {
        safe: bool,
        store_symbol: SymbolId,
    },

    LegacyStateMemberMutationRoot {
        symbol: SymbolId,
    },

    LegacyReactiveImportRead,

    LegacyReactiveImportMemberMutationRoot {
        symbol: SymbolId,
    },

    ImportSubscribedRead {
        store_symbol: SymbolId,
    },

    LegacyEachItemMemberMutationRoot {
        item_sym: SymbolId,
        raw_param: bool,
    },

    EachItemMemberMutationStoreInvalidate {
        item_sym: SymbolId,
        collection_store: SymbolId,
        raw_param: bool,
    },

    EachItemIndexedLegacy {
        item_sym: SymbolId,
        index_sym: Option<SymbolId>,
        index_read: EachIndexStrategy,
    },

    IllegalWrite,

    Unresolved,
}

impl ReferenceSemantics {
    pub fn signal_write_kind(&self) -> Option<StateKind> {
        match self {
            ReferenceSemantics::SignalWrite { kind, .. }
            | ReferenceSemantics::SignalUpdate { kind, .. } => Some(*kind),

            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => None,
        }
    }

    pub(crate) fn store_symbol(&self) -> Option<SymbolId> {
        match self {
            ReferenceSemantics::StoreRead { symbol }
            | ReferenceSemantics::StoreWrite { symbol }
            | ReferenceSemantics::StoreUpdate { symbol } => Some(*symbol),
            _ => None,
        }
    }

    pub fn is_store_subscription(&self) -> bool {
        match self {
            ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. } => true,

            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub fn is_derived_write(&self) -> bool {
        match self {
            ReferenceSemantics::DerivedWrite => true,

            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub fn is_legacy_props_object_read(&self) -> bool {
        match self {
            ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead => true,
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub fn is_prop_mutation(&self) -> bool {
        match self {
            ReferenceSemantics::PropMutation { .. } => true,
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub fn is_bindable_prop_access(&self) -> bool {
        match self {
            ReferenceSemantics::PropRead(PropReferenceSemantics::Source { bindable, .. }) => {
                *bindable
            }
            ReferenceSemantics::PropMutation { bindable, .. } => *bindable,
            ReferenceSemantics::PropRead(
                PropReferenceSemantics::NonSourceStatic { .. }
                | PropReferenceSemantics::NonSourceComputed { .. },
            )
            | ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub fn is_rest_prop_member_rewrite(&self) -> bool {
        match self {
            ReferenceSemantics::RestPropMemberRewrite => true,
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub fn is_legacy_state_member_mutation_root(&self) -> bool {
        match self {
            ReferenceSemantics::LegacyStateMemberMutationRoot { .. } => true,
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }

    pub fn is_legacy_reactive_import_member_mutation_root(&self) -> bool {
        match self {
            ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { .. } => true,
            ReferenceSemantics::NonReactive
            | ReferenceSemantics::Proxy
            | ReferenceSemantics::SignalRead { .. }
            | ReferenceSemantics::SignalWrite { .. }
            | ReferenceSemantics::SignalUpdate { .. }
            | ReferenceSemantics::DerivedWrite
            | ReferenceSemantics::DerivedUpdate
            | ReferenceSemantics::StoreRead { .. }
            | ReferenceSemantics::StoreWrite { .. }
            | ReferenceSemantics::StoreUpdate { .. }
            | ReferenceSemantics::PropRead(_)
            | ReferenceSemantics::PropMutation { .. }
            | ReferenceSemantics::PropSourceMemberMutationRoot { .. }
            | ReferenceSemantics::PropNonSourceMemberMutationRoot { .. }
            | ReferenceSemantics::ConstAliasRead { .. }
            | ReferenceSemantics::ContextualRead(_)
            | ReferenceSemantics::CarrierMemberRead(_)
            | ReferenceSemantics::RestPropMemberRewrite
            | ReferenceSemantics::LegacyPropsIdentifierRead
            | ReferenceSemantics::LegacyRestPropsIdentifierRead
            | ReferenceSemantics::LegacySlotsIdentifierRead
            | ReferenceSemantics::LegacyStateRead { .. }
            | ReferenceSemantics::LegacyStateWrite
            | ReferenceSemantics::LegacyStateUpdate { .. }
            | ReferenceSemantics::LegacyStateSubscribedRead { .. }
            | ReferenceSemantics::LegacyStateSubscribedWrite { .. }
            | ReferenceSemantics::LegacyStateSubscribedUpdate { .. }
            | ReferenceSemantics::LegacyStateMemberMutationRoot { .. }
            | ReferenceSemantics::LegacyReactiveImportRead
            | ReferenceSemantics::ImportSubscribedRead { .. }
            | ReferenceSemantics::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceSemantics::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceSemantics::EachItemIndexedLegacy { .. }
            | ReferenceSemantics::IllegalWrite
            | ReferenceSemantics::Unresolved => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalReferenceKind {
    State(StateKind),
    Derived(DerivedKind),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalReadLocality {
    Cell,
    ElementFragmentLocal,
    Detached,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualReadSemantics {
    pub kind: ContextualReadKind,
    pub owner_node: NodeId,
    pub symbol: SymbolId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualReadKind {
    EachItem {
        accessor: bool,
        signal: bool,
        raw_param: bool,
    },

    EachIndex {
        signal: bool,
        raw_param: bool,
    },

    AwaitValue,

    AwaitError,

    LetDirective,

    LetDirectiveDirect,

    SnippetParam {
        accessor: bool,
        signal: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarrierMemberReadSemantics {
    pub carrier_symbol: SymbolId,
    pub member_symbol: SymbolId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropReferenceSemantics {
    Source {
        bindable: bool,
        lowering_mode: PropEmitMode,
        symbol: SymbolId,
    },

    NonSourceStatic {
        symbol: SymbolId,
    },

    NonSourceComputed {
        symbol: SymbolId,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DerivedFacts {
    pub(crate) decl: DerivedDeclarationSemantics,
    pub(crate) reactive: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum BindingFacts {
    State(StateDeclarationSemantics),
    Derived(DerivedFacts),
    OptimizedDerived(DerivedFacts),
    OptimizedRune(OptimizedRuneSemantics),
    Prop(PropBindingSemantics),

    LegacyBindableProp(LegacyBindablePropSemantics),
    LegacyApiExport,
    LegacyPropsObject,

    LegacyState(LegacyStateSemantics),
    Store(StoreBindingSemantics),
    Const(ConstTagSemantics),
    OptimizedConst(ConstTagSemantics),
    DeclarationTag,
    OptimizedDeclarationTag,
    Contextual(ContextualBindingSemantics),
    RuntimeRune { kind: RuntimeRuneKind },

    CarrierAlias { carrier: SymbolId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ReferenceFacts {
    SignalRead {
        kind: SignalReferenceKind,
        safe: bool,
        locality: SignalReadLocality,
    },
    SignalWrite {
        kind: StateKind,
        proxy: bool,
        store_unsub: Option<SymbolId>,
    },
    SignalUpdate {
        kind: StateKind,
        safe: bool,
        proxy: bool,
        store_unsub: Option<SymbolId>,
    },
    DerivedWrite,
    DerivedUpdate,
    StoreRead {
        symbol: SymbolId,
    },
    StoreWrite {
        symbol: SymbolId,
    },
    StoreUpdate {
        symbol: SymbolId,
    },
    PropRead(PropReferenceSemantics),
    PropMutation {
        bindable: bool,
        symbol: SymbolId,
    },
    PropSourceMemberMutationRoot {
        bindable: bool,
        symbol: SymbolId,
    },
    PropNonSourceMemberMutationRoot {
        symbol: SymbolId,
    },
    ConstAliasRead {
        owner_node: NodeId,
    },
    ContextualRead(ContextualReadSemantics),
    CarrierMemberRead(CarrierMemberReadSemantics),
    RestPropMemberRewrite,

    LegacyPropsIdentifierRead,

    LegacyRestPropsIdentifierRead,

    LegacySlotsIdentifierRead,

    LegacyStateRead {
        safe: bool,
    },

    LegacyStateWrite,

    LegacyStateUpdate {
        safe: bool,
    },
    LegacyStateSubscribedRead {
        safe: bool,
        store_symbol: SymbolId,
    },
    LegacyStateSubscribedWrite {
        store_symbol: SymbolId,
    },
    LegacyStateSubscribedUpdate {
        safe: bool,
        store_symbol: SymbolId,
    },
    LegacyStateMemberMutationRoot {
        symbol: SymbolId,
    },

    LegacyReactiveImportRead,

    LegacyReactiveImportMemberMutationRoot {
        symbol: SymbolId,
    },

    ImportSubscribedRead {
        store_symbol: SymbolId,
    },

    LegacyEachItemMemberMutationRoot {
        item_symbol: SymbolId,
        raw_param: bool,
    },

    EachItemMemberMutationStoreInvalidate {
        item_symbol: SymbolId,
        collection_store: SymbolId,
        raw_param: bool,
    },

    EachItemIndexedLegacy {
        item_symbol: SymbolId,
    },

    IllegalWrite,

    Proxy,
}

impl ReferenceFacts {
    pub(crate) fn subscribed_store_symbol(&self) -> Option<SymbolId> {
        match self {
            ReferenceFacts::StoreRead { symbol }
            | ReferenceFacts::StoreWrite { symbol }
            | ReferenceFacts::StoreUpdate { symbol } => Some(*symbol),
            ReferenceFacts::LegacyStateSubscribedRead { store_symbol, .. }
            | ReferenceFacts::LegacyStateSubscribedWrite { store_symbol }
            | ReferenceFacts::LegacyStateSubscribedUpdate { store_symbol, .. } => {
                Some(*store_symbol)
            }
            ReferenceFacts::SignalRead { .. }
            | ReferenceFacts::SignalWrite { .. }
            | ReferenceFacts::SignalUpdate { .. }
            | ReferenceFacts::DerivedWrite
            | ReferenceFacts::DerivedUpdate
            | ReferenceFacts::PropRead(_)
            | ReferenceFacts::PropMutation { .. }
            | ReferenceFacts::PropSourceMemberMutationRoot { .. }
            | ReferenceFacts::PropNonSourceMemberMutationRoot { .. }
            | ReferenceFacts::ConstAliasRead { .. }
            | ReferenceFacts::ContextualRead(_)
            | ReferenceFacts::CarrierMemberRead(_)
            | ReferenceFacts::RestPropMemberRewrite
            | ReferenceFacts::LegacyPropsIdentifierRead
            | ReferenceFacts::LegacyRestPropsIdentifierRead
            | ReferenceFacts::LegacySlotsIdentifierRead
            | ReferenceFacts::LegacyStateRead { .. }
            | ReferenceFacts::LegacyStateWrite
            | ReferenceFacts::LegacyStateUpdate { .. }
            | ReferenceFacts::LegacyStateMemberMutationRoot { .. }
            | ReferenceFacts::LegacyReactiveImportRead
            | ReferenceFacts::LegacyReactiveImportMemberMutationRoot { .. }
            | ReferenceFacts::ImportSubscribedRead { .. }
            | ReferenceFacts::LegacyEachItemMemberMutationRoot { .. }
            | ReferenceFacts::EachItemMemberMutationStoreInvalidate { .. }
            | ReferenceFacts::EachItemIndexedLegacy { .. }
            | ReferenceFacts::IllegalWrite
            | ReferenceFacts::Proxy => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ConstTagCycleFactLegacy {
    pub(crate) names: String,
    pub(crate) at_node: NodeId,
}

#[derive(Clone, Debug)]
pub struct ReactivitySemantics {
    bindings: IndexVec<SymbolId, Option<BindingFacts>>,

    declarators: IndexVec<OxcNodeId, Option<DeclaratorSemantics>>,

    declarator_node_by_symbol: FxHashMap<SymbolId, OxcNodeId>,

    deferred_derived_sources: Vec<(OxcNodeId, ReferenceId)>,

    store_declaration_symbols: Vec<SymbolId>,

    reference_facts: IndexVec<ReferenceId, Option<ReferenceFacts>>,

    class_field_semantics: FxHashMap<OxcNodeId, ClassFieldSemantics>,

    prop_member_mutation_root_refs: rustc_hash::FxHashSet<ReferenceId>,

    contextual_owner: FxHashMap<SymbolId, NodeId>,

    raw_param_reads: FxHashSet<ReferenceId>,

    element_local_derived_reads: FxHashSet<ReferenceId>,

    detached_const_reads: FxHashSet<ReferenceId>,

    each_rest_symbols: FxHashSet<SymbolId>,

    maybe_reactive_symbols: FxHashSet<SymbolId>,

    init_proxyable: FxHashMap<SymbolId, bool>,

    legacy_bindable_prop_aliases: FxHashMap<SymbolId, String>,

    prop_default_spans: FxHashMap<SymbolId, Span>,

    has_bindable_prop: bool,

    legacy_uses_props: bool,

    legacy_uses_rest_props: bool,

    legacy_has_member_mutated: bool,

    legacy_reads_slots: bool,

    has_named_runes_prop: bool,

    has_named_legacy_prop: bool,

    has_inspect_trace: bool,

    each_item_indirect_sources: FxHashMap<SymbolId, SmallVec<[SymbolId; 2]>>,

    legacy_indirect_bindings: FxHashMap<SymbolId, SmallVec<[SymbolId; 4]>>,

    each_item_collection_store: FxHashMap<SymbolId, SymbolId>,

    each_item_index_legacy: FxHashMap<SymbolId, SymbolId>,

    base_to_store: FxHashMap<SymbolId, SymbolId>,

    uses_runes: bool,

    runes_mode: RunesMode,

    svelte_store_rune_import: Option<SymbolId>,

    const_tag_order_legacy: Vec<SmallVec<[NodeId; 4]>>,

    const_tag_cycle_legacy: Option<ConstTagCycleFactLegacy>,

    legacy_reactive: super::legacy_reactive::LegacyReactivitySemantics,
}

impl ReactivitySemantics {
    pub(crate) fn new(node_count: u32) -> Self {
        let mut declarators: IndexVec<OxcNodeId, Option<DeclaratorSemantics>> =
            IndexVec::with_capacity(node_count as usize);
        declarators.resize_with(node_count as usize, || None);
        let symbols_cap = node_count as usize / 8;
        Self {
            bindings: IndexVec::with_capacity(symbols_cap),
            declarators,
            declarator_node_by_symbol: FxHashMap::with_capacity_and_hasher(
                symbols_cap,
                Default::default(),
            ),
            deferred_derived_sources: Vec::new(),
            store_declaration_symbols: Vec::new(),
            reference_facts: IndexVec::with_capacity(node_count as usize / 4),
            class_field_semantics: FxHashMap::default(),
            prop_member_mutation_root_refs: rustc_hash::FxHashSet::default(),
            contextual_owner: FxHashMap::default(),
            raw_param_reads: rustc_hash::FxHashSet::default(),
            element_local_derived_reads: rustc_hash::FxHashSet::default(),
            detached_const_reads: rustc_hash::FxHashSet::default(),
            each_item_indirect_sources: FxHashMap::default(),
            legacy_indirect_bindings: FxHashMap::default(),
            each_item_collection_store: FxHashMap::default(),
            each_item_index_legacy: FxHashMap::default(),
            base_to_store: FxHashMap::default(),
            each_rest_symbols: FxHashSet::default(),
            maybe_reactive_symbols: FxHashSet::default(),
            init_proxyable: FxHashMap::default(),
            legacy_bindable_prop_aliases: FxHashMap::default(),
            prop_default_spans: FxHashMap::default(),
            has_bindable_prop: false,
            legacy_uses_props: false,
            legacy_uses_rest_props: false,
            legacy_has_member_mutated: false,
            legacy_reads_slots: false,
            has_named_runes_prop: false,
            has_named_legacy_prop: false,
            has_inspect_trace: false,
            uses_runes: false,
            runes_mode: RunesMode::Runes,
            svelte_store_rune_import: None,
            const_tag_order_legacy: Vec::new(),
            const_tag_cycle_legacy: None,
            legacy_reactive: super::legacy_reactive::LegacyReactivitySemantics::new(),
        }
    }

    pub(crate) fn reserve_references(&mut self, reference_count: usize) {
        if self.reference_facts.len() < reference_count {
            self.reference_facts.resize_with(reference_count, || None);
        }
    }

    pub fn uses_runes(&self) -> bool {
        self.uses_runes
    }

    pub fn runes_mode(&self) -> RunesMode {
        self.runes_mode
    }

    pub fn class_field_semantics(&self, access_node: OxcNodeId) -> ClassFieldSemantics {
        self.class_field_semantics
            .get(&access_node)
            .copied()
            .unwrap_or(ClassFieldSemantics::None)
    }

    pub(crate) fn record_class_field_semantics(
        &mut self,
        access_node: OxcNodeId,
        semantics: ClassFieldSemantics,
    ) {
        self.class_field_semantics.insert(access_node, semantics);
    }

    pub(crate) fn set_state_proxied(&mut self, sym: SymbolId, proxied: bool) {
        let Some(facts) = self.binding_facts_mut(sym) else {
            return;
        };
        match facts {
            BindingFacts::State(state) if state.kind == StateKind::State => state.proxied = proxied,
            BindingFacts::OptimizedRune(opt) if opt.kind == StateKind::State => {
                opt.proxy_init = proxied
            }
            _ => {}
        }
    }

    pub(crate) fn set_class_field_proxied(&mut self, decl_node: OxcNodeId, proxied: bool) {
        let DeclaratorSemantics::ClassFieldState(state) = self.declarator_semantics(decl_node)
        else {
            return;
        };
        if state.kind != StateKind::State {
            return;
        }
        self.record_declarator_semantics(
            decl_node,
            DeclaratorSemantics::ClassFieldState(ClassFieldStateSemantics {
                kind: state.kind,
                proxied,
            }),
        );
    }

    pub(crate) fn set_signal_write_proxy(&mut self, ref_id: ReferenceId, proxy: bool) {
        let Some(Some(fact)) = self.reference_facts.get_mut(ref_id) else {
            return;
        };
        match fact {
            ReferenceFacts::SignalWrite { proxy: slot, .. }
            | ReferenceFacts::SignalUpdate { proxy: slot, .. } => *slot = proxy,
            _ => {}
        }
    }

    pub fn binding_semantics(&self, sym: SymbolId) -> BindingSemantics {
        if let Some(facts) = self.lookup_binding_facts(sym) {
            return Self::binding_semantics_from_facts(facts);
        }
        if self.maybe_reactive_symbols.contains(&sym) {
            return BindingSemantics::MaybeReactive;
        }
        BindingSemantics::NonReactive
    }

    pub(crate) fn record_maybe_reactive_symbol(&mut self, sym: SymbolId) {
        self.maybe_reactive_symbols.insert(sym);
    }

    pub(crate) fn set_init_proxyable(&mut self, init_proxyable: FxHashMap<SymbolId, bool>) {
        self.init_proxyable = init_proxyable;
    }

    pub(crate) fn take_init_proxyable(&mut self) -> FxHashMap<SymbolId, bool> {
        mem::take(&mut self.init_proxyable)
    }

    pub fn declarator_semantics(&self, decl_node: OxcNodeId) -> DeclaratorSemantics {
        self.declarators
            .get(decl_node)
            .and_then(|slot| slot.as_ref())
            .cloned()
            .unwrap_or(DeclaratorSemantics::None)
    }

    pub(crate) fn record_declarator_node_for_symbol(&mut self, sym: SymbolId, node: OxcNodeId) {
        self.declarator_node_by_symbol.entry(sym).or_insert(node);
    }

    pub(crate) fn consolidate_legacy_state_declarators(&mut self) {
        let pending: Vec<OxcNodeId> = self
            .bindings
            .iter_enumerated()
            .filter_map(|(sym, facts)| {
                if !matches!(facts, Some(BindingFacts::LegacyState(_))) {
                    return None;
                }
                let node = self.declarator_node_by_symbol.get(&sym).copied()?;
                self.declarators
                    .get(node)
                    .and_then(|slot| slot.as_ref())
                    .is_none()
                    .then_some(node)
            })
            .collect();
        for node in pending {
            self.write_declarator(node, DeclaratorSemantics::LegacyState);
        }
    }

    pub(crate) fn record_deferred_derived_source(&mut self, node: OxcNodeId, source: ReferenceId) {
        self.deferred_derived_sources.push((node, source));
    }

    pub(crate) fn classify_derived_sources(&mut self) {
        let deferred = mem::take(&mut self.deferred_derived_sources);
        for (node, ref_id) in deferred {
            let is_passthrough = matches!(
                self.reference_semantics(ref_id),
                ReferenceSemantics::StoreRead { .. }
                    | ReferenceSemantics::PropRead(PropReferenceSemantics::Source { .. })
            );
            if !is_passthrough {
                continue;
            }
            if let DeclaratorSemantics::RuneDerived {
                kind, async_kind, ..
            } = self.declarator_semantics(node)
            {
                self.write_declarator(
                    node,
                    DeclaratorSemantics::RuneDerived {
                        kind,
                        async_kind,
                        source: DerivedSource::Passthrough,
                    },
                );
            }
        }
    }

    pub fn iter_store_bindings(
        &self,
    ) -> impl Iterator<Item = (SymbolId, StoreBindingSemantics)> + '_ {
        self.store_declaration_symbols.iter().filter_map(|&sym| {
            match self.lookup_binding_facts(sym)? {
                BindingFacts::Store(store) => Some((sym, *store)),
                _ => None,
            }
        })
    }

    pub fn summary(&self) -> ReactivitySummary {
        ReactivitySummary {
            props: self.props_summary(),
            has_store_bindings: !self.store_declaration_symbols.is_empty(),
            has_runes_bindable: self
                .iter_runes_prop_symbols()
                .any(|sym| self.binding_semantics(sym).is_bindable()),
            has_legacy_reactive_statements: self
                .legacy_reactive
                .iter_statements_topo()
                .next()
                .is_some(),
            has_named_runes_prop: self.has_named_runes_prop,
            has_named_legacy_prop: self.has_named_legacy_prop,
            has_inspect_trace: self.has_inspect_trace,
            legacy: LegacySummary {
                has_bindable_prop: self.iter_legacy_bindable_prop_symbols().next().is_some(),
                reads_props_object: self.legacy_uses_props,
                reads_rest_props_object: self.legacy_uses_rest_props,
                has_member_mutated: self.legacy_has_member_mutated,
                reads_slots_object: self.legacy_reads_slots,
            },
        }
    }

    fn props_summary(&self) -> PropsSummary {
        let mut has_props = false;
        let mut has_custom_element = false;
        for facts in self.bindings.iter() {
            let Some(BindingFacts::Prop(prop)) = facts else {
                continue;
            };
            has_props = true;
            match prop.emit_mode {
                PropEmitMode::CustomElement => has_custom_element = true,
                PropEmitMode::Standard => {}
            }
        }
        PropsSummary {
            has_props,
            has_bindable: self.has_bindable_prop,
            has_custom_element,
        }
    }

    pub fn iter_runes_prop_symbols(&self) -> impl Iterator<Item = SymbolId> + '_ {
        self.bindings.iter_enumerated().filter_map(|(sym, facts)| {
            let Some(BindingFacts::Prop(prop)) = facts else {
                return None;
            };
            match prop.kind {
                PropBindingKind::Source { .. } | PropBindingKind::NonSource => Some(sym),
                PropBindingKind::Identifier | PropBindingKind::Rest => None,
            }
        })
    }

    pub fn iter_legacy_bindable_prop_symbols(&self) -> impl Iterator<Item = SymbolId> + '_ {
        self.bindings.iter_enumerated().filter_map(|(sym, facts)| {
            matches!(facts, Some(BindingFacts::LegacyBindableProp(_))).then_some(sym)
        })
    }

    pub fn prop_default_span(&self, sym: SymbolId) -> Option<Span> {
        self.prop_default_spans.get(&sym).copied()
    }

    pub(crate) fn record_prop_default_span(&mut self, sym: SymbolId, span: Span) {
        self.prop_default_spans.insert(sym, span);
    }

    pub(crate) fn is_rest_prop(&self, sym: SymbolId) -> bool {
        matches!(
            self.lookup_binding_facts(sym),
            Some(BindingFacts::Prop(PropBindingSemantics {
                kind: PropBindingKind::Rest,
                ..
            }))
        )
    }

    pub fn legacy_reactive(&self) -> &super::legacy_reactive::LegacyReactivitySemantics {
        &self.legacy_reactive
    }

    pub(crate) fn legacy_reactive_mut(
        &mut self,
    ) -> &mut super::legacy_reactive::LegacyReactivitySemantics {
        &mut self.legacy_reactive
    }

    pub(crate) fn record_legacy_bindable_prop_alias(
        &mut self,
        symbol: SymbolId,
        alias: Option<String>,
    ) {
        if let Some(alias) = alias {
            self.legacy_bindable_prop_aliases.insert(symbol, alias);
        }
    }

    pub fn legacy_bindable_prop_alias(&self, symbol: SymbolId) -> Option<&str> {
        self.legacy_bindable_prop_aliases
            .get(&symbol)
            .map(String::as_str)
    }

    pub(crate) fn record_legacy_api_export_binding(&mut self, symbol: SymbolId) {
        self.write_binding(symbol, BindingFacts::LegacyApiExport);
    }

    pub(crate) fn set_legacy_unresolved_usage(&mut self, uses_props: bool, uses_rest_props: bool) {
        self.legacy_uses_props = uses_props;
        self.legacy_uses_rest_props = uses_rest_props;
    }

    pub(crate) fn set_legacy_has_member_mutated(&mut self, value: bool) {
        self.legacy_has_member_mutated = value;
    }

    pub(crate) fn mark_legacy_reads_slots(&mut self) {
        self.legacy_reads_slots = true;
    }

    pub(crate) fn mark_inspect_trace(&mut self) {
        self.has_inspect_trace = true;
    }

    pub(crate) fn set_named_prop_flags(&mut self, runes: bool, legacy: bool) {
        self.has_named_runes_prop = runes;
        self.has_named_legacy_prop = legacy;
    }

    pub fn reference_semantics(&self, ref_id: ReferenceId) -> ReferenceSemantics {
        match self.lookup_reference_facts(ref_id) {
            Some(ReferenceFacts::SignalRead {
                kind,
                safe,
                locality,
            }) => ReferenceSemantics::SignalRead {
                kind: *kind,
                safe: *safe,
                locality: *locality,
            },
            Some(ReferenceFacts::SignalWrite {
                kind,
                proxy,
                store_unsub,
            }) => ReferenceSemantics::SignalWrite {
                kind: *kind,
                proxy: *proxy,
                store_unsub: *store_unsub,
            },
            Some(ReferenceFacts::SignalUpdate {
                kind,
                safe,
                proxy,
                store_unsub,
            }) => ReferenceSemantics::SignalUpdate {
                kind: *kind,
                safe: *safe,
                proxy: *proxy,
                store_unsub: *store_unsub,
            },
            Some(ReferenceFacts::DerivedWrite) => ReferenceSemantics::DerivedWrite,
            Some(ReferenceFacts::DerivedUpdate) => ReferenceSemantics::DerivedUpdate,
            Some(ReferenceFacts::StoreRead { symbol }) => {
                ReferenceSemantics::StoreRead { symbol: *symbol }
            }
            Some(ReferenceFacts::StoreWrite { symbol }) => {
                ReferenceSemantics::StoreWrite { symbol: *symbol }
            }
            Some(ReferenceFacts::StoreUpdate { symbol }) => {
                ReferenceSemantics::StoreUpdate { symbol: *symbol }
            }
            Some(ReferenceFacts::PropRead(read)) => ReferenceSemantics::PropRead(*read),
            Some(ReferenceFacts::PropMutation { bindable, symbol }) => {
                ReferenceSemantics::PropMutation {
                    bindable: *bindable,
                    symbol: *symbol,
                }
            }
            Some(ReferenceFacts::PropSourceMemberMutationRoot { bindable, symbol }) => {
                ReferenceSemantics::PropSourceMemberMutationRoot {
                    bindable: *bindable,
                    symbol: *symbol,
                }
            }
            Some(ReferenceFacts::PropNonSourceMemberMutationRoot { symbol }) => {
                ReferenceSemantics::PropNonSourceMemberMutationRoot { symbol: *symbol }
            }
            Some(ReferenceFacts::ConstAliasRead { owner_node }) => {
                ReferenceSemantics::ConstAliasRead {
                    owner_node: *owner_node,
                }
            }
            Some(ReferenceFacts::ContextualRead(read)) => ReferenceSemantics::ContextualRead(*read),
            Some(ReferenceFacts::CarrierMemberRead(read)) => {
                ReferenceSemantics::CarrierMemberRead(*read)
            }
            Some(ReferenceFacts::RestPropMemberRewrite) => {
                ReferenceSemantics::RestPropMemberRewrite
            }
            Some(ReferenceFacts::LegacyPropsIdentifierRead) => {
                ReferenceSemantics::LegacyPropsIdentifierRead
            }
            Some(ReferenceFacts::LegacyRestPropsIdentifierRead) => {
                ReferenceSemantics::LegacyRestPropsIdentifierRead
            }
            Some(ReferenceFacts::LegacySlotsIdentifierRead) => {
                ReferenceSemantics::LegacySlotsIdentifierRead
            }
            Some(ReferenceFacts::LegacyStateRead { safe }) => {
                ReferenceSemantics::LegacyStateRead { safe: *safe }
            }
            Some(ReferenceFacts::LegacyStateWrite) => ReferenceSemantics::LegacyStateWrite,
            Some(ReferenceFacts::LegacyStateUpdate { safe }) => {
                ReferenceSemantics::LegacyStateUpdate { safe: *safe }
            }
            Some(ReferenceFacts::LegacyStateSubscribedRead { safe, store_symbol }) => {
                ReferenceSemantics::LegacyStateSubscribedRead {
                    safe: *safe,
                    store_symbol: *store_symbol,
                }
            }
            Some(ReferenceFacts::LegacyStateSubscribedWrite { store_symbol }) => {
                ReferenceSemantics::LegacyStateSubscribedWrite {
                    store_symbol: *store_symbol,
                }
            }
            Some(ReferenceFacts::LegacyStateSubscribedUpdate { safe, store_symbol }) => {
                ReferenceSemantics::LegacyStateSubscribedUpdate {
                    safe: *safe,
                    store_symbol: *store_symbol,
                }
            }
            Some(ReferenceFacts::LegacyStateMemberMutationRoot { symbol }) => {
                ReferenceSemantics::LegacyStateMemberMutationRoot { symbol: *symbol }
            }
            Some(ReferenceFacts::LegacyReactiveImportRead) => {
                ReferenceSemantics::LegacyReactiveImportRead
            }
            Some(ReferenceFacts::ImportSubscribedRead { store_symbol }) => {
                ReferenceSemantics::ImportSubscribedRead {
                    store_symbol: *store_symbol,
                }
            }
            Some(ReferenceFacts::LegacyReactiveImportMemberMutationRoot { symbol }) => {
                ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { symbol: *symbol }
            }
            Some(ReferenceFacts::LegacyEachItemMemberMutationRoot {
                item_symbol,
                raw_param,
            }) => ReferenceSemantics::LegacyEachItemMemberMutationRoot {
                item_sym: *item_symbol,
                raw_param: *raw_param,
            },
            Some(ReferenceFacts::EachItemMemberMutationStoreInvalidate {
                item_symbol: item_sym,
                collection_store,
                raw_param,
            }) => ReferenceSemantics::EachItemMemberMutationStoreInvalidate {
                item_sym: *item_sym,
                collection_store: *collection_store,
                raw_param: *raw_param,
            },
            Some(ReferenceFacts::EachItemIndexedLegacy { item_symbol }) => {
                let index_sym = self.each_item_index_legacy(*item_symbol);
                ReferenceSemantics::EachItemIndexedLegacy {
                    item_sym: *item_symbol,
                    index_sym,
                    index_read: self.each_index_read_legacy(index_sym),
                }
            }
            Some(ReferenceFacts::IllegalWrite) => ReferenceSemantics::IllegalWrite,
            Some(ReferenceFacts::Proxy) => ReferenceSemantics::Proxy,
            None => ReferenceSemantics::NonReactive,
        }
    }

    pub(crate) fn record_prop_member_mutation_root_refs(
        &mut self,
        refs: rustc_hash::FxHashSet<ReferenceId>,
    ) {
        self.prop_member_mutation_root_refs = refs;
    }

    pub(crate) fn is_prop_member_mutation_root_ref(&self, ref_id: ReferenceId) -> bool {
        self.prop_member_mutation_root_refs.contains(&ref_id)
    }

    pub(crate) fn set_uses_runes(&mut self, uses_runes: bool) {
        self.uses_runes = uses_runes;
    }

    pub(crate) fn set_runes_mode(&mut self, runes_mode: RunesMode) {
        self.runes_mode = runes_mode;
    }

    pub(crate) fn set_svelte_store_rune_import(&mut self, symbol: Option<SymbolId>) {
        self.svelte_store_rune_import = symbol;
    }

    pub(crate) fn svelte_store_rune_import(&self) -> Option<SymbolId> {
        self.svelte_store_rune_import
    }

    pub(crate) fn set_const_tag_order_legacy(
        &mut self,
        order: Vec<SmallVec<[NodeId; 4]>>,
        cycle: Option<ConstTagCycleFactLegacy>,
    ) {
        self.const_tag_order_legacy = order;
        self.const_tag_cycle_legacy = cycle;
    }

    pub(crate) fn const_tags_in_order_legacy(&self, fragment: FragmentId) -> &[NodeId] {
        self.const_tag_order_legacy
            .get(fragment.0 as usize)
            .map(|tags| tags.as_slice())
            .unwrap_or(&[])
    }

    pub(crate) fn const_tag_cycle_legacy(&self) -> Option<&ConstTagCycleFactLegacy> {
        self.const_tag_cycle_legacy.as_ref()
    }

    pub(crate) fn binding_facts(&self, sym: SymbolId) -> Option<BindingFacts> {
        self.lookup_binding_facts(sym).cloned()
    }

    pub(crate) fn binding_facts_mut(&mut self, sym: SymbolId) -> Option<&mut BindingFacts> {
        self.bindings.get_mut(sym).and_then(|slot| slot.as_mut())
    }

    pub(crate) fn demote_store_shadowed_rune(&mut self, decl_node: OxcNodeId, symbol: SymbolId) {
        if let Some(slot) = self.declarators.get_mut(decl_node) {
            *slot = None;
        }
        if let Some(slot) = self.bindings.get_mut(symbol) {
            *slot = None;
        }
    }

    pub(crate) fn record_state_binding(
        &mut self,
        sym: SymbolId,
        semantics: StateDeclarationSemantics,
    ) {
        self.write_binding(sym, BindingFacts::State(semantics));
    }

    pub(crate) fn record_optimized_rune_binding(
        &mut self,
        sym: SymbolId,
        semantics: OptimizedRuneSemantics,
    ) {
        self.write_binding(sym, BindingFacts::OptimizedRune(semantics));
    }

    pub(crate) fn record_derived_binding(
        &mut self,
        sym: SymbolId,
        semantics: DerivedDeclarationSemantics,
    ) {
        self.write_binding(
            sym,
            BindingFacts::Derived(DerivedFacts {
                decl: semantics,
                reactive: true,
            }),
        );
    }

    pub(crate) fn derived_reactive(&self, sym: SymbolId) -> bool {
        match self.lookup_binding_facts(sym) {
            Some(BindingFacts::Derived(d)) => d.reactive,
            _ => false,
        }
    }

    pub(crate) fn record_prop_binding(&mut self, sym: SymbolId, semantics: PropBindingSemantics) {
        if semantics.bindable {
            self.has_bindable_prop = true;
        }
        self.write_binding(sym, BindingFacts::Prop(semantics));
    }

    pub(crate) fn record_legacy_bindable_prop_binding(
        &mut self,
        sym: SymbolId,
        semantics: LegacyBindablePropSemantics,
    ) {
        self.write_binding(sym, BindingFacts::LegacyBindableProp(semantics));
    }

    pub(crate) fn record_legacy_state_binding(
        &mut self,
        sym: SymbolId,
        semantics: LegacyStateSemantics,
    ) {
        self.write_binding(sym, BindingFacts::LegacyState(semantics));
    }

    pub(crate) fn record_store_binding(&mut self, sym: SymbolId, semantics: StoreBindingSemantics) {
        let base = semantics.base_symbol;
        let store = semantics.store_symbol;
        self.write_binding(sym, BindingFacts::Store(semantics));
        self.store_declaration_symbols.push(sym);
        self.base_to_store.insert(base, store);
    }

    pub(crate) fn store_shadow_of_internal(&self, base: SymbolId) -> Option<SymbolId> {
        self.base_to_store.get(&base).copied()
    }

    pub(crate) fn record_const_binding(
        &mut self,
        sym: SymbolId,
        destructured: bool,
        initial_is_function: bool,
        owner_node: NodeId,
    ) {
        self.write_binding(
            sym,
            BindingFacts::Const(ConstTagSemantics {
                destructured,
                initial_is_function,
                owner_node,
            }),
        );
    }

    pub(crate) fn record_declaration_tag_binding(&mut self, sym: SymbolId) {
        self.write_binding(sym, BindingFacts::DeclarationTag);
    }

    pub(crate) fn optimize_const_binding(&mut self, sym: SymbolId) {
        let Some(slot) = self.bindings.get_mut(sym) else {
            return;
        };
        match slot {
            Some(BindingFacts::Const(kind)) => *slot = Some(BindingFacts::OptimizedConst(*kind)),
            Some(BindingFacts::DeclarationTag) => {
                *slot = Some(BindingFacts::OptimizedDeclarationTag)
            }
            _ => {}
        }
    }

    pub(crate) fn set_legacy_state_signal_source(&mut self, sym: SymbolId, is_signal_source: bool) {
        if let Some(Some(BindingFacts::LegacyState(state))) = self.bindings.get_mut(sym) {
            state.is_signal_source = is_signal_source;
        }
    }

    pub(crate) fn set_derived_reactive(&mut self, sym: SymbolId, reactive: bool) {
        if let Some(Some(BindingFacts::Derived(d))) = self.bindings.get_mut(sym) {
            d.reactive = reactive;
        }
    }

    pub(crate) fn optimize_derived_rune(&mut self, symbols: &[SymbolId]) {
        for &symbol in symbols {
            let Some(slot) = self.bindings.get_mut(symbol) else {
                continue;
            };
            if let Some(BindingFacts::Derived(derived)) = slot {
                let derived = *derived;
                *slot = Some(BindingFacts::OptimizedDerived(derived));
            }
        }
    }

    pub(crate) fn promote_legacy_api_export_to_state(
        &mut self,
        symbols: &[SymbolId],
        semantics: LegacyStateSemantics,
    ) {
        for &symbol in symbols {
            let Some(slot) = self.bindings.get_mut(symbol) else {
                continue;
            };
            if let Some(BindingFacts::LegacyApiExport) = slot {
                *slot = Some(BindingFacts::LegacyState(semantics));
            }
        }
    }

    pub(crate) fn record_runtime_rune_binding(&mut self, sym: SymbolId, kind: RuntimeRuneKind) {
        self.write_binding(sym, BindingFacts::RuntimeRune { kind });
    }

    pub(crate) fn record_contextual_binding(
        &mut self,
        sym: SymbolId,
        semantics: ContextualBindingSemantics,
    ) {
        self.write_binding(sym, BindingFacts::Contextual(semantics));
    }

    pub(crate) fn record_carrier_alias_binding(&mut self, sym: SymbolId, carrier: SymbolId) {
        self.write_binding(sym, BindingFacts::CarrierAlias { carrier });
    }

    pub(crate) fn record_let_direct_sym(&mut self, sym: SymbolId) {
        self.write_binding(
            sym,
            BindingFacts::Contextual(ContextualBindingSemantics::LetDirectiveDirect),
        );
    }

    fn write_binding(&mut self, sym: SymbolId, facts: BindingFacts) {
        let idx = sym.index();
        if idx >= self.bindings.len() {
            self.bindings.resize_with(idx + 1, || None);
        }
        self.bindings[sym] = Some(facts);
    }

    fn lookup_binding_facts(&self, sym: SymbolId) -> Option<&BindingFacts> {
        self.bindings.get(sym).and_then(|slot| slot.as_ref())
    }

    fn lookup_reference_facts(&self, ref_id: ReferenceId) -> Option<&ReferenceFacts> {
        self.reference_facts
            .get(ref_id)
            .and_then(|slot| slot.as_ref())
    }

    pub(crate) fn reference_facts(&self, ref_id: ReferenceId) -> Option<&ReferenceFacts> {
        self.lookup_reference_facts(ref_id)
    }

    pub(crate) fn record_contextual_owner(&mut self, sym: SymbolId, owner_node: NodeId) {
        self.contextual_owner.insert(sym, owner_node);
    }

    pub(crate) fn contextual_owner(&self, sym: SymbolId) -> Option<NodeId> {
        self.contextual_owner.get(&sym).copied()
    }

    pub(crate) fn record_raw_param_read(&mut self, ref_id: ReferenceId) {
        self.raw_param_reads.insert(ref_id);
    }

    pub(crate) fn is_raw_param_read(&self, ref_id: ReferenceId) -> bool {
        self.raw_param_reads.contains(&ref_id)
    }

    pub(crate) fn record_element_local_derived_read(&mut self, ref_id: ReferenceId) {
        self.element_local_derived_reads.insert(ref_id);
    }

    pub(crate) fn is_element_local_derived_read(&self, ref_id: ReferenceId) -> bool {
        self.element_local_derived_reads.contains(&ref_id)
    }

    pub(crate) fn record_detached_const_read(&mut self, ref_id: ReferenceId) {
        self.detached_const_reads.insert(ref_id);
    }

    pub(crate) fn is_detached_const_read(&self, ref_id: ReferenceId) -> bool {
        self.detached_const_reads.contains(&ref_id)
    }

    pub(crate) fn add_each_item_indirect_source(
        &mut self,
        item_sym: SymbolId,
        source_sym: SymbolId,
    ) {
        let entry = self.each_item_indirect_sources.entry(item_sym).or_default();
        if !entry.contains(&source_sym) {
            entry.push(source_sym);
        }
    }

    pub(crate) fn each_item_indirect_sources(&self, item_sym: SymbolId) -> Option<&[SymbolId]> {
        self.each_item_indirect_sources
            .get(&item_sym)
            .map(|v| v.as_slice())
    }

    pub(crate) fn iter_each_item_indirect_sources(
        &self,
    ) -> impl Iterator<Item = (SymbolId, &[SymbolId])> + '_ {
        self.each_item_indirect_sources
            .iter()
            .map(|(item_sym, sources)| (*item_sym, sources.as_slice()))
    }

    pub(crate) fn add_legacy_indirect_binding(
        &mut self,
        root_sym: SymbolId,
        indirect_sym: SymbolId,
    ) {
        let entry = self.legacy_indirect_bindings.entry(root_sym).or_default();
        if !entry.contains(&indirect_sym) {
            entry.push(indirect_sym);
        }
    }

    pub(crate) fn legacy_indirect_bindings(&self, root_sym: SymbolId) -> Option<&[SymbolId]> {
        self.legacy_indirect_bindings
            .get(&root_sym)
            .map(|v| v.as_slice())
    }

    pub(crate) fn set_each_item_collection_store(
        &mut self,
        item_sym: SymbolId,
        store_sym: SymbolId,
    ) {
        self.each_item_collection_store.insert(item_sym, store_sym);
    }

    pub(crate) fn each_item_collection_store(&self, item_sym: SymbolId) -> Option<SymbolId> {
        self.each_item_collection_store.get(&item_sym).copied()
    }

    pub(crate) fn each_block_iterates_store(&self, each_id: NodeId) -> bool {
        self.contextual_owner.iter().any(|(item_sym, owner)| {
            *owner == each_id && self.each_item_collection_store.contains_key(item_sym)
        })
    }

    pub(crate) fn set_each_item_index_legacy(&mut self, item_sym: SymbolId, index_sym: SymbolId) {
        self.each_item_index_legacy.insert(item_sym, index_sym);
    }

    fn each_item_index_legacy(&self, item_sym: SymbolId) -> Option<SymbolId> {
        self.each_item_index_legacy.get(&item_sym).copied()
    }

    fn each_index_read_legacy(&self, index_sym: Option<SymbolId>) -> EachIndexStrategy {
        match index_sym.and_then(|sym| self.lookup_binding_facts(sym)) {
            Some(BindingFacts::Contextual(ContextualBindingSemantics::EachIndex(strategy))) => {
                *strategy
            }
            _ => EachIndexStrategy::Direct,
        }
    }

    pub(super) fn mark_each_rest(&mut self, sym: SymbolId) {
        self.each_rest_symbols.insert(sym);
    }

    pub(crate) fn is_each_rest(&self, sym: SymbolId) -> bool {
        self.each_rest_symbols.contains(&sym)
    }

    pub(crate) fn record_legacy_props_object_binding(&mut self, sym: SymbolId) {
        self.write_binding(sym, BindingFacts::LegacyPropsObject);
    }

    pub(crate) fn record_reference_semantics(
        &mut self,
        ref_id: ReferenceId,
        semantics: ReferenceFacts,
    ) {
        let idx = ref_id.index();
        if idx >= self.reference_facts.len() {
            self.reference_facts.resize_with(idx + 1, || None);
        }
        self.reference_facts[ref_id] = Some(semantics);
    }

    pub(crate) fn clear_reference_semantics(&mut self, ref_id: ReferenceId) {
        if let Some(slot) = self.reference_facts.get_mut(ref_id) {
            *slot = None;
        }
    }

    pub(crate) fn record_let_carrier_binding(
        &mut self,
        stmt_node_id: OxcNodeId,
        carrier_symbol: SymbolId,
    ) {
        self.write_declarator(
            stmt_node_id,
            DeclaratorSemantics::LetCarrier {
                carrier_symbol: Some(carrier_symbol),
            },
        );
    }

    pub(crate) fn record_let_simple_binding(&mut self, stmt_node_id: OxcNodeId) {
        self.write_declarator(
            stmt_node_id,
            DeclaratorSemantics::LetCarrier {
                carrier_symbol: None,
            },
        );
    }

    pub(crate) fn record_declarator_semantics(
        &mut self,
        node_id: OxcNodeId,
        semantics: DeclaratorSemantics,
    ) {
        self.write_declarator(node_id, semantics);
    }

    fn write_declarator(&mut self, node_id: OxcNodeId, semantics: DeclaratorSemantics) {
        let idx = node_id.index();
        if idx >= self.declarators.len() {
            self.declarators.resize_with(idx + 1, || None);
        }
        self.declarators[node_id] = Some(semantics);
    }
}

impl ReactivitySemantics {
    fn binding_semantics_from_facts(facts: &BindingFacts) -> BindingSemantics {
        match facts {
            BindingFacts::State(state) => BindingSemantics::State(*state),
            BindingFacts::Derived(derived) => BindingSemantics::Derived(derived.decl),
            BindingFacts::OptimizedDerived(derived) => {
                BindingSemantics::OptimizedDerived(derived.decl)
            }
            BindingFacts::OptimizedRune(opt) => BindingSemantics::OptimizedRune(*opt),
            BindingFacts::Prop(prop) => BindingSemantics::Prop(prop.clone()),
            BindingFacts::LegacyBindableProp(legacy) => {
                BindingSemantics::LegacyBindableProp(*legacy)
            }
            BindingFacts::LegacyApiExport => BindingSemantics::LegacyApiExport,
            BindingFacts::LegacyPropsObject => BindingSemantics::LegacyPropsObject,
            BindingFacts::LegacyState(legacy) => BindingSemantics::LegacyState(*legacy),
            BindingFacts::Store(store) => BindingSemantics::Store(*store),
            BindingFacts::Const(kind) => BindingSemantics::Const(*kind),
            BindingFacts::OptimizedConst(kind) => BindingSemantics::OptimizedConst(*kind),
            BindingFacts::DeclarationTag => BindingSemantics::DeclarationTag,
            BindingFacts::OptimizedDeclarationTag => BindingSemantics::OptimizedDeclarationTag,
            BindingFacts::Contextual(kind) => BindingSemantics::Contextual(*kind),
            BindingFacts::RuntimeRune { kind } => BindingSemantics::RuntimeRune { kind: *kind },
            BindingFacts::CarrierAlias { carrier } => BindingSemantics::Contextual(
                ContextualBindingSemantics::LetDirectiveCarrierMember {
                    carrier_symbol: *carrier,
                },
            ),
        }
    }
}
