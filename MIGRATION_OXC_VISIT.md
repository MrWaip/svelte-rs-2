# Migration Plan: Manual Expression Traversal → OXC Visit

## Problem

~28 functions manually match on `Expression::*` variants with hand-written recursion.
This is fragile (misses new JS/TS syntax), duplicates traversal logic, and prevents
combining multiple analyses into a single walk.

## Principle

All OXC Expression/Statement/Program traversal MUST use OXC visitor infrastructure.
No hand-written recursive match on Expression variants.

Allowed visitors per crate:
- **`svelte_analyze`** — `oxc_ast_visit::Visit` / `VisitMut` only
- **`svelte_transform`** — `Visit` / `VisitMut` / `oxc_traverse::Traverse`
- **`svelte_codegen_client`** — `Visit` / `VisitMut` / `oxc_traverse::Traverse`

---

## Stage 1 — Unified ExpressionAnalyzer (svelte_analyze)

**Goal**: Replace 8 functions in `js_analyze.rs` with a single `impl Visit<'a>` visitor
that collects all expression metadata in one pass.

### Functions to replace

| # | Function | Lines | What it collects |
|---|---|---|---|
| 1 | `collect_references()` | 921-1059 | All IdentifierReference + ReferenceFlags |
| 2 | `collect_statement_references()` | 1061-1091 | References from arrow body statements |
| 3 | `expression_has_call()` | 1093-1117 | `bool` — any CallExpression present |
| 4 | `expression_has_rune()` | 844-861 | `bool` — specific rune call present |
| 5 | `has_deep_store_mutation()` | 865-909 | `bool` — `$store.prop = x` pattern |
| 6 | `member_root_is_store()` | 912-919 | Helper for #5 |
| 7 | `expr_needs_context()` | 680-695 | `bool` — references component context |
| 8 | `is_safe_identifier()` | 699-736 | Member chain → root identifier safety |

### Design

```rust
struct ExpressionAnalyzer<'a> {
    references: SmallVec<[Reference; 2]>,
    has_call: bool,
    has_rune: Option<RuneKind>,
    has_store_mutation: bool,
    needs_context: bool,
    offset: u32,
}

impl<'a> Visit<'a> for ExpressionAnalyzer<'a> {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        // replaces collect_references() identifier arm
        // replaces is_safe_identifier() root check
        // replaces expr_needs_context() identifier check
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        self.has_call = true;
        // replaces expression_has_rune() check
        if let Some(rune) = detect_rune_from_call(call) {
            self.has_rune = Some(rune);
        }
        walk_call_expression(self, call);
    }

    fn visit_assignment_expression(&mut self, assign: &AssignmentExpression<'a>) {
        // replaces has_deep_store_mutation() + member_root_is_store()
        walk_assignment_expression(self, assign);
    }

    fn visit_update_expression(&mut self, update: &UpdateExpression<'a>) {
        // replaces has_deep_store_mutation() UpdateExpression arm
        walk_update_expression(self, update);
    }

    fn visit_new_expression(&mut self, node: &NewExpression<'a>) {
        self.needs_context = true;
        walk_new_expression(self, node);
    }
}
```

### Migration steps

1. Create `ExpressionAnalyzer` struct + `impl Visit`
2. Add `analyze_expression(expr, offset) -> ExpressionMetadata` entry point
3. Replace all call sites of the 8 functions with `analyze_expression()`
4. Delete the 8 old functions
5. Run `just test-all` — must be green

---

## Stage 2 — extract_expression_info + unwrap_rune_arg + detect_rune (svelte_analyze)

**Goal**: Migrate shallow pattern-matching functions to Visit.

| # | Function | File | What it does |
|---|---|---|---|
| 9 | `extract_expression_info()` | js_analyze.rs:793 | Top-level expression classification |
| 10 | `unwrap_rune_arg()` | js_analyze.rs:765 | Unwrap rune call → inner arg |
| 11 | `detect_rune()` | script_info.rs:79 | Identify rune kind from call pattern |

### Approach

Incorporate into `ExpressionAnalyzer` from Stage 1. The visitor's `visit_expression()`
at depth 0 handles classification; deeper visits collect references/flags as before.

---

## Stage 3 — collect_idents_recursive (svelte_analyze)

