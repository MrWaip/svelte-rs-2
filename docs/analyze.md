# PRD: Анализ (корневой, рамка слоя)

label: analyze
topics: analyze, smart analyzer, passes, dynamism, build order, analyze phases, semantics frame, semantic diagnostics

Корневой PRD для слоя анализа (`crates/svelte_analyze`). Описывает рамку слоя и догму **smart analyzer**; per-subsystem семантика — в дочерних PRD `component-semantics`, `reactivity-semantics`, `expression-semantics`, `attribute-semantics`, `block-semantics`.

## Назначение

Пред-вычислить каждое решение, чтобы трансформ + кодген остались линейными и тупыми.

Трансформ/кодген НЕ ДОЛЖНЫ: собирать смысл из raw-флагов, разбросанных по side-таблицам; пере-walk'ать AST за фактами; комбинировать raw-индексы, чтобы вывести вопрос. Анализ должен им готовые ответы: enum'ы и структуры, прямо называющие решение. Если трансформу/кодгену нужно `if a && !b && c.kind == X`, чтобы понять что эмитить, — это compound-условие принадлежит одному именованному полю на структуре анализа.

## Target shape

Анализ делится на два класса:

- **3.A семантические подсистемы** — `ComponentSemantics`, `ReactivitySemantics`, `ExpressionSemantics`, `AttributeSemantics`, `BlockSemantics`. Каждая поглощает несколько raw-индексов внутри и выдаёт query-API на один конкретный вопрос за вызов. Не течёт raw-флагами наружу.
- **3.B аналитические side-таблицы** — `ScriptAnalysis`, `ElementAnalysis`, `TemplateAnalysis`, `BlockAnalysis`, `OutputPlanData`, `DynamismData`, `PickledAwaits`. Плоские таблицы на `AnalysisData`, ещё не свёрнутые в 3.A-подсистему. Трекаются в `debt.md`.

Новые факты идут в 3.A-подсистему (или мотивируют новую), **никогда** не в 3.B.

Build order (`crates/svelte_analyze/src/lib.rs` + `passes/mod.rs`):

```
ComponentSemantics
  └── ReactivitySemantics (фаза 1, script-only)
        ├── ExpressionSemantics ──┬── AttributeSemantics
        │                         └── BlockSemantics
        └── ReactivitySemantics (фаза 2, template walk)
              └── BlockSemantics
                    └── Validation (3.C)
```

`BlockSemantics::build` читает `ExpressionSemantics` для per-expression фактов (collection / callee / promise legacy-wrap, async-kind, blockers) вместо параллельной классификации.

## Архитектурные инварианты

1. **Read-only над AST** после build.
2. **Единственный источник истины для метаданных.** Никаких shadow-флагов реактивности/семантики на чужих таблицах.
3. **Analysis vs transformation split.** Анализ не мутирует AST; мутации — трансформ.
4. **Новые факты — только в 3.A.** 3.B заморожена под миграцию.

## BindingPattern handling (cross-cutting)

Для обхода destructuring-паттернов (`let { a, b: { c } } = …`, function-params, each-item, snippet-params) **не уплощать паттерн в данные анализа**. Маппинг формы `BindingPattern` в bespoke side-структуры дублирует AST и молча теряет детали (defaults, rest, nested rest, computed keys).

- Обход паттернов — `walk_bindings`; если не подходит — focused-ручная рекурсия по `oxc_ast`, всё равно без уплощения.
- Анализ публикует per-leaf факты по ключу `OxcNodeId` / `ReferenceId` / `SymbolId` leaf-идентификатора (или pattern-узла) — никогда reshaped pattern-дерево.
- Кодген / трансформ обходит оригинальный `BindingPattern` и запрашивает анализ per leaf.
- Анти-паттерн: `enum AnalyzedPattern { Identifier(...), Object { props: Vec<...> }, … }`, зеркалящий форму `BindingPattern`.

Ментальная модель + OXC-таксономия + дисциплина `walk_bindings` — `bindings-and-references.md`.

## Валидация (3.C)

Модуль `svelte_analyze::validate`. Обходит AST и эмитит user-facing диагностики (warnings + errors) из уже построенных 3.A-подсистем. Запускается после всех подсистем, читает их, никогда не мутирует.

- Read-only над AST и над каждой подсистемой анализа.
- Единственный производитель `Diagnostic` для user-facing парити в слое; трансформ/кодген diagnostic-free.
- Не пере-выводить факты, которыми подсистема уже владеет — запросить и доложить.

## Связь с другими документами

- `context.md` §«Догмы», §«Кросс-каттинг» (диагностики).
- `component-semantics.md`, `reactivity-semantics.md`, `expression-semantics.md`, `attribute-semantics.md`, `block-semantics.md` — 3.A-подсистемы.
- `parser.md` — граница парсер ↔ анализ.
- `transform.md`, `codegen.md` — потребители анализа.
- `bindings-and-references.md` — `walk_bindings`, идентификаторы.
