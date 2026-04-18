# Reactivity Architecture

Working document for redesigning reactive semantics ownership and consumer API.

This document is intentionally not another migration plan.
It records the architecture constraints we already agreed on before we inventory old data and passes.

## Goal

The project should have one cohesive reactive semantics system with:

- one source of truth for reactive meaning
- dumb consumers in `svelte_transform` and `svelte_codegen_client`
- semantic meaning colocated in one subsystem instead of reconstructed from scattered flags and side tables

`reactivity_semantics` should answer the question:

what does this declaration/reference mean, and what kind of operation does the consumer need to perform here?

It should replace consumer-side interpretation like:

- `is_mutated`
- `is_prop_source`
- `is_store`
- ad hoc combinations of rune, prop, store, contextual, and dynamicity flags

Consumers should stop assembling meaning themselves from raw facts.

## Source Of Truth

There must be exactly one source of truth for reactive meaning:

- `crates/svelte_analyze/src/reactivity_semantics`

`ComponentScoping` and other analyzer passes may still provide raw facts or generic symbol/reference facts,
but they must not remain parallel owners of reactive meaning.

## Consumer Model

The target consumer model is:

- `svelte_transform` asks for one semantic answer for the concrete unit it is rewriting
- `svelte_codegen_client` asks for one semantic answer for the concrete unit it is generating
- the returned answer should be sufficient for the common case without follow-up semantic queries
- the answer should be cheap to obtain on the hot consumer path

The desired result is dumb consumer code:

- consumers should mostly pattern-match on a returned semantic object
- consumers should not re-derive meaning from several booleans or side tables
- consumers should not need cascades of semantic lookups just to handle one ordinary use-site
- consumers may still read AST shape as emission input after the semantic branch is chosen
- consumers must not use AST inspection to choose the migrated branch or to reconstruct missing semantic meaning

### Reactive meaning stays inside reactivity_semantics

Consumers must not re-derive reactive meaning on the outside of `reactivity_semantics`.

Reactive meaning includes, but is not limited to, questions like:

- `is_proxy` / `is_proxy_init_state`
- `is_mutated`
- `is_var_declared_state`
- `is_store`
- `is_getter`
- `is_rest_prop`
- `is_bindable`
- `rune_kind` / `source_kind`
- "needs `$.proxy`", "is a signal", "is a store write", "is legal write"
- any other `is_*` / `*_kind` classifier that expresses reactive behavior

If a consumer is about to ask any of those questions on the outside, the semantic contract is incomplete. Extend the answer shape in `reactivity_semantics` so the analyzer makes the decision once, and let the consumer pattern-match on the result.

Consumers may still resolve binding-level **text** on the outside:

- identifier name by `SymbolId` (`component_semantics.symbol_name(sym)`)
- prop alias / binding origin key by `SymbolId` (`analysis.binding_origin_key(sym)`)
- OXC source-level text for emission payload

This is binding text resolved by identity, not reactive meaning. The rule above only forbids re-deriving reactive decisions; identity-keyed text lookups remain allowed.

For a migrated branch, one semantic answer must be sufficient to choose the new path.

- declaration-side consumer code should choose the migrated branch from one call to `declaration_semantics(node_id)`
- reference-side consumer code should choose the migrated branch from one call to `reference_semantics(reference_id)`
- if consumer code needs a second semantic lookup to distinguish cases inside the migrated branch, the semantic contract is incomplete
- in that situation, fix the analyzer-owned semantic answer instead of compensating in the consumer

## Root Consumer Migration Rule

Migration of a semantic cluster must start at the root consumer entrypoint that owns the behavior.

Examples:

- a declaration rewrite entrypoint in `svelte_codegen_client`
- a reference rewrite entrypoint in `svelte_transform`
- a destructuring rewrite entrypoint if destructuring has its own root path

At that root point, the consumer should do:

```rust
match reactivity_semantics.declaration_semantics(node_id) {
    DeclarationSemantics::State(state) => {
        // new migrated path for this cluster
        return lower_from_semantics(state);
    }
    _ => {
        // untouched legacy path
    }
}
```

or the equivalent reference-side pattern:

```rust
match reactivity_semantics.reference_semantics(reference_id) {
    ReferenceSemantics::SignalRead(read) => {
        return lower_from_semantics(read);
    }
    _ => {
        // untouched legacy path
    }
}
```

This rule is strict:

