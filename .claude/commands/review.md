# Architecture Review

Deep structural review of the svelte-rs compiler by 3 parallel agents (+1 optional for performance).

## Scope

Determined by `$ARGUMENTS`:
- **no arguments** → only files changed since master
- **`all`** → full codebase
- **`perf`** → changed files + 4th agent for performance analysis
- **`all perf`** → full codebase + performance agent

## Execution

**Read-only.** Agents may run `git diff` and read files. Do not run `cargo test`, `cargo check`, `cargo build`, or any command that modifies the working tree.

Launch agents immediately. Each agent gathers its own context as its first step — no sequential prefetch.

**Wait for every agent to report completion before writing the report.** Do not poll agent output files manually. Do not fall back to doing the review yourself. Do not start writing REVIEW.md until all agents have returned their results. If an agent takes longer than expected, wait — do not proceed without its findings.

For **default scope**, each agent starts with:
```
git diff master...HEAD --name-only
git diff --name-only
```
If both diffs are empty (no changes since master), stop and suggest running with `all` instead.

Then reads all changed `.rs` files + their direct parent `mod.rs` or `lib.rs` (one level up only).
All agents also read `CODEBASE_MAP.md` for type signatures, AnalysisData fields, and module structure — regardless of scope.

For **`all` scope**, each agent reads `CODEBASE_MAP.md` + `lib.rs` of each crate in `crates/`.

After all agents complete, compile their findings into a single report (see Output rules).

---

## Agent 1 — Data Flow

How information moves through the pipeline. Focus on **what data flows** and whether it arrives where it's needed in the right shape.

**1. Late knowledge**
Where does a downstream phase figure out something that an upstream phase already knew or could have known? Symptoms: pattern matching on raw data, re-parsing from source text, string comparisons that classify things, heuristics that reconstruct information the parser had. The cost: fragile logic, duplicated understanding, bugs when the reconstruction diverges from the original.

**2. Types that lie**
Where does the type system permit states that the data flow never actually produces? Options that are always Some after a certain phase. Vecs that are never empty. Enums where half the variants are impossible in a given context. Generic parameters used in only one way. The cost: every consumer handles impossible cases, obscuring the real data flow.

**3. Raw handoff**
Where does analysis collect data but not draw conclusions from it — leaving downstream phases to interpret raw facts? Symptom: codegen or transform contains `if`/`match` logic that decides *what something means* based on fields from AnalysisData, rather than reading a precomputed answer. A HashMap of raw facts exists, but the *business decision* ("this is a reactive binding", "this needs a teardown") is made in codegen instead of analysis. The cost: codegen accumulates domain logic that doesn't belong there, and the same interpretation may be needed in multiple places.

---

## Agent 2 — Simplicity

Whether the code is simpler than it could be. Focus on **reading experience** — can you understand each function top-to-bottom without jumping elsewhere?

**4. Cognitive complexity**
Code should read as a straight-line algorithm.

Find:
- *Unnecessary indirection* — a type, trait, or function that exists only to be used once. The reader must jump to its definition to understand the caller. Exception: if it enforces a phase boundary or compile-time constraint.
- *Deep nesting* — more than 3 levels of if/match/for. The reader maintains a mental stack of "which branch am I in".
- *Non-local data flow* — mutable variable declared far from where it's read. A struct field set in one place and read in another with no indication of when it becomes valid.
- *Abstraction that obscures* — when inlining a wrapper/helper/trait and writing plain code makes the logic easier to follow, the abstraction is not earning its cost.
- *Hidden sequencing* — steps that must happen in order but nothing enforces it. The fix: make ordering explicit through data dependencies (return value of step N is input of step N+1).

**5. Incidental logic**
Where does a condition or branch exist only because an earlier phase didn't make a clean decision? Symptom: an `if` that checks something the type system could guarantee, a match arm that handles a case impossible at this point in the pipeline, a flag that exists because two unrelated concerns were merged into one code path. The test: can you state what the branch is *for* in one sentence? If the answer is "it handles a case that shouldn't reach here" — the fix is upstream, not a better `if`. The cost: the reader must understand *why* the condition exists, not just *what* it checks, and the answer is often "historical accident".