**Goal**: Replace `collect_idents_recursive()` in `script_info.rs:440` with Visit.

| # | Function | File |
|---|---|---|
| 12 | `collect_idents_recursive()` | script_info.rs:440 |
| 13 | `collect_derived_refs()` | script_info.rs:423 |

### Approach

```rust
struct IdentCollector { refs: Vec<CompactString> }

impl<'a> Visit<'a> for IdentCollector {
    fn visit_identifier_reference(&mut self, ident: &IdentifierReference<'a>) {
        self.refs.push(ident.name.as_str().into());
    }
    // OXC walk handles all recursion — ArrowFunctionExpression, BinaryExpression, etc.
}
```

---

## Stage 4 — CE config extraction (svelte_analyze)

**Goal**: Replace manual ObjectExpression traversal in `ce_config.rs` with Visit.

| # | Function | File |
|---|---|---|
| 14 | `extract_ce_config_from_expr()` | ce_config.rs:6 |
| 15 | `extract_ce_props()` | ce_config.rs:60 |

### Note

These functions walk a known-shape ObjectExpression (not arbitrary expression trees).
Visit adds robustness for nested expressions in `extend` values.

---

## Stage 5 — svelte_transform VisitMut consolidation

**Goal**: Consolidate manual member chain walking into existing `VisitMut` infrastructure.

| # | Function | File | What it does |
|---|---|---|---|
| 16 | `find_expr_root_name()` | rune_refs.rs:241 | Walk member chain to root |
| 17 | `replace_expr_root()` | rune_refs.rs:251 | Replace root of member chain |
| 18 | `replace_expr_root_in_assign_target()` | rune_refs.rs:267 | Same for AssignmentTarget |
| 19 | `replace_expr_root_in_simple_target()` | rune_refs.rs:283 | Same for SimpleAssignmentTarget |
| 20 | `should_proxy()` | rune_refs.rs:214 | Check if value needs $.proxy() |
| 21 | `walk_expr_member_objects()` | lib.rs:592 | Walk member chain objects |

### Approach

`ExprTransformer` already uses `VisitMut`. These helpers should be folded into
the visitor's `visit_expression()` / `visit_member_expression()` methods, or
reimplemented as small `impl Visit` / `impl VisitMut` helpers called from the visitor.

---

## Stage 6 — svelte_codegen_client cleanup

**Goal**: Remove Expression classification from codegen. Use Visit/VisitMut/Traverse
where traversal is needed; move classification to analyze.

| # | Location | What it does | Fix |
|---|---|---|---|
| 22 | `builder.rs:455` `expr_to_assignment_target()` | Expression → AssignmentTarget | Keep — AST conversion, not classification |
| 23 | `builder.rs:778` `make_optional_chain()` | Add optional chain | Keep — AST mutation |
| 24 | `script/mod.rs:412` `should_proxy()` | Duplicate of transform version | Delete, use transform version |
| 25 | `script/mod.rs:436` `extract_assign_member_store_root()` | Member chain walking for store root | Calls `find_expr_root_name()` — will use Visit after Stage 5 |
| 26 | `events.rs:227+333` | Handler form detection | Move classification to analyze → `EventHandlerKind` enum |
| 27 | `script/props.rs:15+29` | Rune detection in init | Move to analyze — already have `DeclarationKind` |
| 28 | `script/state.rs` (~15 occurrences) | $state/$derived detection + transformation | Move classification to analyze; transform logic uses Traverse (already in place) |
| 29 | `script/traverse.rs:78-130` | Rune detection helpers outside Traverse impl | Fold into Traverse visitor methods or use Visit helper |

### Note

#22 and #23 are AST construction/mutation (builder responsibilities), not classification.
These stay in codegen. #24-29 are either boundary violations (classification in codegen)
or manual traversal that should use OXC visitors.

---

## Verification

After each stage:
1. `just test-all` — all tests pass
2. `grep -rn 'Expression::' <modified_files>` — no new manual traversal
3. Count remaining manual `Expression::` matches in crate — must decrease
4. **JS output must not change** — these are pure refactors

## Priority

Stage 1 > Stage 2 > Stage 3 > Stage 5 > Stage 6 > Stage 4

Stage 1 has the highest impact (~300 lines, 8 functions, biggest fragility risk).
Stage 4 is lowest priority (CE config is stable, known-shape data).
