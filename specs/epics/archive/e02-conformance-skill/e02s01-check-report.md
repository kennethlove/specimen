<!-- story: e02s01 -->

STORY KEY: e02s01
TITLE:     Check report
TYPE:      Story
PARENT:    e02
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-03
MATURITY:  3
SIZE:      XS

### 1. Business narrative [draft]

The checker CLI produces JSON, which machines read but reviewers do
not. The conformance skill exists so any coding agent can run the
checker against a spec and hand the human a plain pass/fail listing
per claim. This story creates the skill file itself and its first
procedure: check and list.

The skill lives in this repository so it travels with the project
and stays agent-agnostic.

### 2. Value statement [draft]

As a reviewer, I want a per-claim pass/fail listing produced by the
conformance skill, so that I can read check outcomes without
parsing JSON.

### 3. Actors and permissions [draft]

- Coding agent (system) — follows the skill procedure.
- Reviewer (internal) — reads the listing.

### 4. Trigger and preconditions [draft]

Trigger: an agent loads skills/conformance/SKILL.md and runs its
check procedure on a spec path.
Precondition: the specimen CLI builds and runs (`cargo run -q`);
the spec file exists.

### 5. Main flow and business logic [draft]

1. The agent reads skills/conformance/SKILL.md.
2. It runs `cargo run -q -- check <spec path>`.
3. It reads the JSON results from stdout.
4. It renders one line per claim: claim id, status, and the check
   string for failures.
5. It presents the listing to the reviewer with the process exit
   code noted.

Interruption point: between steps 2 and 4 — the agent may pause
after the run and resume from stored JSON.

### 6. Alternative flows and exceptions [draft]

6a. Any status is `fail` — the listing marks FAIL lines first and
states the exit code was 1.
6b. Any status is `skipped` — the listing marks SKIPPED lines and
notes no check was bound.
6c. The CLI does not build or run — the skill instructs running
`cargo build` first and reports the build error verbatim.
6d. The spec path is missing or unreadable — the skill asks the
reviewer for a path instead of guessing one.

### 7. Interface elements [draft]

Context: new
Static elements: skill procedure steps, listing format
Dynamic elements: pass/fail/skipped lines, exit code note

### 8. Domain model [draft]

Entities touched: Claim, CheckResult (e01).
Entities created: none — the skill renders existing data.
Relationships: none added.

### 9. Integrations and boundaries [draft]

- specimen CLI (perennial, direction: out) — invoked via
  `cargo run -q`.
- Spec markdown file (perennial, direction: in) — path from the
  reviewer.
- skills/conformance/SKILL.md (perennial, direction: in) — the
  procedure itself.

### 10. Background processes [draft]

Not applicable — the skill runs one foreground command.

### 11. Notifications [draft]

Not applicable — output is delivered in the agent conversation.

### 12. Audit and logging [draft]

Audited entity: skill run. Fields: spec path, exit code, listing
produced. Recorded in the conversation transcript only.

### 13. Solution variabilities [draft]

Not applicable — one procedure, one listing format.

### 14. Quality attributes *NFR* [draft]

- Listing renders within 5 s of the check command finishing.
- One output line per claim; no claim omitted from the listing.

### 15. Security and compliance *NFR* [draft]

- AuthN: none.
- AuthZ: repository read access; check strings run with agent
  privileges.
- Data class: none.
- Controls: the skill never edits the spec file.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — text listing in an agent conversation.
- i18n: English only.
- Wording uses the project's canonical terms (claim, check, pass,
  fail, skipped).

### 17. Acceptance criteria [draft]

```gherkin
Scenario: All claims pass
  Given a fixture spec whose checks all pass
  When  an agent runs the skill's check procedure
  Then  the listing shows a PASS line for every claim
  And   the exit code 0 is noted

Scenario: A check fails (6a)
  Given a fixture spec with one failing check
  When  an agent runs the skill's check procedure
  Then  a FAIL line appears for that claim with its check string
  And   the exit code 1 is noted

Scenario: A claim has no check (6b)
  Given a fixture spec with one claim lacking a check
  When  an agent runs the skill's check procedure
  Then  a SKIPPED line appears for that claim
  And   other claims still show their own status

Scenario: CLI will not build (6c)
  Given a repository where cargo build fails
  When  an agent runs the skill's check procedure
  Then  the skill instructs cargo build and reports the build
    error verbatim

Scenario: Spec path missing (6d)
  Given no spec path is available
  When  an agent runs the skill's check procedure
  Then  the skill asks the reviewer for a path
  And   no command runs against a guessed path
```

### 18. Out of scope [draft]

- Hunk mapping (e02s02) and the full report (e02s03).
- Fixing failing checks.
- Storing run history across sessions.

### 19. Open questions [draft]

- Listing format (markdown table vs one line per claim) — owner:
  kenneth, needed by: first real review cycle.

### 20. References [draft]

- docs/brainstorm/spec-tool-requirements.md (skill procedure).
- specs/epics/e01-checker-cli/e01s02-run-checks.md (result JSON).
- specs/UBIQUITOUS_LANGUAGE_LATEST.md (Conformance, Check).
