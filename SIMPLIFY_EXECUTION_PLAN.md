# Simplify Execution Plan

Detailed execution plan for the first four simplify items:

1. `TemplateQueryView`
2. Unified special-host dispatcher
3. Unified event/action emission path
4. Staged `process_element`

This document is intentionally concrete. The next agent should not need to infer scope, ownership, or rollout order.

Note: the numbered items below preserve the simplify initiative numbering. The actual implementation order is defined later in `Execution Order`.

## Goal

Reduce long-term complexity in template analyze/transform/codegen without changing behavior:

- faster feature ports
- lower cognitive load for humans and agents
- fewer duplicated business decisions across host kinds
- better lookup locality on hot paths
- strict phase boundaries preserved

## Hard Constraints

1. Do not change behavior intentionally.
2. Do not move client-only lowering policy into `svelte_analyze`.
3. Do not introduce string-based semantic rediscovery in codegen.
4. Do not create a mega-IR that tries to serve both client and SSR.
5. Do not rewrite all host kinds at once.
6. Keep diffs task-scoped. No unrelated cleanup.

## Boundary Rules For This Plan

`svelte_analyze` may own:

- structural template queries
- target-neutral semantic facts
- host kind facts if they are canonical and not backend-specific
- precomputed scope and fragment lookups

`svelte_codegen_client` must continue to own:

- `init/update/after_update` placement
- runtime helper selection such as `$.event`, `$.delegated`, `$.action`
- blocker wrapping policy application
- output ordering that is specific to client runtime semantics

`svelte_transform` may consume shared query accessors, but should not invent a second template lookup model.

## Out Of Scope

- SSR lowering design
- a global shared lowering IR
- full `TemplateNodeIndex` rewrite
- accessor macro cleanup
- broad `Ctx` cleanup beyond what is needed for this plan
- unrelated feature completion work from existing specs

## Related Specs To Keep In Mind

- [specs/svelte-window-document-body.md](/Users/klobkov/personal-code/svelte-rs-2/specs/svelte-window-document-body.md:1)
- [specs/events.md](/Users/klobkov/personal-code/svelte-rs-2/specs/events.md:1)
- [specs/bind-directives.md](/Users/klobkov/personal-code/svelte-rs-2/specs/bind-directives.md:1)
- [specs/element.md](/Users/klobkov/personal-code/svelte-rs-2/specs/element.md:1)
- [specs/svelte-element.md](/Users/klobkov/personal-code/svelte-rs-2/specs/svelte-element.md:1)
- [specs/use-action.md](/Users/klobkov/personal-code/svelte-rs-2/specs/use-action.md:1)
- [specs/attach-tag.md](/Users/klobkov/personal-code/svelte-rs-2/specs/attach-tag.md:1)

## Execution Order

Run the work in this order:

1. `TemplateQueryView`
2. Unified event/action helpers
3. Unified special-host dispatcher
4. Staged `process_element`

This order is intentional:

- step 1 reduces lookup chaos before codegen refactors
- step 2 removes obvious duplication that the dispatcher will depend on
- step 3 consolidates the smallest high-duplication host family first
- step 4 refactors the most complex path after helper/query conventions are stable

## Step 1: `TemplateQueryView`

### Objective

Create one stable query facade for template lookups so `analyze`, `transform`, and `codegen` stop pulling facts from unrelated side tables directly.

This is a query layer, not a lowering plan.

### Do

Add a thin facade in `svelte_analyze` that composes existing data:

- topology
- fragment data
- element facts
- element flags
- bind semantics
- scope lookups already exposed through analysis

### Do Not Do

- do not store runtime helper names
- do not store emission phases
- do not store client-only opcodes
- do not copy AST payloads into new tables

### Suggested API Surface

Start with the smallest API that removes current cross-table guessing:

```rust
pub struct TemplateQueryView<'a> { /* wraps AnalysisData */ }

impl<'a> TemplateQueryView<'a> {
    pub fn parent(&self, id: NodeId) -> Option<ParentRef>;
    pub fn expr_parent(&self, id: NodeId) -> Option<ParentRef>;
    pub fn node_fragment(&self, id: NodeId) -> Option<FragmentKey>;
    pub fn nearest_element(&self, id: NodeId) -> Option<NodeId>;
    pub fn nearest_element_for_expr(&self, id: NodeId) -> Option<NodeId>;

    pub fn lowered_fragment(&self, key: &FragmentKey) -> Option<&LoweredFragment>;
    pub fn content_type(&self, key: &FragmentKey) -> ContentStrategy;
    pub fn has_dynamic_children(&self, key: &FragmentKey) -> bool;
    pub fn fragment_blockers(&self, key: &FragmentKey) -> &[u32];

    pub fn attr_index(&self, id: NodeId) -> Option<&AttrIndex>;
    pub fn has_spread(&self, id: NodeId) -> bool;
    pub fn namespace(&self, id: NodeId) -> Option<NamespaceKind>;
    pub fn creation_namespace(&self, id: NodeId) -> Option<Namespace>;

    pub fn has_class_attribute(&self, id: NodeId) -> bool;
    pub fn class_directive_info(&self, id: NodeId) -> Option<&[ClassDirectiveInfo]>;
    pub fn style_directives(&self, id: NodeId) -> &[StyleDirective];
    pub fn is_dynamic_attr(&self, id: NodeId) -> bool;
    pub fn event_handler_mode(&self, id: NodeId) -> Option<EventHandlerMode>;
    pub fn has_use_directive(&self, id: NodeId) -> bool;
    pub fn needs_input_defaults(&self, id: NodeId) -> bool;
    pub fn needs_textarea_value_lowering(&self, id: NodeId) -> bool;
    pub fn option_synthetic_value_expr(&self, id: NodeId) -> Option<NodeId>;
    pub fn is_customizable_select(&self, id: NodeId) -> bool;
    pub fn is_selectedcontent(&self, id: NodeId) -> bool;

    pub fn is_mutable_rune_target(&self, id: NodeId) -> bool;
    pub fn has_bind_group(&self, id: NodeId) -> bool;
    pub fn bind_blockers(&self, id: NodeId) -> &[u32];
}
```

### Ownership

Primary files:

- `crates/svelte_analyze/src/types/data/codegen_view.rs`
- `crates/svelte_analyze/src/types/data/analysis.rs`
- `crates/svelte_analyze/src/types/data/template_topology.rs`
- `crates/svelte_analyze/src/types/data/template_element_index.rs`
- `crates/svelte_codegen_client/src/context.rs`
- `crates/svelte_transform/src/lib.rs`

### Required Refactor

1. Add `TemplateQueryView` in `svelte_analyze`.
2. Back it with existing tables only.
3. Make `CodegenView` reuse or embed it instead of mirroring everything manually.
4. Move codegen read paths toward `ctx.query.*`.
5. Move transform template lookup paths toward the same facade where possible.

### Acceptance

- no behavior change
- no new duplicate getters in `Ctx` unless they combine query + mutable state
- `transform` and codegen can answer common template questions through one query contract
- no new direct cross-table lookup spaghetti in touched code

## Step 2: Unified Event And Action Emission Helpers

### Objective

Remove duplicated event/action lowering logic before introducing host dispatch.

### Current Duplication Targets

- legacy event emitters in `template/events/emit.rs`
- special-host event attr emit path
- blocker wrapping in `template/events/actions.rs`

### Do

Introduce small shared helper functions inside `svelte_codegen_client/src/template/events/`:

- one helper to build a legacy event handler expression
- one helper to apply modifier wrappers and event call arguments
- one helper to emit `$.event(...)` to an explicit target
- one helper to wrap a statement in `$.run_after_blockers(...)`

Keep the helpers narrow and output-oriented.

### Do Not Do

- do not move this logic into `svelte_analyze`
- do not invent a generic event IR yet
- do not unify component events into this pass

### Concrete Refactor Targets

Primary files:

- `crates/svelte_codegen_client/src/template/events/emit.rs`
- `crates/svelte_codegen_client/src/template/events/actions.rs`
- `crates/svelte_codegen_client/src/template/events/handlers.rs`
- `crates/svelte_codegen_client/src/template/attributes.rs`

### Required Changes

