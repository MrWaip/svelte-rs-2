use super::super::data::{
    BindingFacts, ConstTagSemantics, ContextualBindingSemantics, ContextualReadSemantics,
    DerivedKind, EachItemStrategy, PropBindingKind, PropDefaultKind, PropEmitMode,
    PropReferenceSemantics, ReferenceFacts, SignalReadLocality, SignalReferenceKind, StateKind,
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
use oxc_ast::ast::{IdentifierReference, Statement};
use oxc_ast_visit::Visit;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use svelte_ast::{Component, FragmentId, FragmentRole, Node};
use svelte_component_semantics::{ReferenceId, walk_bindings};

use super::contextual;

pub(super) fn collect_raw_param_reads<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) -> Vec<ReferenceId> {
    let mut proxy_targets = Vec::new();
    for node in component.store.iter_nodes() {
        if let Node::EachBlock(block) = node
            && let Some(key) = block.key.as_ref()
            && let Some(expr) = parsed.expr(key.id())
        {
            let mut collector = EachKeyRawParamCollector {
                data,
                each_block_id: block.id,
            };
            collector.visit_expression(expr);
        }

        let (attrs, proxy_target): (&[svelte_ast::Attribute], bool) = match node {
            Node::Element(n) => (&n.attributes, false),
            Node::SvelteElement(n) => (&n.attributes, true),
            Node::ComponentNode(n) => (&n.attributes, true),
            Node::SvelteComponentLegacy(n) => (&n.attributes, true),
            Node::SvelteSelf(n) => (&n.attributes, true),
            _ => continue,
        };
        for attr in attrs {
            let svelte_ast::Attribute::BindDirective(d) = attr else {
                continue;
            };
            if d.name != "this" {
                continue;
            }
            let Some(expr) = parsed.expr(d.expression.id()) else {
                continue;
            };
            if proxy_target && let Some(root_ref) = super::util::expression_root_reference_id(expr)
            {
                proxy_targets.push(root_ref);
            }
            let mut collector = BindThisRawParamCollector { data };
            collector.visit_expression(expr);
        }
    }
    proxy_targets
}

pub(super) fn apply_bind_this_proxy_targets(
    data: &mut AnalysisData,
    proxy_targets: &[ReferenceId],
) {
    for &root_ref in proxy_targets {
        let Some(sym) = data.scoping.symbol_for_reference(root_ref) else {
            continue;
        };
        let proxyable_state = matches!(
            data.reactivity.binding_facts(sym),
            Some(BindingFacts::State(state)) if state.kind == StateKind::State
        );
        if proxyable_state {
            data.reactivity.set_signal_write_proxy(root_ref, true);
        }
    }
}

struct EachKeyRawParamCollector<'d, 'a> {
    data: &'d mut AnalysisData<'a>,
    each_block_id: svelte_ast::NodeId,
}

impl<'a> Visit<'a> for EachKeyRawParamCollector<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        let Some(sym) = self.data.scoping.symbol_for_reference(ref_id) else {
            return;
        };
        if self.data.reactivity.contextual_owner(sym) == Some(self.each_block_id) {
            self.data.reactivity.record_raw_param_read(ref_id);
        }
    }
}

struct BindThisRawParamCollector<'d, 'a> {
    data: &'d mut AnalysisData<'a>,
}

impl<'a> Visit<'a> for BindThisRawParamCollector<'_, 'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        let Some(ref_id) = ident.reference_id.get() else {
            return;
        };
        let Some(sym) = self.data.scoping.symbol_for_reference(ref_id) else {
            return;
        };
        let is_each_context = matches!(
            self.data.reactivity.binding_facts(sym),
            Some(BindingFacts::Contextual(
                ContextualBindingSemantics::EachItem(_) | ContextualBindingSemantics::EachIndex(_)
            ))
        );
        if is_each_context {
            self.data.reactivity.record_raw_param_read(ref_id);
        }
    }
}

fn derived_read_locality(data: &AnalysisData, ref_id: ReferenceId) -> SignalReadLocality {
    if data.reactivity.is_element_local_derived_read(ref_id) {
        SignalReadLocality::ElementFragmentLocal
    } else {
        SignalReadLocality::Cell
    }
}

