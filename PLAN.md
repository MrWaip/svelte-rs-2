# Plan: Reduce Crate Verbosity

## 1. Node enum: macro for field accessors + predicates + as_* methods
**File:** `svelte_ast/src/lib.rs` lines 92-162

Currently 12-arm matches for `node_id()`, `span()`, plus 9 individual `is_*()` methods.
Add `as_*()` accessors that return `Option<&T>`.

**Action:** Create `macro_rules! impl_node_enum` that generates:
- `node_id(&self) -> NodeId`
- `span(&self) -> Span`
- `is_<variant>(&self) -> bool` for each variant
- `as_<variant>(&self) -> Option<&T>` for each variant

## 2. Derive Clone on Attribute types, eliminate manual clone_without_fragment
**File:** `svelte_ast/src/lib.rs` lines 196-264

`clone_without_fragment()` manually clones every attribute variant (60+ lines).
All attribute types contain only `String`, `Span`, `bool`, `Vec<ConcatPart>` — all Cloneable.

**Action:** Add `#[derive(Clone)]` to all attribute structs + enums. Simplify `clone_without_fragment` to:
```rust
pub fn clone_without_fragment(&self) -> Element {
    Element {
        id: self.id,
        span: self.span,
        name: self.name.clone(),
        self_closing: self.self_closing,
        attributes: self.attributes.clone(),
        fragment: Fragment::empty(),
    }
}
```

## 3. FragmentItem::node_id() helper
**File:** `svelte_analyze/src/data.rs`

Add method to extract `NodeId` from any variant (except TextConcat which has no single id).

## 4. LoweredFragment helpers for first-item extraction
**File:** `svelte_analyze/src/data.rs`

Used in 4+ places: `match ctx.lowered_fragment(&key).items[0] { FragmentItem::Element(id) => id, _ => unreachable!() }`

**Action:** Add typed extractors:
```rust
impl LoweredFragment {
    pub fn first_element_id(&self) -> NodeId { ... }
    pub fn first_if_block_id(&self) -> NodeId { ... }
    pub fn first_each_block_id(&self) -> NodeId { ... }
}
```

## 5. NodeIndex + Ctx getter macro
**File:** `svelte_codegen_client/src/context.rs` lines 158-191

7 identical getter methods that only differ in field name and panic message.

**Action:** `macro_rules! node_getter` to generate all 7.

## 6. Traverse: deduplicate block item arms
**File:** `svelte_codegen_client/src/template/traverse.rs` lines 90-138

5 arms (ComponentNode, IfBlock, EachBlock, RenderTag, HtmlTag, KeyBlock) all do:
```rust
let node_name = ctx.gen_ident("node");
init.push(ctx.b.var_stmt(&node_name, node_expr));
sibling_offset = 1;
gen_xxx(ctx, *id, ...);
prev_ident = Some(node_name);
```

**Action:** Extract a closure or helper to reduce duplication.

## 7. Composite visitor: inner macro to avoid method repetition
**File:** `svelte_analyze/src/walker.rs` lines 110-158

Each composite visitor impl repeats all 13 method signatures.

**Action:** Use an inner macro to list method signatures once.
