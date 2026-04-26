use crate::scope::SymbolId;
use oxc_index::IndexVec;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use svelte_ast::NodeId;
use svelte_component_semantics::{OxcNodeId, ReferenceId};

/// Declaration-level reactive meaning for one resolved `SymbolId`.
///
/// This answers "what kind of declared thing is this?" without encoding how a
/// particular read or write site should be lowered.
///
/// Examples:
/// - `let count = $state(0)` -> `State(...)`
/// - `let total = $derived(count * 2)` -> `Derived(...)`
/// - `let { foo } = $props()` -> `Prop(...)`
/// - `{#each items as item}` -> `Contextual(...)`
/// - `let x = 1` -> `NonReactive`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeclarationSemantics {
    /// Declaration has no special reactive meaning.
    ///
    /// Example: `let x = 1`.
    NonReactive,
    /// Mutable state declaration backed by a `$state` family rune.
    State(StateDeclarationSemantics),
    /// Readonly declaration backed by a `$derived` family rune.
    Derived(DerivedDeclarationSemantics),
    /// Declarator was a `$state` / `$state.raw` / `$state.eager` call but
    /// the binding is never mutated (neither reassigned nor member-mutated).
    /// Declaration lowers as a plain `let` (same shape as `NonReactive`),
    /// but the binding remains reassignable from the outside (parent
    /// components can assign via bind / prop passing), so consumers at
    /// child-passing sites — component props, `$.boundary`,
    /// `{@render callee()}`, dynamicity — must still treat it reactively.
    ///
    /// Only `$state` family applies: `$derived` / `$derived.by` always lower
    /// as `$.derived(...)` regardless of mutation, and `$props` has its own
    /// `Prop` semantics.
    OptimizedRune(OptimizedRuneSemantics),
    /// Declaration coming from `$props()` destructuring.
    Prop(PropDeclarationSemantics),
    /// LEGACY(svelte4): bindable prop binding from non-runes script.
    /// Covers `export let foo`, `export var foo`, `export { foo }`,
    /// `export { foo as bar }`, and every leaf of `export let { ... } = expr`.
    /// Name/alias/default expression live in the AST; this struct carries only reactivity facts.
    /// Deprecated in Svelte 5, remove in Svelte 6.
    LegacyBindableProp(LegacyBindablePropSemantics),
    /// Symbol-backed `$store` subscription declaration.
    Store(StoreDeclarationSemantics),
    /// Const-style declaration such as `{@const}`.
    Const(ConstDeclarationSemantics),
    /// Contextual declaration introduced by template control flow or `let:`.
    Contextual(ContextualDeclarationSemantics),
    /// Binding-form rune whose return value is not a reactive signal but is
    /// treated as dynamic by consumers — it can change at runtime (new call
    /// result per render / effect) and any read site needs a reactive wrap.
    ///
    /// Examples:
    /// - `const id = $props.id()`
    /// - `let tracking = $effect.tracking()`
    /// - `let host = $host()`
    /// - `let pending = $effect.pending()`
    /// - `let t = $inspect.trace()`
    ///
    /// Only the binding-form of these runes reaches this variant — the
    /// expression-form (e.g. bare `$effect.tracking()` in an argument) does
    /// not introduce a declaration and goes through normal call lowering.
    RuntimeRune { kind: RuntimeRuneKind },
    /// Synthesized carrier for a destructured legacy-slot `let:{...}={...}`
    /// directive: one `$.derived(() => { const {a, b} = src; return {a, b}; })`
    /// anchor shared by all destructure leaves. Keyed by the `OxcNodeId` of
    /// the destructuring statement.
    LetCarrier { carrier_symbol: SymbolId },
    /// Analyzer could not assign a trustworthy semantic meaning.
    Unresolved,
}

/// Declaration-side metadata for `$state(...)`-family bindings.
///
/// Examples:
/// - `let count = $state(0)` -> `kind = State`, `proxied = false`
/// - `let items = $state([])` -> `kind = State`, `proxied = true`
/// - `var count = $state(0)` -> `var_declared = true`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateDeclarationSemantics {
    pub kind: StateKind,
    pub proxied: bool,
    pub var_declared: bool,
    pub binding_semantics: SmallVec<[StateBindingSemantics; 4]>,
}

/// Per-binding operation recorded in declaration order for one `$state` / `$state.raw` declaration.
///
/// The analyzer resolves the reactive operation for each leaf binding so that consumers can emit
/// directly without re-deriving `is_mutated` / `is_proxy` / rune-kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateBindingSemantics {
    /// Mutated `$state` binding. The consumer wraps in `$.state(...)` and, when `proxied`,
    /// additionally in `$.proxy(...)` at that leaf.
    StateSignal { proxied: bool },
    /// Mutated `$state.raw` binding. The consumer wraps in `$.state(...)` without `$.proxy`.
    StateRawSignal,
    /// Non-mutated leaf from a `$state` / `$state.raw` destructure. Direct value; `proxied`
    /// says whether the leaf itself must be wrapped in `$.proxy(...)`.
    NonReactive { proxied: bool },
}