1. Replace `gen_on_directive_legacy` and `gen_legacy_event_on` with a single internal implementation that takes:
   - target name
   - destination statement buffer
   - legacy directive payload
2. Keep the DOM-element entry point and special-host entry point as thin wrappers.
3. Extract blocker wrapping into one shared utility used by:
   - `gen_use_directive`
   - `gen_attach_tag`
   - `gen_transition_directive`
   - `gen_animate_directive`
4. Keep event attr emission for DOM elements and special hosts structurally aligned, even if they still call different public wrappers.

### Acceptance

- special-host event emission no longer duplicates the full legacy-event builder
- blocker wrapping exists in one place
- no change to event ordering or modifier behavior
- no change to delegated event registration behavior on regular DOM elements

## Step 3: Unified Special-Host Dispatcher

### Objective

Collapse business-logic duplication across:

- `<svelte:window>`
- `<svelte:document>`
- `<svelte:body>`

This step is intentionally limited to special hosts first.

### Why Only These Hosts First

They already share the same outer shape:

- one loop over attributes
- dispatch by attribute kind
- target-specific event target
- target-specific bind/action/attach subset

They do not require the same complex child ordering contract as regular elements.

### Do

Create a dispatcher with explicit host descriptors in `svelte_codegen_client`.

Suggested types:

```rust
enum SpecialHostKind {
    Window,
    Document,
    Body,
}

struct SpecialHostTarget<'a> {
    kind: SpecialHostKind,
    runtime_target: &'a str,
}
```

Suggested dispatcher shape:

```rust
fn emit_special_host<'a>(
    ctx: &mut Ctx<'a>,
    target: SpecialHostTarget<'_>,
    attrs: &[Attribute],
    stmts: &mut Vec<Statement<'a>>,
)
```

### Allowed Per-Host Variation

Keep the differences explicit and local:

- `window`: bind support only
- `document`: bind plus `{@attach}`
- `body`: `use:` support only

If a directive kind is not legal for a host, the dispatcher should not try to rediscover that. Validation already owns legality checks.

### Bind Table Requirement

For special-host binds, replace ad hoc host-local `match` trees with a small table-oriented lowering helper.

This does not need to live in `analyze`.

Suggested shape:

```rust
enum SpecialBindOp<'a> {
    WindowScroll { axis: &'a str },
    WindowSize { name: &'a str },
    Online,
    Property { prop: &'a str, event: &'a str, target: &'a str },
    ActiveElement,
}
```

### Ownership

Primary files:

- `crates/svelte_codegen_client/src/template/svelte_window.rs`
- `crates/svelte_codegen_client/src/template/svelte_document.rs`
- `crates/svelte_codegen_client/src/template/svelte_body.rs`
- `crates/svelte_codegen_client/src/template/events/emit.rs`
- `crates/svelte_codegen_client/src/template/events/actions.rs`

### Required Changes

1. Add one shared special-host dispatcher module or helper.
2. Make each host file a thin adapter:
   - get attrs
   - provide host target descriptor
   - call dispatcher
3. Move host-specific bind lowering into shared helper logic with a small operation enum or table.
4. Keep public host entry points separate for readability and grepability.

### Acceptance

- the three host files become thin wrappers, not three independent implementations
- new event behavior on a special host lands in one place
- bind additions for special hosts require editing one bind lowering helper, not two or three files
- diagnostics remain owned by validation, not by dispatcher branching

## Step 4: Staged `process_element`

### Objective

Replace the current mixed spread/non-spread top-level branching with an explicit staged pipeline for regular element lowering.

### Important Constraint

This step is codegen-local. Do not create an analyze-side client lowering IR for it.

### Problem To Fix

Today `process_element` mixes several concerns:

- spread vs non-spread attr flow
- class/style special handling
- directive buffering
- child lowering
- update scheduling
- after-update scheduling

This makes regular-element feature ports expensive and error-prone.

### Target Shape

Introduce an internal execution struct in `svelte_codegen_client/src/template/element.rs`.

Suggested shape:

```rust
struct ElementExecutionPlan<'a> {
    init: Vec<Statement<'a>>,
    update: Vec<Statement<'a>>,
    directive_init: Vec<Statement<'a>>,
    after_update: Vec<Statement<'a>>,
    memo_attrs: Vec<MemoAttr<'a>>,
}
```

