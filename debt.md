# Tech debt

Every unfixed problem spotted mid-work goes here. New section per item. Describe what is wrong and where.

## Parser performs semantic analysis it does not own

Parser must produce AST + lexical/syntactic diagnostics only. Semantic checks (which attributes a tag accepts, which value shapes are valid for a given attribute kind, whether `on*` attribute values must be expressions, etc.) belong to `svelte_analyze`.

Concrete known offenders in `crates/svelte_parser/src/attr_convert.rs`:

- `AttributeInvalidName` for digit/dash-leading names on non-component tags. This is a name-validity rule, not a tokenization rule. Fired in parser regardless of element semantics. Reference performs analogous check in `2-analyze/visitors/shared/element.js`.
- (now fixed) `AttributeInvalidEventHandler` for `on*` attrs with non-expression value used to fire in parser. Removed from `attr_convert.rs`, moved to `template_validation.rs::check_event_handler_value` called per-element in analyze. Pattern: same problem may exist for other "attribute X requires shape Y" rules.

General rule example user surfaced: parser checking presence/absence of an attribute on a tag (e.g. "tag X requires attribute Y", "tag X disallows attribute Z") is semantic. Parser must not emit such diagnostics. They live in analyze visitors per element kind.

Direction: audit `crates/svelte_parser/src/attr_convert.rs` and any other `Parser::*` site that pushes a `Diagnostic` based on element-semantic context (tag name, attribute name semantics, value-shape vs intended use). Move each rule into the matching analyze visitor in `crates/svelte_analyze/src/passes/template_validation.rs`. Parser keeps only: token-level errors, malformed expressions, unmatched tags, syntactic duplicates that cannot be expressed post-parse.

## `expr_deps` builder leaks analysis work into codegen

`AnalysisData::expr_deps(site)` (`crates/svelte_analyze/src/types/data/analysis.rs:653`) is named like a lookup but is a **builder**: every call does `self.expressions.get(id)`, then `collect_blockers(info.ref_symbols())` (iterates ref_symbols + per-symbol HashMap probe + allocates `SmallVec<[u32;2]>` for blockers), then computes `needs_memo` boolean inline.

Codegen call sites (`crates/svelte_codegen_client/src/codegen/fragment/process_children.rs::emit_concat_set`, `crates/svelte_codegen_client/src/codegen/fragment/mod.rs::emit_concat_node_in_fragment`, `crates/svelte_codegen_client/src/codegen/data_structures/memo.rs::push_node_deps`, plus other sites under `codegen/`) invoke `expr_deps()` per template expression — sometimes multiple times for the same id within one helper. This violates the project's `smart analyzer / dumb codegen` dogma: codegen takes the memoization decision (`needs_memo`) at emission time using analyze internals, instead of reading a pre-computed scalar.

`ExpressionInfo` itself is already marked `#[deprecated]` with the direction "use reactivity_semantics for per-reference decisions, or add the needed higher-level answer to the owning semantic cluster". `expr_deps` is the live legacy surface that keeps codegen tied to that deprecated bag.

Direction: pre-compute per-expression memo facts during analyze (e.g. extend `template_side_tables.rs` or add `expression_memo_facts` cluster) producing `MemoFacts { needs_memo: bool, has_await: bool, blockers: SmallVec<[u32; 2]> }` keyed by `NodeId` (and similar for `attr_expressions`). Codegen reads via `data.expr_memo_facts(id)` — pure O(1) lookup, no allocation, no reanalysis. Retire `expr_deps` once all callers migrate.

## `gen_unique_name` allocates 3x per call

`ComponentTransformer::gen_unique_name` (`crates/svelte_transform/src/transformer/state.rs:1239`) builds temp `String` via `format!`, caller proxies again through `builder.alloc_str` into arena `&str`. Plus global `IdentGen` (`svelte_analyze::IdentGen`) lives in parallel solving same task for other sites. One unique ident costs two heap allocs + arena copy instead of one arena alloc. Two independent counters diverge in semantics (per-transformer vs global).

Direction: collapse into a single arena-aware ident generator that writes directly into the bump arena and returns `&'a str`; merge per-transformer counter with `IdentGen` so naming stays consistent across passes.

## Legacy each-item member mutation does not upgrade collection or invalidate inner signals