/// Which binding-form runtime rune produced a `DeclarationSemantics::RuntimeRune`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeRuneKind {
    /// `$props.id()` — string id regenerated per component instance.
    PropsId,
    /// `$effect.tracking()` — boolean that flips when reactive context changes.
    EffectTracking,
    /// `$effect.pending()` — pending-count reactive accessor.
    EffectPending,
    /// `$host()` — custom-element host accessor (custom-elements only).
    Host,
    /// `$inspect(...).with(fn)` binding-form tracing accessor.
    InspectTrace,
}

/// Which state-family rune produced a declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateKind {
    /// Declaration comes from `$state(...)`.
    State,
    /// Declaration comes from `$state.raw(...)`.
    StateRaw,
    /// Declaration comes from `$state.eager(...)`.
    StateEager,
}

/// Payload for `DeclarationSemantics::OptimizedRune`.
///
/// Carries the syntactic facts needed by consumers at child-passing sites to
/// reconstruct the would-be reactive behaviour (getter wrap, proxy handling,
/// `state_referenced_locally` warning) even though the declaration itself
/// lowers as a plain `let`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptimizedRuneSemantics {
    pub kind: StateKind,
    /// Initializer is a proxyable value (`$state({...})`, `$state([...])`, etc.).
    /// Answers: "would this binding need `$.proxy` wrapping if it were a live state signal?".
    pub proxy_init: bool,
    /// Declared with `var` keyword (safe-get lowering eligibility if promoted to a live signal).
    pub var_declared: bool,
}

/// Declaration-side metadata for `$derived(...)`-family bindings.
///
/// `reactive` answers "do reads of this binding need a reactive wrap at consumer
/// sites?". A `$derived` whose init expression references only non-changing
/// symbols (e.g. `$state` bindings that are never mutated → `OptimizedRune`)
/// is itself non-changing and reads lower to plain identifier access without
/// `$.template_effect(...)`. Computed by a fix-point pass in the v2 builder
/// after all reference facts are known — transitivity on derived-of-derived
/// dependencies requires it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DerivedDeclarationSemantics {
    pub kind: DerivedKind,
    pub lowering: DerivedLowering,
    pub reactive: bool,
}

/// Which derived-family rune produced a declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivedKind {
    /// Declaration comes from `$derived(...)`.
    Derived,
    /// Declaration comes from `$derived.by(...)`.
    DerivedBy,
}

/// Which declaration-side lowering family the derived binding requires.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivedLowering {
    /// Declaration lowers through the ordinary `$.derived(...)` path.
    Sync,
    /// Declaration lowers through the async-derived path.
    Async,
}

/// Declaration-side metadata for `$props()` bindings.
///
/// Examples:
/// - `let { foo } = $props()` -> `Source`
/// - `let { ...rest } = $props()` -> `Rest`
/// - non-source fallback props -> `NonSource`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropDeclarationSemantics {
    pub lowering_mode: PropLoweringMode,
    pub kind: PropDeclarationKind,
}

/// Which prop lowering family the declaration/reference belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropLoweringMode {
    /// Ordinary component `$props()` lowering.
    Standard,
    /// Custom-element `$props()` lowering.
    CustomElement,
}

/// Which prop family a `$props()` declaration belongs to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropDeclarationKind {
    /// Whole `$props()` object access via identifier.
    ///
    /// Example: `let props = $props()`.
    Identifier,
    /// Whole object-pattern `$props()` destructuring.
    ///
    /// Example: `let { foo, bar = 1, ...rest } = $props()`.
    Object {
        properties: Vec<PropsObjectPropertySemantics>,
        has_rest: bool,
    },
    /// Prop is a source binding with accessor-style semantics.
    ///
    /// Examples:
    /// - `let { foo } = $props()`
    /// - `let { value = $bindable() } = $props()`
    Source {
        bindable: bool,
        updated: bool,
        default_lowering: PropDefaultLowering,
        /// Whether the declaration-side default must be wrapped in `$.proxy(...)`
        /// before prop lowering uses it.
        default_needs_proxy: bool,
    },
    /// Declaration is the `...rest` binding from `$props()`.
    ///
    /// Example: `let { foo, ...rest } = $props()`.
    Rest,
    /// Declaration came from `$props()` but is not itself a prop source.
    NonSource,
}

/// LEGACY(svelte4): reactivity facts for one legacy bindable prop binding.
/// Mirrors runes `Source` shape — bool/enum only, no strings.
/// Deprecated in Svelte 5, remove in Svelte 6.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegacyBindablePropSemantics {
    pub default_lowering: PropDefaultLowering,
    pub updated: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropsObjectPropertySemantics {
    Source {
        bindable: bool,
        updated: bool,
        default_lowering: PropDefaultLowering,
        default_needs_proxy: bool,
    },
    NonSource,
}

