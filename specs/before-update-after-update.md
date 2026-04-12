# `beforeUpdate` / `afterUpdate`

## Current state
- **Working**: 0/4 use cases
- **Tests**: 0/4 green
- Last updated: 2026-04-12

## Source
- `ROADMAP.md` Legacy Svelte 4: `beforeUpdate` / `afterUpdate`
- User request: `/audit beforeUpdate / afterUpdate`

## Syntax variants
```svelte
<script>
	import { beforeUpdate } from 'svelte';

	beforeUpdate(() => {});
</script>
```

```svelte
<script>
	import { afterUpdate } from 'svelte';

	afterUpdate(() => {});
</script>
```

```svelte
<script>
	import { beforeUpdate as before, afterUpdate as after } from 'svelte';

	before(() => {});
	after(() => {});
</script>
```

```svelte
<script>
	import { beforeUpdate } from 'svelte';
	import { afterUpdate } from 'svelte';

	beforeUpdate(() => {});
	afterUpdate(() => {});
</script>
```

## Use cases

- [ ] Legacy components with direct `beforeUpdate` / `afterUpdate` imports preserve the hook registrations and emit `$.init()` before DOM creation (test: legacy_before_after_update_basic, #[ignore], moderate)
- [ ] Legacy components with aliased `beforeUpdate` / `afterUpdate` imports preserve the local call-sites and emit `$.init()` before DOM creation (test: legacy_before_after_update_alias, #[ignore], moderate)
- [ ] Runes mode rejects `beforeUpdate` / `afterUpdate` imports from `svelte`, including aliased imports, with the reference `runes_mode_invalid_import` diagnostic span (test: runes/validate_before_after_update_invalid_import, #[ignore], quick fix)
- [ ] Runes mode rejects split `beforeUpdate` and `afterUpdate` import declarations from `svelte` in the same component, matching the current reference `runes_mode_invalid_import` count and span (test: runes/validate_before_after_update_invalid_import_split_statements, #[ignore], quick fix)

## Out of scope

- Runtime scheduling changes from the Svelte 5 migration guide: initial-render double-run removal, parent/child `afterUpdate` ordering, and slot-content update behavior
- SSR lifecycle behavior

## Reference
### Svelte
- `reference/docs/06-runtime/03-lifecycle-hooks.md`
- `reference/docs/07-misc/07-v5-migration-guide.md`
- `reference/compiler/phases/2-analyze/visitors/ImportDeclaration.js`
- `reference/compiler/migrate/index.js`

### Our code
- `crates/svelte_component_semantics/src/builder/js_visitor.rs`
- `crates/svelte_analyze/src/validate/runes.rs`
- `crates/svelte_diagnostics/src/lib.rs`
- `crates/svelte_codegen_client/src/lib.rs`
- `tasks/compiler_tests/test_v3.rs`
- `tasks/diagnostic_tests/test_diagnostics.rs`

## Test cases

- [ ] `legacy_before_after_update_basic`
- [ ] `legacy_before_after_update_alias`
- [ ] `runes/validate_before_after_update_invalid_import`
- [ ] `runes/validate_before_after_update_invalid_import_split_statements`