pub(super) fn collect_element_local_derived_reads<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    let mut owning: FxHashMap<FragmentId, SmallVec<[SymbolId; 2]>> = FxHashMap::default();
    for node in component.store.iter_nodes() {
        let Node::DeclarationTag(tag) = node else {
            continue;
        };
        let Some(fragment) = component.store.node_fragment(tag.id) else {
            continue;
        };
        if component.store.fragment(fragment).role != FragmentRole::Element {
            continue;
        }
        let Some(Statement::VariableDeclaration(decl)) = parsed.stmt(tag.declaration.id()) else {
            continue;
        };
        for declarator in &decl.declarations {
            walk_bindings(&declarator.id, |visit| {
                if matches!(
                    data.reactivity.binding_facts(visit.symbol),
                    Some(BindingFacts::Derived(_) | BindingFacts::OptimizedDerived(_))
                ) {
                    owning.entry(fragment).or_default().push(visit.symbol);
                }
            });
        }
    }

    if owning.is_empty() {
        return;
    }

    for node in component.store.iter_nodes() {
        let Node::ExpressionTag(tag) = node else {
            continue;
        };
        let Some(fragment) = component.store.node_fragment(tag.id) else {
            continue;
        };
        let Some(owning_syms) = owning.get(&fragment) else {
            continue;
        };
        let Some(expr) = parsed.expr(tag.expression.id()) else {
            continue;
        };
        for ref_id in contextual::expression_reference_ids(expr) {
            let Some(sym) = data.scoping.symbol_for_reference(ref_id) else {
                continue;
            };
            if !owning_syms.contains(&sym) {
                continue;
            }
            let declaration_scope = data.scoping.semantics().symbol_scope_id(sym);
            if data.scoping.get_reference(ref_id).scope_id() == declaration_scope {
                data.reactivity.record_element_local_derived_read(ref_id);
            }
        }
    }
}

fn emit_parent_fragment(component: &Component, fragment: FragmentId) -> Option<FragmentId> {
    let frag = component.store.fragment(fragment);
    let owner = frag.owner?;
    if frag.role == FragmentRole::SnippetBody {
        let containing = component.store.node_fragment(owner)?;
        if component.store.fragment(containing).role == FragmentRole::ComponentChildren {
            let component_node = component.store.fragment(containing).owner?;
            return component.store.node_fragment(component_node);
        }
    }
    component.store.node_fragment(owner)
}

fn const_read_is_detached(
    component: &Component,
    declaration_fragment: FragmentId,
    read_fragment: FragmentId,
) -> bool {
    let mut cursor = Some(read_fragment);
    while let Some(fragment) = cursor {
        if fragment == declaration_fragment {
            return false;
        }
        cursor = emit_parent_fragment(component, fragment);
    }
    true
}

