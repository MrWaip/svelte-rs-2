# Цель:

Написать минимально жизнеспособный компилятор для svelte компонентов на языке Rust
У меня нет, задачи написать, копию JS версии svelte, и покрыть все возможные фичи, которые имеются

Так как, вышел svelte 5, ориентироваться будем на него. API svelte 4, реализовывать не планируется, пока

# Фичи, которые охота реализовать

## Svelte Синтаксис

- [x] interpolation (`{ variable }`) with expressions
- [x] attribute interpolation (`input={variable}`)
- [x] render statement
- [] component definition `<MyComponent />`
- [] language context in script tag attributes (`<script lang="ts">`)
- [x] if / else template (`{#if a} text {else} none {/if}`)
- [] style tag

## Статический анализ

- [] TODO

## Компиляция

- [x] Скомпилировать простой компонент, в javascript код, аналогичный JS Svelte