- migration must begin at the root consumer point, not in helper layers underneath it
- the new semantic branch must be selected by one top-level pattern match on the semantic answer
- that top-level match must be based on exactly one semantic query for the concrete unit being rewritten
- all unsupported cases must fall through to the existing legacy branch at that same root point
- do not invent a second semantic enum in transform/codegen as a bridge
- do not combine `reference_semantics(...)` with `declaration_semantics(...)` in one migrated consumer decision
- do not combine several semantic lookups just to recover a distinction that should already be encoded in the migrated semantic answer
- do not use AST inspection to recover a distinction that should already be encoded in the migrated semantic answer
- do not push semantic decision-making downward into helper functions before the root consumer match exists
- do not replace the whole consumer path at once if only one semantic cluster is migrated

Reading AST in a migrated consumer branch is still allowed when it is only used as emission detail.

Examples of allowed AST use in a migrated branch:

- local binding name from a binding pattern
- object destructuring key text needed to emit `$.prop(..., "foo", ...)`
- expression payload that will be wrapped by an already-selected lowering recipe

Examples of forbidden AST use in a migrated branch:

- deciding whether a `$props()` binding is source vs non-source
- deciding whether a prop binding is updated
- deciding whether a declaration should take the migrated branch at all

In other words, the migration shape is:

- root consumer point
- one semantic `match`
- one migrated cluster
- legacy fallback for everything else

not:

- many helper rewrites first
- local pseudo-semantic enums in consumer code
- broad consumer refactors before one root path is actually migrated

## Migration Verification Rule

Any migration that changes analyzer or consumer behavior must be verified with:

```bash
just test-compiler
just test-diagnostics
```

Both are mandatory even if a few targeted tests already pass.

Do not treat:

- one or two focused compiler cases
- crate-local unit tests
- ad hoc spot checks

as sufficient verification for an analyze/transform/codegen migration.

Targeted tests are still useful while iterating, but the checkpoint is not done until:

- `just test-compiler` is green
- `just test-diagnostics` is green

## Identity Model

The public semantic API is intentionally built around two canonical identities:

- declaration semantics are keyed by declaration-root `OxcNodeId`
- reference semantics are keyed by `ReferenceId`

This is the consumer-facing model:

- `declaration_semantics(node_id)`
- `reference_semantics(reference_id)`

The declaration query must answer for the concrete declaration root being lowered.
Examples:

- a `VariableDeclarator`
- a class field/property declaration
- another analyzer-owned declaration root that already has a stable `OxcNodeId`

`SymbolId` remains valid as internal analyzer identity for:

- binding ownership
- reference resolution
- internal links such as `reference -> symbol -> declaration_root`

But `SymbolId` is no longer part of the public declaration semantic query surface.

This matters most for cases like:

```rust
let { a, b, ...rest } = $props();
```

The root consumer question is about the whole declarator, not any single binding symbol.
That is why declaration semantics are node-owned.

## API Direction

The external API should be consumer-oriented, not migration-oriented.

What we want:

- an answer object that tells the consumer what kind of operation is required
- explicit enums/structs that encode semantic decisions
- minimal need for follow-up queries in the common case
- efficient retrieval of semantic meaning for common consumer operations

What we do not want:

- an API that mostly mirrors old fact getters
- an API that exposes scattered internal indices
- consumers manually combining several raw facts into one decision
- a design where one semantic answer requires multiple dependent lookups in normal operation

The current getter-oriented surface in `reactivity_semantics` should be treated as transitional.

That includes the current style of APIs like:

- `source_kind(...)`
- `read_semantics(...)`
- `write_semantics(...)`
- `alias_source(...)`
- `context_owner(...)`
- `non_source_prop_name(...)`
- `const_tag_owner(...)`

These APIs are not the desired final consumer contract.

They should be treated as deprecated migration surface and gradually replaced by:

- one declaration-oriented answer per declaration-root `OxcNodeId`
- one reference/use-site-oriented answer per `ReferenceId`

The point of the redesign is not just to rename getters.
It is to stop forcing consumers to assemble meaning from several partial queries.

The required standard for a migrated path is stronger than "mostly semantic":

- the consumer gets one semantic answer for the concrete declaration or reference it is handling
- that one answer is enough to decide whether the migrated branch applies
- that one answer is enough to choose the concrete migrated lowering path for the supported cluster
- if this is not possible, the semantic API is still missing information and must be extended upstream

For a migrated path, the semantic answer must already encode the operation the consumer needs to perform.

The target consumer shape is:

```rust
fn root_point(...) {
    let semantic = get_semantic_for_concrete_unit(...);

    match semantic {
        MigratedCase(op) => {
            return emit_from_semantic_op(op, ast_payload_only);
        }
        _ => {
            return legacy_fallback(...);
        }
    }
}
```

Where:

- `root_point` is the function that owns the behavior
- `get_semantic_for_concrete_unit(...)` is exactly one semantic query
- `MigratedCase(op)` is an operation-level semantic answer, not a bag of partial facts
- `emit_from_semantic_op(...)` may read AST payload needed for emission, but must not perform semantic redispatch
- `legacy_fallback(...)` remains the untouched fallback at the same root point

This means a migrated consumer path is only correct if:

- the root point contains one top-level semantic `match`
- that `match` is driven by one semantic query for the concrete unit being lowered
- the semantic answer already determines the migrated operation family
- the migrated branch does not perform additional `declaration_semantics(...)` or `reference_semantics(...)` queries
- the migrated branch does not build a consumer-local semantic bridge enum or plan in order to recover reactive meaning
- the migrated branch does not inspect AST in order to recover missing semantic distinctions

If a migrated branch still needs another semantic query or a consumer-local semantic bridge, the semantic contract is incomplete.
Do not "improve the consumer a bit more" in that situation.
Stop, state that the analyzer-owned contract is incomplete, and extend it upstream first.

When this happens during implementation, the correct workflow is:

- stop before writing more consumer logic
- explicitly state which semantic distinction is still missing
- propose concrete analyzer-owned contract options
- only resume consumer work after one of those options is chosen and implemented

Do not compensate for missing semantic data by:

- helper-local redispatch
- additional semantic lookups inside the migrated branch
- AST-based semantic reconstruction
- consumer-side "plan" structs that encode reactive meaning the analyzer should already own

The current direction is closer to:

- declaration semantics by declaration-root `OxcNodeId`
- reference semantics by `ReferenceId`

and less like:

- many small fact getters inherited from unfinished migration work

## Storage Versus API

The internal storage may stay sparse.

That means:

- declaration semantics do not need an explicit stored entry for every declaration-root `OxcNodeId`
- reference semantics do not need an explicit stored entry for every `ReferenceId`
- only symbols/references with special reactive meaning need stored specialized facts

But the public semantic API should still behave as total for normal resolved cases.

This means:

- missing declaration entry should not leak as `None` to consumers
- missing reference entry should not leak as `None` to consumers
- the public answer should normalize missing special entries into an explicit non-reactive result

This keeps storage small without forcing consumer code to interpret missing data.

Sparse internal storage is acceptable only if it does not make ordinary semantic queries expensive.

The external contract should still optimize for:

- one query per ordinary declaration/reference use-site
- minimal branching in consumer code
- minimal repeated interpretation on hot paths

### Storage content rule

`reactivity_semantics` storage must not hold strings.

Allowed identity and payload forms inside stored facts and answer shapes:

- `SymbolId`
- `ReferenceId`
- `OxcNodeId`
- booleans and enum variants that encode semantic decisions
- numeric payloads (flags, counts, indices)

Forbidden inside `reactivity_semantics` storage and answer shapes:

- `String`, `Box<str>`, `&str`, `Cow<str>` for binding names, prop aliases, source text, or any other textual payload

Rationale: reactive meaning is the contract `reactivity_semantics` owns; surface text has one canonical owner in `ComponentSemantics` / analysis identity tables. Duplicating text inside reactive answers would turn the semantic layer into a second source of truth for identifiers and force synchronization with the identity owner.

Consumer code obtains text (identifier names, prop aliases, binding origin keys) at consumption time via identity-keyed lookups on `ComponentSemantics` or analysis side tables, using identities carried by the semantic answer.

## Destructuring Rule

Destructuring needs one additional clarification because it is easy to hide semantic redispatch inside a loop.

If the concrete unit owned by the root point is an entire destructuring declarator, for example:

```rust
let { a, b, ...rest } = $props();
```

then the semantic query must also be for that whole declarator.

The required shape is:

```rust
let semantic = declaration_semantics(declarator.node_id());

match semantic {
    DeclarationSemantics::Prop(prop) => {
        return emit_from_semantics(prop, ast_payload_only);
    }
    _ => {
        return legacy_fallback(...);
    }
}
```

This is intentionally stricter than "each binding can be queried separately".

For a migrated destructuring path, the consumer must not:

- enter the migrated branch from one semantic answer and then query semantics again for each binding
- recover `Source` vs `NonSource` vs `Rest` by looping over binding symbols
- reconstruct a declarator-level plan in codegen or transform from multiple leaf semantic answers

