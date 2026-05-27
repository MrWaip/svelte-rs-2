# svelte:fragment

## Current state
- **Working**: 14/14 use cases
- **Tests**: 15/15 green
- Last updated: 2026-05-17

## Source
- ROADMAP legacy item: `<slot>` + `let:` + `<svelte:fragment>` + `slot attribute` → [legacy-slots](legacy-slots.md)
- User request: re-audit `<svelte:fragment>` after real-world panic on `Disclaimer.svelte` (`NotImplemented { feature: "<svelte:fragment> (legacy slot wrapper)" }` at `crates/svelte_codegen_client/src/codegen/containers/legacy_slot.rs:20`).

## How It Works

`<svelte:fragment>` is a legacy Svelte 4 wrapper: a placeholder element with no DOM output. Valid only as a direct child of `Component` / `SvelteComponent` / `SvelteSelf`. Two roles:

1. **Named-slot fill** — `<svelte:fragment slot="x">…</svelte:fragment>` lifts its children into the parent component's `$$slots.x` arrow. Reference path: `phases/3-transform/client/visitors/shared/component.js` (slot grouping).
2. **Default-slot fill** — `<svelte:fragment>…</svelte:fragment>` without `slot=` participates in the parent's default-slot grouping (`children` prop or `$$slots.default`). Reference: same grouping pass; the fragment's `LetDirective`s populate `lets.default`.

`<svelte:fragment>` itself never reaches `client/visitors/SvelteFragment.js` when the parent component picks it up by slot grouping — the visitor is only used when a `<svelte:fragment>` survives into a generic fragment walk (Svelte 5 boundary content or future-proofing). In that path it visits its `LetDirective`s and inlines its fragment body without a DOM wrapper.

Validation (`phases/2-analyze/visitors/SvelteFragment.js`): parent must be Component-like; only `slot` attribute and `LetDirective` allowed; everything else → `svelte_fragment_invalid_attribute`. `<svelte:fragment>` is also an allowed direct parent for `{@const}` (`visitors/ConstTag.js:22`).

## Syntax variants
```svelte
<Component><svelte:fragment slot="footer">named slot content</svelte:fragment></Component>
<Component><svelte:fragment slot="item" let:item>{item.text}</svelte:fragment></Component>
<Component><svelte:fragment slot="item" let:item={{ text }}>{text}</svelte:fragment></Component>
<Component><svelte:fragment>default slot content</svelte:fragment></Component>
<Component><svelte:fragment let:item>{item}</svelte:fragment></Component>
<Component><svelte:fragment slot="x">{@const v = compute()}<p>{v}</p></svelte:fragment></Component>
<svelte:self><svelte:fragment slot="x" /></svelte:self>
<svelte:component this={C}><svelte:fragment slot="x" /></svelte:component>
```

## Use cases

