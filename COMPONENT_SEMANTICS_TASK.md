# Task: Design a Component-Wide Semantics Core for `.svelte` Compilation

## Context

This repository is a Rust implementation of the Svelte compiler. The current semantic model is split:

- `instance <script>` semantics are built using `oxc_semantic::SemanticBuilder`
- `module <script module>` semantics are also built through OXC and then merged/imported into our component scoping model
- template expressions/statements use custom copied passes because `oxc_semantic::Scoping` is not designed for incrementally attaching template JS into the same semantic graph

Relevant current code:

- `crates/svelte_analyze/src/scope.rs`
- `crates/svelte_analyze/src/passes/template_scoping.rs`
- `crates/svelte_analyze/src/passes/template_semantic.rs`
- `crates/svelte_analyze/src/passes/js_analyze/script_body.rs`
- `crates/svelte_analyze/src/lib.rs`

Original OXC semantics to study and potentially fork/adapt:

- local cargo sources, if available on disk via Cargo registry:
  - `~/.cargo/registry/src/**/oxc_semantic-*/src/builder.rs`
  - `~/.cargo/registry/src/**/oxc_semantic-*/src/scoping.rs`
  - `~/.cargo/registry/src/**/oxc_semantic-*/src/scope.rs`
  - `~/.cargo/registry/src/**/oxc_semantic-*/src/symbol.rs`
  - `~/.cargo/registry/src/**/oxc_semantic-*/src/reference.rs`
- if the crate source is vendored or pinned elsewhere in this repo, prefer that exact checked-in revision
- otherwise use the upstream OXC repository matching the version in `Cargo.lock`

When looking up the upstream source, focus first on:

- the semantic builder implementation
- scope creation and parent tracking
- symbol creation and binding registration
- reference creation and resolution
- `Read` / `Write` / `ReadWrite` propagation for assignments and updates

## Core problem

`oxc_semantic::Scoping` is the wrong source-of-truth model for a `.svelte` component.

Why:

- it assumes semantics are built over one ready-made JS `Program`
- it is not convenient to incrementally register template `Expression` / `Statement` nodes into an already-built semantic graph
- we already had to copy OXC semantic/scoping logic for template JS because we cannot attach those nodes naturally to existing script scoping
- module, instance, and template semantics are conceptually one component-wide graph, but today they are built in separate stages and glued together afterward

## Goal

Design a new component-wide semantics storage and builder API that becomes the real source of truth for:

- scopes
- symbols
- references
- reference resolution
- JS node lookup
- ownership boundaries between module script, instance script, template, and synthetic nodes

The new design should let us eventually stop using `oxc_semantic::Scoping` as the primary semantic storage.

## Important design direction

We do want to reuse or fork the useful semantic machinery from OXC:

- AST traversal patterns
- scope creation rules
- binding registration
- reference creation
- `Read` / `Write` / `ReadWrite` propagation
- lexical name resolution

But we do **not** want to keep OXC's storage model or its assumption that semantics belong to a single `Program`.

In other words:

- fork algorithms where useful
- own the storage ourselves
- extend the model for Svelte component needs

## Crate boundary

Assume this should most likely become a separate crate rather than remain an internal `svelte_analyze` module.

Recommended crate name:

- `svelte_component_semantics`

Reasoning:

- this is infrastructure with its own ids, storage, builder, and query API
- it is larger than a local refactor of `scope.rs`
- it should be testable independently from the rest of analyze
- it should not inherit accidental responsibilities from `svelte_analyze`

Expected ownership of the new crate:

- semantic ids: `ScopeId`, `SymbolId`, `ReferenceId`, `SemanticNodeId`
- storage tables and reverse lookup maps
- owner/source metadata
- component root scopes: module, instance, template
- builder API
- lexical resolution
- reference flag propagation
- JS node mappings via `OxcNodeId`

Things that should probably stay in `svelte_analyze`:

- rune semantics
- props/store-specific semantic classification
- snippet/each/await classification policy beyond raw scope/symbol/reference construction
- codegen-oriented side tables
- diagnostics policy

I want you to explicitly evaluate whether the crate boundary above is the right one. If you disagree, explain why and propose a better boundary.

## Specific design requirements

### 1. One component-wide semantic graph

The storage must represent one `.svelte` component as one semantic graph containing:

- module script
- instance script
- template expressions
- template statements
- synthetic declarations/references introduced by analysis if needed

This graph must support lexical resolution across the component in the Svelte direction:

- module can see module only
- instance can see instance + module
- template can see template + instance + module

The structural parent chain should be:

```text
module root
  -> instance root
    -> template root
      -> nested template/local JS scopes
```

### 2. Separate semantic ownership from lookup

The design must distinguish between:

- lexical parent chain
- ownership/source of a node

For example:

- a template-owned JS reference may resolve to an instance or module symbol
- a synthetic helper symbol may live in template scope but still be marked as synthetic

Suggested owner model:

- `ModuleScript`
- `InstanceScript`
- `Template`
- `Synthetic`

### 3. Keep `svelte_ast::NodeId` and `oxc::NodeId` separate

Do **not** try to replace all ids with one universal id type.

We need both domains:

- `svelte_ast::NodeId` for template/component ownership
- `oxc::NodeId` for JS AST lookup

These should be connected by explicit mappings in the new storage.

### 4. Use `oxc::NodeId` for JS lookup

We want `oxc::NodeId` to become the primary bridge for JS-node lookup.

This means:

- every JS AST node we care about should have an `OxcNodeId`
- module, instance, and template JS nodes should live in one component-wide `OxcNodeId` space if feasible
- JS-related lookup should stop depending on span keys as the canonical mechanism

