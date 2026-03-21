# Compiler Architecture

## Crate Dependency Graph

```
                         ┌──────────────────┐
                         │  wasm_compiler   │  WASM target (browser)
                         └────────┬─────────┘
                                  │
                         ┌────────▼─────────┐
                         │ svelte_compiler  │  Orchestrator (public API)
                         └──┬────┬────┬──┬──┘
                            │    │    │  │
              ┌─────────────┘    │    │  └──────────────┐
              │                  │    │                  │
   ┌──────────▼──────────┐      │    │    ┌─────────────▼──────────┐
   │   svelte_parser     │      │    │    │  svelte_codegen_client │
   │  (Phase 1: Parse)   │      │    │    │  (Phase 4: Codegen)    │
   └──────────┬──────────┘      │    │    └──────┬────────┬────────┘
              │           ┌─────▼────▼───┐       │        │
              │           │svelte_transform│      │        │
              │           │(Phase 3: Xform)│      │        │
              │           └──────┬────────┘       │        │
              │                  │                │        │
              │           ┌──────▼────────┐       │        │
              │           │svelte_analyze │◄──────┘        │
              │           │(Phase 2: 11p) │                │
              │           └──┬─────────┬──┘                │
              │              │         │                   │
   ┌──────────▼──────────┐   │
   │     svelte_ast      │◄──┘
   │  (Template AST)     │
   └──────────┬──────────┘
              │
   ┌──────────▼──────────┐    ┌────────────────────┐
   │    svelte_span      │    │  svelte_diagnostics │
   │  (Span, NodeId)     │    │  (Error reporting)  │
   └─────────────────────┘    └────────────────────┘
```

## Compilation Pipeline (Data Flow)

```
 .svelte source (String)
       │
       ▼
 ┌─────────────────────────────────────────────────────────────────┐
 │  Phase 1: PARSE                                    svelte_parser│
 │                                                                 │
 │  Source ──► Lexer ──► Parser ──► Component                      │
 │                                  ├── fragment: Fragment         │
 │                                  │    └── nodes: Vec<Node>      │
 │                                  ├── script: Option<Script>     │
 │                                  ├── css: Option<RawBlock>      │
 │                                  └── source: String             │
 │                                                                 │
 │  JS expressions stored as Span (byte offsets), not parsed yet.  │
 └───────────────────────────────┬─────────────────────────────────┘
                                 │  Component (AST)
                                 ▼
 ┌─────────────────────────────────────────────────────────────────┐
 │  Phase 2: ANALYZE (11 passes)                    svelte_analyze │
 │                                                                 │
 │  Pass  1  parse_js ─────────► ParsedExprs<'a> + ExpressionInfo  │
 │  Pass  2  build_scoping ────► ComponentScoping (scope tree)     │
 │  Pass  3  resolve_references► symbol resolution, mutation marks │
 │  Pass  4  store_subscriptions► $store detection                 │
 │  Pass  5  known_values ─────► const literal map                 │
 │  Pass  6  props ────────────► PropsAnalysis ($props() shape)    │
 │  Pass  7  lower ────────────► whitespace trim, Text+Expr groups │
 │  Pass  8  composite_walk ───► 4 visitors in 1 tree walk:        │
 │           │  ReactivityVisitor   → dynamic_nodes                │
 │           │  ElseifVisitor       → alt_is_elseif                │
 │           │  ElementFlagsVisitor → element_flags                │
 │           │  HoistableVisitor    → snippets.hoistable           │
 │  Pass  9  classify ─────────► ContentType per fragment          │
 │  Pass 10  needs_var ────────► needs_var / needs_ref flags       │
 │  Pass 11  validate ─────────► diagnostics (semantic errors)     │
 │                                                                 │
 │  OUTPUT:  AnalysisData (side tables, no AST mutation)           │
 │           ParsedExprs<'a> (OXC Expression ASTs)                 │
 │           Vec<Diagnostic>                                       │
 └───────────────────────────────┬─────────────────────────────────┘
                                 │  AnalysisData + ParsedExprs<'a>
                                 ▼
 ┌─────────────────────────────────────────────────────────────────┐
 │  Phase 3: TRANSFORM                              svelte_transform│
 │                                                                 │
 │  Rewrites OXC expressions in ParsedExprs in-place:             │
 │                                                                 │
 │    $state read    →  $.get(name)                                │
 │    $state write   →  $.set(name, val)                           │
 │    $state update  →  $.update(name)                             │
 │    $derived read  →  $.get(name)                                │
 │    prop source    →  name()         (thunk call)                │
 │    prop non-src   →  $$props.name                               │
 │    each ctx var   →  $.get(name)                                │
 │    snippet param  →  name()         (thunk call)                │
 │    const alias    →  $.get(tmp).prop                            │
 │                                                                 │
 │  Also fills: ConstTagData.tmp_names                             │
 └───────────────────────────────┬─────────────────────────────────┘
                                 │  mutated ParsedExprs<'a>
                                 ▼
 ┌─────────────────────────────────────────────────────────────────┐
 │  Phase 4: CODEGEN                          svelte_codegen_client │
 │                                                                 │
 │  ┌─────────────┐   ┌──────────────────────────────────────┐    │
 │  │ script.rs   │   │ template/                            │    │
 │  │             │   │  ├── mod.rs      (fragment dispatch) │    │
 │  │ Script body │   │  ├── element.rs  (DOM elements)      │    │
 │  │ Imports     │   │  ├── attributes.rs (attrs, bind)     │    │
 │  │ Exports     │   │  ├── if_block.rs   ($.if)            │    │
 │  │ Rune decls  │   │  ├── each_block.rs ($.each)          │    │
 │  └──────┬──────┘   │  ├── expression.rs (expr tags)       │    │
 │         │          │  ├── html.rs     (template strings)  │    │
 │         │          │  ├── traverse.rs (DOM navigation)    │    │
 │         │          │  ├── const_tag.rs                    │    │
 │         │          │  └── svelte_head.rs                  │    │
 │         │          └──────────────────┬───────────────────┘    │
 │         │                             │                        │
 │         └──────────┬──────────────────┘                        │
 │                    ▼                                            │
 │              OXC AST → oxc_codegen → JavaScript String         │
 └───────────────────────────────────────┬─────────────────────────┘
                                         │
                                         ▼
                                   JavaScript output
```