**6. Dead weight**
Code that survived its context. Not "unreachable" (the compiler catches that) — but code that is technically reachable yet pointless. A function parameter every caller passes the same value. A struct field always equal to one value. A match arm that does the same as default. An enum variant never handled differently from its siblings. The cost: noise that slows down every future reader.

---

## Agent 3 — Boundaries

Whether decisions are made in the right place. Focus on **where logic lives** — not what data flows (that's Agent 1), but which phase owns which decision.

**7. Wrong phase, wrong abstraction level**
Where does a phase work at an abstraction level that isn't its job? Analysis stringifying things. Codegen re-deriving semantic information. Transform making decisions based on output format. Symptom: a function needs to import types from two non-adjacent phases. The cost: phases can't evolve independently.

**8. Scattered ownership**
Where is a single conceptual decision owned by nobody — instead, pieces of it live in 3+ places across different phases? Distinct from "wrong phase" (one thing in the wrong place) — this is about a decision that has no single home at all. Symptom: to add a new variant, you must touch 3+ files in lockstep with no type or contract guiding you. The cost: every new feature requires a scavenger hunt.

**9. Naming that misleads**
Not bad style — names that actively lie about what the code does. A function called `analyze_*` that also mutates. A variable called `attributes` holding filtered attributes. A type called `Context` that is really the entire codegen state. A module called `utils` that became a dumping ground. The cost: the reader builds a wrong mental model and carries it forward.

**10. Implicit coupling**
Where do two modules depend on the same assumption without it being encoded in a shared type or contract? Symptom: changing one module silently breaks another — not at compile time but at runtime or through wrong output. This is about *contracts between boundaries*, not data shape (that's Agent 1). The cost: invisible breakage that no test catches until production.

---

## Agent 4 — Performance (only with `perf` argument)

Algorithmic efficiency of the compiler pipeline.

**Redundant work**
- O(n²) patterns: nested loops over AST nodes, repeated HashMap lookups by NodeId that could be indexed by Vec
- Redundant tree walks: multiple passes that could be fused
- Allocations proportional to input size that could be avoided: building a Vec<String> just to join(), cloning data between phases that could be passed by reference

**Missing benchmarks**
For each finding, check if a criterion benchmark exists for that code path. If not, propose one.

For each finding:
- Pipeline phase (parse/analyze/transform/codegen)
- Current complexity estimate (O(n), O(n²), O(n·m))
- Concrete fix
- Expected impact (hot path on every input vs cold path on rare inputs)

---

## Output rules

**This command produces a report only. Never modify source files, branches, or commits.**

After all agents complete, merge their findings into one ranked report.

1. **Deduplicate**: if two agents flagged the same code location, merge into one finding — pick the dimension that best describes the root cause
2. **Rank** by: how many downstream workarounds or unnecessary checks would disappear if the root cause were fixed
3. **Include all findings** — no limit
4. Skip: clippy-level issues, formatting, import ordering, style preferences

For each finding:

```
### #N — [One-line title]

**Dimension**: [which of the 10 dimensions]
**Severity**: critical | warning | suggestion
**Evidence**:
- `file_path:line` — what happens here
- `file_path:line` — what happens here

**Problem**: [What structural mismatch exists — 2-3 sentences]

**Fix**: [Describe the new shape of data or phase boundary — not a code patch, a design change]

**Simplifies**: [What becomes unnecessary once this is fixed — list the downstream code that gets simpler or deleted]
```

**Save the complete report to `REVIEW.md` in the project root (overwrite if exists). This file is read by `/review-fix`.**

In chat, output only a summary:
- Total findings count by severity (critical / warning / suggestion)
- Total findings count by agent
- Top 3 titles with their #N for quick `/review-fix` reference
- "Full report saved to REVIEW.md"
