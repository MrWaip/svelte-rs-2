# PRD: AST (корневой)

label: ast
topics: ast, node, tree shape, fragment, element/RegularElement/Component, *Tag, EachBlock, BindDirective, NodeId/OxcNodeId

Корневой PRD для слоя AST: крэйты `svelte_ast`, `svelte_css`, плюс `oxc_ast` для JS-выражений/стейтментов.
Owns формы дерева, производимые парсингом. Без семантики, скоупов, логики мутаций.

## Назначение

Хранит формы дерева, которые производит парсинг. Не API-референс — карта типов в `context.md`.

## Что здесь живёт

- `svelte_ast::Component` — корень: `AstStore`, опциональные `instance_script`/`module_script` (только метаданные `Script`), опциональный `css: RawBlock`, `options: SvelteOptions`, полный `source: String`.
- `AstStore` — плоская арена. Все template-узлы и фрагменты в векторах по ключу `NodeId(u32)` / `FragmentId(u32)`.
- Template-enum `Node` с вариантами `Element`, `IfBlock`, `EachBlock`, `Component`, … Каждый вариант несёт `pub id: NodeId` и `pub span: Span`.
- `Fragment` — список детей + `FragmentRole` + опциональный owner `NodeId`.
- `ExprRef` / `StmtRef` (`expr_ref.rs`) — указатель из Svelte AST на JS-узел внутри `oxc::Program`. Держит `Span` плюс late-bound `Cell<OxcNodeId>`.
- `svelte_css::ast` — CSS-дерево. `CompactString` для идентификаторов и `Option<String>` override-поля (`Declaration.value_override`, `AtRule.prelude_override`), чтобы CSS-pruning/rewriting подменял текст на месте.

## Архитектурные инварианты

1. **Без лайфтаймов на `svelte_ast`-типах.** AST должен быть `'static`-friendly. Template-узлы не заимствуют из исходника.
2. **Без строк в template AST.** Идентификаторы, имена атрибутов и т.д. — через `Span`, резолвятся `Component::source_text(span)`.
3. **Плоское хранилище.** Форма дерева — `NodeId`/`FragmentId`-индексы в `AstStore`, не `Box`/`Rc`. Дети — `Vec<NodeId>` внутри `Fragment`.
4. **У каждого узла `NodeId` и `Span`.** Энфорсится макросом enum'а `Node`.
5. **JS-мост через `OxcNodeId`.** Svelte AST никогда не держит `oxc::Expression`/`Statement` напрямую — держит `ExprRef`/`StmtRef`. Сами `oxc_ast`-узлы живут внутри `oxc::Program`, которым владеет анализ.
6. **`OxcNodeId` биндится поздно.** `ExprRef::new` стартует с `OxcNodeId::DUMMY`; реальные id присваивает `ComponentSemanticsBuilder` при семантик-build, обходя соответствующую `oxc::Program` и матча по span. `ExprRef::id()` паникует, если прочитан до биндинга.
7. **Две oxc-программы на компонент.** Одна для `<script module>`, одна для `<script>`. Обе строит `svelte_parser::parse_js` в переданный `oxc_allocator::Allocator`. NodeId'ы двух программ держатся непересекающимися через offset `next_node_id` в `ComponentSemanticsBuilder`.
8. **CSS AST может держать owned-строки.** Единственный AST, мутируемый post-parse (CSS-pruning, scoping). Override-поля держат переписанный текст; оригинальные span'ы остаются для диагностик.

## Связь с другими документами

- `context.md` §«Слои крэйтов» — место AST в пайплайне; §«AST» в `Flagged ambiguities` — Svelte AST vs OXC AST.
- `parser.md` — единственный производитель этого дерева.
- `component-semantics.md` — кто биндит `OxcNodeId` и строит scope-граф.
- `bindings-and-references.md` — `ExprRef`/`StmtRef`, `NodeId` vs `OxcNodeId`.