## Key Type Flow

```
                    Component
                   (svelte_ast)
                       │
        ┌──────────────┼──────────────┐
        │              │              │
   Fragment        Script         RawBlock
   (template)    (JS block)      (CSS block)
        │              │              │
   Vec<Node>       Span only     Span only
   with NodeId                        │
        │              │              │
        └──────┬───────┘              │
               ▼                      │
         AnalysisData                 │
        ┌──────────────────┐          │
        │ expressions      │          │
        │ scoping          │          │
        │ dynamic_nodes    │          │
        │ element_flags    │          │  ◄── Future: Tier 6
        │ fragments        │          │      CSS scoping
        │ props            │          │
        │ snippets         │          │
        │ const_tags       │          │
        │ store_subs       │          │
        │ exports          │          │
        └──────────────────┘          │
               +                      │
         ParsedExprs<'a>              │
        (OXC lifetimes                │
         contained here)              │
               │                      │
               ▼                      ▼
        JavaScript output      ┌─────────────┐
                               │ CSS output   │
                               │ (future)     │
                               └─────────────┘
```

## Future Layers (from ROADMAP)

```
 Current state          Planned additions
─────────────────────────────────────────────────────────────
 ┌──────────┐
 │ Parser   │ ✅ Done    + {#await}, {@debug} (Tier 2)
 └──────────┘
 ┌──────────┐
 │ Analyzer │ ✅ Core    + expression memoization (Tier 1d)
 │ (11 pass)│            + a11y validation (Tier 7)
 └──────────┘            + module validation (Tier 1b)
 ┌──────────┐
 │Transform │ ✅ Core    + $inspect, $host runes (Tier 1)
 └──────────┘            + non-delegatable events (Tier 1c)
 ┌──────────┐
 │ Codegen  │ ✅ Client  + <svelte:window/document/body> (Tier 5)
 │ (client) │            + <svelte:boundary> (Tier 5)
 └──────────┘

 ┌──────────────────────────────────────────────────┐
 │  NEW SUBSYSTEMS (not yet started)                │
 │                                                  │
 │  ┌────────────────┐   CSS scoping, :global(),    │
 │  │ CSS Processor  │   class directives,          │
 │  │ (Tier 6)       │   unused CSS pruning         │
 │  └────────────────┘                              │
 │                                                  │
 │  ┌────────────────┐   Source maps for            │
 │  │ Source Maps    │   JS + CSS output            │
 │  │ (Tier 8)       │                              │
 │  └────────────────┘                              │
 │                                                  │
 │  ┌────────────────┐   Dev-time warnings,         │
 │  │ Dev Mode       │   HMR support,               │
 │  │ (Tier 8)       │   runtime checks             │
 │  └────────────────┘                              │
 │                                                  │
 │  ┌────────────────┐   Svelte 4 compat layer,     │
 │  │ Legacy Support │   isolated for easy removal   │
 │  │ (Tier 10)      │   (LEGACY(svelte4): tag)     │
 │  └────────────────┘                              │
 └──────────────────────────────────────────────────┘
```

## Key Architectural Invariants

| Rule | Why |
|------|-----|
| OXC as direct dependency | OXC types flow through `ParsedExprs<'a>` from parser to codegen; `svelte_parser` provides shared domain types |
| AST is immutable after parsing | All analysis → side tables (`AnalysisData`, keyed by `NodeId`) |
| Spans for JS in AST, re-parse in codegen | No JS subtree copying between phases |
| `FxHashMap` everywhere | Faster hashing for integer keys (NodeId, SymbolId) |
| Single composite visitor in analysis | Multiple independent passes → one tree walk (pass 8) |
| Direct recursion in codegen | No visitor pattern — explicit control flow for codegen |
| `pub(crate)` by default | Narrow interfaces; `pub` only for entry points |
