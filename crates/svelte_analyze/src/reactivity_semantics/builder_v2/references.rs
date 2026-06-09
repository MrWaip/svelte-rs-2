use super::super::data::{
    BindingFacts, ConstBindingSemantics, ContextualReadSemantics, DerivedKind, PropBindingKind,
    PropDefaultKind, PropEmitMode, PropReferenceSemantics, ReferenceFacts, SignalReferenceKind,
    StateKind,
};
use crate::scope::SymbolId;
use crate::types::data::{AnalysisData, JsAst};
use svelte_component_semantics::OriginKind;

fn prop_non_source_variant(data: &AnalysisData, sym: SymbolId) -> PropReferenceSemantics {
    let (alias, kind) = match data.binding_origin_key(sym) {
        Some(pair) => pair,
        None => return PropReferenceSemantics::NonSourceStatic { symbol: sym },
    };
    let computed = match kind {
        OriginKind::Numeric => true,
        OriginKind::Ident => false,
        OriginKind::String => !is_valid_js_identifier(alias.as_ref()),
    };
    if computed {
        PropReferenceSemantics::NonSourceComputed { symbol: sym }
    } else {
        PropReferenceSemantics::NonSourceStatic { symbol: sym }
    }
}

fn is_valid_js_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}
use oxc_ast::ast::IdentifierReference;
use oxc_ast_visit::Visit;
use svelte_ast::{Component, Node};
use svelte_component_semantics::ReferenceId;

use super::contextual;

pub(super) fn collect_each_key_contextual_reads<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    for node in component.store.iter_nodes() {
        let Node::EachBlock(block) = node else {
            continue;
        };
        let Some(key) = block.key.as_ref() else {
            continue;
        };
        let Some(expr) = parsed.expr(key.id()) else {
            continue;
        };
        let mut collector = EachKeyRefCollector {
            data,
            each_block_id: block.id,
        };
        collector.visit_expression(expr);
    }
}

struct EachKeyRefCollector<'d, 'a> {
    data: &'d mut AnalysisData<'a>,
    each_block_id: svelte_ast::NodeId,
}

impl<'a> Visit<'a> for EachKeyRefCollector<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        if let Some(ref_id) = ident.reference_id.get() {
            self.data
                .reactivity
                .record_contextual_read_in_each_key(ref_id, self.each_block_id);
        }
    }
}

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
                if matches!(
                    data.reactivity.reference_facts(ref_id),
                    Some(super::super::data::ReferenceFacts::StoreRead { .. })
                        | Some(super::super::data::ReferenceFacts::StoreWrite { .. })
                        | Some(super::super::data::ReferenceFacts::StoreUpdate { .. })
                ) {
                    return None;
                }
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
                    ref_id,
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
    ref_id: ReferenceId,
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
            let is_signal_source = data.script.is_state_source(
                data.scoping.is_mutated(sym) || data.scoping.is_reexported_specifier_local(sym),
            );
            if is_write && is_read {
                Some(ReferenceFacts::SignalUpdate {
                    kind: state.kind,
                    safe: state.var_declared,
                })
            } else if is_write {
                Some(ReferenceFacts::SignalWrite { kind: state.kind })
            } else if is_read && is_signal_source {
                Some(ReferenceFacts::SignalRead {
                    kind: SignalReferenceKind::State(state.kind),
                    safe: state.var_declared,
                })
            } else {
                None
            }
        }
        BindingFacts::Derived(derived) | BindingFacts::OptimizedDerived(derived) => {
            if is_write && !is_read {
                Some(ReferenceFacts::DerivedWrite)
            } else if is_write {
                Some(ReferenceFacts::IllegalWrite)
            } else if is_read {
                Some(ReferenceFacts::SignalRead {
                    kind: SignalReferenceKind::Derived(derived.decl.kind),
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
                    let reads_as_source =
                        !bindable || *updated || !matches!(default_lowering, PropDefaultKind::None);
                    if reads_as_source {
                        Some(ReferenceFacts::PropRead(PropReferenceSemantics::Source {
                            bindable: *bindable,
                            lowering_mode: prop.emit_mode,
                            symbol: sym,
                        }))
                    } else {
                        Some(ReferenceFacts::PropRead(prop_non_source_variant(data, sym)))
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
                    Some(ReferenceFacts::PropRead(prop_non_source_variant(data, sym)))
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
        BindingFacts::Const(ConstBindingSemantics::ConstTag {
            destructured,
            owner_node,
            ..
        }) => {
            if is_write {
                Some(ReferenceFacts::IllegalWrite)
            } else if is_read {
                if *destructured {
                    Some(ReferenceFacts::ConstAliasRead {
                        owner_node: *owner_node,
                    })
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
            if is_member_mutation_root {
                if data.reactivity.each_item_indirect_sources(sym).is_some() {
                    return Some(ReferenceFacts::LegacyEachItemMemberMutationRoot {
                        item_symbol: sym,
                    });
                }
                if let Some(collection_store) = data.reactivity.each_item_collection_store(sym) {
                    return Some(ReferenceFacts::EachItemMemberMutationStoreInvalidate {
                        item_symbol: sym,
                        collection_store,
                    });
                }
            }
            let owner_node = data.reactivity.contextual_owner(sym)?;
            let read_kind = contextual::classify_contextual_read_kind(data, sym, *kind);
            let in_key_expression = data
                .reactivity
                .contextual_read_in_each_key(ref_id)
                .map(|each_block_id| each_block_id == owner_node)
                .unwrap_or(false);
            Some(ReferenceFacts::ContextualRead(ContextualReadSemantics {
                kind: read_kind,
                owner_node,
                symbol: sym,
                in_key_expression,
            }))
        }

        BindingFacts::LegacyState(state) => {
            let store_shadow = data.reactivity.store_shadow_of_internal(sym);
            if is_member_mutation_root {
                Some(ReferenceFacts::LegacyStateMemberMutationRoot { symbol: sym })
            } else if is_write && is_read {
                if let Some(store_symbol) = store_shadow {
                    Some(ReferenceFacts::LegacyStateSubscribedUpdate {
                        safe: state.var_declared,
                        store_symbol,
                    })
                } else {
                    Some(ReferenceFacts::LegacyStateUpdate {
                        safe: state.var_declared,
                    })
                }
            } else if is_write {
                if let Some(store_symbol) = store_shadow {
                    Some(ReferenceFacts::LegacyStateSubscribedWrite { store_symbol })
                } else {
                    Some(ReferenceFacts::LegacyStateWrite)
                }
            } else if is_read {
                if let Some(store_symbol) = store_shadow {
                    Some(ReferenceFacts::LegacyStateSubscribedRead {
                        safe: state.var_declared,
                        store_symbol,
                    })
                } else {
                    Some(ReferenceFacts::LegacyStateRead {
                        safe: state.var_declared,
                    })
                }
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
                    lowering_mode: PropEmitMode::Standard,
                    symbol: sym,
                }))
            } else {
                None
            }
        }
        BindingFacts::LegacyApiExport => {
            if is_write {
                Some(ReferenceFacts::IllegalWrite)
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
                    member_symbol: sym,
                },
            ))
        }
    }
}
