# User-facing language (shared by all PARITY skills)

Single source of truth for how PARITY skills (`/dig`) talk **to the user**. Edit here once — every skill that points at this file inherits the change.

## The rule

Everything the user reads — gate, brief, questions, decisions, sections — is in **domain and behavior terms**: Svelte features (`$state`, `$props`, `$effect`, `$derived`, bindings, directives), pipeline stages (parse / analyze / transform / codegen), and what the emitted JS does at runtime.

Internal vocabulary stays in **your own reasoning only**. It is **banned from any text the user reads** — translate it first:

| Banned in user-facing text | Say instead |
|---|---|
| carrier / consumer | "the data that records X" / "the code that reads it" — or just name the behavior |
| Quick fix / Need research | "scope is clear, ready to fix" / "new machinery — settle the architecture first" |
| fork-set / probes / red-all | "a test case per behavior of the Original" |
| green guard | "cases that must not break" |
| deep-modules | name each unit by what it does |
| blast radius | "what changes and where (which crates / data structures)" |
| HITL / AFK | "needs your call" / "mechanical, can run unattended" |
| slicing | "how we split the work" |
| tracer-bullet | "a first thin end-to-end slice" |
| leaf / lowering / shape / dyn | banlisted (CLAUDE.md `## Never use`) — rephrase entirely |

## Two hard constraints

1. **Function and type names appear only inside a pasted code snippet — never as the subject of a sentence.** The user reconstructs the system from behavior, not from identifiers. A sentence they can't parse without opening the source is a failed sentence. "We leave `$state` destructuring untouched" ✅ — "`dispatch_identifier_assignment` returns false" ❌.

2. **The internal terms above are fine in your private reasoning** (and in skill `examples/` that document your method). The ban is strictly on what you *show* the user.