Important:

- `OxcNodeId` is for JS AST domain only
- it should not replace `svelte_ast::NodeId`
- semantic identity should still use our own ids for `ScopeId`, `SymbolId`, `ReferenceId`, and possibly `SemanticNodeId`

### 5. Incremental registration

The builder must allow incremental registration of semantic inputs:

- register module program
- register instance program
- later attach template expressions/statements
- later attach synthetic symbols/references if needed

This is one of the main reasons the current OXC storage is not sufficient.

### 6. JS semantics to support

The design must cover at least the semantic machinery we already rely on:

- function scopes
- arrow function scopes
- block scopes
- `for`, `for in`, `for of`
- `catch`
- parameter bindings
- variable/function/class/import bindings
- destructuring bindings
- identifier references
- assignment targets
- update expressions
- `Read`, `Write`, `ReadWrite` flags
- lexical resolution through parent scopes

Please also address explicitly:

- `var` behavior and hoisting-sensitive lookup
- function declaration hoisting
- class declaration behavior
- import bindings
- whether unresolved references are stored as first-class records
- whether mutation/reassignment tracking belongs in the core storage or in `svelte_analyze`

It is acceptable to scope the first implementation to the semantic features currently required by this compiler. It does not need to reimplement every OXC feature on day one.

## What I need from you

Produce a detailed technical design for the new semantics core.

I do **not** want a generic high-level essay. I want something concrete enough that we can implement from it.

The answer should include:

### A. Proposed storage model

Define the central storage types, including:

- scope records
- symbol records
- reference records
- optional semantic-node/source-node records
- owner/source metadata
- reverse lookup maps
- component root scopes

I want clear answers to:

- what ids exist
- what each table stores
- what is the source of truth
- how `svelte_ast::NodeId` and `oxc::NodeId` are related
- whether unresolved references are stored, and if so where
- where mutation/reassignment information lives
- whether hoisting-sensitive metadata is stored directly in symbol/scope tables or derived later

### B. Proposed builder API

Define the builder entry points and lower-level mutation methods.

The API should support:

- registering module program semantics
- registering instance program semantics
- registering template expressions/statements
- declaring synthetic symbols
- creating child scopes
- creating bindings
- creating references
- resolving references

Please cover both declaration-time and assignment-time destructuring:

- declaration patterns
- assignment target patterns
- nested patterns
- default values
- rest elements

If useful, split the builder into layers:

- public high-level API
- internal low-level primitives used by semantic walkers

Also propose the minimal public API surface of the crate itself. For example:

- which modules it exports
- which types should be public
- which builder/query methods should be stable entry points

### C. `NodeId` strategy

Think carefully about whether we can and should synchronize `oxc::NodeId` across:

- module script
- instance script
- template expressions/statements

I want an explicit recommendation:

- can we make `OxcNodeId` unique component-wide?
- what are the risks?
- what assumptions in OXC might make this hard?
- if component-wide `OxcNodeId` turns out unsafe, what is the fallback design?

### D. What semantic core to fork from OXC

List which parts of OXC semantic machinery should be copied or adapted:

- traversal rules
- scope-enter/scope-exit logic
- binding registration
- resolution
- reference flag propagation

Also list what **should not** be copied wholesale:

- storage assumptions
- program-local assumptions
- any parts that would overfit us to OXC internals

### E. Dependency model

Describe the dependency and ownership relationships between:

- module script
- instance script
- template
- synthetic nodes

I want a clear model for:

- lexical lookup
- ownership metadata
- template statements that introduce bindings
- snippet/each/await/etc. scopes
- shorthand template references such as `bind:`, `class:`, and `style:`
- `@const` declarations
- each context/index bindings
- await `then` / `catch` bindings
- snippet parameters

Also explain whether template-specific conveniences like shorthand references are represented as normal references in the core storage or as higher-level sugar handled outside it.

### F. Migration plan

Provide an implementation plan that can be done in slices.

It should explicitly cover:

1. building the new storage
2. moving template semantics first
3. moving script semantics after that
4. keeping the codebase working during migration
5. deleting old OXC-backed source-of-truth plumbing at the end

The migration plan should also say whether to:

- create the crate immediately and move code into it from day one
- or first prototype inside `svelte_analyze` and extract afterward

I want an explicit recommendation here.

Also include a compatibility map from current `ComponentScoping` consumers to the new API. I want to know which existing responsibilities should have direct replacements, for example:

- `find_binding`
- `symbol_scope_id`
- `symbol_name`
- mutation checks
- template scope lookup
- source-node to semantic-record lookup

## Constraints

- Do not propose hand-wavy abstractions without data ownership details
- Do not collapse `svelte_ast::NodeId` and `oxc::NodeId` into one id type
- Do not assume we can keep using `oxc_semantic::Scoping` as the real backing store
- Prefer a design that is append-friendly and component-centric
- Keep phase boundaries in mind: this is analyze-layer infrastructure, not a parser or codegen concern

## Desired output format

I want the response structured like this:

1. Summary of the proposed direction
2. Storage design
3. Builder API
4. NodeId strategy
5. OXC fork surface
6. Dependency/ownership model
7. Migration plan
8. Risks and open questions

Concrete Rust-like type sketches and API signatures are strongly preferred.

## Additional note

We already suspect the right direction is:

- our own semantic storage
- OXC algorithms reused where useful
- component-wide `OxcNodeId` space for JS nodes if technically safe
- explicit bridges between JS nodes and Svelte nodes

But I want a rigorous design, not validation of that instinct.
