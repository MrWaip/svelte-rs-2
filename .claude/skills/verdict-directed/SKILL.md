---
name: verdict-directed
description: 🔪 SDT · verdict-directed review of backend codegen/transform. Use when auditing codegen/transform for smart-analyzer / dumb-codegen violations, or when porting a transform/codegen visitor from the Original.
allowed-tools: Bash, Read, Grep, Glob, mcp__narsil-mcp__*, Agent
---

# verdict-directed

Review a backend crate against the **verdict-directed** principle (`docs/context.md` §«Принципы»): transform and codegen are a **syntax-directed translation** over analysis verdicts. This skill finds where the backend instead recovers a domain fact on its own. It is a review, not a linter: a finding is closed by re-homing the fact to analysis, never by silencing.

## Required reading (step 0)

Re-read every invocation; do not assume in-session.

- `docs/context.md` §«Принципы» — the keystone and its three violation shapes.
- `docs/context.md` §«Language» — the three border anti-patterns, the finding vocabulary.

## The razor

Two kinds of complexity live in a backend. Only one is a violation.

- **Instruction selection (legit).** Choosing the target print-form of an already-decided verdict: escaping, whitespace, `clsx`-wrap, css-hash merge, boolean-attribute table, dev vs prod, client vs server, which `$.` call. Stays in the backend.
- **Domain-fact recovery (violation).** Deciding *what a unit means*: is it reactive, static, which role, how many references. Must be born in analysis and consumed as a verdict.

Litmus, per branch: **swap the target runtime / printing — does this branch still need to exist?** Yes → instruction selection, legit. No, because it changes what the unit *means* → violation.

## Scope

Four backend crates only: `svelte_codegen_client`, `svelte_codegen_server`, `svelte_transform_client`, `svelte_transform_server`. Analysis, parser, ast are out of scope — a fact missing there is the *fix*, not a finding here.

Porting an Original transform/codegen visitor is a first-class trigger: run the pass on the new backend code before committing — the port is where a violation enters.

## Finder — a reading pass, not a grep

The tell is not a syntax, it is **provenance**: trace the deciding input of every backend branch to its origin. Legit when the input is a *verdict handle* — the result of one analysis query. Violation when the input is a *value read out of source text or AST structure*. The syntactic form is irrelevant, so an example never bounds it — a name-match hides just as well inside a helper, a `name.starts_with(…)`, a `HashSet` lookup, or an enum discriminant.

Ripgrep only *seeds* the pass — it catches the obvious forms, never all of them. Seed shapes:

- `.name` / `element.name` / `attr.name()` vs a string literal, or `matches!(…, "x" | "y")` on a name.
- `self.component.source` / `.source` read to inspect a value, not to slice a known span.
- `walk` / `Visit` / `.iter().any(` / `.iter().find(` over children to derive a boolean.
- one emit decision reading several analysis side-tables joined with `&&`.

A seed hit is a candidate, not a finding — trace it, don't tally it.

## Judge — classify every candidate

Apply the razor litmus and label each candidate exactly one of:

- **legit** — instruction selection / target-printing. Record which print-form axis.
- **Анализ в кодгене** — name-match or re-walk recovering a fact analysis owns.
- **Сырые факты** — several side-tables assembled where one verdict should exist.
- **Эмит-форма семантики** — the fact is in analysis but stored as a runtime-call form the backend just relays.

For each violation, name the **owed verdict**: the *domain answer* analysis must produce so the backend collapses to one `match`. It must pass the codegen-agnostic litmus — swap the runtime and the verdict is unchanged. A raw index or side-table keyed by node (`is_class_attr: bool`, a set of special names) is **not** an owed verdict: it only relocates the violation from **Анализ в кодгене** to **Сырые факты** / **Эмит-форма семантики** — the same triangle, moved one corner.

## Report

Per violation: `file:line`, the shape, the owed verdict, and the one-`match` backend shape after the fix.

This skill applies no fix. A finding is **never** closed by `#[allow]`, a suppression, an allowlist, or "match the surrounding code". Those are forbidden as resolutions — the neighbour breaking the principle is not precedent. A finding closes only by (a) re-homing the fact to analysis **as a domain verdict** — not as a raw index or emit-form, which merely pushes the violation around the triangle — or (b) an explicit human decision recorded in the report.

## The fix is a new task — gate it

Naming the owed verdict is review; *producing* it means editing `analyze`, a new layer whose PRD invariants govern where a verdict lives and how it is keyed. No green gate validates that model — tests stay green on a mis-keyed one (an element-level aggregate crammed into one variant, two unrelated verdicts merged for a shared key). So before writing it, invoke **`/required`** to route the analyze/attribute PRDs into context, and design before code. Key the verdict by the node it belongs to; a slot conflict means the key is wrong, not that you may merge.

## Completion criterion

Every backend decision-branch is traced to its input origin and labelled — verdict handle (`legit`) or source/AST re-derivation (one of the three shapes). The unit of coverage is the branch, not the seed hit: an exhausted seed list is not completion. If any in-scope backend file was not read through, the review is not done.
