<!-- story: e01s01 -->

STORY KEY: e01s01
TITLE:     Parse claims to JSON
TYPE:      Story
PARENT:    e01
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-03
MATURITY:  3
SIZE:      S

### 1. Business narrative [draft]

Specs are written as plain markdown. Reviewers cannot grep them
reliably, and no tool can check whether each stated requirement is
enforced. The team needs every claim in a spec lifted into a stable
machine-readable form so later steps can run checks against claims
and map diff hunks to claim IDs.

The checker CLI is the first deliverable of the MVP. It turns one
markdown file into a JSON array of claims on stdout, with diagnostics
on stderr. Nothing else in the MVP works without this translation.

### 2. Value statement [draft]

As a reviewer, I want claims extracted from a spec as JSON, so that
tools can check and map them without re-parsing markdown.

### 3. Actors and permissions [draft]

- Reviewer (internal) — runs the CLI on a local spec file.
- Conformance skill (system) — consumes the JSON on stdout.

### 4. Trigger and preconditions [draft]

Trigger: a user runs `specimen parse <path>`.
Precondition: the file exists and is readable; the working directory
is the repository root.

### 5. Main flow and business logic [draft]

1. The CLI reads the file at `<path>`.
2. It scans lines in document order.
3. A claim line starts with `- [` and closes the bracket, then
   carries a sentence and an optional `check:` field.
4. For each claim line it builds an object: `id` (bracket text),
   `sentence` (text after the bracket up to `check:`), `check` (text
   after `check:` to the end of the line), `scope` (array, empty when
   absent), `tags` (array, empty when absent).
5. It prints a JSON array of those objects to stdout.
6. It exits 0.

Claim line example:

```text
- [C1] The CLI SHALL exit non-zero on failure. check: cargo test
```

Interruption point: N/A — single-shot command, no resumable state.

### 6. Alternative flows and exceptions [draft]

6a. A claim line has no `check:` field — emit the claim with an
empty `check` string, print a warning to stderr naming file and
line number, exit 0.
6b. A line is not a claim line (prose, heading, code fence) — skip
it silently, exit 0.
6c. Two claim lines carry the same `id` — print an error to stderr
naming both line numbers and exit 1 without JSON output.
6d. The file does not exist — print an error to stderr, exit 1.

### 7. Interface elements [draft]

Context: new
Static elements: CLI usage line, stdout JSON contract
Dynamic elements: stderr diagnostics, exit code

### 8. Domain model [draft]

Entities touched: none (first story).
Entities created: Claim (id, sentence, check, scope, tags).
Relationships: none yet.

### 9. Integrations and boundaries [draft]

- Spec markdown file (perennial, direction: in) — path argument.
- stdout JSON (perennial, direction: out) — consumed by e01s02 and
  the e02 conformance skill.

### 10. Background processes [draft]

Not applicable — the CLI is a single foreground process.

### 11. Notifications [draft]

Not applicable — no notification channels; diagnostics go to stderr.

### 12. Audit and logging [draft]

Audited entity: parse run. Fields: file path, line number of each
warning or error, exit code. Written to stderr only.

### 13. Solution variabilities [draft]

Not applicable — no configuration parameters in this story.

### 14. Quality attributes *NFR* [draft]

- Parse completes in under 100 ms for a 1000-line spec file.
- Output JSON validates against serde_json round-trip with zero
  rejections across the fixture set.

### 15. Security and compliance *NFR* [draft]

- AuthN: none — local CLI, no network access.
- AuthZ: filesystem permissions of the invoking user.
- Data class: none — spec text is not personal data.
- Controls: no shell execution in this story.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — CLI, no graphical interface.
- i18n: stderr messages in English only.
- Machine contract: stdout is JSON only; humans read stderr.

### 17. Acceptance criteria [draft]

```gherkin
Scenario: Parse a complete claim line
  Given a spec file with a claim line carrying id, sentence,
    and check
  When  the user runs specimen parse on that file
  Then  stdout is a JSON array with one object whose id,
    sentence, and check fields match the line
  And   the exit code is 0

Scenario: Claim without a check field (6a)
  Given a spec file with a claim line that has no check field
  When  the user runs specimen parse on that file
  Then  the claim appears in the JSON array with an empty check
  And   a warning naming file and line is printed to stderr
  And   the exit code is 0

Scenario: Prose lines are not claims (6b)
  Given a spec file containing headings, prose, and one claim
    line
  When  the user runs specimen parse on that file
  Then  only the claim line appears in the JSON array
  And   the exit code is 0

Scenario: Duplicate claim id (6c)
  Given a spec file where two claim lines share one id
  When  the user runs specimen parse on that file
  Then  no JSON is written to stdout
  And   an error naming both line numbers is printed to stderr
  And   the exit code is 1

Scenario: Missing file (6d)
  Given a path that does not exist
  When  the user runs specimen parse on that path
  Then  an error is printed to stderr
  And   the exit code is 1
```

### 18. Out of scope [draft]

- Running check commands (covered by e01s02).
- Scope and tag list parsing rules beyond empty defaults (e01s03).
- Diff hunk mapping (e02).
- Claim authoring help or grilling (deferred build 3).

### 19. Open questions [draft]

- Claim-line format may be revised after the first real spec is
  parsed — owner: kenneth, needed by: first review cycle.

### 20. References [draft]

- docs/brainstorm/spec-tool-requirements.md (MVP requirements).
- specs/release-plan.yaml (e01 sequencing).
- specs/UBIQUITOUS_LANGUAGE_LATEST.md (Claim, Claims JSON).