The analyzer-owned contract for a migrated destructuring path must therefore provide a declarator-level semantic answer.

The consumer may still use AST inside that branch for emission payload such as:

- local binding names
- object property keys
- default expressions
- path segments

But all reactive meaning for the declarator must already be encoded in the one root semantic answer.

## Pre-Code Shape Check

Before implementing a migrated path, the author should be able to write a short pseudocode sketch that shows:

- the root point
- the one semantic query
- the one top-level semantic `match`
- the migrated branch as emit-only code over semantic operations plus AST payload
- the legacy fallback

If that pseudocode cannot be written in a short, linear form, then the semantic contract is not ready and consumer implementation should not start yet.

## Resolved Versus Unresolved

`NonReactive` and `Unresolved` are not the same thing.

- `NonReactive` means the system understood the declaration/reference and determined that no special reactive behavior is required
- `Unresolved` means the system could not determine the declaration/reference meaning correctly enough to treat it as a normal resolved case

The API must not collapse unresolved cases into non-reactive behavior.

Otherwise consumer code would silently generate ordinary JS behavior for broken or recovery-only analysis states.

## Terminology Direction

We do not want one overloaded `Plain` term everywhere.

The agreed direction is:

- use `NonReactive` for the absence of special reactive meaning
- use operation-specific names for ordinary behavior on each axis

Examples:

- declaration semantics: `NonReactive`
- read semantics: something closer to `Direct`
- write semantics: something closer to `DirectAssign`
- recovery/analysis failure: `Unresolved`

The exact enum names are still open, but the model should distinguish:

- non-reactive resolved meaning
- ordinary direct operation kinds
- unresolved meaning

## Draft Declaration Shape

The current draft shape for declaration-level meaning is:

```rust
pub enum DeclarationSemantics {
    NonReactive,

    State(StateDeclarationSemantics),
    Derived(DerivedDeclarationSemantics),

    Prop(PropDeclarationSemantics),
    Store(StoreDeclarationSemantics),

    Const(ConstDeclarationSemantics),
    Contextual(ContextualDeclarationSemantics),

    Unresolved,
}
```

With the following intended refinements:

```rust
pub struct StateDeclarationSemantics {
    pub kind: StateKind,
    pub proxied: bool,
    pub var_declared: bool,
}
```

```rust
pub enum StateKind {
    State,
    StateRaw,
    StateEager,
}
```

```rust
pub struct DerivedDeclarationSemantics {
    pub kind: DerivedKind,
}
```

```rust
pub enum DerivedKind {
    Derived,
    DerivedBy,
}
```

```rust
pub struct PropDeclarationSemantics {
    pub lowering_mode: PropLoweringMode,
    pub kind: PropDeclarationKind,
}
```

```rust
pub enum PropDeclarationKind {
    Identifier,
    Object {
        properties: Vec<PropsObjectPropertySemantics>,
        has_rest: bool,
    },

    Source {
        bindable: bool,
        updated: bool,
        default_lowering: PropDefaultLowering,
        default_needs_proxy: bool,
    },
    Rest,
    NonSource,
}
```

```rust
pub enum PropsObjectPropertySemantics {
    Source {
        bindable: bool,
        updated: bool,
        default_lowering: PropDefaultLowering,
        default_needs_proxy: bool,
    },
    NonSource,
}
```

```rust
pub struct StoreDeclarationSemantics;
```

```rust
pub enum ConstDeclarationSemantics {
    ConstTag,
}
```

```rust
pub enum ContextualDeclarationSemantics {
    EachItem,
    EachIndex,
    AwaitValue,
    AwaitError,
    LetDirective,
    SnippetParam,
}
```

This answer is declaration-oriented only.

It should describe what one declaration root means, not how one concrete reference use-site should be lowered.

## Draft Reference Shape

The current draft direction for reference/use-site meaning is:

```rust
pub enum ReferenceSemantics {
    NonReactive,

    SignalRead(SignalReadSemantics),
    SignalWrite(SignalWriteSemantics),

    StoreRead(StoreReadSemantics),
    StoreWrite(StoreWriteSemantics),

    PropRead(PropReadSemantics),
    PropMutation(PropMutationSemantics),

    ConstAliasRead(ConstAliasReadSemantics),
    CarrierRead(CarrierReadSemantics),

    IllegalWrite,
    Unresolved,
}
```

This is intentionally:

- one answer per `ReferenceId`
- operation-oriented
- not split into separate public `read_*` / `write_*` query methods

The consumer should ask once for a concrete reference use-site and then pattern-match on the returned operation kind.

Two important constraints:

- `NonReactive` means no special reactive handling is required for that reference use-site
- the AST context still tells the consumer whether it is handling a read, assignment, update, bind target, and so on

This avoids forcing the consumer into:

- multiple queries for one ordinary use-site
- `if write { ... } else { ... }` control flow driven by separate semantic API calls

The exact payload shapes for:

- `SignalReadSemantics`
- `SignalWriteSemantics`
- `StoreReadSemantics`
- `StoreWriteSemantics`
- `PropReadSemantics`
- `PropMutationSemantics`
- `ConstAliasReadSemantics`
- `CarrierReadSemantics`

are still open, but the outer shape is intended to stay operation-oriented and single-query.

## Lowering Boundary

We want consumer-facing answers that are stronger than raw classification bits.

That means `reactivity_semantics` may tell consumers things like:

- plain read
- signal read
- safe signal read
- store read
- prop read
- illegal write
- signal write
- store write

But it must not become backend-specific lowering IR.

In particular, `reactivity_semantics` should not own:

- runtime helper names
- emission ordering
- backend-specific sequencing
- concrete codegen builder recipes

The contract should stay:

- analyzer decides required reactive operation
- transform/codegen decide how that operation is realized in the current backend

This also means generic facts like `is_mutated` must not leak through as consumer-facing semantics.

- `ComponentSemantics` may answer generic questions like whether a symbol has writes
- `reactivity_semantics` may use those facts internally
- transform/codegen should not consume those generic facts directly as reactive meaning

## Dependency Boundary

`reactivity_semantics` should be built on top of `crates/svelte_component_semantics`.

`svelte_component_semantics` remains the generic source of truth for:

- symbol identity
- reference identity
- scope ownership
- generic declaration/reference resolution

`reactivity_semantics` then interprets those facts reactively.

It must not duplicate generic component semantics ownership.

Two concrete consequences already agreed:

- `is_mutated` is a generic input from `ComponentSemantics`, not a public reactive semantic answer
- proxy-ness for `$state` is reactive meaning, but it should live as normalized state semantics rather than as a separate name-based raw table contract

Builder input rule:

- `reactivity_semantics` may use only Svelte-agnostic facts from `ComponentSemantics`
- if a helper on top of `ComponentSemantics` encodes Svelte-specific meaning, that meaning belongs in `reactivity_semantics`, not in the builder input surface

## Builder Inputs

The target direction is:

- `reactivity_semantics` depends on `ComponentSemantics`
- `reactivity_semantics` depends on AST / parsed syntax
- `reactivity_semantics` does not depend on legacy reactive classification tables as required semantic inputs

In other words:

- `ComponentSemantics` provides identity and generic symbol/reference facts
- AST traversal provides the Svelte- and syntax-specific structure
- the reactivity builder should derive reactive meaning itself

For clarity, allowed generic `ComponentSemantics` inputs include facts like:

- symbol and reference identity
- scope ownership
- resolved declaration/reference relationships
- symbol flags
- generic write/mutation facts such as whether a symbol has writes

Forbidden architectural inputs include Svelte-specific helpers such as:

- `rune_kind`
- `is_proxy_init_state`
- `is_var_declared_state`
- `is_store`
- `is_getter`
- `is_rest_prop`
- `known_value_by_sym`
- snippet/each/template-specific classification flags

Examples of facts the reactivity builder should be able to derive by walking AST:

- that a declaration came from `$props()`
- that a binding is a props rest binding
- that a declaration came from `{@const}`
- that a binding is an each context or index binding
- that a binding came from `let:`
- that a reference use-site requires signal/store/prop/non-reactive handling

This means old feature-specific classification tables should not be treated as the architectural dependency of the new system.

At most, they may temporarily survive as migration debt or as raw helper inputs while ownership is being moved.

The final ownership goal is for `reactivity_semantics` to collect the reactive meaning it needs directly from:

- `ComponentSemantics`
- Svelte AST
- parsed JS/template AST

## What This System Is Replacing

At a high level, the new system should replace scattered ownership of reactive meaning.

It should absorb semantic interpretation currently spread across combinations of:

- `ComponentScoping`
- feature-specific passes
- side tables that already encode reactive policy
- consumer fallback logic in transform/codegen

It should not blindly absorb all template or analyzer data.

Data that is only:

- structural
- topological
- scoping-related
- parser/analyze glue
- raw source fact storage

may remain outside `reactivity_semantics` as inputs to its builder.

The detailed keep/move/delete inventory is intentionally deferred to the next pass over existing data.

## Migration Direction

The migration direction is:

1. deprecate the old semantic surface
2. deprecate the current transitional reactivity builder
3. introduce `reactivity_semantics v2`
4. have the new builder collect reactive meaning directly from `ComponentSemantics + AST`
5. migrate old features onto the new meaning model
6. delete old ownership as each migrated cluster closes

This is not a plan to grow a second long-lived parallel system.

The old API and old builder may temporarily survive during migration,
but only as deprecated transitional surface.

## Deprecated Surface

The following are now considered deprecated migration surface rather than final architecture:

- the current getter-oriented `reactivity_semantics` API
- the current transitional builder shape
- old consumer paths that reassemble meaning from scattered reactive facts

The target is:

- new declaration-oriented answers keyed by declaration-root `OxcNodeId`
- new reference/use-site-oriented answers keyed by `ReferenceId`
- new builder ownership that derives meaning directly from `ComponentSemantics + AST`

## Migration Unit

The migration unit must be a semantic cluster closed end to end.

Do not treat these as valid migration units:

- a new enum variant without a real consumer
- a new getter without migrated call sites
- builder logic added but unused
- partial duplication where both old and new semantic owners remain active for the same cluster

The minimum valid migration unit is:

- builder support for one semantic cluster
- public answer shape for that cluster
- at least one real consumer migrated to the new answer
- old consumer-side meaning assembly for that cluster removed
- tests covering the new path

## Cluster-Based Migration

Migration should proceed by semantic clusters, not by random files or tiny scattered edits.

Examples of valid clusters:

- state and derived declarations plus reference operations
- props declarations plus prop reads and prop mutations
- store reads, writes, updates, and deep mutations
- `{@const}` declarations plus const-alias reads
- contextual bindings such as each, await, `let:`, and snippet params
- bind-target semantics and mutation behavior
- legacy props and legacy reactive behavior

Each cluster should land as a bounded, shippable step.

## Definition Of Done For A Migrated Cluster

A cluster is not migrated just because new types or builder code exist.

A cluster is done only when:

- the `v2` builder derives that cluster's meaning directly from `ComponentSemantics + AST`
- the new API can answer the declaration/reference questions for that cluster
- at least one real consumer uses the new answer instead of old scattered facts
- old ownership for that cluster is removed or reduced to non-consumer transitional debt
- tests cover the migrated path

## Anti-Stall Rule

The migration should avoid tiny checkpoints that do not materially reduce old ownership.

In particular, do not spend a session landing:

- only scaffolding
- only a new enum
- only a new accessor
- only producer logic without consumer migration
- only consumer call-site rewiring without deleting old meaning assembly

The point of each checkpoint is to close real semantic ownership, not to prepare indefinitely.

## Current Agreed Principles

- `reactivity_semantics` is the only source of truth for reactive meaning.
- Consumers should ask for semantic answers, not raw fact bits.
- Codegen and transform should become dumb consumers.
- Consumer-facing semantic queries should be cheap on ordinary hot paths.
- Declaration semantics are keyed by declaration-root `OxcNodeId`.
- Operation semantics are keyed by `ReferenceId`.
- Internal storage may be sparse, but the public API should be total for resolved cases.
- Missing special entries should normalize to `NonReactive`, not leak as `None`.
- `Unresolved` must stay distinct from `NonReactive`.
- `SymbolId` remains valid as internal ownership identity, but not as the public declaration semantic key.
- Consumer-facing answers may be close to lowering decisions, but must remain backend-agnostic.
- `svelte_component_semantics` is a dependency and foundation, not something to reimplement inside reactivity.
- `is_mutated` is an input to reactive semantics, not a consumer-facing reactive query.
- proxy state behavior is reactive meaning and should be represented inside state semantics, not as an external raw API shape.
- the reactivity builder should depend on `ComponentSemantics` plus AST, not on legacy reactive classification tables as architectural inputs.

## Open Questions

Still intentionally unresolved:

- exact names of the public query methods
- exact answer object shapes for declaration semantics
- exact answer object shapes for reference semantics
- exact enum naming for `NonReactive`, direct operations, and `Unresolved`
- which existing types in `crates/svelte_analyze/src/types/data` stay raw inputs
- which existing passes/tables are deleted versus slimmed down
