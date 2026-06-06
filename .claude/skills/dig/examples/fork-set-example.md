# Worked example — fork-set on one Original decision

Real walkthrough of dig steps 1–4 from this codebase's history (commits `14722eff` capture, `a25b4dbe` fix). Names and paths are real; the shape is what you replicate, not the strings.

> The internal terms below (fork-set, probe, green guard, Quick fix / Need research) are **your** working vocabulary for steps 1–4. They are banned from the Gate-1 message (step 5), which translates all of this to plain domain language — see `_shared/parity-language.md`.

## 1. Finding (handle)

Pasted Svelte:

```svelte
<script>
    import Child from './Child.svelte';
    let y = $state('y1');
</script>

<Child {y} />
```

`just quick-check <file> --print=both` → our output wraps the prop as a getter `{ get y() { return y; } }`, Original emits shorthand `{y}`. `y` is `$state` but never reassigned.

## 2. Original decision-fork

In `original/compiler/`, component-prop lowering has ONE decision-fork that produces the divergence:

> "For a component prop passed by shorthand, should the analyzer mark the source binding as needing a getter wrap, or pass it as a plain reference?"

Branches in the Original — by binding kind / mutation profile of the source:

| Branch | Original behavior |
|---|---|
| plain `const` bound to a literal | shorthand |
| plain `const` bound to a call (may leak reactivity inside) | getter |
| `$state` literal, never reassigned | **shorthand** (the finding — we emitted getter) |
| `$state` object/array, never written | **shorthand** (mutation is on inner state, not the binding) |
| `$state`, reassigned somewhere | getter |
| `$state` initialized from a function value | getter |
| `$derived` | getter (always — derived is reactive) |

Seven branches, all read from the Original's prop-emit decision — NOT memory, NOT our existing siblings.

## 3. Red-all probes (materialize first)

Under `tasks/compiler_tests/cluster_cases/component_props/`, one probe per branch, registered in `clusters/component_props.rs` via `compiler_case!`:

```
literal_const_shorthand/      # branch 1 — guard (must stay GREEN)
const_call_getter/            # branch 2 — guard
state_literal_shorthand/      # branch 3 — the finding (RED)
state_object_never_written/   # branch 4 — RED (same family as 3)
state_reassigned_getter/      # branch 5 — guard
state_function_getter/        # branch 6 — guard
derived_getter/               # branch 7 — guard
```

`just generate` → expected JS regenerated from Original for each probe. Run the suite:

| Probe | Status before fix |
|---|---|
| literal_const_shorthand | GREEN (guard) |
| const_call_getter | GREEN (guard) |
| state_literal_shorthand | RED — wraps shorthand $state as getter |
| state_object_never_written | RED — same shape, different kind |
| state_reassigned_getter | GREEN (guard) |
| state_function_getter | GREEN (guard) |
| derived_getter | GREEN (guard) |

Two reds = scope. Five greens = the guards that catch a patch sneaking past. Without `state_reassigned_getter` as a guard, the fix could silently flatten the kind-discrimination and pass the failing inputs while breaking reassignment — that's the patch pattern red-all blocks.

## 4. A/B verdict (read code, not assume)

Walk the three diamonds against our code:

- crate boundary? — fix lives in `crates/svelte_analyze/src/attribute_semantics/builder/mod.rs` only → **no**.
- shared data structure? — the predicate (`references_need_wrap`) already exists; new arm doesn't change its shape → **no**.
- net-new machinery? — the branch table is already there; missing arm for `OptimizedRune(_)` → **no**.

**Quick fix.** Next: fix the cluster — scope is clear.

The fix (committed `a25b4dbe`) added an `OptimizedRune(_)` arm in `references_need_wrap`. The would-be «Need research» framing — "we need a new mutation-tracker" — was the absence-list trap. A subagent dispatched cold tends to invent that machinery; reading the existing code refuted it.

## What this example shows

- **Probe from the file**, not the sample name — `state_literal_shorthand` came from observing the actual output of the pasted component.
- **Fork-set from the Original**, not from our existing four siblings of `references_need_wrap`.
- **Green guards are non-negotiable** — they're what makes the verdict honest.
- **A/B verified by reading**, not by sub-agent intuition; the diamonds answered "no" only after looking at the predicate's actual call site.
