# Цель:

Написать минимально жизнеспособный компилятор для svelte компонентов на языке Rust
У меня нет, задачи написать, копию JS версии svelte, и покрыть все возможные фичи, которые имеются

Так как, вышел svelte 5, ориентироваться будем на него. API svelte 4, реализовывать планируется, пока

# Фичи, которые охота реализовать

## Svelte Синтаксис
- [] interpolation (`{ variable }`) with expressions
- [] if / else template (`{#if a} text {else} none {/if}`)
- [] attribute interpolation (`input={variable}`)
- [] render statement
- [] component definition `<MyComponent />`
- [] language context in script tag attributes (`<script lang="ts">`)
- [] style tag

## Статический анализ
- [] TODO

## Компиляция
- [] Скомпилировать простой компонент, в javascript код, аналогичный JS Svelte

# Тех. долг

Тут будет указан, список допущений, недоработок, оставленных на потом, чтобы не усложнять себя задачу.
Их стоит записать, чтобы проработать, если будет желание

- `source: &'static str` - не эффективно, когда сканнеров много нужно
- `self.source.len` - не подойдет для unicode