- [x] Dedicated `SvelteFragmentLegacy` AST node at parse time (tests: `legacy_svelte_fragment_converts_to_dedicated_node`, `svelte_fragment_named_slot`)
- [x] Analyze/codegen consume the dedicated `SvelteFragmentLegacy` node (test: `component_named_slot_mapping_uses_svelte_fragment_legacy_wrapper_id`)
- [x] Named-slot wrapper `<svelte:fragment slot="X">` lifts into parent `$$slots.X` without wrapper DOM (test: `svelte_fragment_named_slot`)
- [x] `let:` on named-slot `<svelte:fragment>` lowers inside the generated named-slot arrow, incl. object destructuring (tests: `component_named_slot_let_fragment`, `component_named_slot_let_fragment_destructure`)
- [x] Validation: non-component parent → `svelte_fragment_invalid_placement` (test: `slots/svelte_fragment_invalid_placement_root`)
- [x] Validation: any non-`slot` non-`let:` attribute (incl. `class:`) → `svelte_fragment_invalid_attribute` (test: `slots/svelte_fragment_invalid_attribute_class`)
- [x] **Default-slot `<svelte:fragment>` (no `slot=`) lowers as default-slot content / `children` prop without wrapper DOM** — codegen currently panics `NotImplemented` at `crates/svelte_codegen_client/src/codegen/containers/legacy_slot.rs:20-22`. Reference path: `phases/3-transform/client/visitors/shared/component.js:385-413` (default-slot grouping treats `<svelte:fragment>` exactly like a `<div slot="default">`-style wrapper but without DOM). (test: `svelte_fragment_default_slot_wrapper`, missing, **moderate**)
- [x] **`let:` on default-slot `<svelte:fragment>` forces `$$slots.default` arrow path instead of `children` prop** — reference: `client/visitors/shared/component.js:390-393` short-circuits the `children` prop when any default `<svelte:fragment>` carries a `LetDirective`. Depends on default-slot wrapper lowering above. (test: `svelte_fragment_default_slot_let`, missing, **moderate**)
- [x] **`{@const}` as direct child of `<svelte:fragment>` is accepted by validator and emitted into the slot arrow body** — reference `visitors/ConstTag.js:22` lists `SvelteFragment` among allowed grand-parents. Our validator path goes through `template_validation.rs` `ParentKind::SvelteFragmentLegacy` (line 1346) — needs explicit test coverage. (test: `svelte_fragment_named_slot_with_const_tag`, missing, **quick fix**)
- [x] **Validation: `<svelte:fragment>` not a direct child of Component (e.g. inside an element inside Component) → `svelte_fragment_invalid_placement`** — reference checks `context.path.at(-2)` strictly. Currently only root-level placement test exists. (test: `slots/svelte_fragment_invalid_placement_nested_in_element`, missing, **quick fix**)
- [x] **Validation: non-`class:` invalid directives (`bind:`, `on:`, `use:`, `transition:`, `in:`, `out:`, `animate:`, `style:`) → `svelte_fragment_invalid_attribute`** — only `class:` currently covered. Reference `visitors/SvelteFragment.js:21-23` rejects every non-`LetDirective` non-`slot` attribute uniformly. (test: `slots/svelte_fragment_invalid_attribute_bind`, missing, **quick fix**)
- [x] **AttributeSemantics walker recurses into `SvelteFragmentLegacy.fragment`** — layer: 3.A.4 `AttributeSemantics`. Component-prop expression attribute on a `ComponentNode` nested inside `<svelte:fragment slot="X">` is classified as `NonSpecial` instead of `ComponentProp::Expression`, then codegen panics in `component_props/dispatch.rs:191` with `unsupported NonSpecial attribute on ComponentNode`. Root cause: `walk_fragment` in `crates/svelte_analyze/src/attribute_semantics/builder/mod.rs` has no arm for `Node::SvelteFragmentLegacy`, so its inner fragment is never visited; `cn.legacy_slots` walks the slot fragment but stops at the `SvelteFragmentLegacy` wrapper. Reference does this implicitly via a generic visitor descent. (test: `svelte_fragment_named_slot_component_expr_attr`, **quick fix**)
- [x] **Explicit `slot="default"` on `<svelte:fragment>` is normalized to the default-slot group (`children` prop + `$$slots: { default: true }`)** — layer: parser. `slot="default"` is semantically identical to the absent `slot` attribute, but `Parser::slot_name_of` in `crates/svelte_parser/src/lib.rs:211` returns `Some("default")` for any non-empty `slot` value, so `partition_component_children` pushes the fragment into `legacy_slots` as a named slot called `"default"`. Codegen then emits `$$slots: { default: ($$anchor, $$slotProps) => {…} }` instead of hoisting the body to a `children` prop. Reference normalizes the value in `phases/3-transform/client/visitors/shared/component.js` (default-slot grouping). Fix candidate: treat `slot="default"` as `None` in `slot_name_of` so the fragment joins the default group. (test: `svelte_fragment_explicit_default_slot_attribute_lowers_to_children_prop`, **quick fix**)
- [x] **`<svelte:fragment slot="X">` as direct child of `<svelte:component>` / `<svelte:self>` must validate** — layer: 3.C `validate`. `visit_svelte_fragment_legacy` in `crates/svelte_analyze/src/passes/template_validation.rs:1094-1103` only accepts `ParentKind::ComponentNode` as the direct parent, so every `<svelte:fragment>` inside `<svelte:component>` or `<svelte:self>` raises a false-positive `svelte_fragment_invalid_placement` + `slot_attribute_invalid_placement` and aborts codegen. The element-level slot check at line 935-944 already lists all three parent kinds (`ComponentNode | SvelteComponentLegacy | SvelteSelf`); apply the same set here. Spec's `Syntax variants` block already shows both `<svelte:component>` and `<svelte:self>` as legal parents. Fix candidate: extend `is_direct_child_of_component` to match `ParentKind::ComponentNode | SvelteComponentLegacy | SvelteSelf`. (test: `svelte_fragment_named_slot_inside_svelte_component`, **quick fix**)