Surfaced while extending `smoke_legacy_contextual_mutations_all` (ignored). When a legacy `let items = [...]` is iterated via `{#each items as item}` and an item member is mutated through the template (`{item.x = 1}`, `{item.x++}`, etc.), reference does two things our compiler skips:

1. Upgrades the collection declarator from `let items = [...]` to `let items = $.mutable_source([...])`, so the array itself is reactive.
2. Wraps each member-mutation in the template effect with `$.invalidate_inner_signals(() => $.get(items))` to propagate the indirect mutation to dependents reading the collection.

Owning area: `crates/svelte_analyze/src/reactivity_semantics/builder_v2/legacy.rs` for the `let` → mutable_source upgrade decision when each-item member mutation is observed; `crates/svelte_codegen_client/src/codegen/expr.rs::maybe_wrap_legacy_coarse_expr` (or a new helper) for emitting the `invalidate_inner_signals` tail when `legacy_indirect_bindings` carry the collection reference.

## Inline `{await call()}` with global callee not memoized into async_values

Surfaced by `inline_await_global_callee` (ignored). With `experimental.async = true`, template `{await fetch()}` (or any `{await callee(args)}` where callee resolves to a non-blocker symbol) should extract the callee/argument thunk into `Memoizer.async_values` and replace the await in the body with the corresponding `$N` parameter, mirroring reference output:

```js
$.template_effect(($0) => $.set_text(text, ` ${$0 ?? ""}`), void 0, [fetch]);
```

Our compiler keeps the raw `await` inline:

```js
$.template_effect(() => $.set_text(text, ` ${await fetch() ?? ""}`));
```

The `inline_await_basic` case already passes because there the awaited expression depends on a TLA blocker symbol (`response`), which routes through `Memoizer.async_values` via `expr_deps(...).blockers`. With a non-blocker callee, the memoization branch is skipped and the await leaks into the body.

Owning area: `crates/svelte_codegen_client/src/codegen/data_structures/memo.rs::add_memoized_expr` (decides async vs sync push), and `crates/svelte_codegen_client/src/codegen/fragment/process_children.rs::emit_concat_set` / `emit_text_set` (build the `set_text` template-effect body — they take the raw expression via `cg.take_node_expr` without consulting the memoizer for ExpressionTag with `has_await`). The fix likely needs `emit_concat_set` to route through Memoizer when any `ConcatPart::Expr` has `has_await`, hoisting the await into `async_values` and replacing it with the corresponding `$N` param expression.

## `experimental_async` diagnostic split across two mechanisms

Reference compiler centralizes await suspend logic in one `AwaitExpression` visitor driven by `state.expression` + `function_depth`. Our impl scatters the same diagnostic across two unrelated mechanisms:

1. Per-site checks in `crates/svelte_analyze/src/passes/template_validation.rs` for `use:` / `transition:` / `animate:` / `bind:` / `{@attach}` / `{@const}` / `ExpressionTag` / `ExpressionAttribute`. Each site gates on `attr_expression(id).has_await()` then calls `first_await_span` / `first_await_span_in_stmt` and emits via `emit_directive_await_diagnostic` or `emit_template_await_experimental`.
2. Script walker `crates/svelte_analyze/src/validate/experimental_async.rs` for `$derived` / `$derived.by` / TLA — separate state machine tracking `function_depth` + `expression_active`.

Both produce the same `DiagnosticKind::ExperimentalAsync` (and dual-branch `IllegalAwaitExpression`). Logic to find first await duplicated (`FirstAwaitVisitor` template-side, `ExperimentalAsyncValidator` script-side). Adding the next site (`IfBlock` test, `EachBlock` / `KeyBlock` / `AwaitBlock` expression, `HtmlTag`, `RenderTag`, `SvelteElement` tag, `ConcatenationAttribute` parts, `StyleDirective` values) means bolting on yet another per-site override.

Direction: collapse into single `AwaitExpression`-driven walker over template + script with proper `expression_active` framing matching reference, so adding new sites becomes data, not code. Per-site `has_await` gates remain available where dual-branch (`IllegalAwaitExpression` vs `ExperimentalAsync`) precedence is required.

## `$state.eager(0)` declarator path panics in oxc_traverse

