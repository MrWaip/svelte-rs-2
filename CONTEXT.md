# Context

Rust-компилятор Svelte v5. `.svelte` → client-side JS + scoped CSS.

Доменный язык и инварианты живут в:

- **`ARCHITECTURE.md`** — правила и инварианты по крэйтам (ast → parser → analyze → transform → codegen → compiler).
- **`CODEBASE_MAP.md`** — крэйты, API, типы, pipeline.
- **`CLAUDE.md`** — догмы (smart analyzer / dumb codegen), терминология, глоссарий запрещённых терминов.
- **`ROADMAP.md`** — статус портирования по фичам, ссылки на спеки.
- **`debt.md`** — известный технический долг.
- **`specs/`** — спеки портирования по фичам.
- **`reference/compiler/`** — JS-референс. Используется для понимания **что** портировать, не **как**.

ADR не ведём — архитектурные решения фиксируются в `ARCHITECTURE.md` и `debt.md`.