The important part is the stage order, not the exact type name.

### Required Stages

1. `collect_attrs`
2. `emit_init_attrs`
3. `emit_children`
4. `emit_directive_init`
5. `emit_update`
6. `emit_after_update`

### Required Design Rules

1. Spread becomes one attr collection mode, not the top-level control-flow mode for the whole function.
2. Class and style handling must remain deterministic and explicit.
3. Directive ordering relative to children must stay exactly compatible with the current contract.
4. Child lowering must not need to know whether attrs came from spread or non-spread path.

### Minimum Extraction Plan

1. Extract an internal attr collection helper that fills stage buffers.
2. Keep `process_attr` and `process_attrs_spread` initially, but make them feed the same stage buffers.
3. Remove the second pass that re-discovers skipped directives after spread by routing them through the same collection contract.
4. Only after the shared contract is stable, consider merging more attr helper logic.

### Ownership

Primary files:

- `crates/svelte_codegen_client/src/template/element.rs`
- `crates/svelte_codegen_client/src/template/attributes.rs`
- `crates/svelte_codegen_client/src/template/bind.rs`
- `crates/svelte_codegen_client/src/template/events/actions.rs`
- `crates/svelte_codegen_client/src/template/events/emit.rs`

### Acceptance

- `process_element` reads as stage orchestration rather than a feature switchboard
- spread and non-spread paths feed the same stage buffers
- attr/directive ordering is preserved
- adding a new directive on regular elements has one obvious place to hook in

## Rollout Slices

Apply the plan in these PR-sized slices.

### Slice A: Query foundation

- add `TemplateQueryView`
- make `CodegenView` reuse it
- move touched codegen/transform call sites to the new query surface

Validation:

- `just test-analyzer`
- `just test-compiler`

### Slice B: Event/action helper unification

- unify legacy event emit internals
- extract blocker wrap helper
- keep public behavior unchanged

Validation:

- `just test-case svelte_window_event_legacy`
- `just test-case svelte_document_events`
- `just test-case svelte_body_event_legacy`
- `just test-case use_action_basic`
- `just test-case attach_on_document`
- `just test-case event_mixed_delegation`

### Slice C: Special-host dispatcher

- introduce dispatcher
- thin host wrappers
- move special-host bind lowering to shared helper

Validation:

- `just test-case svelte_window_combined`
- `just test-case svelte_document_combined`
- `just test-case svelte_body_combined`
- `just test-case special_elements_all`

### Slice D: Staged regular-element emission

- stage buffers
- unified attr collection contract
- remove spread-path directive rediscovery

Validation:

- `just test-case spread_attribute`
- `just test-case class_concat`
- `just test-case textarea_child_value_dynamic`
- `just test-case option_expr_child_value`
- `just test-case customizable_select_select_div`
- `just test-case bind_use_deferral`
- `just test-case attach_with_directives`
- `just test-case on_directive`

## Benchmark Discipline

After each slice:

- run `just compare-benchmark`
- compare before/after for regressions

Do not assume a structural cleanup is automatically faster.

## Review Checklist For The Next Agent

Before ending any slice, verify:

1. analyze still owns facts, not client runtime decisions
2. codegen still owns emission phase placement
3. no string-based semantic rediscovery was added
4. touched code uses the new query facade instead of opening new cross-table reads
5. no special-host legality checks were duplicated from validation
6. tests cover both regular elements and special hosts where affected

## Stop Conditions

Stop and report instead of continuing if:

1. a proposed helper starts serving regular elements, special hosts, components, and `svelte:element` with incompatible ordering rules
2. a new analyze data structure needs to encode `$.event`, `$.delegated`, `$.action`, `init`, `after_update`, or similar client-only policy
3. a simplification requires reworking SSR assumptions
4. a slice expands into unrelated feature completion work

## Definition Of Done For This Document

This plan is complete when:

- steps 1-4 are implemented in order
- each slice preserved parity
- query semantics are unified
- special-host duplication is collapsed
- regular-element emission is stage-based
- no client-only lowering plan leaked into `svelte_analyze`