Surfaced by `smoke_runes_state_eager_panic` (ignored). For `let eager = $state.eager(0)`, our `rewrite_shared_call` replaces the call expression with a freshly-built `$.eager(thunk)` node that has no `oxc::NodeId` set. The oxc traverser then panics on `Option::unwrap()` at `oxc_traverse/walk.rs:2452` when descending into the replacement. Reference's behavior is also unusual (declaration silently dropped, identifier left dangling) so test parity is degenerate, but our panic blocks compile entirely. Owning area: `crates/svelte_transform/src/transformer/rewrites.rs::rewrite_shared_call` `$state.eager` branch — needs to either reuse the existing call's NodeId or bypass traversal of the replacement node.


## `unreachable\!` consistency in `convert_svelte_element`

`crates/svelte_parser/src/svelte_elements.rs::convert_svelte_element` ловит non-Element ветку `match store.take(id)` через `Diagnostic::error(InternalError(...))` + `store.replace`. Ломает паттерн файла — 9 других converter'ов в этом же файле (`convert_svelte_body`, `convert_svelte_document`, `convert_svelte_window`, `convert_svelte_options`, `convert_svelte_head`, `convert_slot_element_legacy`, `convert_svelte_fragment_legacy`, `convert_svelte_boundary`, итд) используют `unreachable\!()` после идентичной `as_element` guard'а.

Inconsistent: добавлен `&mut diagnostics` параметр + диагностика для ветки которая никогда не fires (invariant guaranteed by `is_some_and(|el| el.name == SVELTE_ELEMENT)` check сверху).

Варианты:
- **Откатить** обратно на `unreachable\!()` — restore consistency, паника никогда не сработает.
- **Глобальный sweep** — изменить `AstStore::take` API на `take_element(id) -> Option<Element>` (типизированный extractor), убрать все 10 unreachable. Большой scope, трогает все converter'ы.

## Post-pass AST rewriting in parser

`crates/svelte_parser/src/svelte_elements.rs` содержит набор `convert_svelte_*` пост-пассов: `convert_svelte_fragment_legacy`, `convert_svelte_element`, `convert_svelte_boundary`, `convert_svelte_body`, `convert_svelte_document`, `convert_svelte_window`, `convert_svelte_options`, `convert_svelte_head`, `convert_slot_element_legacy`. Все вызываются из `Parser::parse` после построения базового AST (`crates/svelte_parser/src/lib.rs:543-545`).

Логика каждого: построить `Node::Element { name: "svelte:X" }` сначала, потом walk по дереву, найти такие узлы по строковому сравнению `el.name == SVELTE_X`, через `store.take` / `store.replace` переписать на типизированный `Node::SvelteX`. Двойная работа: парсер уже знает имя тега в момент `StartTag` (`lib.rs:267-308`), но строит generic `Element` и роняет тип, который сразу же восстанавливается walk'ом. Промежуточный AST некорректен: краткое окно когда `Node::Element { name: "svelte:component" }` существует — все downstream consumer'ы должны знать про эту фазу.

Симптомы:
- Reparsing-стиль heuristic в parser-крате: посимвольная классификация по строковым константам.
- N walk'ов по всему дереву на N специальных тегов вместо одного match при создании.
- ID-rewriting через `store.take`/`store.replace` усложняет invariants стора.
- Для нового `<svelte:component>` повторение паттерна предложено и отвергнуто пользователем — корректный подход inline в `StartTag`-обработчике.

Direction: вынести name-detection в `StartTag`/`SelfClosing`-обработчиках `Parser::parse`. Для `name == SVELTE_X` строить сразу `Node::SvelteX` с правильной формой fragment (role `SvelteElementBody` итд) и нужными полями. Удалить все `convert_svelte_*` пост-пассы. Существующие `as_element() + name == SVELTE_X` фильтры вне parser'а (analyze/codegen/walk_js) перестанут видеть `Element { name: "svelte:X" }` — упростится.

Scope большой, трогает 9 пост-пассов + точку конструкции в `lib.rs`. Оставить как known debt; новые специальные теги (включая `SvelteComponentLegacy`) делать сразу inline без копирования паттерна.

## Запретить паники итд

Разрешить только сейфовые операции и запретить паники, Unwrap, expect