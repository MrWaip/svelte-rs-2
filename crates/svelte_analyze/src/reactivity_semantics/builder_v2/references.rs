//! Reference-level classification: maps each resolved `ReferenceId` to a
//! `V2ReferenceFacts` operation-level answer.
//!
//! Consumers of `reactivity_semantics::reference_semantics(ref_id)` get back
//! `ReferenceSemantics` which is derived from these facts. The root consumer
//! entry point is `collect_symbol_semantics` — run once per build, iterating
//! every symbol and every resolved reference.

use svelte_component_semantics::OxcNodeId;

use super::super::data::{
    ContextualReadSemantics, DerivedKind, PropDeclarationKind, PropDeclarationSemantics,
    PropDefaultLowering, PropReferenceSemantics, SignalReferenceKind, V2DeclarationFacts,
    V2ReferenceFacts,
};
use crate::scope::SymbolId;
use crate::types::data::AnalysisData;

use super::contextual;
use super::prop_binding_kind;

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
                .record_reference_semantics_v2(ref_id, semantics);
        }
    }
}

fn symbol_declaration_facts(data: &AnalysisData, sym: SymbolId) -> Option<V2DeclarationFacts> {
    // Props lookup by symbol — covers both leaf bindings of destructure
    // patterns and the `const props = $props()` identifier form. The v2
    // builder records `prop_facts` per-symbol with the post-`finish()`
    // `updated` flag already applied, while `declaration_facts_v2` is keyed
    // by node and may not reflect late-`finish` mutation updates for
    // non-leaf symbols.
    if let Some(facts) = data.reactivity.prop_facts(sym) {
        return Some(V2DeclarationFacts::Prop(PropDeclarationSemantics {
            lowering_mode: facts.lowering_mode,
            kind: prop_binding_kind(&facts),
        }));
    }

    let node_id: OxcNodeId = data.scoping.symbol_declaration(sym);
    data.reactivity.declaration_facts_v2(node_id)
}

