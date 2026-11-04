<!-- story: e04s01 -->

STORY KEY: e04s01
TITLE:     Render static dashboard
TYPE:      Story
PARENT:    e04
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-04
MATURITY:  3
SIZE:      M

### 1. Business narrative [draft]

Reviewers read conformance results as terminal prose today. The
dashboard turns one command into a static HTML page a human can
open, skim, and share: the project, every claim, pass or fail,
and summary counts — no server, no process. It is the visible
half of conformance: the checker proves claims, the dashboard
makes the proof legible.

### 2. Value statement [draft]

As a reviewer, I want a static page of claim results, so that I
can judge a project's conformance without reading a diff or a
terminal scrollback.

### 3. Actors and permissions [draft]

- Reviewer (internal) — opens the generated `index.html`.
- Coding agent (system) — runs the command, writes the file.
- Author (internal) — owns the spec consumed by the run.

### 4. Trigger and preconditions [draft]

Trigger: `specimen dashboard <spec.md> <out-dir>` is run.
Preconditions: the repo builds (`cargo run -q`); the spec file
exists; `<out-dir>` is writable (created if absent).

### 5. Main flow and business logic [draft]

1. Parse `<spec.md>` with the existing claim parser; empty claim
   set is a distinct state (e04s02), not an error here.
2. Run every claim check sequentially in document order via the
   existing check runner; per-claim failures are results, not
   command failures.
3. Determine the project name from the spec file stem (the
   directory the spec lives in is the project).
4. Render `<out-dir>/index.html` as a single static file:
   project name in the heading; one row per claim showing claim
   id, full sentence (C10 — never truncated), check command, and
   pass/fail status; a summary counts line with total, passed,
   failed.
5. Exit 0 when the page is written, regardless of per-claim
   pass/fail — statuses are content, not exit state.

Interruption point: none — one command, one run.

### 6. Alternative flows and exceptions [draft]

6a. Spec file missing or unreadable — print
    `error: cannot read <path>: <cause>` to stderr, exit 1, no
    page written (the failure page is e04s02's territory: a spec
    that parses but whose checks error still renders).
6b. `<out-dir>` does not exist — create it (including parents),
    then write.
6c. Checks that fail — statuses render as failed rows; the run
    still exits 0 (C3 content, not command failure).
6d. Wrong argument count — usage line to stderr, exit 2.

### 7. Interface elements [draft]

Context: new
Static elements: heading (project name), claim rows, summary
counts line, status badges
Dynamic elements: pass/fail per claim, counts

### 8. Domain model [draft]

Entities touched: Spec, Claim, Check, Coverage (glossary).
Entities created: Dashboard page (rendered view over claims +
results), Summary counts (derived: total/passed/failed).
Relationships: one page per run; rows are 1:1 with claims.

### 9. Integrations and boundaries [draft]

- Claim parser (perennial, direction: out) — `parse_claims`.
- Check runner (perennial, direction: out) — sequential `sh -c`.
- Filesystem (perennial, direction: out) — `<out-dir>/index.html`.

### 10. Background processes [draft]

Not applicable — synchronous command.

### 11. Notifications [draft]

Not applicable — stdout one-line summary, file is the output.

### 12. Audit and logging [draft]

Audited entity: run record. Fields: spec path, out dir, counts,
exit code. Recorded: stdout one line (`wrote <path>: <n> claims
<t> passed <f> failed`).

### 13. Solution variabilities [draft]

- Template shape (HTML string in code) may move to external
  files later; rendering contract (one static file) stays.
- Additional counts (human, skipped) are additive.

### 14. Quality attributes *NFR* [draft]

- Single command, single run, page opens by double-click —
  static, no network (C7).
- Fail loudly: unreadable spec is an error, never a silent
  empty page (author's answer 2).

### 15. Security and compliance *NFR* [draft]

- AuthN: none.
- AuthZ: filesystem write limited to `<out-dir>`.
- Data class: none — spec content is repo-local.
- Controls: claim text HTML-escaped when rendered (a claim
  containing `<script>` must not execute).

### 16. UX and accessibility *NFR* [draft]

- Not applicable — this story emits structure; visual states are
  e04s02.
- i18n: English only.
- Rows readable as text even without CSS (semantic HTML).

### 17. Acceptance criteria [draft]

```gherkin
Scenario: Happy path render
  Given a spec with passing and failing claims
  When  `specimen dashboard <spec> <out>` runs
  Then  <out>/index.html exists
  And   it contains the project name, every claim sentence,
    and a pass or fail status per claim
  And   the summary line lists total, passed, failed counts
  And   the command exits 0

Scenario: Missing spec (6a)
  Given a nonexistent spec path
  When  the command runs
  Then  stderr reports `cannot read`
  And   no index.html is written
  And   the exit code is 1

Scenario: Out dir absent (6b)
  Given a target directory that does not exist
  When  the command runs
  Then  the directory tree is created and the page written

Scenario: Some checks fail (6c)
  Given claims whose checks fail
  When  the command runs
  Then  those rows render as failed
  And   the exit code is 0

Scenario: Wrong arguments (6d)
  Given no arguments
  When  the command runs
  Then  usage prints to stderr
  And   the exit code is 2
```

### 18. Out of scope [draft]

- Empty state, failure page, human highlighting (e04s02).
- Config-driven fields (e04s03).
- Agent JSON interface (post-MVP, agreed).
- Server, live reload — C7 forbids.

### 19. Open questions [draft]

- Summary counts line wording/format — owner: kenneth, needed
  by: first render review.

### 20. References [draft]

- specs/product/conformance-dashboard.md (C1, C2, C3, C7, C9,
  C10, C11).
- specs/epics/e01-checker-cli/ (parser + runner reuse).