## Out of scope

- SSR rendering of `<svelte:fragment>` (`original/compiler/phases/3-transform/server/visitors/SvelteFragment.js`)
- Migration to Svelte 5 snippets (`original/compiler/migrate/index.js`)
- Single-child-not-needing-template template optimization (`client/visitors/Fragment.js:50`) — current codegen does not branch on this; cosmetic emit difference only

## Reference
### Svelte
- `original/docs/99-legacy/22-legacy-svelte-fragment.md`
- `original/compiler/phases/1-parse/state/element.js`
- `original/compiler/phases/2-analyze/visitors/SvelteFragment.js`
- `original/compiler/phases/2-analyze/visitors/ConstTag.js`
- `original/compiler/phases/2-analyze/visitors/LetDirective.js`
- `original/compiler/phases/2-analyze/visitors/shared/attribute.js`
- `original/compiler/phases/scope.js`
- `original/compiler/phases/3-transform/client/visitors/SvelteFragment.js`
- `original/compiler/phases/3-transform/client/visitors/Fragment.js`
- `original/compiler/phases/3-transform/client/visitors/shared/component.js`
- `original/compiler/errors.js`

### Our code
- `crates/svelte_ast/src/lib.rs` (`SvelteFragmentLegacy`)
- `crates/svelte_parser/src/svelte_elements.rs`
- `crates/svelte_analyze/src/passes/template_validation.rs`
- `crates/svelte_analyze/src/walker/traverse.rs`
- `crates/svelte_codegen_client/src/codegen/containers/legacy_slot.rs`
- `crates/svelte_codegen_client/src/codegen/containers/element.rs`
- `crates/svelte_codegen_client/src/codegen/fragment/legacy_slot_fragment.rs`
- `crates/svelte_codegen_client/src/codegen/let_directive_legacy.rs`

## Test cases

- [x] svelte_fragment_named_slot
- [x] component_named_slot_let_fragment
- [x] component_named_slot_let_fragment_destructure
- [x] legacy_svelte_fragment_converts_to_dedicated_node
- [x] component_named_slot_mapping_uses_svelte_fragment_legacy_wrapper_id
- [x] slots/svelte_fragment_invalid_placement_root
- [x] slots/svelte_fragment_invalid_attribute_class
- [x] svelte_fragment_default_slot_wrapper
- [x] svelte_fragment_default_slot_let
- [x] svelte_fragment_named_slot_with_const_tag
- [x] slots/svelte_fragment_invalid_placement_nested_in_element
- [x] slots/svelte_fragment_invalid_attribute_bind
- [x] svelte_fragment_named_slot_component_expr_attr
- [x] svelte_fragment_explicit_default_slot_attribute_lowers_to_children_prop
- [x] svelte_fragment_named_slot_inside_svelte_component
