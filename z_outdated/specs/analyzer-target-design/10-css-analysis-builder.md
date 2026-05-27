# `CssAnalysisBuilder` consolidation

## Parent

`specs/analyzer-target-design.md`

## What to build

Один `CssAnalysisBuilder` поглощает текущие три CSS-pass-а: `passes/css_analyze`, `passes/css_prune_index`, `passes/css_prune`. Внутри builder-а — три внутренних этапа (parse → index → prune), но снаружи он один builder в Phase 5 с одним output-ом.

Output — `CssAnalysis`, top-level поле `AnalysisData::css`.

CSS-семантика и output-формат не меняются. `compiler_tests/cases2/` snapshot-ы для CSS должны оставаться зелёными bit-for-bit.

Builder остаётся `Send` и pure (input → output, без shared mutable state помимо своего выхода).

Decision basis: §16 + Out-of-scope §9.

## Acceptance criteria

- [ ] `CssAnalysisBuilder` существует и регистрируется в Phase 5
- [ ] `CssAnalysis` — top-level поле `AnalysisData::css`
- [ ] Три внутренних этапа (parse → index → prune) видны в реализации builder-а
- [ ] `passes/css_analyze.rs` удалён
- [ ] `passes/css_prune_index.rs` удалён
- [ ] `passes/css_prune.rs` удалён
- [ ] CSS-семантика не меняется — все CSS snapshot-ы в `compiler_tests/cases2/` зелёные bit-for-bit
- [ ] Builder pure (input → output)
- [ ] `just test-compiler` зелёный
- [ ] `just test-diagnostics` зелёный
- [ ] `just clippy-strict` зелёный
- [ ] Запись в `debt.md` снята / обновлена

## Blocked by

None — can start immediately (independent of cluster migrations).
