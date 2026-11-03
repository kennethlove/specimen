<!-- story: e01s02 -->

STORY KEY: e01s02
TITLE:     Run checks
TYPE:      Story
PARENT:    e01
STATUS:    Draft
AUTHOR:    kenneth           DATE: 2026-11-03
MATURITY:  3
SIZE:      XS

### 1. Business narrative [draft]

A claim with a check field only helps if something executes it.
Reviewers want one command that runs every check bound to a spec and
reports, per claim, whether the check passed. The result must be
machine-readable so the e02 conformance skill can fold it into a
report, and the process exit code must be honest so CI can gate on it.

Checks are plain shell strings. The CLI runs them in order, records
each exit status, and never stops at the first failure — a reviewer
wants the full picture in one run.

### 2. Value statement [draft]

As a reviewer, I want every claim check executed in document order,
so that I can see pass or fail per claim from one command.

### 3. Actors and permissions [draft]

- Reviewer (internal) — runs the CLI, reads the report.
- CI pipeline (system) — gates on the process exit code.

### 4. Trigger and preconditions [draft]

Trigger: a user runs `specimen check <path>`.
Precondition: the file parses as claims (e01s01); a shell (`sh`) is
available on PATH.

### 5. Main flow and business logic [draft]

1. The CLI parses `<path>` into claims (same rules as e01s01).
2. It iterates claims in document order.
3. For each claim with a non-empty check it runs `sh -c <check>`
   and records the child exit code.
4. It builds one result object per claim: `id`, `status`
   (`pass`, `fail`, or `skipped`), `exit_code` (null when skipped).
5. It prints the JSON array of results to stdout.
6. It exits 0 when no claim failed, 1 when at least one failed.

Exit codes: 0 = all pass or skipped; 1 = at least one fail;
2 = usage or input error.

### 6. Alternative flows and exceptions [draft]

6a. A check command exits non-zero — status `fail`, remaining
claims still run, process exits 1.
6b. A claim has an empty check — status `skipped`, no process
spawned, warning to stderr, exit code unaffected.
6c. The check string cannot start a shell — status `fail` with
exit_code 127, process continues and exits 1.
6d. The input file does not parse — error to stderr, exit 2, no
JSON output.

### 7. Interface elements [draft]

Context: new
Static elements: CLI usage line, JSON result contract
Dynamic elements: stderr warnings, process exit code

### 8. Domain model [draft]

Entities touched: Claim (e01s01).
Entities created: CheckResult (id, status, exit_code).
Relationships: CheckResult belongs to exactly one Claim.

### 9. Integrations and boundaries [draft]

- Spec markdown file (perennial, direction: in) — path argument.
- Local shell `sh -c` (perennial, direction: out) — one child
  process per checked claim, sequential.

### 10. Background processes [draft]

Not applicable — all child processes run inside the foreground
invocation.

### 11. Notifications [draft]

Not applicable — no notification channels; stderr carries warnings.

### 12. Audit and logging [draft]

Audited entity: check run. Fields: claim id, check string, child
exit code, status. Emitted as JSON on stdout; summary line on
stderr.

### 13. Solution variabilities [draft]

Not applicable — no configuration parameters in this story.

### 14. Quality attributes *NFR* [draft]

- A 50-claim spec with instant checks completes in under 2 s.
- Claims run strictly sequentially; no parallel process spawn.

### 15. Security and compliance *NFR* [draft]

- AuthN: none — local CLI, no network access.
- AuthZ: filesystem permissions of the invoking user.
- Data class: none.
- Controls: check strings execute with the caller's privileges;
  specs are trusted input, same trust level as CI scripts.

### 16. UX and accessibility *NFR* [draft]

- Not applicable — CLI, no graphical interface.
- i18n: stderr messages in English only.
- Machine contract: stdout is JSON only; humans read stderr.

### 17. Acceptance criteria [draft]

```gherkin
Scenario: All checks pass
  Given a spec file whose claims all carry passing checks
  When  the user runs specimen check on that file
  Then  stdout is a JSON array with status pass for every claim
  And   the exit code is 0

Scenario: One check fails (6a)
  Given a spec file where one claim check exits non-zero
  When  the user runs specimen check on that file
  Then  that claim has status fail with its exit code recorded
  And   every other claim still has a result
  And   the exit code is 1

Scenario: Claim without check is skipped (6b)
  Given a spec file with one claim that has no check field
  When  the user runs specimen check on that file
  Then  that claim has status skipped and exit_code null
  And   a warning is printed to stderr
  And   the exit code is 0 when all other checks pass

Scenario: Check string cannot start (6c)
  Given a claim whose check references a command that does not
    exist
  When  the user runs specimen check on that file
  Then  that claim has status fail with exit_code 127
  And   the exit code is 1

Scenario: Input does not parse (6d)
  Given a path that does not exist
  When  the user runs specimen check on that path
  Then  no JSON is written to stdout
  And   an error is printed to stderr
  And   the exit code is 2
```

### 18. Out of scope [draft]

- Mapping diff hunks to claims (e02).
- Parallel or timeout handling for slow checks.
- Caching check results between runs.

### 19. Open questions [draft]

- Check timeout policy for long-running checks — owner: kenneth,
  needed by: first review cycle with real specs.

### 20. References [draft]

- docs/brainstorm/spec-tool-requirements.md (check requirements).
- specs/epics/e01-checker-cli/e01s01-parse-claims.md (parse rules).
- specs/UBIQUITOUS_LANGUAGE_LATEST.md (Check, pass, fail).
