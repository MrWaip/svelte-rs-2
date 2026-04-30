use super::super::data::{
    BindingFacts, ContextualReadSemantics, DerivedKind, PropBindingKind, PropDefaultLowering,
    PropLoweringMode, PropReferenceSemantics, ReferenceFacts, SignalReferenceKind, StateKind,
};
use crate::scope::SymbolId;
use crate::types::data::AnalysisData;

use super::contextual;

pub(super) fn collect_symbol_semantics(data: &mut AnalysisData) {
    let symbols: Vec<SymbolId> = data.scoping.symbol_ids().collect();

    for sym in symbols {
        let Some(declaration) = symbol_declaration_facts(data, sym) else {
            continue;
        };

        let ref_facts: Vec<_> = data
            .scoping
            .get_resolved_reference_ids(sym)
            .iter()
            .copied()
            .filter_map(|ref_id| {
                let reference = data.scoping.get_reference(ref_id);
                let is_member_mutation_root =
                    data.reactivity.is_prop_member_mutation_root_ref(ref_id);
                classify_reference_semantics(
                    data,
                    sym,
                    &declaration,
                    reference.is_read(),
                    reference.is_write(),
                    is_member_mutation_root,
                )
                .map(|semantics| (ref_id, semantics))
            })
            .collect();

        for (ref_id, semantics) in ref_facts {
            data.reactivity
                .record_reference_semantics(ref_id, semantics);
        }
    }
}

fn symbol_declaration_facts(data: &AnalysisData, sym: SymbolId) -> Option<BindingFacts> {
    data.reactivity.binding_facts(sym)
}

fn classify_reference_semantics(
    data: &AnalysisData,
    sym: SymbolId,
    declaration: &BindingFacts,
    is_read: bool,
    is_write: bool,
    is_member_mutation_root: bool,
) -> Option<ReferenceFacts> {
    match declaration {
        BindingFacts::OptimizedRune(opt) => {
            if is_read && opt.proxy_init {
                Some(ReferenceFacts::Proxy)
            } else {
                None
            }
        }
        BindingFacts::State(state) => {
            if state.kind == StateKind::StateEager {
                return None;
            }
            if is_write && is_read {
                Some(ReferenceFacts::SignalUpdate {
                    kind: state.kind,
                    safe: state.var_declared,
                })
            } else if is_write {
                Some(ReferenceFacts::SignalWrite { kind: state.kind })
            } else if is_read && data.scoping.is_mutated(sym) {
                Some(ReferenceFacts::SignalRead {
                    kind: SignalReferenceKind::State(state.kind),
                    safe: state.var_declared,
                })
            } else {
                None
            }
        }
        BindingFacts::Derived(derived) => {
            if is_write {
                Some(ReferenceFacts::IllegalWrite)
            } else if is_read {
                Some(ReferenceFacts::SignalRead {
                    kind: SignalReferenceKind::Derived(derived.kind),
                    safe: false,
                })
            } else {
                None
            }
        }
        BindingFacts::Store(_) => None,
        BindingFacts::Prop(prop) => match &prop.kind {
            PropBindingKind::Source {
                bindable,
                updated,
                default_lowering,
                ..
            } => {
                if is_member_mutation_root {
                    Some(ReferenceFacts::PropSourceMemberMutationRoot {
                        bindable: *bindable,
                        symbol: sym,
                    })
                } else if is_write {
                    Some(ReferenceFacts::PropMutation {
                        bindable: *bindable,
                        symbol: sym,
                    })
                } else if is_read {
                    let reads_as_source = !bindable
                        || *updated
                        || !matches!(default_lowering, PropDefaultLowering::None);
                    if reads_as_source {
                        Some(ReferenceFacts::PropRead(PropReferenceSemantics::Source {
                            bindable: *bindable,
                            lowering_mode: prop.lowering_mode,
                            symbol: sym,
                        }))
                    } else {
                        Some(ReferenceFacts::PropRead(
                            PropReferenceSemantics::NonSource { symbol: sym },
                        ))
                    }
                } else {
                    None
                }
            }
            PropBindingKind::NonSource => {
                if is_member_mutation_root {
                    Some(ReferenceFacts::PropNonSourceMemberMutationRoot { symbol: sym })
                } else if is_write {
                    Some(ReferenceFacts::IllegalWrite)
                } else if is_read {
                    Some(ReferenceFacts::PropRead(
                        PropReferenceSemantics::NonSource { symbol: sym },
                    ))
                } else {
                    None
                }
            }
            PropBindingKind::Rest | PropBindingKind::Identifier => None,
        },

        BindingFacts::RuntimeRune { .. } => {
            if is_write {
                Some(ReferenceFacts::IllegalWrite)
            } else {
                None
            }
        }
        BindingFacts::Const(_) => {
            if is_write {
                Some(ReferenceFacts::IllegalWrite)
            } else if is_read {
                if let Some(owner_node) = data.reactivity.const_alias_owner_internal(sym) {
                    Some(ReferenceFacts::ConstAliasRead { owner_node })
                } else {
                    Some(ReferenceFacts::SignalRead {
                        kind: SignalReferenceKind::Derived(DerivedKind::Derived),
                        safe: false,
                    })
                }
            } else {
                None
            }
        }
        BindingFacts::Contextual(kind) => {
            if is_write {
                return Some(ReferenceFacts::IllegalWrite);
            }
            if !is_read {
                return None;
            }
            if is_member_mutation_root && data.reactivity.each_item_indirect_sources(sym).is_some()
            {
                return Some(ReferenceFacts::LegacyEachItemMemberMutationRoot { item_sym: sym });
            }
            let owner_node = data.reactivity.contextual_owner(sym)?;
            let read_kind = contextual::classify_contextual_read_kind(data, sym, *kind);
            Some(ReferenceFacts::ContextualRead(ContextualReadSemantics {
                kind: read_kind,
                owner_node,
                symbol: sym,
            }))
        }

        BindingFacts::LegacyState(state) => {
            if is_member_mutation_root {
                Some(ReferenceFacts::LegacyStateMemberMutationRoot { symbol: sym })
            } else if is_write && is_read {
                Some(ReferenceFacts::LegacyStateUpdate {
                    safe: state.var_declared,
                })
            } else if is_write {
                Some(ReferenceFacts::LegacyStateWrite)
            } else if is_read {
                Some(ReferenceFacts::LegacyStateRead {
                    safe: state.var_declared,
                })
            } else {
                None
            }
        }

        BindingFacts::LegacyBindableProp(_) => {
            if is_member_mutation_root {
                Some(ReferenceFacts::PropSourceMemberMutationRoot {
                    bindable: true,
                    symbol: sym,
                })
            } else if is_write {
                Some(ReferenceFacts::PropMutation {
                    bindable: true,
                    symbol: sym,
                })
            } else if is_read {
                Some(ReferenceFacts::PropRead(PropReferenceSemantics::Source {
                    bindable: true,
                    lowering_mode: PropLoweringMode::Standard,
                    symbol: sym,
                }))
            } else {
                None
            }
        }
        BindingFacts::CarrierAlias { carrier } => {
            if is_write {
                return Some(ReferenceFacts::IllegalWrite);
            }
            if !is_read {
                return None;
            }
            Some(ReferenceFacts::CarrierMemberRead(
                super::super::data::CarrierMemberReadSemantics {
                    carrier_symbol: *carrier,
                    leaf_symbol: sym,
                },
            ))
        }
    }
}