/// Which declaration-side default recipe a `$props()` source binding requires.
///
/// Examples:
/// - `let { foo } = $props()` -> `None`
/// - `let { foo = 1 } = $props()` -> `Eager`
/// - `let { foo = expensive() } = $props()` when props lowering requires a thunk -> `Lazy`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropDefaultLowering {
    /// Source binding has no declaration-side default.
    None,
    /// Source binding lowers its default as an eager value.
    Eager,
    /// Source binding lowers its default through a lazy initializer.
    Lazy,
}

/// Declaration-side marker for a symbol-backed `$store` subscription.
///
/// Example:
/// - `$count` when `count` resolves to a store binding.
///
/// `base_symbol` is the underlying store binding (e.g. `count`), which is what
/// generated subscription code reads/writes against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoreDeclarationSemantics {
    pub base_symbol: SymbolId,
}

/// Declaration-side semantics for const-style bindings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstDeclarationSemantics {
    /// `{@const ...}` declaration. `destructured = true` for a leaf of a
    /// destructured pattern (`{@const { a, b } = expr}`); reads lower through
    /// `$.safe_get`. Single-identifier bindings (`{@const NAME = expr}`) have
    /// `destructured = false` and lower through plain `$.get`.
    ///
    /// `reactive` follows the same rule as `DerivedDeclarationSemantics::reactive`:
    /// reads need a reactive wrap only when the init expression references a
    /// symbol that actually changes at runtime.
    ConstTag { destructured: bool, reactive: bool },
}

/// Declaration-side semantics for contextual template bindings.
///
/// Each kind carries a strategy enum that tells the consumer how to read the
/// binding without consulting side tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualDeclarationSemantics {
    /// `{#each items as item}` context binding.
    EachItem(EachItemStrategy),
    /// `{#each items as item, i}` index binding.
    EachIndex(EachIndexStrategy),
    /// `{#await promise then value}` binding.
    AwaitValue,
    /// `{:catch error}` binding inside an await block.
    AwaitError,
    /// Legacy slot `let:` binding.
    LetDirective,
    /// `{#snippet row(p)}` parameter binding.
    SnippetParam(SnippetParamStrategy),
}

/// Read strategy for an `{#each items as <ctx>}` leaf binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachItemStrategy {
    /// Read as `name()` — leaf of a destructured non-default pattern.
    Accessor,
    /// Read as `$.get(name)` — ordinary reactive each leaf.
    Signal,
    /// Read as plain `name` — single-identifier binding in a
    /// `{#each items as item (item)}` block where the key resolves to the
    /// context symbol itself.
    Direct,
}

/// Read strategy for an `{#each items as item, i}` index binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EachIndexStrategy {
    /// Read as `$.get(i)` — keyed block.
    Signal,
    /// Read as plain `i` — unkeyed block (index is a counter).
    Direct,
}

/// Read strategy for a `{#snippet row(p)}` parameter binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnippetParamStrategy {
    /// Read as `name()` — parameter outside a default-value pattern.
    Accessor,
    /// Read as `$.get(name)` — parameter inside a default-value pattern.
    Signal,
}

