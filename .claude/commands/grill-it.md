---
description: Challenge recent work against the quality bar. Use after implementation work to verify no shortcuts were taken.
argument-hint: "[session | diff | commit | branch | file-path]"
---

# Grill it: $ARGUMENTS

Interpret `$ARGUMENTS`:

- **empty or `session`** — grill everything done in the current conversation
- **`diff`** — grill uncommitted changes
- **`commit`** — grill the last commit (`HEAD~1..HEAD`)
- **`branch`** — grill all changes on the current branch vs master (`git diff master...HEAD`)
- **file path** — grill that specific file

Review as a skeptical senior compiler engineer.

Focus on material issues only.
Be concrete and evidence-based.
Do not invent issues just to be critical.
Prefer a few strong observations over broad commentary.

Pay special attention to compiler phase ownership, data flow, and long-term maintainability.
Prefer clear parser/analyze/transform/codegen boundaries over clever local fixes.

Answer these questions:

- Are these changes systematic, without workarounds or temporary solutions, respecting crate and module boundaries?
- Will these changes age well, or create long-term problems in the project?
- Did this change duplicate or reinvent something that should have been extended instead?
- Will another person understand this without additional explanation?
- Is the implementation straightforward, or more complex than necessary?

Rules:

- Do not report style nitpicks.
- Do not force findings when nothing serious stands out.
- Prefer pointing out the most suspicious or highest-risk parts over trying to review everything equally.
- Call out shortcuts, boundary violations, duplicated logic, hidden assumptions, and workaround-shaped code directly.
- If nothing serious stands out, say so plainly.