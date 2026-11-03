# Ubiquitous Language

Domain: spec-conformance tooling (specimen repo). Source: brainstorm
`docs/brainstorm/spec-tool.md` + requirements `docs/brainstorm/spec-tool-requirements.md`.

## Spec content

| Term            | Definition                                                                  | Aliases to avoid        |
| --------------- | --------------------------------------------------------------------------- | ----------------------- |
| **Spec**        | A plain markdown file of claims plus untagged context prose                 | requirements doc, PRD   |
| **Claim**       | One atomic falsifiable proposition in a spec                                | requirement, story      |
| **Claim ID**    | Stable identifier (e.g. `C4`) tracing a claim to code and hunks             | requirement number      |
| **Context prose** | Untagged spec text that informs readers but never blocks conformance      | background, notes       |
| **Fit**         | The criterion naming what proves a claim true                               | acceptance criterion    |
| **Check**       | Arbitrary shell command bound to a claim that exits pass or fail            | validator, assertion    |
| **Scope line**  | A declared boundary attached to a claim; code inside it fails conformance   | constraint, guard       |
| **Tag**         | A claim's check-type classification: `[test]`, `[invariant]`, `[structural]`, `[out-of-scope]`, `[human]` | label, category |
| **Human claim** | A claim whose verdict needs judgment and cannot be automated                | TODO, judgment call     |

## Conformance and review

| Term                 | Definition                                                                 | Aliases to avoid     |
| -------------------- | -------------------------------------------------------------------------- | -------------------- |
| **Conformance**      | The state of a diff satisfying a spec's claims, established by checks and hunk mapping | compliance     |
| **Claims JSON**      | Machine-readable `parse` output: one record per claim (id, sentence, check, scope, tags) | parse output |
| **Hunk**             | A contiguous block of changed lines in a git diff                          | change, patch        |
| **Orphan hunk**      | A hunk that maps to no claim, flagged before human review                  | unmatched change     |
| **Conformance report** | The build-2 report of claim coverage, pass/fail results, and orphan hunks  | review report        |
| **Coverage**         | The share of hunks mapped to claim IDs                                     | traceability         |
| **Grill**            | The senior-reviewer question pass that turns a rough idea into claims      | interview, Q&A       |
| **Review volume**    | The amount of line-by-line diff a human must read                          | review effort        |

## Measurement

| Term              | Definition                                                     | Aliases to avoid |
| ----------------- | -------------------------------------------------------------- | ---------------- |
| **Authoring time** | Interval from a spec's first draft to approval, logged by build 3 | cycle time      |

## Relationships

- A **Spec** contains **Claims** and **Context prose**; prose never blocks.
- A **Claim** has exactly one **Claim ID**, zero or one **Check**, zero or more **Scope lines**, zero or more **Tags**.
- A **Fit** states what proves a claim; a **Check** automates that proof when possible.
- A **Hunk** maps to zero or one **Claim**; zero mappings makes it an **Orphan hunk**.
- The **Conformance report** combines **Check** results with hunk mapping to cut **Review volume**.
- A **Grill** produces **Claims** from a rough idea; **Human claims** are few by design.

## Example dialogue

> **Dev:** "This diff adds a second email-verification field. Which **Claim** covers it?"
>
> **Domain expert:** "None — that's an **Orphan hunk**. The **Spec** puts verification in a **Scope line**, so flag it before I review."
>
> **Dev:** "**C4** has a **Check** on the 409 path. Does it also need a **Fit**?"
>
> **Domain expert:** "**Fit** is the prose criterion you read; **Check** is the shell command that proves it. Every claim needs a **Fit**; only automatable claims get a **Check**."
>
> **Dev:** "So the **Conformance report** tells you what to read, and I stop sending you full diffs?"
>
> **Domain expert:** "Right — report plus the **Human claims**. That's the whole review."

## Flagged ambiguities

- **check** — three meanings in circulation: the `check:` field (a shell command), the `specimen check` subcommand, and the act of verifying. Recommendation: **Check** = the command bound to a claim; `specimen check` stays verbatim tool syntax; the act = "run checks".
- **scope** — brainstorm has a `scope:` line; the tag taxonomy has `[out-of-scope]`. Probably one concept (declared negative space) in two encodings. Recommendation: treat as one concept; freeze the encoding after the first real spec exercises it.
- **requirement** — appears in source standards (Volere, ISO 29148) but is not the atom here. Canonical term is **Claim**; "requirement" only when citing a standard.