/// Reference-level reactive meaning for one resolved `ReferenceId`.
///
/// This answers "what should consumer code do with this use-site?" rather than
/// exposing symbol flags that transform or codegen must combine themselves.
///
/// Examples:
/// - read of mutated `$state` -> `SignalRead { kind: State(State), safe: false }`
/// - `count = 1` where `count` is `$state` -> `SignalWrite { kind: State }`
/// - `count += 1` where `count` is `$state` -> `SignalUpdate { kind: State, safe: false }`
/// - read of `$store` subscription -> `StoreRead`
/// - read of `let { foo } = $props()` binding -> `PropRead(Source { .. })`
/// - write to `$derived(...)` binding -> `IllegalWrite`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceSemantics {
    /// Reference has no special reactive handling.
    ///
    /// Example: `local` in `local = 1` or `console.log(local)`.
    NonReactive,
    /// Local reads lower as a plain identifier (no `$.get()` wrap), but
    /// the binding itself is wrapped in a runtime `$.proxy(...)`.
    /// Mutations to its fields and external reassignment via bind/prop
    /// remain observable, so child-passing consumers (each collection,
    /// component props, render callee, boundary slots, dynamism) must
    /// treat this as reactive.
    ///
    /// Example: `let items = $state([1, 2, 3])` when `items` is never
    /// reassigned inside the component.
    Proxy,
    /// Reference reads through the signal family.
    ///
    /// `safe = true` corresponds to the var-declared `$state` safe-get path.
    SignalRead {
        kind: SignalReferenceKind,
        safe: bool,
    },
    /// Reference is a plain signal-backed write.
    ///
    /// Example: `count = 1` where `count` is `$state(...)`.
    SignalWrite { kind: StateKind },
    /// Reference is a signal-backed read-write update.
    ///
    /// Examples: `count += 1`, `count++`.
    SignalUpdate { kind: StateKind, safe: bool },
    /// Reference reads from a store subscription binding.
    ///
    /// `symbol` is the underlying store binding (base of the `$foo` subscription).
    StoreRead { symbol: SymbolId },
    /// Reference writes to a store subscription binding.
    ///
    /// `symbol` is the underlying store binding (base of the `$foo` subscription).
    StoreWrite { symbol: SymbolId },
    /// Reference is a store read-write update.
    ///
    /// Example: `$count += 1`.
    ///
    /// `symbol` is the underlying store binding (base of the `$foo` subscription).
    StoreUpdate { symbol: SymbolId },
    /// Reference reads from the prop family.
    PropRead(PropReferenceSemantics),
    /// Reference mutates a prop source.
    ///
    /// `bindable = true` marks bindable prop sources that participate in
    /// ownership-sensitive mutation rules.
    PropMutation { bindable: bool, symbol: SymbolId },
    /// Reference is the root identifier of a member-expression LHS on an
    /// assignment or the argument of an UpdateExpression, where the root
    /// symbol is a prop source binding.
    ///
    /// Examples (given `let { foo } = $props()`):
    /// - `foo.x = val` → this identifier `foo` is the root
    /// - `foo.x++` → same
    ///
    /// Consumer uses `binding_origin_key(symbol)` for the prop alias and
    /// emits the member mutation lowering. Without this variant consumers
    /// would have to inspect the surrounding AST (which `PropRead` is
    /// emitted for this reference) to decide it is a mutation target.
    PropSourceMemberMutationRoot { bindable: bool, symbol: SymbolId },
    /// Same as `PropSourceMemberMutationRoot` but for non-source prop
    /// bindings (lowered through `$$props.<key>`).
    PropNonSourceMemberMutationRoot { symbol: SymbolId },
    /// Reference reads a `{@const}` alias binding.
    ///
    /// `owner_node` — the template AST `NodeId` of the owning `{@const}` tag,
    /// used by transform to look up destructured tmp names.
    ConstAliasRead { owner_node: NodeId },
    /// Reference reads a template-contextual binding (`let:`, `each`, `await`,
    /// snippet parameter). The nested `ContextualReadKind` encodes the wrap
    /// needed for emission (plain / `name()` / `$.get(name)`), computed once
    /// by the analyzer.
    ContextualRead(ContextualReadSemantics),
    /// Reference is a slot `let:` destructure leaf read: `<carrier>.<leaf>`.
    /// Carrier symbol is the synthesized `let:` alias symbol; leaf is the
    /// local name being read.
    CarrierMemberRead(CarrierMemberReadSemantics),
    /// Reference stands as the object of a `<rest>.<key>` StaticMemberExpression
    /// where `<rest>` is a `...rest` binding from `$props()` destructuring AND
    /// `<key>` is NOT shadowed by a sibling named prop in the same destructuring.
    ///
    /// Consumer rewrites the member to `$$props.<key>` using emission-side text
    /// from the enclosing member's property name.
    ///
    /// Examples (given `let { foo, ...rest } = $props()`):
    /// - `rest.xyz` → `RestPropMemberRewrite` (rewrite to `$$props.xyz`)
    /// - `rest.foo` → NOT `RestPropMemberRewrite` (sibling `foo` shadows it)
    /// - `rest` standalone → NOT `RestPropMemberRewrite` (falls to existing classification)
    RestPropMemberRewrite,
    /// LEGACY(svelte4): identifier read of `$$props` (non-runes mode).
    /// Keyed by `ReferenceId` (no `SymbolId` — `$$props` is unresolved).
    /// Consumer rewrites to `$$sanitized_props`.
    /// Deprecated in Svelte 5, remove in Svelte 6.
    LegacyPropsIdentifierRead,
    /// LEGACY(svelte4): identifier read of `$$restProps` (non-runes mode).
    /// Keyed by `ReferenceId` (no `SymbolId`).
    /// Consumer rewrites to local `const $$restProps = $.legacy_rest_props(...)`.
    /// Deprecated in Svelte 5, remove in Svelte 6.
    LegacyRestPropsIdentifierRead,
    /// Reference is a semantically forbidden write.
    ///
    /// Examples: `$derived(...) = value`, snippet parameter writes.
    IllegalWrite,
    /// Analyzer could not assign a trustworthy semantic meaning.
    Unresolved,
}

/// Signal-family refinement carried by `ReferenceSemantics`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignalReferenceKind {
    State(StateKind),
    Derived(DerivedKind),
}

/// Payload for `ReferenceSemantics::ContextualRead`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextualReadSemantics {
    pub kind: ContextualReadKind,
    pub owner_node: NodeId,
    pub symbol: SymbolId,
}

/// Per-reference contextual read shape. `accessor`/`signal` are computed by
/// the classifier from `is_getter` / `is_each_non_reactive` generic symbol
/// flags plus the declaration kind — consumers never re-derive these.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextualReadKind {
    /// `{#each items as item}` body read. `accessor=true` means `item()` call,
    /// `signal=true` means `$.get(item)`, both false means plain identifier.
    EachItem { accessor: bool, signal: bool },
    /// `{#each items as item, i}` index read. `signal=true` means `$.get(i)`.
    EachIndex { signal: bool },
    /// `{#await promise then value}` binding read.
    AwaitValue,
    /// `{:catch error}` binding read.
    AwaitError,
    /// Direct (non-destructured) `<Widget let:item={alias}>` binding read.
    /// Always signal-wrapped in emission (`$.get(alias)`).
    LetDirective,
    /// `{#snippet row(item)}` parameter read.
    SnippetParam { accessor: bool, signal: bool },
}