pub(super) fn collect_detached_const_reads<'a>(
    component: &Component,
    parsed: &JsAst<'a>,
    data: &mut AnalysisData<'a>,
) {
    for node in component.store.iter_nodes() {
        let Node::ExpressionTag(tag) = node else {
            continue;
        };
        let Some(read_fragment) = component.store.node_fragment(tag.id) else {
            continue;
        };
        let Some(expr) = parsed.expr(tag.expression.id()) else {
            continue;
        };
        for ref_id in contextual::expression_reference_ids(expr) {
            let Some(sym) = data.scoping.symbol_for_reference(ref_id) else {
                continue;
            };
            let owner_node = match data.reactivity.binding_facts(sym) {
                Some(BindingFacts::Const(c) | BindingFacts::OptimizedConst(c))
                    if !c.destructured =>
                {
                    c.owner_node
                }
                _ => continue,
            };
            let Some(declaration_fragment) = component.store.node_fragment(owner_node) else {
                continue;
            };
            if const_read_is_detached(component, declaration_fragment, read_fragment) {
                data.reactivity.record_detached_const_read(ref_id);
            }
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
            let is_signal_source = state.is_signal_source;
            let store_unsub = data.reactivity.store_shadow_of_internal(sym);
            if is_write && is_read {
                Some(ReferenceFacts::SignalUpdate {
                    kind: state.kind,
                    safe: state.var_declared,
                    proxy: false,
                    store_unsub,
                })
            } else if is_write {
                Some(ReferenceFacts::SignalWrite {
                    kind: state.kind,
                    proxy: false,
                    store_unsub,
                })
            } else if is_read && is_signal_source {
                Some(ReferenceFacts::SignalRead {
                    kind: SignalReferenceKind::State(state.kind),
                    safe: state.var_declared,
                    locality: SignalReadLocality::Cell,
                })
            } else {
                None
            }
        }
        BindingFacts::Derived(derived) | BindingFacts::OptimizedDerived(derived) => {
            if is_write && is_read {
                Some(ReferenceFacts::DerivedUpdate)
            } else if is_write {
                Some(ReferenceFacts::DerivedWrite)
            } else if is_read {
                Some(ReferenceFacts::SignalRead {
                    kind: SignalReferenceKind::Derived(derived.decl.kind),
                    safe: derived.decl.var_declared,
                    locality: derived_read_locality(data, ref_id),
                })
            } else {
                None
            }
        }
        BindingFacts::Store(_) => None,
        BindingFacts::LegacyPropsObject => None,
        BindingFacts::Prop(prop) => match &prop.kind {
            PropBindingKind::Source {
                updated,
                default_lowering,
                ..
            } => {
                let bindable = prop.bindable;
                if is_member_mutation_root {
                    Some(ReferenceFacts::PropSourceMemberMutationRoot {
                        bindable,
                        symbol: sym,
                    })
                } else if is_write {
                    Some(ReferenceFacts::PropMutation {
                        bindable,
                        symbol: sym,
                    })
                } else if is_read {
                    let reads_as_source =
                        !bindable || *updated || !matches!(default_lowering, PropDefaultKind::None);
                    if reads_as_source {
                        Some(ReferenceFacts::PropRead(PropReferenceSemantics::Source {
                            bindable,
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
        BindingFacts::Const(ConstTagSemantics {
            destructured,
            owner_node,
            ..
        })
        | BindingFacts::OptimizedConst(ConstTagSemantics {
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
                    let locality = if data.reactivity.is_detached_const_read(ref_id) {
                        SignalReadLocality::Detached
                    } else {
                        SignalReadLocality::Cell
                    };
                    Some(ReferenceFacts::SignalRead {
                        kind: SignalReferenceKind::Derived(DerivedKind::Derived),
                        safe: false,
                        locality,
                    })
                }
            } else {
                None
            }
        }
        BindingFacts::DeclarationTag | BindingFacts::OptimizedDeclarationTag => None,
        BindingFacts::Contextual(kind) => {
            if matches!(
                kind,
                ContextualBindingSemantics::EachItem(EachItemStrategy::IndexedLegacy)
            ) && !data.reactivity.is_raw_param_read(ref_id)
            {
                return Some(ReferenceFacts::EachItemIndexedLegacy { item_symbol: sym });
            }
            if is_write {
                if !is_member_mutation_root
                    && data.reactivity.each_item_indirect_sources(sym).is_some()
                {
                    return Some(ReferenceFacts::EachItemDestructuredWriteLegacy {
                        item_symbol: sym,
                    });
                }
                return Some(ReferenceFacts::IllegalWrite);
            }
            if !is_read {
                return None;
            }
            let raw_param = data.reactivity.is_raw_param_read(ref_id);
            if is_member_mutation_root {
                if data.reactivity.each_item_indirect_sources(sym).is_some() {
                    return Some(ReferenceFacts::LegacyEachItemMemberMutationRoot {
                        item_symbol: sym,
                        raw_param,
                    });
                }
                if let Some(collection_store) = data.reactivity.each_item_collection_store(sym) {
                    return Some(ReferenceFacts::EachItemMemberMutationStoreInvalidate {
                        item_symbol: sym,
                        collection_store,
                        raw_param,
                    });
                }
            }
            let owner_node = data.reactivity.contextual_owner(sym)?;
            let read_kind = contextual::classify_contextual_read_kind(data, sym, *kind, raw_param);
            Some(ReferenceFacts::ContextualRead(ContextualReadSemantics {
                kind: read_kind,
                owner_node,
                symbol: sym,
            }))
        }

        BindingFacts::LegacyState(state) => {
            let store_shadow = data.reactivity.store_shadow_of_internal(sym);
            let is_signal_source = state.is_signal_source;
            if !is_signal_source && store_shadow.is_none() {
                None
            } else if is_member_mutation_root {
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
