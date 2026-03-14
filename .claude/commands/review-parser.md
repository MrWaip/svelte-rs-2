# Review: Parser Crate

You are reviewing the **parser crate** (`crates/svelte_parser/`) of a Svelte compiler written in Rust, built on top of oxc.

## Preparation

Read `CODEBASE_MAP.md` and `CLAUDE.md`. Then read all `.rs` files in `crates/svelte_parser/src/`. The author is a senior Rust developer — skip trivial suggestions (clippy lints, add docs everywhere).

Use up to 3 parallel Explore agents to read the source files and perform the review.

## Focus Areas (in priority order)

### 1. Correctness of Parsing

- **Svelte syntax coverage**: Does the parser handle all current Svelte syntax? Look for TODO/FIXME or hardcoded assumptions that skip constructs (`{#await}`, `{@html}`, `{@const}`, shorthand attributes `{name}`, spread `{...props}`, `bind:`, `on:`, `transition:`, `animate:`, `use:`, `class:`, `style:`, etc.).
- **Edge cases in template parsing**: Self-closing tags, void elements, components vs. HTML elements (casing), `<svelte:self>`, `<svelte:component>`, `<svelte:element>`, `<svelte:window>`, `<svelte:body>`, `<svelte:head>`, `<svelte:fragment>`, `<svelte:options>`.
- **Expression parsing inside templates**: `{expression}` in text, attributes, directives — correct delegation to oxc? Edge cases (nested braces, template literals with `${}` inside `{}`)?
- **Script block handling**: How does the parser hand off `<script>` / `<script context="module">` to oxc? Is offset/span mapping correct?

### 2. Error Handling & Recovery

- **Malformed input**: Unclosed tags, mismatched tags, unterminated expressions, unclosed strings inside expressions.
- **Error quality**: Accurate spans? Specific messages (not just "unexpected token")?
- **Panic surface**: Every `unwrap()`, `expect()`, `unreachable!()`, and array index `[i]` without bounds check — which can be triggered by user input?

### 3. Span & Position Accuracy

- **Span correctness**: Are AST node spans tight (start at first meaningful character, end right after last)?
- **Offset mapping**: When delegating JS parsing to oxc, correct translation of oxc spans back to absolute file positions?
- **Off-by-one**: Flag any span arithmetic that looks off-by-one.

### 4. Integration with oxc

- **Parser reuse**: Is oxc's parser used for all JS/TS expression parsing, or are parts reimplemented?
- **Allocator usage**: Is oxc's arena allocator used consistently?

### 5. Structure & Maintainability

- **State machine clarity**: Is parsing state explicit and understandable?
- **Separation of concerns**: Tokenization/lexing separate from AST construction?

## Output Format

```
## Summary
[2-3 sentences: overall parser quality, biggest risk area]

## Critical Issues
[Bugs, panics on valid input, incorrect AST output]
### [Title]
- **File**: path:line
- **Problem**: ...
- **Suggestion**: ...

## Important Improvements
[Error handling gaps, missing syntax, structural issues]

## Minor Notes
[Max 3-5 items]

## Questions for the Author
[Intentional limitations? Planned improvements?]

## What's Done Well
[2-3 strengths worth preserving]
```

## Rules

- Max ~15 findings total. Prioritize ruthlessly.
- Always include file paths and line numbers.
- Check ROADMAP.md before flagging missing syntax — it might be explicitly planned.
- Do not suggest adding tests for every function. Only flag missing coverage for tricky parsing logic.