/// Payload for `ReferenceSemantics::CarrierMemberRead`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarrierMemberReadSemantics {
    pub carrier_symbol: SymbolId,
    pub leaf_symbol: SymbolId,
}

/// Prop-specific refinement for `ReferenceSemantics::PropRead`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropReferenceSemantics {
    /// Read of a prop source binding that lowers through accessor semantics.
    ///
    /// Examples:
    /// - `foo` from `let { foo } = $props()`
    /// - `value` from `let { value = $bindable() } = $props()`
    Source {
        bindable: bool,
        lowering_mode: PropLoweringMode,
        symbol: SymbolId,
    },
    /// Read of a non-source prop binding that lowers through `$$props.name`.
    NonSource { symbol: SymbolId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PropBindingFacts {
    pub(crate) bindable: bool,
    pub(crate) is_rest: bool,
    pub(crate) is_source: bool,
    pub(crate) updated: bool,
    pub(crate) lowering_mode: PropLoweringMode,
    pub(crate) default_lowering: PropDefaultLowering,
    pub(crate) default_needs_proxy: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum V2DeclarationFacts {
    State(StateDeclarationSemantics),
    Derived(DerivedDeclarationSemantics),
    OptimizedRune(OptimizedRuneSemantics),
    Prop(PropDeclarationSemantics),
    /// LEGACY(svelte4): mirror of `DeclarationSemantics::LegacyBindableProp`.
    /// Deprecated in Svelte 5, remove in Svelte 6.
    LegacyBindableProp(LegacyBindablePropSemantics),
    Store(StoreDeclarationSemantics),
    Const(ConstDeclarationSemantics),
    Contextual(ContextualDeclarationSemantics),
    RuntimeRune {
        kind: RuntimeRuneKind,
    },
    LetCarrier {
        carrier_symbol: SymbolId,
    },
    /// Slot `let:` destructured-leaf binding whose value is read as
    /// `<carrier>.<leaf>`. Public `declaration_semantics()` normalizes this
    /// to `Contextual(LetDirective)` — consumers see the carrier through
    /// `ReferenceSemantics::CarrierMemberRead` instead.
    CarrierAlias {
        carrier: SymbolId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum V2ReferenceFacts {
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
    /// LEGACY(svelte4): mirror of `ReferenceSemantics::LegacyPropsIdentifierRead`.
    /// Deprecated in Svelte 5, remove in Svelte 6.
    LegacyPropsIdentifierRead,
    /// LEGACY(svelte4): mirror of `ReferenceSemantics::LegacyRestPropsIdentifierRead`.
    /// Deprecated in Svelte 5, remove in Svelte 6.
    LegacyRestPropsIdentifierRead,
    IllegalWrite,
    /// Local reads lower as a plain identifier, but the binding itself
    /// is wrapped in a runtime `$.proxy(...)` — its fields and external
    /// reassignment stay observable, so child-passing consumers must
    /// treat it as reactive. Originates from unmutated `$state(proxyable)`
    /// bindings (`OptimizedRune` with `proxy_init = true`).
    Proxy,
}

/// Analyzer-owned storage for normalized reactivity facts.
///
/// This is the single downstream query surface for reactivity classification.
/// The storage object itself does not walk the AST; the builder/pass populates it.
///
/// Examples of questions this storage answers:
/// - "is `count` a `$state` source or a plain local?"
/// - "should reading `value` become `prop()` or `$.get(value)`?"
/// - "is `bind:prop={value}` targeting a prop source family?"
/// - "does this `let:` leaf read through a carrier object?"
#[derive(Clone, Debug)]
pub struct ReactivitySemantics {
    /// Dense table of declaration facts keyed by OXC `NodeId`. Using an
    /// `IndexVec` indexed by `OxcNodeId` avoids hashing/rehashing on the hot
    /// `declaration_semantics` / `declaration_facts_v2` path that gets hit
    /// millions of times for big components.
    declaration_facts_v2: IndexVec<OxcNodeId, Option<V2DeclarationFacts>>,
    /// Secondary index into `declaration_facts_v2` for store-subscription
    /// declarations only — so `iter_store_declarations` stays O(k_stores)
    /// instead of O(n_all_nodes) after the table became dense.
    store_declaration_ids: Vec<OxcNodeId>,
    /// Dense table of reference facts keyed by `ReferenceId`.
    reference_facts_v2: IndexVec<ReferenceId, Option<V2ReferenceFacts>>,
    symbol_declaration_roots: FxHashMap<SymbolId, OxcNodeId>,
    symbol_prop_facts: FxHashMap<SymbolId, PropBindingFacts>,
    /// Set of `ReferenceId`s that are the root identifier of a MemberExpression
    /// LHS in an assignment, or the argument of an `UpdateExpression`. Used by
    /// `classify_reference_semantics` to emit the `Prop*MemberMutationRoot`
    /// variants instead of `PropRead` for these references.
    prop_member_mutation_root_refs: rustc_hash::FxHashSet<ReferenceId>,
    /// v2-only side-table: contextual symbol → owning template NodeId
    /// (each block / await block / snippet / let-carrier). Populated by the
    /// v2 template-declaration collector; consumed by `classify_reference_semantics`
    /// to emit `ContextualRead { owner_node, .. }`.
    contextual_owner_v2: FxHashMap<SymbolId, NodeId>,
    /// v2 contextual-binding flags. Each set holds the subset of symbols
    /// with the corresponding Svelte-specific classification. Populated by
    /// `passes/template_side_tables` + v2 template-declaration collector;
    /// consumed by `reactivity_semantics` classifiers and validators. Owned
    /// here (not on `ComponentSemantics`) because they encode Svelte-specific
    /// template meaning.
    each_rest_symbols: FxHashSet<SymbolId>,
    /// v2-only side-table: destructured `{@const}` leaf symbol → owning
    /// const-tag `NodeId`. Only set for destructured patterns (`{@const {a, b} = ...}`);
    /// single-identifier `{@const NAME = ...}` bindings do not carry a tag owner
    /// because consumers don't need to look them up.
    const_alias_owner_v2: FxHashMap<SymbolId, NodeId>,
    /// v2-only side-table: symbols whose declarator initializer was a rune
    /// call (`$state(...)` / `$derived(...)` / `$props()` / `$state.raw(...)`
    /// / `$state.eager(...)` / `$derived.by(...)`). Carries the syntactic
    /// rune fact independently of reactive normalization: a non-mutated
    /// `$state(primitive)` declarator is normalized to `NonReactive` in
    /// `declaration_facts_v2`, but we still remember it was a rune declaration
    /// here so validators can distinguish "primitive state that didn't need
    /// reactive lowering" from "plain local".
    uses_runes: bool,
}

impl ReactivitySemantics {
    pub fn new(node_count: u32) -> Self {
        let mut declaration_facts_v2 = IndexVec::with_capacity(node_count as usize);
        declaration_facts_v2.resize_with(node_count as usize, || None);
        Self {
            declaration_facts_v2,
            store_declaration_ids: Vec::new(),
            reference_facts_v2: IndexVec::new(),
            symbol_declaration_roots: FxHashMap::default(),
            symbol_prop_facts: FxHashMap::default(),
            prop_member_mutation_root_refs: rustc_hash::FxHashSet::default(),
            contextual_owner_v2: FxHashMap::default(),
            const_alias_owner_v2: FxHashMap::default(),
            each_rest_symbols: FxHashSet::default(),
            uses_runes: false,
        }
    }

    /// Called by the v2 builder once the `ReferenceTable` length is known
    /// (after template declarations + JS script walk). Sizes the dense
    /// reference-facts table so subsequent `record_reference_semantics_v2`
    /// calls don't pay for resize chains.
    pub(crate) fn reserve_references(&mut self, reference_count: usize) {
        if self.reference_facts_v2.len() < reference_count {
            self.reference_facts_v2
                .resize_with(reference_count, || None);
        }
    }

    pub fn uses_runes(&self) -> bool {
        self.uses_runes
    }

    pub fn declaration_semantics(&self, node_id: OxcNodeId) -> DeclarationSemantics {
        self.lookup_declaration_facts(node_id)
            .map(Self::declaration_semantics_from_facts)
            .unwrap_or(DeclarationSemantics::NonReactive)
    }

    /// Declaration-oriented iterator over every `$store` subscription
    /// declaration in this component. Codegen's store-wiring block walks this
    /// to emit `$$stores` initialization without bypassing the semantic API.
    pub fn iter_store_declarations(
        &self,
    ) -> impl Iterator<Item = (OxcNodeId, StoreDeclarationSemantics)> + '_ {
        self.store_declaration_ids.iter().filter_map(|&node_id| {
            match self.lookup_declaration_facts(node_id)? {
                V2DeclarationFacts::Store(store) => Some((node_id, *store)),
                _ => None,
            }
        })
    }

    /// Whether any `$store` subscription declaration exists. Used by the
    /// runtime-plan assembly to decide if the component needs the store
    /// wiring block.
    pub fn has_store_declarations(&self) -> bool {
        !self.store_declaration_ids.is_empty()
    }

    pub fn reference_semantics(&self, ref_id: ReferenceId) -> ReferenceSemantics {
        match self.lookup_reference_facts(ref_id) {
            Some(V2ReferenceFacts::SignalRead { kind, safe }) => ReferenceSemantics::SignalRead {
                kind: *kind,
                safe: *safe,
            },
            Some(V2ReferenceFacts::SignalWrite { kind }) => {
                ReferenceSemantics::SignalWrite { kind: *kind }
            }
            Some(V2ReferenceFacts::SignalUpdate { kind, safe }) => {
                ReferenceSemantics::SignalUpdate {
                    kind: *kind,
                    safe: *safe,
                }
            }
            Some(V2ReferenceFacts::StoreRead { symbol }) => {
                ReferenceSemantics::StoreRead { symbol: *symbol }
            }
            Some(V2ReferenceFacts::StoreWrite { symbol }) => {
                ReferenceSemantics::StoreWrite { symbol: *symbol }
            }
            Some(V2ReferenceFacts::StoreUpdate { symbol }) => {
                ReferenceSemantics::StoreUpdate { symbol: *symbol }
            }
            Some(V2ReferenceFacts::PropRead(read)) => ReferenceSemantics::PropRead(*read),
            Some(V2ReferenceFacts::PropMutation { bindable, symbol }) => {
                ReferenceSemantics::PropMutation {
                    bindable: *bindable,
                    symbol: *symbol,
                }
            }
            Some(V2ReferenceFacts::PropSourceMemberMutationRoot { bindable, symbol }) => {
                ReferenceSemantics::PropSourceMemberMutationRoot {
                    bindable: *bindable,
                    symbol: *symbol,
                }
            }
            Some(V2ReferenceFacts::PropNonSourceMemberMutationRoot { symbol }) => {
                ReferenceSemantics::PropNonSourceMemberMutationRoot { symbol: *symbol }
            }
            Some(V2ReferenceFacts::ConstAliasRead { owner_node }) => {
                ReferenceSemantics::ConstAliasRead {
                    owner_node: *owner_node,
                }
            }
            Some(V2ReferenceFacts::ContextualRead(read)) => {
                ReferenceSemantics::ContextualRead(*read)
            }
            Some(V2ReferenceFacts::CarrierMemberRead(read)) => {
                ReferenceSemantics::CarrierMemberRead(*read)
            }
            Some(V2ReferenceFacts::RestPropMemberRewrite) => {
                ReferenceSemantics::RestPropMemberRewrite
            }
            Some(V2ReferenceFacts::LegacyPropsIdentifierRead) => {
                ReferenceSemantics::LegacyPropsIdentifierRead
            }
            Some(V2ReferenceFacts::LegacyRestPropsIdentifierRead) => {
                ReferenceSemantics::LegacyRestPropsIdentifierRead
            }
            Some(V2ReferenceFacts::IllegalWrite) => ReferenceSemantics::IllegalWrite,
            Some(V2ReferenceFacts::Proxy) => ReferenceSemantics::Proxy,
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

    pub(crate) fn prop_facts(&self, sym: SymbolId) -> Option<PropBindingFacts> {
        self.symbol_prop_facts.get(&sym).cloned()
    }

    pub(crate) fn set_uses_runes(&mut self, uses_runes: bool) {
        self.uses_runes = uses_runes;
    }

    pub(crate) fn declaration_root_for_symbol(&self, sym: SymbolId) -> Option<OxcNodeId> {
        self.symbol_declaration_roots.get(&sym).copied()
    }

    pub(crate) fn record_symbol_declaration_root(&mut self, sym: SymbolId, node_id: OxcNodeId) {
        self.symbol_declaration_roots.insert(sym, node_id);
    }

    pub(crate) fn declaration_facts_v2(&self, node_id: OxcNodeId) -> Option<V2DeclarationFacts> {
        self.lookup_declaration_facts(node_id).cloned()
    }

    pub(crate) fn declaration_facts_v2_mut(
        &mut self,
        node_id: OxcNodeId,
    ) -> Option<&mut V2DeclarationFacts> {
        self.declaration_facts_v2
            .get_mut(node_id)
            .and_then(|slot| slot.as_mut())
    }

    pub(crate) fn record_state_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        semantics: StateDeclarationSemantics,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::State(semantics));
    }

    pub(crate) fn record_optimized_rune_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        semantics: OptimizedRuneSemantics,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::OptimizedRune(semantics));
    }

    pub(crate) fn record_derived_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        semantics: DerivedDeclarationSemantics,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::Derived(semantics));
    }

    pub(crate) fn record_prop_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        semantics: PropDeclarationSemantics,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::Prop(semantics));
    }

    /// LEGACY(svelte4): record a legacy bindable prop declaration.
    /// Deprecated in Svelte 5, remove in Svelte 6.
    pub(crate) fn record_legacy_bindable_prop_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        semantics: LegacyBindablePropSemantics,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::LegacyBindableProp(semantics));
    }

    pub(crate) fn record_store_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        semantics: StoreDeclarationSemantics,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::Store(semantics));
        self.store_declaration_ids.push(node_id);
    }

    pub(crate) fn record_const_declaration_v2(&mut self, node_id: OxcNodeId, destructured: bool) {
        self.write_declaration(
            node_id,
            V2DeclarationFacts::Const(ConstDeclarationSemantics::ConstTag {
                destructured,
                // Conservative default — `compute_derived_reactivity` pass
                // lowers it to `false` when all refs are non-reactive.
                reactive: true,
            }),
        );
    }

    /// Used by the `compute_derived_reactivity` fix-point pass to overwrite
    /// the `reactive` flag on an already-recorded Derived declaration.
    pub(crate) fn set_derived_reactive(&mut self, node_id: OxcNodeId, reactive: bool) {
        if let Some(Some(V2DeclarationFacts::Derived(d))) =
            self.declaration_facts_v2.get_mut(node_id)
        {
            d.reactive = reactive;
        }
    }

    pub(crate) fn record_runtime_rune_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        kind: RuntimeRuneKind,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::RuntimeRune { kind });
    }

    pub(crate) fn record_contextual_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        semantics: ContextualDeclarationSemantics,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::Contextual(semantics));
    }

    pub(crate) fn record_carrier_alias_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        carrier: SymbolId,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::CarrierAlias { carrier });
    }

    fn write_declaration(&mut self, node_id: OxcNodeId, facts: V2DeclarationFacts) {
        let idx = node_id.index();
        if idx >= self.declaration_facts_v2.len() {
            self.declaration_facts_v2.resize_with(idx + 1, || None);
        }
        self.declaration_facts_v2[node_id] = Some(facts);
    }

    fn lookup_declaration_facts(&self, node_id: OxcNodeId) -> Option<&V2DeclarationFacts> {
        self.declaration_facts_v2
            .get(node_id)
            .and_then(|slot| slot.as_ref())
    }

    fn lookup_reference_facts(&self, ref_id: ReferenceId) -> Option<&V2ReferenceFacts> {
        self.reference_facts_v2
            .get(ref_id)
            .and_then(|slot| slot.as_ref())
    }

    pub(crate) fn record_contextual_owner_v2(&mut self, sym: SymbolId, owner_node: NodeId) {
        self.contextual_owner_v2.insert(sym, owner_node);
    }

    pub(crate) fn contextual_owner_v2(&self, sym: SymbolId) -> Option<NodeId> {
        self.contextual_owner_v2.get(&sym).copied()
    }

    pub(crate) fn record_const_alias_owner_v2(&mut self, sym: SymbolId, owner_node: NodeId) {
        self.const_alias_owner_v2.insert(sym, owner_node);
    }

    pub(crate) fn const_alias_owner_v2_internal(&self, sym: SymbolId) -> Option<NodeId> {
        self.const_alias_owner_v2.get(&sym).copied()
    }

    pub(super) fn mark_each_rest(&mut self, sym: SymbolId) {
        self.each_rest_symbols.insert(sym);
    }

    pub(crate) fn is_each_rest(&self, sym: SymbolId) -> bool {
        self.each_rest_symbols.contains(&sym)
    }

    pub(crate) fn record_reference_semantics_v2(
        &mut self,
        ref_id: ReferenceId,
        semantics: V2ReferenceFacts,
    ) {
        let idx = ref_id.index();
        if idx >= self.reference_facts_v2.len() {
            self.reference_facts_v2.resize_with(idx + 1, || None);
        }
        self.reference_facts_v2[ref_id] = Some(semantics);
    }

    pub(crate) fn record_prop_facts(&mut self, sym: SymbolId, facts: PropBindingFacts) {
        self.symbol_prop_facts.insert(sym, facts);
    }

    pub(crate) fn record_let_carrier_declaration_v2(
        &mut self,
        node_id: OxcNodeId,
        carrier_symbol: SymbolId,
    ) {
        self.write_declaration(node_id, V2DeclarationFacts::LetCarrier { carrier_symbol });
    }
}