fn classify_reference_semantics(
    data: &AnalysisData,
    sym: SymbolId,
    declaration: &V2DeclarationFacts,
    is_read: bool,
    is_write: bool,
    is_member_mutation_root: bool,
) -> Option<V2ReferenceFacts> {
    match declaration {
        // Binding lowers as a plain `let`; reference-level reads are plain
        // identifiers. Child-passing consumers (boundary / component props /
        // render callee / dynamicity) branch on `DeclarationSemantics::OptimizedRune`
        // at the passing site, not here.
        V2DeclarationFacts::OptimizedRune(_) => None,
        V2DeclarationFacts::State(state) => {
            if is_write && is_read {
                Some(V2ReferenceFacts::SignalUpdate {
                    kind: state.kind,
                    safe: state.var_declared,
                })
            } else if is_write {
                Some(V2ReferenceFacts::SignalWrite { kind: state.kind })
            } else if is_read && data.scoping.is_mutated(sym) {
                Some(V2ReferenceFacts::SignalRead {
                    kind: SignalReferenceKind::State(state.kind),
                    safe: state.var_declared,
                })
            } else {
                None
            }
        }
        V2DeclarationFacts::Derived(derived) => {
            if is_write {
                Some(V2ReferenceFacts::IllegalWrite)
            } else if is_read {
                Some(V2ReferenceFacts::SignalRead {
                    kind: SignalReferenceKind::Derived(derived.kind),
                    safe: false,
                })
            } else {
                None
            }
        }
        V2DeclarationFacts::Store(_) => None,
        V2DeclarationFacts::Prop(prop) => match &prop.kind {
            PropDeclarationKind::Source {
                bindable,
                updated,
                default_lowering,
                ..
            } => {
                if is_member_mutation_root {
                    // `foo.x = val` / `foo.x++` where `foo` is a prop source.
                    // Root identifier carries is_read on OXC side; give consumers
                    // a direct operation-level answer so they don't re-check AST.
                    Some(V2ReferenceFacts::PropSourceMemberMutationRoot {
                        bindable: *bindable,
                        symbol: sym,
                    })
                } else if is_write {
                    Some(V2ReferenceFacts::PropMutation {
                        bindable: *bindable,
                        symbol: sym,
                    })
                } else if is_read {
                    let reads_as_source = !bindable
                        || *updated
                        || !matches!(default_lowering, PropDefaultLowering::None);
                    if reads_as_source {
                        Some(V2ReferenceFacts::PropRead(PropReferenceSemantics::Source {
                            bindable: *bindable,
                            lowering_mode: prop.lowering_mode,
                            symbol: sym,
                        }))
                    } else {
                        Some(V2ReferenceFacts::PropRead(
                            PropReferenceSemantics::NonSource { symbol: sym },
                        ))
                    }
                } else {
                    None
                }
            }
            PropDeclarationKind::NonSource => {
                if is_member_mutation_root {
                    Some(V2ReferenceFacts::PropNonSourceMemberMutationRoot { symbol: sym })
                } else if is_write {
                    Some(V2ReferenceFacts::IllegalWrite)
                } else if is_read {
                    Some(V2ReferenceFacts::PropRead(
                        PropReferenceSemantics::NonSource { symbol: sym },
                    ))
                } else {
                    None
                }
            }
            PropDeclarationKind::Rest
            | PropDeclarationKind::Identifier
            | PropDeclarationKind::Object { .. } => None,
        },
        // Binding-form runtime runes ($props.id, $effect.tracking, $host,
        // $effect.pending, $inspect.trace): reads stay as *plain identifier*
        // accesses at the transform layer — no `$.get(...)` wrap. Dynamism
        // classification still flags them through `DeclarationSemantics::
        // RuntimeRune`, independently of reference_semantics. Writes would
        // target a `const` binding — forbid them.
        V2DeclarationFacts::RuntimeRune { .. } => {
            if is_write {
                Some(V2ReferenceFacts::IllegalWrite)
            } else {
                None
            }
        }
        V2DeclarationFacts::Const(_) => {
            if is_write {
                Some(V2ReferenceFacts::IllegalWrite)
            } else if is_read {
                if let Some(owner_node) = data.reactivity.const_alias_owner_v2_internal(sym) {
                    Some(V2ReferenceFacts::ConstAliasRead { owner_node })
                } else {
                    Some(V2ReferenceFacts::SignalRead {
                        kind: SignalReferenceKind::Derived(DerivedKind::Derived),
                        safe: false,
                    })
                }
            } else {
                None
            }
        }
        V2DeclarationFacts::Contextual(kind) => {
            if is_write {
                return Some(V2ReferenceFacts::IllegalWrite);
            }
            if !is_read {
                return None;
            }
            let owner_node = data.reactivity.contextual_owner_v2(sym)?;
            let read_kind = contextual::classify_contextual_read_kind(data, sym, *kind);
            Some(V2ReferenceFacts::ContextualRead(ContextualReadSemantics {
                kind: read_kind,
                owner_node,
                symbol: sym,
            }))
        }
        // LetCarrier is the synthesized carrier itself (declared on the
        // destructuring statement). Reference-level reads of its symbol
        // don't appear at this identity — leaves carry `CarrierAlias`
        // below and resolve to `CarrierMemberRead`. Nothing to emit here.
        V2DeclarationFacts::LetCarrier { .. } => None,
        V2DeclarationFacts::CarrierAlias { carrier } => {
            if is_write {
                return Some(V2ReferenceFacts::IllegalWrite);
            }
            if !is_read {
                return None;
            }
            Some(V2ReferenceFacts::CarrierMemberRead(
                super::super::data::CarrierMemberReadSemantics {
                    carrier_symbol: *carrier,
                    leaf_symbol: sym,
                },
            ))
        }
    }
}
