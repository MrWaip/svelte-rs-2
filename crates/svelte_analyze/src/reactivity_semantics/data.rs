use crate::scope::SymbolId;
use oxc_index::IndexVec;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::NodeId;
use svelte_component_semantics::{OxcNodeId, ReferenceId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindingSemantics {
    NonReactive,

    State(StateDeclarationSemantics),

    Derived(DerivedDeclarationSemantics),

    OptimizedRune(OptimizedRuneSemantics),

    Prop(PropBindingSemantics),

    LegacyBindableProp(LegacyBindablePropSemantics),

    LegacyState(LegacyStateSemantics),

    Store(StoreBindingSemantics),

    Const(ConstBindingSemantics),

    Contextual(ContextualBindingSemantics),

    RuntimeRune { kind: RuntimeRuneKind },

    Unresolved,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateDeclarationSemantics {
    pub kind: StateKind,
    pub proxied: bool,
    pub var_declared: bool,
    pub binding_semantics: SmallVec<[StateBindingSemantics; 4]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateBindingSemantics {
    StateSignal { proxied: bool },

    StateRawSignal,

    NonReactive { proxied: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeRuneKind {
    PropsId,

    EffectTracking,

    EffectPending,

    Host,

    InspectTrace,
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
    pub lowering: DerivedLowering,
    pub reactive: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivedKind {
    Derived,

    DerivedBy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivedLowering {
    Sync,

    Async,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropBindingSemantics {
    pub lowering_mode: PropLoweringMode,
    pub kind: PropBindingKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropLoweringMode {
    Standard,

    CustomElement,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropBindingKind {
    Identifier,

    Source {
        bindable: bool,
        updated: bool,
        default_lowering: PropDefaultLowering,

        default_needs_proxy: bool,
    },

    Rest,

    NonSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegacyBindablePropSemantics {
    pub default_lowering: PropDefaultLowering,
    pub flags: crate::PropsFlags,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegacyStateSemantics {
    pub var_declared: bool,
    pub immutable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropDefaultLowering {
    None,

    Eager,

    Lazy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoreBindingSemantics {
    pub base_symbol: SymbolId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstBindingSemantics {
    ConstTag { destructured: bool, reactive: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualBindingSemantics {
    EachItem(EachItemStrategy),

    EachIndex(EachIndexStrategy),

    AwaitValue,

    AwaitError,

    LetDirective,

    SnippetParam(SnippetParamStrategy),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachItemStrategy {
    Accessor,

    Signal,

    Direct,
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

    PropsIdentifier {
        sym: SymbolId,
    },

    PropsObject {
        leaves: SmallVec<[SymbolId; 4]>,
        has_rest: bool,
    },

    LegacyStateDestructure {
        leaves: SmallVec<[SymbolId; 4]>,
    },

    LetCarrier {
        carrier_symbol: SymbolId,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceSemantics {
    NonReactive,

    Proxy,

    SignalRead {
        kind: SignalReferenceKind,
        safe: bool,
    },

    SignalWrite {
        kind: StateKind,
    },

    SignalUpdate {
        kind: StateKind,
        safe: bool,
    },

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

    LegacyStateRead {
        safe: bool,
    },

    LegacyStateWrite,

    LegacyStateUpdate {
        safe: bool,
    },

    LegacyStateMemberMutationRoot {
        symbol: SymbolId,
    },

    LegacyReactiveImportRead,

    LegacyReactiveImportMemberMutationRoot {
        symbol: SymbolId,
    },

    LegacyEachItemMemberMutationRoot {
        item_sym: SymbolId,
    },

    IllegalWrite,

    Unresolved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalReferenceKind {
    State(StateKind),
    Derived(DerivedKind),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualReadSemantics {
    pub kind: ContextualReadKind,
    pub owner_node: NodeId,
    pub symbol: SymbolId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualReadKind {
    EachItem { accessor: bool, signal: bool },

    EachIndex { signal: bool },

    AwaitValue,

    AwaitError,

    LetDirective,

    SnippetParam { accessor: bool, signal: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarrierMemberReadSemantics {
    pub carrier_symbol: SymbolId,
    pub leaf_symbol: SymbolId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropReferenceSemantics {
    Source {
        bindable: bool,
        lowering_mode: PropLoweringMode,
        symbol: SymbolId,
    },

    NonSource {
        symbol: SymbolId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum BindingFacts {
    State(StateDeclarationSemantics),
    Derived(DerivedDeclarationSemantics),
    OptimizedRune(OptimizedRuneSemantics),
    Prop(PropBindingSemantics),

    LegacyBindableProp(LegacyBindablePropSemantics),

    LegacyState(LegacyStateSemantics),
    Store(StoreBindingSemantics),
    Const(ConstBindingSemantics),
    Contextual(ContextualBindingSemantics),
    RuntimeRune { kind: RuntimeRuneKind },

    CarrierAlias { carrier: SymbolId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ReferenceFacts {
    SignalRead {
        kind: SignalReferenceKind,
        safe: bool,
    },
    SignalWrite {
        kind: StateKind,
    },
    SignalUpdate {
        kind: StateKind,
        safe: bool,
    },
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

    LegacyStateRead {
        safe: bool,
    },

    LegacyStateWrite,

    LegacyStateUpdate {
        safe: bool,
    },
    LegacyStateMemberMutationRoot {
        symbol: SymbolId,
    },

    LegacyReactiveImportRead,

    LegacyReactiveImportMemberMutationRoot {
        symbol: SymbolId,
    },

    LegacyEachItemMemberMutationRoot {
        item_sym: SymbolId,
    },

    IllegalWrite,

    Proxy,
}

#[derive(Clone, Debug)]
pub struct ReactivitySemantics {
    bindings: IndexVec<SymbolId, Option<BindingFacts>>,

    declarators: IndexVec<OxcNodeId, Option<DeclaratorSemantics>>,

    store_declaration_symbols: Vec<SymbolId>,

    reference_facts: IndexVec<ReferenceId, Option<ReferenceFacts>>,

    prop_member_mutation_root_refs: rustc_hash::FxHashSet<ReferenceId>,

    contextual_owner: FxHashMap<SymbolId, NodeId>,

    each_rest_symbols: FxHashSet<SymbolId>,

    legacy_bindable_prop_symbols: Vec<SymbolId>,

    legacy_uses_props: bool,

    legacy_uses_rest_props: bool,

    legacy_has_member_mutated: bool,

    each_item_indirect_sources: FxHashMap<SymbolId, SmallVec<[SymbolId; 2]>>,

    const_alias_owner: FxHashMap<SymbolId, NodeId>,

    uses_runes: bool,

    legacy_reactive: super::legacy_reactive::LegacyReactivitySemantics,
}

impl ReactivitySemantics {
    pub fn new(node_count: u32) -> Self {
        let mut declarators: IndexVec<OxcNodeId, Option<DeclaratorSemantics>> =
            IndexVec::with_capacity(node_count as usize);
        declarators.resize_with(node_count as usize, || None);
        Self {
            bindings: IndexVec::new(),
            declarators,
            store_declaration_symbols: Vec::new(),
            reference_facts: IndexVec::new(),
            prop_member_mutation_root_refs: rustc_hash::FxHashSet::default(),
            contextual_owner: FxHashMap::default(),
            each_item_indirect_sources: FxHashMap::default(),
            const_alias_owner: FxHashMap::default(),
            each_rest_symbols: FxHashSet::default(),
            legacy_bindable_prop_symbols: Vec::new(),
            legacy_uses_props: false,
            legacy_uses_rest_props: false,
            legacy_has_member_mutated: false,
            uses_runes: false,
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

    pub fn binding_semantics(&self, sym: SymbolId) -> BindingSemantics {
        self.lookup_binding_facts(sym)
            .map(Self::binding_semantics_from_facts)
            .unwrap_or(BindingSemantics::NonReactive)
    }

    pub fn declarator_semantics(&self, decl_node: OxcNodeId) -> DeclaratorSemantics {
        self.declarators
            .get(decl_node)
            .and_then(|slot| slot.as_ref())
            .cloned()
            .unwrap_or(DeclaratorSemantics::None)
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

    pub fn has_store_bindings(&self) -> bool {
        !self.store_declaration_symbols.is_empty()
    }

    pub fn legacy_bindable_prop_symbols(&self) -> &[SymbolId] {
        &self.legacy_bindable_prop_symbols
    }

    pub fn has_legacy_bindable_prop(&self) -> bool {
        !self.legacy_bindable_prop_symbols.is_empty()
    }

    pub fn legacy_uses_props(&self) -> bool {
        self.legacy_uses_props
    }

    pub fn legacy_uses_rest_props(&self) -> bool {
        self.legacy_uses_rest_props
    }

    pub fn legacy_has_member_mutated(&self) -> bool {
        self.legacy_has_member_mutated
    }

    pub fn legacy_reactive(&self) -> &super::legacy_reactive::LegacyReactivitySemantics {
        &self.legacy_reactive
    }

    pub(crate) fn legacy_reactive_mut(
        &mut self,
    ) -> &mut super::legacy_reactive::LegacyReactivitySemantics {
        &mut self.legacy_reactive
    }

    pub(crate) fn record_legacy_bindable_prop_symbol(&mut self, symbol: SymbolId) {
        self.legacy_bindable_prop_symbols.push(symbol);
    }

    pub(crate) fn set_legacy_unresolved_usage(&mut self, uses_props: bool, uses_rest_props: bool) {
        self.legacy_uses_props = uses_props;
        self.legacy_uses_rest_props = uses_rest_props;
    }

    pub(crate) fn set_legacy_has_member_mutated(&mut self, value: bool) {
        self.legacy_has_member_mutated = value;
    }

    pub fn reference_semantics(&self, ref_id: ReferenceId) -> ReferenceSemantics {
        match self.lookup_reference_facts(ref_id) {
            Some(ReferenceFacts::SignalRead { kind, safe }) => ReferenceSemantics::SignalRead {
                kind: *kind,
                safe: *safe,
            },
            Some(ReferenceFacts::SignalWrite { kind }) => {
                ReferenceSemantics::SignalWrite { kind: *kind }
            }
            Some(ReferenceFacts::SignalUpdate { kind, safe }) => ReferenceSemantics::SignalUpdate {
                kind: *kind,
                safe: *safe,
            },
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
            Some(ReferenceFacts::LegacyStateRead { safe }) => {
                ReferenceSemantics::LegacyStateRead { safe: *safe }
            }
            Some(ReferenceFacts::LegacyStateWrite) => ReferenceSemantics::LegacyStateWrite,
            Some(ReferenceFacts::LegacyStateUpdate { safe }) => {
                ReferenceSemantics::LegacyStateUpdate { safe: *safe }
            }
            Some(ReferenceFacts::LegacyStateMemberMutationRoot { symbol }) => {
                ReferenceSemantics::LegacyStateMemberMutationRoot { symbol: *symbol }
            }
            Some(ReferenceFacts::LegacyReactiveImportRead) => {
                ReferenceSemantics::LegacyReactiveImportRead
            }
            Some(ReferenceFacts::LegacyReactiveImportMemberMutationRoot { symbol }) => {
                ReferenceSemantics::LegacyReactiveImportMemberMutationRoot { symbol: *symbol }
            }
            Some(ReferenceFacts::LegacyEachItemMemberMutationRoot { item_sym }) => {
                ReferenceSemantics::LegacyEachItemMemberMutationRoot {
                    item_sym: *item_sym,
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

    pub(crate) fn binding_facts(&self, sym: SymbolId) -> Option<BindingFacts> {
        self.lookup_binding_facts(sym).cloned()
    }

    pub(crate) fn binding_facts_mut(&mut self, sym: SymbolId) -> Option<&mut BindingFacts> {
        self.bindings.get_mut(sym).and_then(|slot| slot.as_mut())
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
        self.write_binding(sym, BindingFacts::Derived(semantics));
    }

    pub(crate) fn record_prop_binding(&mut self, sym: SymbolId, semantics: PropBindingSemantics) {
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
        self.write_binding(sym, BindingFacts::Store(semantics));
        self.store_declaration_symbols.push(sym);
    }

    pub(crate) fn record_const_binding(&mut self, sym: SymbolId, destructured: bool) {
        self.write_binding(
            sym,
            BindingFacts::Const(ConstBindingSemantics::ConstTag {
                destructured,
                reactive: true,
            }),
        );
    }

    pub(crate) fn set_derived_reactive(&mut self, sym: SymbolId, reactive: bool) {
        if let Some(Some(BindingFacts::Derived(d))) = self.bindings.get_mut(sym) {
            d.reactive = reactive;
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

    pub(crate) fn record_contextual_owner(&mut self, sym: SymbolId, owner_node: NodeId) {
        self.contextual_owner.insert(sym, owner_node);
    }

    pub(crate) fn contextual_owner(&self, sym: SymbolId) -> Option<NodeId> {
        self.contextual_owner.get(&sym).copied()
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

    pub(crate) fn record_const_alias_owner(&mut self, sym: SymbolId, owner_node: NodeId) {
        self.const_alias_owner.insert(sym, owner_node);
    }

    pub(crate) fn const_alias_owner_internal(&self, sym: SymbolId) -> Option<NodeId> {
        self.const_alias_owner.get(&sym).copied()
    }

    pub(super) fn mark_each_rest(&mut self, sym: SymbolId) {
        self.each_rest_symbols.insert(sym);
    }

    pub(crate) fn is_each_rest(&self, sym: SymbolId) -> bool {
        self.each_rest_symbols.contains(&sym)
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

    pub(crate) fn record_let_carrier_binding(
        &mut self,
        stmt_node_id: OxcNodeId,
        carrier_symbol: SymbolId,
    ) {
        self.write_declarator(
            stmt_node_id,
            DeclaratorSemantics::LetCarrier { carrier_symbol },
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
            BindingFacts::State(state) => BindingSemantics::State(state.clone()),
            BindingFacts::Derived(derived) => BindingSemantics::Derived(*derived),
            BindingFacts::OptimizedRune(opt) => BindingSemantics::OptimizedRune(*opt),
            BindingFacts::Prop(prop) => BindingSemantics::Prop(prop.clone()),
            BindingFacts::LegacyBindableProp(legacy) => {
                BindingSemantics::LegacyBindableProp(*legacy)
            }
            BindingFacts::LegacyState(legacy) => BindingSemantics::LegacyState(*legacy),
            BindingFacts::Store(store) => BindingSemantics::Store(*store),
            BindingFacts::Const(kind) => BindingSemantics::Const(*kind),
            BindingFacts::Contextual(kind) => BindingSemantics::Contextual(*kind),
            BindingFacts::RuntimeRune { kind } => BindingSemantics::RuntimeRune { kind: *kind },
            BindingFacts::CarrierAlias { .. } => {
                BindingSemantics::Contextual(ContextualBindingSemantics::LetDirective)
            }
        }
    }
}