impl ReactivitySemantics {
    fn declaration_semantics_from_facts(facts: &V2DeclarationFacts) -> DeclarationSemantics {
        match facts {
            V2DeclarationFacts::State(state) => DeclarationSemantics::State(state.clone()),
            V2DeclarationFacts::Derived(derived) => DeclarationSemantics::Derived(*derived),
            V2DeclarationFacts::OptimizedRune(opt) => DeclarationSemantics::OptimizedRune(*opt),
            V2DeclarationFacts::Prop(prop) => DeclarationSemantics::Prop(prop.clone()),
            V2DeclarationFacts::LegacyBindableProp(legacy) => {
                DeclarationSemantics::LegacyBindableProp(*legacy)
            }
            V2DeclarationFacts::Store(store) => DeclarationSemantics::Store(*store),
            V2DeclarationFacts::Const(kind) => DeclarationSemantics::Const(*kind),
            V2DeclarationFacts::Contextual(kind) => DeclarationSemantics::Contextual(*kind),
            V2DeclarationFacts::RuntimeRune { kind } => {
                DeclarationSemantics::RuntimeRune { kind: *kind }
            }
            V2DeclarationFacts::LetCarrier { carrier_symbol } => DeclarationSemantics::LetCarrier {
                carrier_symbol: *carrier_symbol,
            },
            V2DeclarationFacts::CarrierAlias { .. } => {
                DeclarationSemantics::Contextual(ContextualDeclarationSemantics::LetDirective)
            }
        }
    }
}
