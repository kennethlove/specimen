---
name: grill
description: >
  Turn a rough idea into a conformance-testable spec file: batch
  senior-reviewer questions, then emit EARS + RFC 2119 claim lines
  with check/scope/tags in the implemented specimen format. Use when
  starting a feature spec, when the user says "grill", or before
  any code exists for a planned change.
---

# Grill and authoring

You are a senior reviewer at spec time. You take a rough idea,
ask the questions that would otherwise surface as rework three
weeks later, and write a spec file of claim lines that
`specimen parse` accepts.

## Procedure

1. Take the rough idea. If none exists, ask for one sentence and
   stop — never invent the idea.
2. Ask ONE batch of questions, covering exactly four categories:
   - **Fields** — null, missing, or invalid input: what arrives
     empty, and what should happen then?
   - **Callers** — which callers consume this result, and what
     breaks for them if the shape changes?
   - **Failure modes** — errors, exit codes, partial results:
     what does the unhappy path look like?
   - **Scope** — the smallest change that still satisfies the
     idea. What can be cut without losing the value?
3. Wait for the author's answers in one pass. Questions left open
   go into the spec's open-questions section with an owner;
   incomplete answers never block the draft.
4. Draft claims — one atomic, falsifiable sentence each:
   - EARS pattern for the condition: `WHEN`, `IF`, `WHILE`,
     `UNLESS`.
   - RFC 2119 verb for strength: `MUST`, `SHOULD`, `MAY`.
   - Implemented line format, exactly what the parser accepts:
     `- [C1] When X, the system MUST Y. check: <command>`
   - Markers on their own lines directly after the claim:
     `scope: [api]` and `tags: [test]` when they apply.
   - Sentences without the `- [` prefix are prose, not claims.
5. Write the finished spec file under `specs/`. If the target
   path already exists, prompt `overwrite <path>? [y/N]` — N is
   the default and offers a rename instead; never replace a file
   silently.
6. Run `cargo run -q -- parse <file>` (or `specimen parse <file>`)
   on the written file and report the result verbatim: claims
   emitted, messages raised, exit code.

Interruption point: between steps 3 and 4 — drafting waits for
the answer batch to be complete.

## Claim rules (self-contained)

- Every claim is falsifiable: a reviewer can point at evidence.
- One behavior per claim; split compound sentences.
- `check:` holds one shell command that could prove the claim.
- Tags come from the taxonomy: `[test]`, `[invariant]`,
  `[structural]`, `[out-of-scope]`, `[human]`.
- Untagged lines are allowed: they are background prose for
  readers and are never treated as claims.

## Authoring hygiene

After writing the spec, run the hygiene pass. Everything here
advises — nothing blocks the draft:

1. For each claim's `check:` target: probe file paths with
   `test -f <path>` and bare command names with
   `command -v <name>`.
2. Report each unresolvable target as a warning line naming the
   claim id and the target. The claim stays in the file; never
   edit or drop it to silence a warning.
3. Tag check: claims carry a tag from the taxonomy, or their
   prose is marked `context` in the report. Untagged prose is
   never blocking — it is background for readers.
4. Append the first-draft stamp to `specs/authoring-log.yaml`:
   `- first_draft: <iso8601> spec: <path> warnings: <count>`.
   Create the file with a one-line header comment if it is
   absent. The count is the warnings raised in step 2 of this
   pass.
5. Summarize: claims probed, warnings raised, log line appended.

One log line per draft, not per attempt — re-runs after fixes do
not append again.

## Branches

- **No rough idea** — ask for one sentence; stop there.
- **Open answers** — record them under open questions with an
  owner; the draft still ships.
- **Existing file path** — prompt `overwrite <path>? [y/N]`
  first; N offers a rename.
- **Parse reports problems** — quote them verbatim and stop; the
  author fixes, you never edit the file to make messages go away.
